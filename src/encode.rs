use image::Luma;
use qrcode::{EcLevel, QrCode};
use std::fs;

pub fn encode_img_ecc(message: String, file_name: String, ecc: u16) {
    let ecc_lvl: EcLevel;

    if ecc == 0 {
        ecc_lvl = EcLevel::L;
    } else if ecc == 1 {
        ecc_lvl = EcLevel::M;
    } else if ecc == 2 {
        ecc_lvl = EcLevel::Q;
    } else if ecc == 3 {
        ecc_lvl = EcLevel::H;
    } else {
        ecc_lvl = EcLevel::H;
    }
    let code = QrCode::with_error_correction_level(message.as_bytes(), ecc_lvl).unwrap();

    let image = code.render::<Luma<u8>>().build();

    image.save(file_name).unwrap();
}

pub fn encode_from_file(path: String, file_name: String) {
    let input = fs::read_to_string(path).expect("Failed to read file.");
    encode_img(input, file_name);
}

pub fn encode_from_file_ecc(path: String, file_name: String, ecc_lvl: u16) {
    let input = fs::read_to_string(path).expect("Failed to read file.");
    encode_img_ecc(input, file_name, ecc_lvl);
}
pub fn encode_img(message: String, file_name: String) {
    let code = QrCode::new(message.as_bytes()).unwrap();

    let image = code.render::<Luma<u8>>().build();

    image.save(file_name).unwrap();
}
