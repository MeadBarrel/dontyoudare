use opencv::core::{absdiff, ToInputArray};
use opencv::Result;
use opencv::prelude::*;


pub fn absdiff_prep(src1: &dyn ToInputArray, src2: &dyn ToInputArray) -> Result<Mat> {
    let mut result = Mat::default();
    absdiff(src1, src2, &mut result)?;
    Ok(result)
}
