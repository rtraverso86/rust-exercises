use std::collections::HashMap;
use std::io::Cursor;
use std::sync::{Arc, Mutex};

use bytes::{Buf, Bytes, BytesMut};
use tokio::io::{self, AsyncReadExt, AsyncWriteExt, BufWriter};
use tokio::net::{TcpListener, TcpStream};
use mini_redis::{Frame, Result};
use mini_redis::frame::Error::Incomplete;

type Db = Arc<Mutex<HashMap<String, Bytes>>>;

struct Connection {
    stream: BufWriter<TcpStream>,
    buffer: BytesMut,
}

impl Connection {
    pub fn new(stream: TcpStream) -> Connection {
        Connection {
            stream: BufWriter::new(stream),
            // Allocate the buffer with 4kb of capacity.
            buffer: BytesMut::with_capacity(4096),
        }
    }

    /// Read a frame from the connection.
    /// 
    /// Returns `None` if EOF is reached
    pub async fn read_frame(&mut self) -> Result<Option<Frame>>
    {
        loop {
            // Attempt to parse a frame from the buffered data. If enough data has
            // been buffered, the frame is returned.
            if let Some(frame) = self.parse_frame()? {
                return Ok(Some(frame));
            }
    
            // There is not enough buffered data to read a frame. Attempt to read more
            // data from the socket.
            //
            // On success, the number of bytes is returned. `0` indicates "end of stream".
            if 0 == self.stream.read_buf(&mut self.buffer).await? {
                // The remote closed the connection. For this to be  a clean shutdown,
                // there should be no data in the read buffer. If there is, this means
                // that the peer closed the socket while sending a frame.
                if self.buffer.is_empty() {
                    return Ok(None);
                } else {
                    return Err("connection reset by peer".into());
                }
            }
        }
    }

    fn parse_frame(&mut self) -> Result<Option<Frame>> {
        // Create the `T: Buf` type.
        let mut buf = Cursor::new(&self.buffer[..]);

        // Check whether a full frame is available. After a successful call,
        // buf.position() will point one byte ahead of the frame end.
        match Frame::check(&mut buf) {
            Ok(_) => {
                // Get the byte length of the frame
                let len = buf.position() as usize;

                // Reset the internal cursor for the call to `parse`.
                buf.set_position(0);

                // Parse the frame
                let frame = Frame::parse(&mut buf)?;

                // Discard the frame from the buffer
                self.buffer.advance(len);

                // Return the frame to the caller.
                Ok(Some(frame))
            }
            // Not enough data has been buffered
            Err(Incomplete) => Ok(None),
            // An error was encountered
            Err(e) => Err(e.into()),
        }
    }

    /// Write a frame to the connection.
    pub async fn write_frame(&mut self, frame: &Frame) -> Result<()>
    {
        match frame {
            Frame::Simple(val) => {
                self.stream.write_u8(b'+').await?;
                self.stream.write_all(val.as_bytes()).await?;
                self.stream.write_all(b"\r\n").await?;
            }
            Frame::Error(val) => {
                self.stream.write_u8(b'-').await?;
                self.stream.write_all(val.as_bytes()).await?;
                self.stream.write_all(b"\r\n").await?;
            }
            Frame::Integer(val) => {
                self.stream.write_u8(b':').await?;
                self.write_decimal(*val).await?;
            }
            Frame::Null => {
                self.stream.write_all(b"$-1\r\n").await?;
            }
            Frame::Bulk(val) => {
                let len = val.len();
    
                self.stream.write_u8(b'$').await?;
                self.write_decimal(len as u64).await?;
                self.stream.write_all(val).await?;
                self.stream.write_all(b"\r\n").await?;
            }
            Frame::Array(_val) => unimplemented!(),
        }
    
        self.stream.flush().await?;
    
        Ok(())
    }

    async fn write_decimal(&mut self, val: u64) -> io::Result<()> {
        use std::io::Write;

        // Convert the value to a string
        let mut buf = [0u8; 12];
        let mut buf = Cursor::new(&mut buf[..]);
        write!(&mut buf, "{}", val)?;

        let pos = buf.position() as usize;
        self.stream.write_all(&buf.get_ref()[..pos]).await?;
        self.stream.write_all(b"\r\n").await?;

        Ok(())
    }
}

#[tokio::main]
async fn main() {
    // Bind the listener to the address
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    println!("Listening");

    let db : Db = Arc::new(Mutex::new(HashMap::new()));

    loop {
        // The second item contains the IP and port of the new connection.
        let (socket, who) = listener.accept().await.unwrap();
        // Clone the handle to the hash map.
        let db = db.clone();

        println!("[{}] accepted", &who);

        // A new task is spawned for each inbound socket. The socket is
        // moved to the new task and processed there.
        tokio::spawn(async move {
            process(socket, db).await;
        });
    }
}

async fn process(socket: TcpStream, db : Db) {
    use mini_redis::Command::{self, Get, Set};

    // Connection, provided by `mini-redis`, handles parsing frames from
    // the socket
    let peer = socket.peer_addr().unwrap();
    let mut connection = Connection::new(socket);

    // Use `read_frame` to receive a command from the connection.
    while let Some(frame) = connection.read_frame().await.unwrap() {
        let response = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                let mut db = db.lock().unwrap();
                // The value is stored as `Vec<u8>`
                db.insert(cmd.key().to_string(), cmd.value().clone());
                println!("[{}] {:?} = OK", &peer, cmd);
                Frame::Simple("OK".to_string())
            }
            Get(cmd) => {
                let db = db.lock().unwrap();
                if let Some(value) = db.get(cmd.key()) {
                    // `Frame::Bulk` expects data to be of type `Bytes`. This
                    // type will be covered later in the tutorial. For now,
                    // `&Vec<u8>` is converted to `Bytes` using `into()`.
                    println!("[{}] {:?} = {:?}", &peer, cmd, &value);
                    Frame::Bulk(value.clone().into())
                } else {
                    println!("[{}] {:?} = nil", &peer, cmd);
                    Frame::Null
                }
            }
            cmd => panic!("unimplemented {:?}", cmd),
        };

        // Write the response to the client
        connection.write_frame(&response).await.unwrap();
    }
}
