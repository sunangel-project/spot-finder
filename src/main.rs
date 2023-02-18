use std::process::exit;
use std::{io, fs::File};
use osmpbfreader;

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


fn main() {
    let mut pbf = osmpbfreader::OsmPbfReader::new(get_osm_file_reader());
    let objs = pbf.get_objs_and_deps(|obj| {
        obj.is_way() && obj.tags().contains_key("highway")
    })
    .unwrap();
    for (id, obj) in &objs {
        println!("{:?}: {:?}", id, obj);
    }
}
