use std::process::exit;
use std::{io, fs::File};
use osmpbfreader::{self, Node, OsmObj};

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
    o.tags().contains_key("bench")
}

fn is_in_search_area(node: &Node, point: &Location, radius: f64) -> bool {
    point.dist(&Location::from(node)) < radius
}

/*
fn is_in_search_area_filter(point: &Location, radius: f64) -> impl Fn(&Node) -> bool {
    move |node| -> bool { is_in_search_area(node, point, radius) }
} */

fn main() {

    let point: &Location = &Location {
        lat: 48.81434,
        long: 9.57961,
    };
    let radius: f64 = 0.05; // in degree, one degree roughly 100km


    let mut pbf = osmpbfreader::OsmPbfReader::new(get_osm_file_reader());
    
    pbf.par_iter()
        .filter_map(|o_res| match o_res {
            Ok(o) => Some(o),
            Err(_) => None,
        })
        .filter(|o| o.is_node()
        )
        .filter(|o|
            is_bench(o)
        )
        .filter(|o| match o.node() {
            Some(node) => is_in_search_area(node, point, radius),
            None => false
        })
        .for_each(|o|
            println!("https://www.openstreetmap.org/edit?editor=id&node={:?}", o.id().inner_id())
        )

}
