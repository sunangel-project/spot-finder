use std::process::exit;
use std::{io, fs::File};
use osmpbfreader::{self, OsmObj};

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

fn is_in_search_area(o: &OsmObj) -> bool {
    true
}

fn main() {
    let mut pbf = osmpbfreader::OsmPbfReader::new(get_osm_file_reader());
    
    pbf.par_iter().filter_map(|o_res| match o_res {
        Ok(o) => Some(o),
        Err(_) => None,
    }).filter(|o|
        is_bench(o) && is_in_search_area(o)
    ).for_each(|o| {
        println!("{o:?}")
    })

}
