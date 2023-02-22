use std::future;

use futures_util::stream::StreamExt;

pub mod spot_finder;
pub mod location;

#[tokio::main]
async fn main() -> Result<(), async_nats::Error> {
    let connection = async_nats::connect("localhost").await?;
    let subscriber = connection.subscribe("search".to_string()).await?;
    
    subscriber.for_each(|msg| {
        println!("{msg:?}");

        future::ready(())
    }).await;
    
    Ok(())
}
