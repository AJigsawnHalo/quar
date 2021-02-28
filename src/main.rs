mod decode;
mod encode;
use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Unable to get input");
    encode::encode_img(input);
    let data = decode::decode_img(String::from("test.png"));
    println!("{}", data.1);
}
