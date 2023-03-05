use rumqttc::{MqttOptions, AsyncClient, QoS};
use tokio::{task, time, self};
use std::time::Duration;

#[tokio::main(flavor = "current_thread")]
pub async fn main() {
    let mut mqttoptions = MqttOptions::new("rumqtt-async", "localhost", 1883);
    mqttoptions.set_keep_alive(Duration::from_secs(5));

    let (mut client, mut eventloop) = AsyncClient::new(mqttoptions, 10);
    client.subscribe("hello/rumqtt", QoS::AtMostOnce).await.unwrap();

    task::spawn(async move {
        for i in 0..10 {
            client.publish("hello/rumqtt", QoS::AtLeastOnce, false, vec![i; i as usize]).await.unwrap();
            time::sleep(Duration::from_millis(100)).await;
        }
    });

    loop {
        let notification = eventloop.poll().await.unwrap();
        println!("Received = {:?}", notification);
    }
}