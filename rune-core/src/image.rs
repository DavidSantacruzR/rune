use std::path::Path;

pub struct RuneImage {
    pub width: u32,
    pub height: u32,
    pub channels: u8
}

impl RuneImage {
    pub fn from_path<P: AsRef<Path>>(_path: P) -> Result<Self, String> {
        /*TODO: Implement me.*/
        Ok(Self {
            width: 0,
            height: 0,
            channels: 3
        })
    }
}
