use std::{
    error::Error,
    str::{self, FromStr},
};

use anyhow::anyhow;
use async_nats::{Client, Message};
use futures_util::stream::StreamExt;
use serde::{Deserialize, Serialize};

pub mod direction;
pub mod location;
pub mod spot_finder;

use location::Location;
use serde_json::Value;
use spot_finder::{find_spots, Spot};

#[derive(Debug, Serialize, Deserialize)]
struct InMessage {
    search_query: SearchQuery,
}

#[derive(Debug, Serialize, Deserialize)]
struct SearchQuery {
    id: String,
    loc: Location,
    rad: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct PartMessage {
    id: usize,
    of: usize,
}

#[tokio::main]
async fn main() -> Result<(), async_nats::Error> {
    let client = &async_nats::connect("nats").await?;
    let subscriber = client
        .queue_subscribe("search".to_string(), "spot-finder".to_string())
        .await?;

    subscriber
        .for_each_concurrent(16, |msg| async move {
            if let Err(err) = handle_message(client, &msg).await {
                println!("[ERROR] {err:?}");
            }
        })
        .await;

    Ok(())
}

// Event Loop
async fn handle_message(client: &Client, msg: &Message) -> Result<(), Box<dyn Error>> {
    let payload = str::from_utf8(&msg.payload)?;

    let in_message: InMessage = serde_json::from_str(payload)?;
    let query = in_message.search_query;

    let spots = find_spots(&query.loc, query.rad).await?;
    let total_num = spots.len();

    let in_value = Value::from_str(payload)?;
    for (i, spot) in spots.into_iter().enumerate() {
        client
            .publish(
                "spots".to_string(),
                build_output_payload(spot, i, total_num, &in_value)?
                    .to_string()
                    .into(),
            )
            .await?;
    }

    Ok(())
}

fn build_output_payload(
    spot: Spot,
    part_num: usize,
    total_num: usize,
    query_value: &Value,
) -> Result<Value, Box<dyn Error>> {
    let mut output = query_value.clone();
    let output_obj = output
        .as_object_mut()
        .ok_or(anyhow!("query was not an object: {query_value:?}"))?;

    output_obj.insert("spot".into(), serde_json::to_value(spot)?);
    output_obj.insert(
        "part".into(),
        serde_json::to_value(PartMessage {
            id: part_num,
            of: total_num,
        })?,
    );

    Ok(output)
}
