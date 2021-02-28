mod decode;
mod encode;
use clap::{load_yaml, App};

fn main() {
    let yaml = load_yaml!("args.yaml");
    let args = App::from(yaml).get_matches();

    if let Some(ref args) = args.subcommand_matches("decode") {
        if args.is_present("input") {
            let path = args.value_of("input").unwrap().to_string();
            decode::decode_img(path);
        }
    }
    if let Some(ref args) = args.subcommand_matches("encode") {
        if args.is_present("input") && args.is_present("output") {
            let message = args.value_of("input").unwrap().to_string();
            let file_name = args.value_of("output").unwrap().to_string();
            encode::encode_img(message, file_name);
        }
    }
}
