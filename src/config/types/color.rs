use opencv::core::Scalar;
use serde::{Deserialize, Deserializer, de::Error};
use serde;
use regex::Regex;


#[derive(Eq, PartialEq, Debug)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}


impl Into<Scalar> for Color {
    fn into(self) -> Scalar {
        Scalar::new(
            self.red as f64, self.green as f64, self.blue as f64, self.alpha as f64)
    }
}


impl<'de> Deserialize<'de> for Color {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        let src_text: &str = Deserialize::deserialize(deserializer)?;
        let re = Regex::new("#[0-9|A-F|a-f]{8}").map_err(D::Error::custom)?;

        if !re.is_match(&src_text) {
            return Err(Error::custom("Incorrect color format"))
        }

        Ok(
            Color {
                red: u8::from_str_radix(&src_text[1..3], 16).map_err(D::Error::custom)?,
                green: u8::from_str_radix(&src_text[3..5], 16).map_err(D::Error::custom)?,
                blue: u8::from_str_radix(&src_text[5..7], 16).map_err(D::Error::custom)?,
                alpha: u8::from_str_radix(&src_text[7..9], 16).map_err(D::Error::custom)?,
            }
        )
    }
}


pub fn deserialize_color<'de, D>(deserializer: D) -> Result<Scalar, D::Error>
    where D: Deserializer<'de> {
    let src_color: Color = Deserialize::deserialize(deserializer)?;
    let result: Scalar = src_color.into();
    Ok(result)
}


#[cfg(test)]
mod tests {
    use super::Color;
    use serde_json;


    #[test]
    fn deserialize_color() {
        let c: Color = serde_json::from_str("\"#Ff0010FF\"").unwrap();
        assert_eq!(c, Color {red: 255, green: 0, blue: 16, alpha: 255} );
    }

    #[test]
    #[should_panic(expected="Incorrect color format")]
    fn deserialize_color_incorrect_format() {
        let c: Color = serde_json::from_str("\"FF0010FF\"").unwrap();
    }
}