use image::Luma;
use qrcode::QrCode;
use std::fs;

pub fn encode_img(message: String, file_name: String) {
    let code = QrCode::new(message.as_bytes()).unwrap();

    let image = code.render::<Luma<u8>>().build();

    image.save(file_name).unwrap();
}

pub fn encode_from_file(path: String, file_name: String) {
    let input = fs::read_to_string(path).expect("Failed to read file.");
    encode_img(input, file_name);
}
