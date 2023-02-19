use std::process::exit;
use std::{io, fs::File};
use osmpbfreader::{self, OsmObj, Node};

pub mod location;

use crate::location::Location;

const OSM_DATA_FILE: &str = "data/germany-latest.osm.pbf";

fn get_osm_file_reader() -> io::BufReader<File> {
    match File::open(OSM_DATA_FILE) {
        Ok(file) => io::BufReader::new(file),
        Err(_e) => {
            // TODO: warning

            exit(1)
        },
    }
}

fn is_bench(o: &OsmObj) -> bool {
    o.is_node() && o.tags().contains_key("bench")
}

fn is_in_search_area(o: &OsmObj, point: &Location, radius: f64) -> bool {
    true
}

fn main() {

    let point: &Location = &Location {
        lat: 48.81434,
        long: 9.57961,
    };
    let radius: f64 = 0.1; // in degree, one degree roughly 100km


    let mut pbf = osmpbfreader::OsmPbfReader::new(get_osm_file_reader());
    
    pbf.par_iter().filter_map(|o_res| match o_res {
        Ok(o) => Some(o),
        Err(_) => None,
    }).filter(|o|
        is_bench(o) && is_in_search_area(o, point, radius)
    ).for_each(|o| match o.node() {
        Some(node) => println!("{:?}", Location::from(node)),
        _ => (),
    })

}
