use regex::Regex;

pub const ARTIST: usize = 0;
pub const ALBUM_NAME: usize = 1;
pub const YEAR: usize = 2;
pub const MEDIA: usize = 3;

/// Will receive a slice representing an album directory name to be parsed
/// using regex. The it will return a vector with all the matches.
/// 
pub fn parse_artist(input: &str) -> Vec<Option<String>> {
    lazy_static! {
        //(?P<artist>\w+)(\s*-\s*)(?P<name>\w+)
        //([\w]+)(\s*-\s*)([\w\d,´]+[\w\s\d&´\(\)]*)(\s?\()(\d{4})(\))(\s*\[{1})(DD|FLAC)(\]{1})
        static ref RE: Regex = Regex::new(r"(?P<artist>[\w]+)(\s*-\s*)(?P<name>[\w\d\.,´]+[\w\s\d&´\-\+\(\)]*)(\s?\()(?P<year>\d{4})(\))(\s*\[{1})(?P<media>DD|FLAC)(\]{1})").unwrap();
    }
    
    let mut vec:Vec<Option<String>> = Vec::new();
    vec.push(RE.captures(input).and_then(|cap| cap.name("artist").map(|artist| String::from(artist.as_str().trim()))));
    vec.push(RE.captures(input).and_then(|cap| cap.name("name").map(|name| String::from(name.as_str().trim()))));
    vec.push(RE.captures(input).and_then(|cap| cap.name("year").map(|name| String::from(name.as_str().trim()))));
    vec.push(RE.captures(input).and_then(|cap| cap.name("media").map(|name| String::from(name.as_str().trim()))));
    
    vec
    
}



#[cfg(test)]
mod tests {
    use crate::parsers::*;

    #[test]
    fn simple_album() {
        let album = parse_artist(r"Prince - Parade (1980) [FLAC]");
        
        assert_eq!(album.get(ARTIST).unwrap(), &Some(String::from("Prince")));
        assert_eq!(album.get(ALBUM_NAME).unwrap(), &Some(String::from("Parade")));
        assert_eq!(album.get(YEAR).unwrap(), &Some(String::from("1980")));
        assert_eq!(album.get(MEDIA).unwrap(), &Some(String::from("FLAC")));
    }

     #[test]
    fn album_title_with_number() {
        let album = parse_artist(r"Prince - 13 (1980) [FLAC]");
        
        assert_eq!(album.get(ARTIST).unwrap(), &Some(String::from("Prince")));
        assert_eq!(album.get(ALBUM_NAME).unwrap(), &Some(String::from("13")));
        assert_eq!(album.get(YEAR).unwrap(), &Some(String::from("1980")));
        assert_eq!(album.get(MEDIA).unwrap(), &Some(String::from("FLAC")));
    }

     #[test]
    fn album_title_with_number_and_tilde() {
        let album = parse_artist(r"Prince - 20 chansons d´or (1980) [FLAC]");
        
        assert_eq!(album.get(ARTIST).unwrap(), &Some(String::from("Prince")));
        assert_eq!(album.get(ALBUM_NAME).unwrap(), &Some(String::from(r"20 chansons d´or")));
        assert_eq!(album.get(YEAR).unwrap(), &Some(String::from("1980")));
        assert_eq!(album.get(MEDIA).unwrap(), &Some(String::from("FLAC")));
    }

     #[test]
    fn album_title_with_number_and_ampersand() {
        let album = parse_artist(r"Prince - 3 & 3 (1980) [FLAC]");
        
        assert_eq!(album.get(ARTIST).unwrap(), &Some(String::from("Prince")));
        assert_eq!(album.get(ALBUM_NAME).unwrap(), &Some(String::from(r"3 & 3")));
        assert_eq!(album.get(YEAR).unwrap(), &Some(String::from("1980")));
        assert_eq!(album.get(MEDIA).unwrap(), &Some(String::from("FLAC")));
    }

     #[test]
    fn album_title_with_number_tildes_and_commas() {
        let album = parse_artist(r"Prince - 50,000,000 Elvis Fans Can´t Be Wrong (1980) [FLAC]");
        
        assert_eq!(album.get(ARTIST).unwrap(), &Some(String::from("Prince")));
        assert_eq!(album.get(ALBUM_NAME).unwrap(), &Some(String::from(r"50,000,000 Elvis Fans Can´t Be Wrong")));
        assert_eq!(album.get(YEAR).unwrap(), &Some(String::from("1980")));
        assert_eq!(album.get(MEDIA).unwrap(), &Some(String::from("FLAC")));
    }

    #[test]
    fn album_title_with_minus() {
        let album = parse_artist(r"Prince - All-Time Greatest Hits (1980) [FLAC]");
        
        assert_eq!(album.get(ARTIST).unwrap(), &Some(String::from("Prince")));
        assert_eq!(album.get(ALBUM_NAME).unwrap(), &Some(String::from(r"All-Time Greatest Hits")));
        assert_eq!(album.get(YEAR).unwrap(), &Some(String::from("1980")));
        assert_eq!(album.get(MEDIA).unwrap(), &Some(String::from("FLAC")));
    }

    #[test]
    fn album_title_with_parenthesis() {
        let album = parse_artist(r"Prince - Anos de Soledad (Live) (1980) [FLAC]");
        
        assert_eq!(album.get(ARTIST).unwrap(), &Some(String::from("Prince")));
        assert_eq!(album.get(ALBUM_NAME).unwrap(), &Some(String::from(r"Anos de Soledad (Live)")));
        assert_eq!(album.get(YEAR).unwrap(), &Some(String::from("1980")));
        assert_eq!(album.get(MEDIA).unwrap(), &Some(String::from("FLAC")));
    }

}