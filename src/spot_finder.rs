use std::io::Cursor;

use anyhow::bail;
use osm_xml::{OSM, Node};
use reqwest::StatusCode;
use serde::{Serialize, Deserialize};

use crate::location::Location;

const LENGTH_DEG: f64 = 111_000.0;

const OVERPASS_URL: &str = "https://lz4.overpass-api.de/api/interpreter";

async fn get_osm_data(loc: &Location, rad: u32) -> Result<String, anyhow::Error>  {
    let rad_deg = 1.2 * (rad as f64 / LENGTH_DEG);

    let south = loc.lat - rad_deg;
    let west = loc.lon - rad_deg;
    let north = loc.lat + rad_deg;
    let east = loc.lon + rad_deg;

    let body = format!(
        "(node({},{},{},{}); <;); out meta;",
        south, west, north, east,
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
    r#type: String,
    loc: Location,
    dir: Option<u32>,
}

// Searching

fn is_bench(n: &&Node) -> bool {
    n.tags.iter().any(|t|
        t.key == "bench"
    )
}

/*
fn is_in_search_area(node: &Node, point: &Location, radius: f64) -> bool {
    point.dist(&Location::from(node)) < radius
} */

pub async fn find_spots(loc: &Location, rad: u32) -> Result<Vec<Spot>, async_nats::Error> {
    println!("send request");
    let osm_data = get_osm_data(loc, rad).await?;
    println!("parse data");
    let osm = OSM::parse(Cursor::new(osm_data))?;
    println!("parsed data");
    
    let spots = osm.nodes.iter()
    .map(|(_, node)| node)
    .filter(is_bench)
    .filter(|_| true) // TODO: filter search area
    .map(|node| Spot {
        r#type: "bench".to_string(),
        loc: Location::from(node),
        dir: None,
    }).collect();
    
    println!("filtered");

    Ok(spots)
}
