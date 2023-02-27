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
    r#type: String,
    loc: Location,
    dir: Option<u32>,
}

// Searching

fn is_bench(n: &&Node) -> bool {
    n.tags.iter().any(|t|
        (t.key == "amenity" && t.key == "bench")
        || t.key == "bench"
    )
}

pub async fn find_spots(loc: &Location, rad: u32) -> Result<Vec<Spot>, async_nats::Error> {
    let osm_data = get_osm_data(loc, rad).await?;
    let osm = OSM::parse(Cursor::new(osm_data))?;
    
    let spots = osm.nodes.iter()
    .map(|(_, node)| node)
    .filter(is_bench)
    .map(|node| Spot {
        r#type: "bench".to_string(),
        loc: Location::from(node),
        dir: None,
    }).collect();

    Ok(spots)
}
