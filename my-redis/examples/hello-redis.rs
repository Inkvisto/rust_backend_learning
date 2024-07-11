use mini_redis::{clients::Client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let mut client = Client::connect("127.0.0.1:6379").await?;
    client.set("a", "Robert".into()).await?;
    let result = client.get("a").await?;
    dbg!(result);
    Ok(())
}
