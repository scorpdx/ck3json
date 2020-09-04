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
        .version("0.1.0")
        .author("J. Zebedee <zebedee@code.gripe>")
        .about("Convert CK3txt-format files to JSON")
        .arg(Arg::with_name("file")
                 .required(true)
                 .takes_value(true)
                 .index(1)
                 .help("CK3txt-format file to parse"))
        .arg(Arg::with_name("grammar")
                 .possible_values(&["ck3txt"])
                 .default_value("ck3txt"))
        .get_matches();

    let filename = matches.value_of("file").unwrap();
    let file = File::open(filename).expect("cannot open file");

    let mut transcoded = DecodeReaderBytesBuilder::new().build(file);

    let mut file_text = String::new();
    transcoded.read_to_string(&mut file_text).expect("cannot transcode file");

    let grammar_name = matches.value_of("grammar").unwrap();
    let json = match grammar_name {
        "ck3txt" => ck3parser::parse(&file_text).expect("unsuccessful parse"),
        _ => unreachable!("unknown grammar type")
    };

    println!("{}", serialize_jsonvalue(&json));
}