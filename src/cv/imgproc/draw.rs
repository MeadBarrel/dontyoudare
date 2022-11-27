use opencv::prelude::*;
use opencv::core::{no_array, Point, Scalar, ToInputArray, ToInputOutputArray};
use opencv::imgproc::{draw_contours, LINE_AA};
use opencv::Result;


#[derive(Debug)]
pub struct DrawContours {
    color: Scalar,
    thickness: i32,
    line_type: i32,
}


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
