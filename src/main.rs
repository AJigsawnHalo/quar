mod decode;
mod encode;
use clap::{load_yaml, App};

fn main() {
    let yaml = load_yaml!("args.yaml");
    let args = App::from(yaml).get_matches();

    if let Some(ref args) = args.subcommand_matches("decode") {
        if args.is_present("input") && args.is_present("output") == false {
            let path = args.value_of("input").unwrap().to_string();
            decode::decode_img(path);
        } else if args.is_present("output") && args.is_present("input") {
            let path = args.value_of("input").unwrap().to_string();
            let file_name = args.value_of("output").unwrap().to_string();
            decode::decode_to_file(path, file_name);
        }
    }
    if let Some(ref args) = args.subcommand_matches("encode") {
        if args.is_present("input")
            && args.is_present("output")
            && args.is_present("from-file") == false
        {
            let message = args.value_of("input").unwrap().to_string();
            let file_name = args.value_of("output").unwrap().to_string();
            encode::encode_img(message, file_name);
        } else if (args.is_present("input") || args.is_present("from-file"))
            && args.is_present("output") == false
        {
            println!("Please provide a file name for the output.");
        } else if args.is_present("input") == false && args.is_present("from-file") == false {
            println!("Please provide an input to encode.");
        } else if args.is_present("from-file")
            && args.is_present("output")
            && args.is_present("input") == false
        {
            let path = args.value_of("from-file").unwrap().to_string();
            let file_name = args.value_of("output").unwrap().to_string();
            encode::encode_from_file(path, file_name);
        }
    }
}
