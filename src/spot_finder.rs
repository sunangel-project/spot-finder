use std::{io, fs::File};
use osmpbfreader::{self, Node, OsmObj};
use serde::{Serialize, Deserialize};

use crate::location::Location;

const OSM_DATA_FILE: &str = "data/baden-wuerttemberg-latest.osm.pbf";

fn get_osm_file_reader() -> Result<io::BufReader<File>, async_nats::Error> {
    Ok(io::BufReader::new(File::open(OSM_DATA_FILE)?))
}

// Spot

#[derive(Debug, Serialize, Deserialize)]
pub struct Spot {
    r#type: String,
    loc: Location,
    dir: Option<u32>,
}

// Searching

fn is_bench(o: &OsmObj) -> bool {
    o.tags().contains_key("bench")
}

fn is_in_search_area(node: &Node, point: &Location, radius: f64) -> bool {
    point.dist(&Location::from(node)) < radius
}

pub fn find_spots(loc: &Location, rad: u32) -> Result<Vec<Spot>, async_nats::Error> {
    let mut pbf = osmpbfreader::OsmPbfReader::new(get_osm_file_reader()?);

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
        
    Ok(spots)
}
