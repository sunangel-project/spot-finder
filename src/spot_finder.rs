use std::io::Cursor;

use anyhow::bail;
use osm_xml::{OSM, Node};
use reqwest::StatusCode;
use serde::{Serialize, Deserialize};

use crate::location::Location;

const OVERPASS_URL: &str = "https://lz4.overpass-api.de/api/interpreter";

async fn get_osm_data(loc: &Location, rad: u32) -> Result<String, anyhow::Error>  {
    let body = format!( // TODO: if need more nodes, adjust query
        "nwr(around:{},{},{})->.all;
        (
            node.all[amenity=bench];
            node.all[bench=yes];
        );
        out meta;",
        rad, loc.lat, loc.lon,
    );
    
    let client = reqwest::Client::new();
    let request = client
        .post(OVERPASS_URL)
        .body(body);
    let response = request.send().await?;
    
    if response.status() == StatusCode::OK {
        Ok(response.text().await?)
    } else {
        bail!(
            "overpass returned {}",
            response.status(),
        )
    }
}

// Spot
#[derive(Debug, Serialize, Deserialize)]
pub struct Spot {
    pub r#type: String,
    pub loc: Location,
    pub dir: Option<f64>,
}

// Searching

fn is_bench(n: &&Node) -> bool {
    n.tags.iter().any(|t|
        (t.key == "amenity" && t.val == "bench")
        || t.key == "bench"
    )
}

fn direction_from_string(input: &str) -> Option<f64> {
    None
}

fn get_direction(node: &Node) -> Option<f64> {
    // TODO:  assumes degrees in float. what happens if NE, W, etc.
    (&node.tags).into_iter()
        .find(|tag| tag.key == "direction")
        .map(|tag| {
            match str::parse::<f64>(&tag.val) {
                Ok(dir) => Some(dir),
                Err(_) => direction_from_string(&tag.val),
            }
        }).flatten()
}

pub async fn find_spots(loc: &Location, rad: u32) -> Result<Vec<Spot>, async_nats::Error> {
    let osm_data = get_osm_data(loc, rad).await?;
    let osm = OSM::parse(Cursor::new(osm_data))?;
    
    let spots = osm.nodes.iter()
    .map(|(_, node)| node)
    .filter(is_bench)
    .map(|node| {
        Spot {
        r#type: "bench".to_string(),
        loc: Location::from(node),
        dir: get_direction(node),
    }}).collect();

    Ok(spots)
}
