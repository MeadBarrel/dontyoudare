use opencv::highgui::imshow;
use opencv::prelude::*;
use opencv::imgproc::{bounding_rect, THRESH_BINARY};
use opencv::Result;
use opencv::types::VectorOfMat;
use redis::pipe;
use crate::cv::*;


#[derive(Default)]
pub struct MatDiffPipe {
    pub mat1: Mat,
    pub mat2: Mat,
    pub diff: Mat,
    pub threshold: Mat,
    pub dilate: Mat,
    pub contours: VectorOfMat,
}


impl MatDiffPipe {
    pub fn are_different(&self) -> bool {
        !self.contours.is_empty()
    }
}


pub struct MatDiff {
    pub blur: GaussianBlur,
    pub dilate: Dilate,
    pub threshold: Threshold,
    pub contours: FindContours,
    pub contour_area_threshold: i32,
}


impl Default for MatDiff {
    fn default() -> Self {
        Self {
            blur: GaussianBlur::default(),
            dilate: Dilate::default().with_iterations(4).with_kernel(
                StructuringElement::default().with_radius(6)),
            threshold: Threshold::new(6_f64, 255_f64, THRESH_BINARY),
            contours: FindContours::default(),
            contour_area_threshold: 2000,
        }
    }
}


impl MatDiff {
    pub fn new(
        blur: GaussianBlur,
        dilate: Dilate,
        threshold: Threshold,
        contours: FindContours,
        contour_area_threshold: i32) -> Self
    {
        Self { blur, dilate, threshold, contours, contour_area_threshold }
    }

    pub fn with_blur_radius(&self, radius: i32) -> Self {
        Self {
            blur: self.blur.with_blur_radius(radius),
            ..*self
        }
    }

    pub fn with_dilate_iterations(&self, iterations: i32) -> Self {
        Self {
            dilate: self.dilate.with_iterations(iterations),
            ..*self
        }
    }

    pub fn with_thresh(&self, thresh: f64) -> Self {
        Self {
            threshold: self.threshold.with_thresh(thresh),
            ..*self
        }
    }

    fn prepare_mat(&self, src: &Mat) -> Result<Mat> {
        self.blur.prep(
            &CvtColor::gray().prep(src)?
        )
    }

    pub fn diff(&self, src1: &Mat, src2: &Mat) -> Result<MatDiffPipe> {

        let mat1 = self.prepare_mat(src1)?;
        let mat2 = self.prepare_mat(src2)?;

        let diff = absdiff_prep(&mat1, &mat2)?;
        let threshold = self.threshold.prep(&diff)?;
        let dilate = self.dilate.prep(&threshold)?;
        let contours = self.contours.prep(&dilate)?;

        let result_contours: VectorOfMat = contours.iter().filter(
            |x| {
                match bounding_rect(x) {
                    Ok(x) => x.area() > self.contour_area_threshold,
                    Err(_) => false
                }
            }
        ).collect::<VectorOfMat>();

        Ok(
            MatDiffPipe {
                mat1,
                mat2,
                diff,
                dilate,
                threshold,
                contours: result_contours,
            }
        )

    }
}