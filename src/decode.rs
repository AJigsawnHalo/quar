use image;
use rqrr;
use rqrr::MetaData;
use std::fs::OpenOptions;
use std::io::Write;

fn decoder(path: String) -> (MetaData, String) {
    let img = image::open(path).unwrap().to_luma8();
    let mut img = rqrr::PreparedImage::prepare(img);
    let grids = img.detect_grids();
    let (meta, content) = grids[0].decode().unwrap();
    (meta, content)
}

pub fn decode_img(path: String, ret_ecc: bool) {
    let (meta, content) = decoder(path);
    let content = content;

    if ret_ecc == true {
        let ecc_lvl = return_ecc(meta);
        println!("Error Correction Level: {}", ecc_lvl);
    }

    println!("Data: {}", content);
}

pub fn decode_to_file(path: String, file_name: String, ret_ecc: bool) {
    let (meta, content) = decoder(path);
    let content = content;
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(file_name)
        .expect("Unable to open file.");

    if ret_ecc == true {
        let ecc_lvl = return_ecc(meta);
        writeln!(&file, "ECC Level: {}", ecc_lvl).expect("Unable to write to file.")
    }
    writeln!(file, "{}", content).expect("Unable to write file.");
}

fn return_ecc(meta: MetaData) -> u16 {
    let ecc_lvl = meta.ecc_level;
    ecc_lvl
}
