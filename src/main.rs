use std::str;

use serde::{Deserialize, Serialize};
use async_nats::{Message, Client};
use futures_util::stream::StreamExt;

pub mod spot_finder;
pub mod location;

use location::Location;
use spot_finder::{find_spots, Spot};

#[derive(Debug, Serialize, Deserialize)]
struct SearchQuery {
    id: String,
    loc: Location,
    rad: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct SpotMessage {
    spot: Spot,
    part_num: usize,
    total_num: usize,
}

#[tokio::main]
async fn main() -> Result<(), async_nats::Error> {
    let client = async_nats::connect("localhost").await?;
    let mut subscriber = client
        .queue_subscribe("search".to_string(), "spot-finder".to_string())
        .await?;
    
    while let Some(msg) = subscriber.next().await {
        if let Err(err) = handle_message(&client, &msg).await {
            println!("[ERROR] {err:?}");
        }
    }
    
    Ok(())
}

// Event Loop
async fn handle_message(client: &Client, msg: &Message) -> Result<(), async_nats::Error> {

    let payload = str::from_utf8(&msg.payload)?;
    let query: SearchQuery = serde_json::from_str(payload)?;
    
    let spots = find_spots(&query.loc, query.rad).await?;
    let total_num = spots.len();
    
    for (i, spot) in spots.into_iter().enumerate() {
        let spot_msg = SpotMessage {
            spot,
            part_num: i,
            total_num
        };
        let spot_payload = serde_json::to_string(&spot_msg)?;

        client.publish(
            "spots".to_string(),
            spot_payload.into(),
        ).await?;
    }

    Ok(())
}
