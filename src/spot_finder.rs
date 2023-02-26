use std::io::Cursor;
use serde::{Serialize, Deserialize};

use crate::location::Location;

async fn get_osm_data(loc: &Location, rad: u32) -> Result<String, reqwest::Error>  {
    // TODO: build url
    let url = "https://www.openstreetmap.org/api/0.6/map?bbox=48.2,9,48.3,9.1";
    let response = reqwest::get(url)
        .await?
        .text()
        .await?;
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

/*
fn is_bench(o: &OsmObj) -> bool {
    o.tags().contains_key("bench")
}

fn is_in_search_area(node: &Node, point: &Location, radius: f64) -> bool {
    point.dist(&Location::from(node)) < radius
} */

pub async fn find_spots(loc: &Location, rad: u32) -> Result<Vec<Spot>, async_nats::Error> {
    let osm_data = get_osm_data(loc, rad).await?;
    let osm = osm_xml::OSM::parse(Cursor::new(osm_data))?;
    /*
    let nodes = pbf.par_iter()
        .map(Result::unwrap)
        .filter(|o| o.is_node());

    let spots = nodes
        .filter(is_bench)
        .filter(|o| match o.node() {
            Some(node) => is_in_search_area(node, loc, rad.into()),
            _ => false,
        })
        .filter_map(|o| match o.node() {
            Some(node) => Some(Spot {
                r#type: "bench".to_string(),
                loc: Location::from(node),
                dir: None, // get_direction(node),
            }),
            _ => None,
        }).collect();
        
    Ok(spots) */
    println!("finished");

    Ok(Vec::new())
}
