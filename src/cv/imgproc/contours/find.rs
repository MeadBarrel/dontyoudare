use opencv::Result;
use opencv::prelude::*;
use opencv::core::{InputArray, Mat, Point, ToInputArray};
use opencv::imgproc::{CHAIN_APPROX_NONE, CHAIN_APPROX_SIMPLE, CHAIN_APPROX_TC89_KCOS, CHAIN_APPROX_TC89_L1, find_contours, RETR_EXTERNAL, RETR_FLOODFILL, RETR_TREE};
use opencv::types::VectorOfMat;


#[derive(Debug, Clone, Copy)]
pub struct FindContours {
    mode: i32,
    color_approximation: i32,
    offset: Point,
}


impl Default for FindContours {
    fn default() -> Self {
        Self {
            mode: RETR_TREE,
            color_approximation: CHAIN_APPROX_TC89_KCOS,
            offset: Point::new(0, 0)
        }
    }
}


impl FindContours {
    pub fn new(mode: i32, color_approximation: i32, offset: Point) -> Self {
        Self {
            mode, color_approximation, offset
        }
    }

    pub fn dest(&self, src: &dyn ToInputArray, dest: &mut VectorOfMat) -> Result<()> {
        find_contours(
            src,
            dest,
            self.mode,
            self.color_approximation,
            self.offset
        )?;
        Ok(())
    }

    pub fn prep(&self, src: &dyn ToInputArray) -> Result<VectorOfMat> {
        let mut result = VectorOfMat::default();
        self.dest(src, &mut result)?;
        Ok(result)
    }
}
