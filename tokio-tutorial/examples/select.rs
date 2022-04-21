//! This example aggregates the most interesting parts from https://tokio.rs/tokio/tutorial/select

use std::time::Duration;
use tokio::time;
use tokio::sync::{mpsc, oneshot};

#[tokio::main]
async fn main() {
    println!("main1:");
    main1().await;
    println!("\nmain2:");
    main2().await;
    println!("\nmain3:");
    main3().await;
    println!("\nmain4:");
    main4().await;
}

async fn main1() {
    let (tx1, rx1) = oneshot::channel();
    let (tx2, rx2) = oneshot::channel();

    tokio::spawn(async {
        let _ = tx1.send("one");
    });

    tokio::spawn(async {
        let _ = tx2.send("two");
    });

    // This macro supports up to 64 branches.
    //* Only the first matching branch executes, the others are dropped.
    //
    //* It runs all branches concurrently on the same task. Because all branches of the
    //* select! macro are executed on the same task, they will never run simultaneously
    tokio::select! {
        //* syntax:
        //*    <pattern> = <async expression> => <handler>,
        // 
        //* Any `?` operator used in the async expression propagates the `Err` to the pattern.
        //* Any `?` operator used in the handler propagates the `Err` to the select macro
        //* result instead .
        // 
        //* The async expression may simply borrow data, instead of needing to own it as it is
        //  usually the case when spawning. The rust borrow checker of course is still
        //  applied.
        //* The select guarantees that only a single <handler> runs though, therefore
        //  each <handler> may mutably borrow the same data.
        val = rx1 => {
            println!("rx1 completed first with {:?}", val);
        },
        val = rx2 => {
            println!("rx2 completed first with {:?}", val);
        },
        //* branches may include an else with this syntax:
        //*   else => <handler>
        // 
        //* The handler is executed if none of the other branches matches their respective
        //* patterns after returning.
        // In example, with two different `Some(x)` branches,
        // having `None` returned from both would disable each of them, and end up
        // running the else handler instead. When pattern matching no branch may
        // match indeed, but the select macro expression still must return a value.
    }
}


async fn some_operation() -> String {
    println!("some_operation() started waiting");
    // We use `tokio::time::sleep()` which is the async version of `thread::sleep`.
    // Should we use the latter instead we'd always see this operation completing,
    // since the tokio runtime woulnd't get control back during the sleep any more.
    time::sleep(Duration::from_millis(10)).await;
    println!("some_operation() is completing");
    return "take this".to_owned();
}

async fn main2() {
    let (mut tx1, rx1) = oneshot::channel();
    let (tx2, rx2) = oneshot::channel();

    tokio::spawn(async {
        // Select on the operation and the oneshot's  `closed()` notification.
        tokio::select! {
            val = some_operation() => {
                println!("some_operation() completed");
                let _ = tx1.send(val);
            }
            _ = tx1.closed() => {
                // `some_operation()` is canceled, the task completes and `tx1` is dropped.
                println!("some_operation() is canceled");
            }
        }
    });

    tokio::spawn(async {
        let _ = tx2.send("two");
    });

    tokio::select! {
        val = rx1 => {
            println!("rx1 completed first with {:?}", val);
        }
        val = rx2 => {
            println!("rx2 completed first with {:?}", val);
        }
    }
}


async fn main3() {
    let (tx1, mut rx1) = mpsc::channel(128);
    let (tx2, mut rx2) = mpsc::channel(128);
    let (tx3, mut rx3) = mpsc::channel(128);

    tokio::spawn(async move {
        let one_ms = Duration::from_millis(1);
        let _ = tx2.send("hello tx2").await;
        time::sleep(one_ms).await;
        let _ = tx1.send("hello tx1").await;
        time::sleep(one_ms).await;
        let _ = tx3.send("hello tx3").await;
        time::sleep(one_ms).await;
        let _ = tx1.send("more for you tx1").await;
        time::sleep(one_ms).await;
        let _ = tx3.send("more for you as well tx3").await;
    });

    loop {
        let msg = tokio::select! {
            Some(msg) = rx1.recv() => msg,
            Some(msg) = rx2.recv() => msg,
            Some(msg) = rx3.recv() => msg,
            else => { break }
        };

        println!("Got {:?}", msg);
    }

    println!("All channels have been closed.");
}


async fn action(input: Option<i32>) -> Option<String> {
    // If the input is `None`, return `None`.
    // This could also be written as `let i = input?;`
    let i = match input {
        Some(input) => input,
        None => return None,
    };
    // async logic
    time::sleep(Duration::from_millis(1)).await;
    Some(i.to_string())
}

async fn main4() {
    let (tx, mut rx) = tokio::sync::mpsc::channel(128);
    
    let mut done = false;
    let operation = action(None);
    tokio::pin!(operation);
    
    tokio::spawn(async move {
        let _ = tx.send(1).await;
        let _ = tx.send(3).await;
        let _ = tx.send(2).await;
    });
    
    loop {
        tokio::select! {
            //* This precondition is checked before select! awaits on the branch.
            //* If the condition evaluates to false then the branch is disabled.
            //
            // The done variable is used to track whether operation has already been
            // used or not. Trying to use again a completed async operation would lead
            // to a panic.
            res = &mut operation, if !done => {
                done = true;

                if let Some(v) = res {
                    println!("GOT = {}", v);
                    return;
                }
            }
            Some(v) = rx.recv() => {
                if v % 2 == 0 {
                    // `.set` is a method on `Pin`.
                    operation.set(action(Some(v)));
                    done = false;
                }
            }
        }
    }
}