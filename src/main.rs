#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use tokio::stream::StreamExt;
    let opt = nats::Options::new().reconnect_callback(|| println!("Reconnecting"));
    let nc = opt.connect_async("0.0.0.0:4444").await?;

    println!("Connected to server 0.0.0.0:4444");

    let mut sub = nc.subscribe("test").await?;

    while let Some(msg) = sub.next().await {
        println!("Message: {}", String::from_utf8(msg.data)?);
    }
    Ok(())
}
