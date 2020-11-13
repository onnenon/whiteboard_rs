use tokio::stream::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = nats::Options::new()
        .reconnect_callback(|| println!("Reconnected"))
        .disconnect_callback(|| println!("Disconnected"));

    let nc = opt.connect_async("0.0.0.0:4444").await?;

    println!("Connected to server: 0.0.0.0:4444");

    let status_sub = nc.subscribe("status").await?;

    handle_status(status_sub).await?;

    Ok(())
}

async fn handle_status(
    mut sub: nats::asynk::Subscription,
) -> Result<(), Box<dyn std::error::Error>> {
    while let Some(msg) = sub.next().await {
        println!("Message: {}", String::from_utf8(msg.data)?);
    }
    Ok(())
}
