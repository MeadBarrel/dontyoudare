use opencv::core::{BorderTypes, Point, Size, Scalar};
use opencv::imgproc::THRESH_BINARY;

use crate::cv::*;
use crate::camera::matdiff::MatDiff;

use serde::Deserialize;


#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct MatDiffConfig {
    blur_radius: i8,
    blur_sigma: f64,
    dilate_radius: i8,
    dilate_iterations: i8,
    threshold: i8,
    contour_area_threshold: i32
}


impl Default for MatDiffConfig {
    fn default() -> Self {
        Self {
            blur_radius: 3,
            blur_sigma: 4.0,
            dilate_radius: 6,
            dilate_iterations: 4,
            threshold: 6,
            contour_area_threshold: 2000
        }
    }
}


impl Into<MatDiff> for MatDiffConfig {
    fn into(self) -> MatDiff {
        let blur = GaussianBlur::new(
            Size::new(self.blur_radius as i32, self.blur_radius as i32),
            self.blur_sigma,
            self.blur_sigma,
            0
        );
        let se = StructuringElement::default().with_radius(self.dilate_radius as i32);
        let dilate = Dilate::new(
            se,
            Point::new(-1, -1),
            self.dilate_iterations as i32,
            BorderTypes::BORDER_ISOLATED as i32,
            Scalar::default(),
        );
        let threshold = Threshold::new(
            self.threshold as f64,
            255_f64,
            THRESH_BINARY
        );
        let contours = FindContours::default();

        MatDiff::new(
            blur,
            dilate,
            threshold,
            contours,
            self.contour_area_threshold as i32
        )
    }
}
