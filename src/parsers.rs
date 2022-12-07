use regex::Regex;



fn parse_artist(input: &str) -> Option<&str>{
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?P<artist>\w+)(\s*-\s*)(?P<album>\w+)").unwrap();
    }

    RE.captures(input).and_then(|cap| {
        cap.name("artist").map(|artist| artist.as_str())
    })
}



#[cfg(test)]
mod tests {
    use crate::parsers::parse_artist;

    #[test]
    fn simple_album() {
        assert_eq!(parse_artist(r"Prince - Parade"), Some(r"Prince"));
    }

    #[test]
    fn simple_album_no_artist() {
        assert_ne!(parse_artist(r" - Parade"), Some(r"Prince"));
    }
}