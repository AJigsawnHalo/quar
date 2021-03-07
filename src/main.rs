mod decode;
mod encode;
use clap::{load_yaml, App};

fn main() {
    let yaml = load_yaml!("args.yaml");
    let args = App::from(yaml).get_matches();

    if let Some(ref args) = args.subcommand_matches("decode") {
        if args.is_present("input") && args.is_present("output") == false {
            let path = args.value_of("input").unwrap().to_string();
            if args.is_present("ecc-level") {
                decode::decode_img(path, true);
            } else {
                decode::decode_img(path, false);
            }
        } else if args.is_present("output") && args.is_present("input") {
            let path = args.value_of("input").unwrap().to_string();
            let file_name = args.value_of("output").unwrap().to_string();
            if args.is_present("ecc-level") {
                decode::decode_to_file(path, file_name, true);
            } else {
                decode::decode_to_file(path, file_name, false);
            }
        }
    }
    if let Some(ref args) = args.subcommand_matches("encode") {
        if args.is_present("input")
            && args.is_present("output")
            && args.is_present("from-file") == false
            && args.is_present("ecc-level") == false
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
            && args.is_present("ecc-level") == false
        {
            let path = args.value_of("from-file").unwrap().to_string();
            let file_name = args.value_of("output").unwrap().to_string();
            encode::encode_from_file(path, file_name);
        } else if (args.is_present("input") || args.is_present("from-file"))
            && args.is_present("output")
            && args.is_present("ecc-level")
        {
            let file_name = args.value_of("output").unwrap().to_string();
            let ecc_lvl = args.value_of("ecc-level").unwrap().parse::<u16>().unwrap();
            println!("{}", &ecc_lvl);
            if ecc_lvl >= 4 {
                println!("Invalid ECC level. Possible values are '0', '1', '2', '3'.");
            } else {
                if args.is_present("input") {
                    let message = args.value_of("input").unwrap().to_string();
                    encode::encode_img_ecc(message, file_name, ecc_lvl);
                } else if args.is_present("from-file") {
                    let path = args.value_of("from-file").unwrap().to_string();
                    encode::encode_from_file_ecc(path, file_name, ecc_lvl);
                }
            }
        }
    }
}
