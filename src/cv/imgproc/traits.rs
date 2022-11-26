use opencv::core::{ToInputArray, ToOutputArray};
use opencv::prelude::Mat;
use opencv::Result;


pub trait OneToOneConvert {
    fn dest(&self, src: &dyn ToInputArray, dest: &mut dyn ToOutputArray) -> Result<()>;
}


pub trait OneToOneConvertPrep: OneToOneConvert {
    fn prep(&self, src: &Mat) -> Result<Mat> {
        let mut dest = Mat::default();
        self.dest(src, &mut dest)?;
        Ok(dest)
    }
}
