use opencv::core::Size;
use serde::{Deserialize, Deserializer};


pub fn deserialize_size<'de, D>(deserializer: D) -> Result<Size, D::Error>
    where D: Deserializer<'de>
{
    let src_arr: [i32; 2] = Deserialize::deserialize(deserializer)?;
    Ok(Size::new(src_arr[0], src_arr[1]))
}