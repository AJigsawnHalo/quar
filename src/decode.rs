use image;
use rqrr;
use rqrr::MetaData;
pub fn decode_img(path: String) -> (MetaData, String) {
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
