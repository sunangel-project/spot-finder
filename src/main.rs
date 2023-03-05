use std::{str::{self, FromStr}, error::Error};

use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use async_nats::{Message, Client};
use futures_util::stream::StreamExt;

pub mod spot_finder;
pub mod location;
pub mod direction;

use location::Location;
use serde_json::Value;
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
    let client = &async_nats::connect("localhost").await?;
    let subscriber = client
        .queue_subscribe("search".to_string(), "spot-finder".to_string())
        .await?;
    
    subscriber.for_each_concurrent(16, |msg| async move {
        if let Err(err) = handle_message(client, &msg).await {
            println!("[ERROR] {err:?}");
        }
    }).await;
    
    Ok(())
}

// Event Loop
async fn handle_message(client: &Client, msg: &Message) -> Result<(), Box<dyn Error>> {
    let payload = str::from_utf8(&msg.payload)?;

    let query_value = Value::from_str(payload)?;
    let query: SearchQuery = serde_json::from_str(payload)?;
    
    let spots = find_spots(&query.loc, query.rad).await?;
    let total_num = spots.len();
    
    for (i, spot) in spots.into_iter().enumerate() {
        client.publish(
            "spots".to_string(),
           build_output_payload(spot, i, total_num, &query_value)?.to_string().into(),
        ).await?;
    }

    Ok(())
}

fn build_output_payload(spot: Spot, part_num: usize, total_num: usize, query_value: &Value) -> Result<Value, Box<dyn Error>>{
    let mut output = query_value.clone();
    let output_obj = output.as_object_mut()
        .ok_or(anyhow!("query was not an object: {query_value:?}"))?;

    let spot_msg = SpotMessage {
        spot, part_num, total_num
    };
    output_obj.insert("spot".into(), serde_json::to_value(spot_msg)?);

    Ok(output)
}
