use image::Luma;
use qrcode::QrCode;

pub fn encode_img() {
    let message = "Hello. It works!~";
    let code = QrCode::new(message.as_bytes()).unwrap();

    let image = code.render::<Luma<u8>>().build();

    image.save("test.png").unwrap();
}
