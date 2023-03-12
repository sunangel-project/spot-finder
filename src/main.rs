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
use serde_json::{json, Value};
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

#[derive(Debug, Serialize, Deserialize)]
struct ErrorMessage {
    sender: String,
    reason: String,
    input: String,
}

#[tokio::main]
async fn main() -> Result<(), async_nats::Error> {
    let host = std::env::var("NATS_HOST").unwrap_or("localhost".into());
    let client = &async_nats::connect(host).await?;
    let subscriber = client
        .queue_subscribe("search".to_string(), "spot-finder".to_string())
        .await?;

    subscriber
        .for_each_concurrent(16, |msg| async move {
            if let Err(err) = handle_message(client, &msg).await {
                send_error_message(client, &msg, err).await
            }
        })
        .await;

    Ok(())
}

// Event Loop
async fn handle_message(client: &Client, msg: &Message) -> Result<(), Box<dyn Error>> {
    let payload = str::from_utf8(&msg.payload)?;

    let spots = handle_payload(payload).await?;
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

async fn handle_payload(payload: &str) -> Result<Vec<Spot>, Box<dyn Error>> {
    let in_message: InMessage = serde_json::from_str(payload)?;
    let query = in_message.search_query;
    find_spots(&query.loc, query.rad).await
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

async fn send_error_message(client: &Client, msg: &Message, err: Box<dyn Error>) {
    if let Err(_) = client
        .publish(
            "error".to_string(),
            build_error_payload(msg, &err).to_string().into(),
        )
        .await
    {
        println!("Could not send error out!\n{}", err)
    }
}

fn build_error_payload(msg: &Message, err: &Box<dyn Error>) -> String {
    json!(ErrorMessage {
        sender: "spot-finder".to_string(),
        reason: format!("{err}"),
        input: format!("{msg:?}"),
    })
    .to_string()
}
