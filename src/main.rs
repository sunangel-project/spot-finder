use std::{future::{self, Ready}, str};

use std::error::Error;

use async_nats::Message;
use futures_util::stream::StreamExt;

use crate::search_messages::SearchQuery;

pub mod spot_finder;
pub mod search_messages;
pub mod location;

#[tokio::main]
async fn main() -> Result<(), async_nats::Error> {
    let connection = async_nats::connect("localhost").await?;
    let subscriber = connection
        .queue_subscribe("search".to_string(), "spot-finder".to_string())
        .await?;
    
    subscriber.for_each(handle_message).await;
    
    Ok(())
}

// Event Loop
fn handle_message(msg: Message) -> Ready<()> {
        if let Err(err) = decode_and_search(&msg) {
            println!("[ERROR]: {err:?}");
            println!("Mesage: {msg:?}");
        }

        future::ready(())   
}

fn decode_and_search(msg: &Message) -> Result<(), Box<dyn Error>> {
    let payload = str::from_utf8(&msg.payload)?;
    let query: SearchQuery = serde_json::from_str(payload)?;
    println!("{query:?}");

    Ok(())
}
