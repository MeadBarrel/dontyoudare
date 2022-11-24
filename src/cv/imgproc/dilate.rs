use opencv::Result;
use super::traits::*;


use opencv::{
    prelude::Mat,
    core::Size,
    imgproc::gaussian_blur
};
use opencv::core::{BorderTypes, Point, Scalar, ToInputArray, ToOutputArray};
use opencv::imgproc::{dilate, get_structuring_element, MORPH_ELLIPSE, MORPH_GRADIENT, MORPH_RECT, MORPH_TOPHAT};


#[derive(Clone, Copy)]
pub struct StructuringElement {
    shape: i32,
    ksize: Size,
    anchor: Point
}


impl Default for StructuringElement {
    fn default() -> Self {
        Self {
            shape: MORPH_RECT,
            ksize: Size::new(3, 3),
            anchor: Point::new(-1, -1)
        }
    }
}


impl StructuringElement {
    pub fn with_shape(&self, shape: i32) -> Self {
        Self {
            shape,
            ..*self
        }
    }

    pub fn with_ksize(&self, ksize: Size) -> Self {
        Self {
            ksize,
            ..*self
        }
    }

    pub fn with_radius(&self, radius: i32) -> Self {
        self.with_ksize(Size::new(radius, radius))
    }

    pub fn with_anchor(&self, anchor: Point) -> Self {
        Self {
            anchor,
            ..*self
        }
    }

    pub fn with_anchor_center(&self) -> Self {
        self.with_anchor(Point::new(-1, -1))
    }

    pub fn get_mat(&self) -> Result<Mat> {
        get_structuring_element(self.shape, self.ksize, self.anchor)
    }
}


#[derive(Clone, Copy)]
pub struct Dilate {
    kernel: StructuringElement,
    anchor: Point,
    iterations: i32,
    border_type: i32,
    border_value: Scalar
}


impl Dilate {
    pub fn new(
        kernel: StructuringElement,
        anchor: Point,
        iterations: i32,
        border_type: i32,
        border_value: Scalar
    ) -> Self
    {
        Self {
            kernel,
            anchor,
            iterations,
            border_type,
            border_value
        }
    }
}


impl Default for Dilate {
    fn default() -> Self {
        Self {
            kernel: StructuringElement::default(),
            anchor: Point::new(-1, -1),
            iterations: 1,
            border_type: BorderTypes::BORDER_ISOLATED as i32,
            border_value: Scalar::default()
        }
    }
}


impl OneToOneConvert for Dilate {
    fn dest(&self, src: &dyn ToInputArray, dest: &mut dyn ToOutputArray) -> Result<()> {
        dilate(src, dest, &self.kernel.get_mat()?, self.anchor, self.iterations, self.border_type, self.border_value)?;
        Ok(())
    }
}


impl OneToOneConvertPrep for Dilate {}