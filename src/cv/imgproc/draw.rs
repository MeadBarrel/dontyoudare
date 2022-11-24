use opencv::prelude::*;
use opencv::core::{no_array, Point, Scalar, ToInputArray, ToInputOutputArray};
use opencv::imgproc::{draw_contours, LINE_AA};
use opencv::Result;
use std::result::{Result as StdResult};


#[derive(Debug)]
pub struct DrawContours {
    // #[serde(deserialize_with="from_hex_color_code")]
    color: Scalar,
    thickness: i32,
    line_type: i32,
}

//
// fn from_hex_color_code<'de, D>(deserializer: D) -> StdResult<Scalar, D::Error>
//     where D: Deserializer<'de>
// {
//     let mut s: &str = Deserialize::deserialize(deserializer)?;
//     s = s.trim();
//
//     if s.len() != 9 { return Err(de::Error::custom("abc")) }
//
//     let first_char = s.chars().nth(0_usize);
//
//     match first_char {
//         Some('#') => {}
//         _ => { return Err(de::Error::custom("abc")) }
//     }
//
//     let red = u8::from_str_radix(&s[1..3], 16).map_err(de::Error::custom)?;
//     let green = u8::from_str_radix(&s[3..5], 16).map_err(de::Error::custom)?;
//     let blue = u8::from_str_radix(&s[5..7], 16).map_err(de::Error::custom)?;
//     let alpha = u8::from_str_radix(&s[7..9], 16).map_err(de::Error::custom)?;
//
//     Ok(
//         Scalar::new(red as f64, green as f64, blue as f64, alpha as f64)
//     )
// }


impl Default for DrawContours {
    fn default() -> Self {
        Self {
            color: Scalar::new(0 as f64, 255 as f64, 0 as f64, 255 as f64),
            thickness: 1,
            line_type: LINE_AA,
        }
    }
}


impl DrawContours {
    pub fn new(color: Scalar, thickness: i32, line_type: i32) -> Self {
        Self { color, thickness, line_type }
    }

    pub fn dest(&self, dest: &mut dyn ToInputOutputArray, contours: &dyn ToInputArray) -> Result<()> {
        let idx: i32 = 0;
        let hierarchy = no_array();
        let max_result = 2;
        let zero_offset = Point::new(0, 0);

        draw_contours(
            dest,
            contours,
            idx,
            self.color,
            self.thickness,
            self.line_type,
            &hierarchy,
            max_result,
            zero_offset
        )?;

        Ok(())
    }

    pub fn prep(
        &self, src: &Mat, contours: &dyn ToInputArray
    ) -> Result<Mat> {
        let mut result = src.clone();
        self.dest(&mut result, contours)?;
        Ok(result)
    }
}


#[cfg(feature="file_config")]
pub mod config {
    use serde::Deserialize;

    use crate::config::Color;
    use super::DrawContours;

    #[derive(Deserialize)]
    struct DrawContoursConfig {
        color: Color,
        thickness: i32,
        line_type: i32
    }

    impl Into<DrawContours> for DrawContoursConfig {
        fn into(self) -> DrawContours {
            DrawContours::new(self.color.into(), self.thickness, self.line_type)
        }
    }

}