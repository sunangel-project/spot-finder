use std::str;

use serde::{Deserialize, Serialize};
use async_nats::{Message, Client, HeaderMap};
use futures_util::stream::StreamExt;

pub mod spot_finder;
pub mod location;

use location::Location;
use spot_finder::find_spots;

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchQuery {
    loc: Location,
    rad: u32,
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
    let headers_in = msg.headers.as_ref()
        .expect("expected message to have headers");

    let payload = str::from_utf8(&msg.payload)?;
    let query: SearchQuery = serde_json::from_str(payload)?;
    
    let spots = find_spots(&query.loc, query.rad).await?;
    
    for (i, spot) in spots.iter().enumerate() {
        let payload = serde_json::to_string(&spot)?;
        
        let mut headers_out = HeaderMap::from(headers_in.to_owned());
        headers_out.insert("part-id", i.to_string().as_str());
        headers_out.insert("num-parts", spots.len().to_string().as_str());

        client.publish_with_headers(
            "spots".to_string(),
            headers_out,
            payload.into(),
        ).await?;
    }

    Ok(())
}
