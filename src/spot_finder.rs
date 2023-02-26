use std::io::Cursor;
use osm_xml::{OSM, Node};
use serde::{Serialize, Deserialize};

use crate::location::Location;

const LENGTH_DEG: f64 = 111_000.0;

async fn get_osm_data(loc: &Location, rad: u32) -> Result<String, reqwest::Error>  {
    let rad_deg = 1.2 * (rad as f64 / LENGTH_DEG);

    let left = loc.lon - rad_deg;
    let bot = loc.lat - rad_deg;
    let right = loc.lon + rad_deg;
    let top = loc.lat + rad_deg;

    let url = format!(
        "https://www.openstreetmap.org/api/0.6/map?bbox={},{},{},{}",
        left, bot, right, top,
    );
    
    println!("{url}");
    println!("sending req");

    let response = reqwest::get(url)
        .await?
        .text()
        .await?;
    
    println!("got some response");

    Ok(response)
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
    let osm_data = get_osm_data(loc, rad).await?;
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
