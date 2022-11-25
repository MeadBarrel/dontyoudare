use opencv::videoio::VideoWriter;
use serde::{Deserialize, Deserializer, de::Error};
use regex::Regex;


pub fn deserialize_fourcc<'de, D>(deserializer: D) -> Result<i32, D::Error>
    where D: Deserializer<'de> {
    let src_string: &str = Deserialize::deserialize(deserializer)?;
    let re = Regex::new("[A-Z]{4}").map_err(D::Error::custom)?;

    if !re.is_match(&src_string) {
        return Err(Error::custom("Incorrect fourcc format"))
    }

    let mut chars = src_string.chars();

    Ok(
        VideoWriter::fourcc(
            chars.next().unwrap(),
            chars.next().unwrap(),
            chars.next().unwrap(),
            chars.next().unwrap()
        ).map_err(D::Error::custom)?
    )
}