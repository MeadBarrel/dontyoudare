use opencv::Result;
use super::traits::*;


use opencv::{
    prelude::Mat,
    core::Size,
    imgproc::gaussian_blur
};
use opencv::core::{ToInputArray, ToOutputArray};


#[derive(Clone, Copy)]
pub struct GaussianBlur {
    pub blur_size: Size,
    pub sigma_x: f64,
    pub sigma_y: f64,
    pub border_type: i32,
}


impl Default for GaussianBlur {
    fn default() -> Self {
        Self {
            blur_size: Size::new(3, 3),
            sigma_x: 3.5,
            sigma_y: 3.5,
            border_type: 0
        }
    }
}


impl GaussianBlur {
    pub fn new(blur_size: Size, sigma_x: f64, sigma_y: f64, border_type: i32) -> Self {
        Self { blur_size, sigma_x, sigma_y, border_type }
    }

    pub fn with_blur_radius(&self, radius: i32) -> Self {
        Self {
            blur_size: Size::new(radius, radius),
            ..*self
        }
    }

    pub fn with_sigma(&self, sigma: f64) -> Self {
        Self {
            sigma_x: sigma,
            sigma_y: sigma,
            ..*self
        }
    }

    pub fn with_border_type(&self, border_type: i32) -> Self {
        Self {
            border_type,
            ..*self
        }
    }
}


impl OneToOneConvert for GaussianBlur {
    fn dest(&self, src: &dyn ToInputArray, dest: &mut dyn ToOutputArray) -> Result<()> {
        gaussian_blur(
            src, dest, self.blur_size, self.sigma_x, self.sigma_y, self.border_type)?;
        Ok(())
    }
}


impl OneToOneConvertPrep for GaussianBlur {}
