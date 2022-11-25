use opencv::imgproc::THRESH_BINARY;
use serde::{Deserialize, Deserializer, de::Error};
use crate::cv::Threshold;


pub fn deserialize_threshold<'de, D>(deserializer: D) -> Result<Threshold, D::Error>
    where D: Deserializer<'de>
{
    let thresh: f64 = Deserialize::deserialize(deserializer)?;

    if thresh < 0. || thresh > 255. {
        return Err(Error::custom("Threshold must be within 0..255"))
    }

    Ok(
        Threshold::new(
            thresh as f64,
            255.,
            THRESH_BINARY
        )
    )
}