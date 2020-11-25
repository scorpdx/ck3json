extern crate clap;
use clap::{Arg, App};

#[macro_use]
extern crate pest_derive;

use std::io::prelude::*;
use std::fs::File;
use encoding_rs_io::DecodeReaderBytesBuilder;

mod json;
use json::*;

mod ck3json;
use ck3json::*;

fn main() {
    let matches = App::new("ck3json")
        .version("0.3.1")
        .author("J. Zebedee <zebedee@code.gripe>")
        .about("Convert CK3 files to JSON")
        .arg(Arg::with_name("file")
                 .required(true)
                 .takes_value(true)
                 .index(1)
                 .help("CK3 file to parse"))
        .arg(Arg::with_name("grammar")
                 .possible_values(&["ck3txt", "ck3bin"])
                 .default_value("ck3txt"))
        .arg(Arg::from_usage("-m --melt-only 'Melt ck3bin-format and return text without converting to JSON'"))
        .get_matches();

    let filename = matches.value_of("file").unwrap();
    let mut file = File::open(filename).expect("cannot open file");

    let grammar_name = matches.value_of("grammar").unwrap();
    match grammar_name {
        "ck3txt" => {
            let reader = DecodeReaderBytesBuilder::new();
            let mut transcoded = reader.build(file);

            let mut file_text = String::new();
            transcoded.read_to_string(&mut file_text).expect("cannot transcode file");
        
            let x = ck3parser::parse(&file_text).expect("unsuccessful parse");
            println!("{}", serialize_jsonvalue(&x));
        },
        "ck3bin" => {
            let mut bin_buf = Vec::new();
            file.read_to_end(&mut bin_buf).expect("failed to read binary file");
            
            use ck3save::{Melter, FailedResolveStrategy};
            let melt_bytes = Melter::new()
                .with_on_failed_resolve(FailedResolveStrategy::Ignore)
                .melt(&bin_buf)
                .expect("failed to melt ck3bin");
            let melt_string = std::str::from_utf8(&melt_bytes).unwrap();

            let melt_only = matches.is_present("melt-only");
            match melt_only {
                true => println!("{}", melt_string),
                false => {
                    let parsed = ck3parser::parse(&melt_string).expect("failed to parse melted ck3bin text");
                    println!("{}", serialize_jsonvalue(&parsed));
                }
            };
        },
        _ => unreachable!("unknown grammar type")
    };
}