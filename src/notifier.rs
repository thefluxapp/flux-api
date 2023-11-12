use std::env;

pub async fn run() -> Result<(), async_nats::Error> {
    let client = async_nats::connect(env::var("NATS_ADDR").unwrap()).await?;

    // for _ in 0..10 {
    //     info!("SEND DATA");
    //     client.publish("messages", "data".into()).await?;
    // }

    Ok(())
}
