use opencv::core::Point;
use serde::{Deserialize, Deserializer, de::Error};


pub fn deserialize_point<'de, D>(deserializer: D) -> Result<Point, D::Error>
    where D: Deserializer<'de> {
    let src_arr: [i32; 2] = Deserialize::deserialize(deserializer)?;
    Ok(Point::new(src_arr[0], src_arr[1]))
}