use opencv::core::{ToInputArray, ToOutputArray};
use opencv::imgproc::{COLOR_BGR2GRAY, cvt_color};
use opencv::Result;
use super::traits::*;


pub struct CvtColor {
    code: i32,
    dst_cn: i32,
}



impl CvtColor {
    pub fn with_code(code: i32) -> Self {
        Self {
            code,
            dst_cn: 0,
        }
    }

    pub fn gray() -> Self {
        Self {
            code: COLOR_BGR2GRAY,
            dst_cn: 0
        }
    }
}


impl OneToOneConvert for CvtColor {
    fn dest(&self, src: &dyn ToInputArray, dest: &mut dyn ToOutputArray) -> Result<()> {
        cvt_color(src, dest, self.code, self.dst_cn)?;
        Ok(())
    }
}


impl OneToOneConvertPrep for CvtColor {}
