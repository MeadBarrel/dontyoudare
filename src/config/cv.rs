use opencv::core::{BorderTypes, Point, Point_, Scalar, Size};
use serde::{Deserialize};

use crate::cv::*;

use super::types::*;


// Schema
#[derive(Deserialize)]
struct DrawContoursConfig {
    color: Color,
    thickness: i32,
    line_type: i32
}


#[derive(Deserialize)]
struct FindContoursConfig {
    mode: i32,
    color_approximation: i32,
}


#[derive(Deserialize)]
struct GaussianBlurConfig {
    radius: i8,
    sigma: f64,
    border_type: i32,
}


#[derive(Deserialize)]
struct DilateConfig {
    radius: i8,
    iterations: i8,
}


// Into definitions
impl Into<DrawContours> for DrawContoursConfig {
    fn into(self) -> DrawContours {
        DrawContours::new(self.color.into(), self.thickness, self.line_type)
    }
}

impl Into<FindContours> for FindContoursConfig {
    fn into(self) -> FindContours {
        FindContours::new(
            self.mode,
            self.color_approximation,
            Point::new(0, 0)
        )
    }
}


impl Into<GaussianBlur> for GaussianBlurConfig {
    fn into(self) -> GaussianBlur {
        GaussianBlur::new(
            Size::new(self.radius as i32, self.radius as i32),
            self.sigma,
            self.sigma,
            self.border_type
        )
    }
}


impl Into<Dilate> for DilateConfig {
    fn into(self) -> Dilate {
        Dilate::new(
            StructuringElement::default().with_radius(self.radius as i32),
            Point::new(-1, -1),
            self.iterations as i32,
            BorderTypes::BORDER_ISOLATED as i32,
            Scalar::default(),
        )
    }
}