mod ced;

use std::path::PathBuf;

use clap::error::ErrorKind;
use clap::{arg, value_parser, Command};

fn detect_encoding(data: &[u8]) -> String {
    let (encoding, _, _) = ced::detect_encoding(
        data,
        "",
        "",
        "",
        ced::Encoding::UNKNOWN_ENCODING,
        ced::Language::UNKNOWN_LANGUAGE,
        ced::TextCorpusType::QUERY_CORPUS,
        false,
    );

    ced::mime_encoding_name(encoding)
}

fn decode(data: &[u8], mime_encoding_name: &str) -> Option<String> {
    let encoding = match encoding_rs::Encoding::for_label(mime_encoding_name.as_bytes()) {
        Some(encoding) => encoding,
        None => return None,
    };

    let (cow, _, has_errors) = encoding.decode(data);

    if has_errors {
        return None;
    } else {
        Some(cow.into_owned())
    }
}

fn show_error(cmd: &mut Command, message: impl std::fmt::Display) -> ! {
    let err = cmd.error(ErrorKind::InvalidValue, message);
    err.exit();
}

fn main() {
    let mut cmd = Command::new(env!("CARGO_CRATE_NAME"))
        .about("Decode text file to UTF-8")
        .arg(
            arg!(-o --output <FILE> "Store decode results to file")
                .id("output")
                .required(false)
                .value_parser(value_parser!(PathBuf)),
        )
        .arg(
            arg!([INPUT] "Input text file path")
                .id("input")
                .required(true)
                .value_parser(value_parser!(PathBuf)),
        );

    let matches = cmd.get_matches_mut();

    let input_path = match matches.get_one::<PathBuf>("input") {
        Some(input) => input,
        None => show_error(&mut cmd, "Failed to get input path"),
    };

    let (output_path, is_store) = match matches.get_one::<PathBuf>("output") {
        Some(output) => (Some(output), true),
        None => (None, false),
    };

    let data = match std::fs::read(input_path) {
        Ok(data) => data,
        Err(_) => show_error(
            &mut cmd,
            format!("Failed to read file: {}", input_path.display().to_string()),
        ),
    };

    let encoding = detect_encoding(data.as_slice());
    let u8 = match decode(data.as_slice(), encoding.as_str()) {
        Some(u8) => u8,
        None => show_error(&mut cmd, "Failed to decode text"),
    };

    if is_store {
        if let Err(_) = std::fs::write(output_path.unwrap(), u8) {
            show_error(
                &mut cmd,
                format!(
                    "Failed to write text into file: {}",
                    output_path.unwrap().display().to_string()
                ),
            );
        }
    } else {
        println!("{}", u8);
    }
}
