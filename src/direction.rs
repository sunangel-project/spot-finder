use osm_xml::Node;

fn direction_from_cardinals(input: &str) -> Option<f64> {
    None
}

fn direction_from_string(input: &str) -> Option<f64> {
    match str::parse::<f64>(input) {
        Ok(dir) => Some(dir),
        Err(_) => direction_from_cardinals(input),
    }   
}

pub fn direction_of_node(node: &Node) -> Option<f64> {
    // TODO:  assumes degrees in float. what happens if NE, W, etc.
    (&node.tags).into_iter()
        .find(|tag| tag.key == "direction")
        .map(|tag| direction_from_string(&tag.val)).flatten()
}


#[cfg(test)]
mod tests {
    use super::direction_from_string;


    #[test]
    fn empty() {
        assert_eq!(None, direction_from_string(""))
    }
}
