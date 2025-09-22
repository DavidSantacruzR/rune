use crate::image::RuneImage;

pub struct RunePipeline;

impl RunePipeline {
    pub fn run(image: RuneImage) -> String {
        /*TODO: preprocessing - segmentation - recognition*/
        format!("Stub OCR: {}x{} ({} channels)", image.width, image.height, image.channels)
    }
}

#[test]
fn stub_pipeline_successful() {
    let img = RuneImage::from_path("test.png").unwrap();
    let output = RunePipeline::run(img);
    assert!(output.contains("Stub OCR"));
}