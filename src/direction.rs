use anyhow::{bail, Error};
use osm_xml::Node;

fn parse_direction_cardinals(input: &str) -> Result<f64, Error> {
    let in_upper = input.to_uppercase();
    let mut in_iter = in_upper.chars();

    let mut allowed_chars = vec!['N', 'E', 'S', 'W'];
    
    let first_char = match in_iter.next() {
        None => bail!("input string has 0 length"),
        Some(first_char) => first_char,
    };
    
    let mut dir = match first_char {
        'N' => 0.,
        'E' => 90.,
        'S' => 180.,
        'W' => 270.,
        _ => bail!("first character {first_char} not allowed"),
    };

    Ok(dir)
}

fn parse_direction(input: &str) -> Option<f64> {
    str::parse::<f64>(input)
        .or(parse_direction_cardinals(input))
        .ok()
}

fn direction_from_string(input: &str) -> Option<f64> {
    if input.is_empty() {
        None
    } else {
        parse_direction(input)
    }
}

pub fn direction_of_node(node: &Node) -> Option<f64> {
    (&node.tags).into_iter()
        .find(|tag| tag.key == "direction")
        .map(|tag| tag.val.as_str())
        .map(direction_from_string)
        .flatten()
}


#[cfg(test)]
mod tests {
    use super::direction_from_string;

    #[test]
    fn empty() {
        assert_eq!(None, direction_from_string(""))
    }
    
    #[test]
    fn int() {
        assert_eq!(Some(10.), direction_from_string("10"))
    }
    
    #[test]
    fn float() {
        assert_eq!(Some(0.5), direction_from_string("0.5"))
    }
    
    #[test]
    fn card_one() {
        assert_eq!(Some(0.), direction_from_string("N"));
        assert_eq!(Some(90.), direction_from_string("E"));
        assert_eq!(Some(180.), direction_from_string("S"));
        assert_eq!(Some(270.), direction_from_string("W"));
    }
}
