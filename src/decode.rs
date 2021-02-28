use image;
use rqrr;
use rqrr::MetaData;
use std::fs;
fn decoder(path: String) -> (MetaData, String) {
    // Load on image to search, convert it to grayscale
    let img = image::open(path).unwrap().to_luma8();
    // Prepare for detection
    let mut img = rqrr::PreparedImage::prepare(img);
    // Search for grids, without decoding
    let grids = img.detect_grids();
    // Decode the grid
    let (meta, content) = grids[0].decode().unwrap();
    (meta, content)
}

pub fn decode_img(path: String) {
    let img_content = decoder(path);
    let content = img_content.1;

    println!("{}", content);
}

pub fn decode_to_file(path: String, file_name: String) {
    let img_content = decoder(path);
    let content = img_content.1;
    fs::write(file_name, content).expect("Failed to write file");
}