extern crate clap;

#[macro_use]
mod utils;

mod ssss;

use clap::{App, Arg, SubCommand};
use std::fs;

pub fn main() {
  let matches = App::new("pad")
    .version("0.0.1")
    .author("Marcel Garus <marcel.garus@gmail.com>")
    .about("Utilities for .pad files.")
    .subcommand(
      SubCommand::with_name("format")
        .about("formats .pad files")
        .version("1.0")
        .arg(
          Arg::with_name("files")
            .multiple(true)
            .required(true)
            .help("files to format"),
        ),
    )
    .subcommand(
      SubCommand::with_name("analyze")
        .about("analyzes .pad files")
        .version("1.0")
        .arg(
          Arg::with_name("files")
            .multiple(true)
            .required(true)
            .help("files to analyze"),
        ),
    )
    .get_matches();
  // Formatting.
  if let Some(matches) = matches.subcommand_matches("format") {
    for file_name in &matches.args.get("files").unwrap().vals {
      println!("Formatting {:?}...", file_name);
      let content = fs::read_to_string(file_name).expect("Couldn't open file.");
      let parse_result = ssss::parse(&content);
      for element in &parse_result.body {
        println!("{}", element);
      }
      if parse_result.has_errors() {
        for error in parse_result.errors {
          println!("{:?}", error);
        }
      } else {
        let body = parse_result.body;
        let formatted = ssss::format(
          &body,
          &ssss::FormatConfig {
            limit: 80,
            indentation: 2,
          },
        );
        let mut file_name: std::ffi::OsString = std::ffi::OsString::from(file_name);
        file_name.push(".formatted");
        fs::write(file_name, formatted).expect("Unable to write file");
      }
    }
    // if let Some(files) = matches.values_of("files") {
    //   println!("{:?}", files);
    // }
  }

  // for element in &elements {
  //   println!("{:?}", element);
  // }
  // println!(
  //   "{}",
  //   ssss::format(
  //     &elements,
  //     &ssss::FormatConfig {
  //       limit: 80,
  //       indentation: "  ".to_string(),
  //     }
  //   )
  // );
}
