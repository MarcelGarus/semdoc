extern crate clap;

use clap::{App, Arg, SubCommand};
use std::fs;
mod ssss;

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
    .get_matches();
  if let Some(matches) = matches.subcommand_matches("format") {
    for file_name in &matches.args.get("files").unwrap().vals {
      println!("Formatting {:?}...", file_name);
      let content = fs::read_to_string(file_name).expect("Couldn't open file.");
      let elements = ssss::parse(&content);
      let formatted = ssss::format(
        &elements,
        &ssss::FormatConfig {
          limit: 80,
          indentation: "  ".to_string(),
        },
      );
      fs::write(file_name, formatted).expect("Unable to write file");
    }
    // if let Some(files) = matches.values_of("files") {
    //   println!("{:?}", files);
    // }
  }

  let elements = ssss::parse(
    "document {
  title {Sample title.}
  mlml-version {1}
} {
  section {Formatting lines} {
    section {Text} {
      In 4s (surprisingly simple structure of strings), text is pretty easy to
      format: With the current indention, word after word is just added. If a
      word doesn't fit into the limit and it's not the only word on the line, a
      newline is inserted before and it's indented.
    }
    section {Blocks} {
      Blocks have three different formatting modes:
      inline-section {Inlining.} {
        If a block contains only text and no other blocks, it can be inlined,
        meaning that it may start in the middle of a line while still breaking
        its content:
        code {mlml} {
          This is text with some bold{blocks that are
          formatted inline}. Notice that italic{their
          contents may break}.
        }
      }
      inline-section {One-lining} {}
    }
  }
  image {
    url {https://someimage.com/image}
    alternative {Alternative text}
  }
  code {mlml} {...}
  list {
    item {Apples}
    item {Bananas}
  }
}


document {
  name {Sample title.}
  content {
    section {
      title {Section 1}
      content {}
    }
    This is a bold{ sample } document written in italic{ Marcel's legendary
    markup language }.
    image {
      url {https://someimage.com/image}
      alternative {Alternative text}
    }
    code {
      language {mlml}
      content {...}
    }
  }
}
",
  );
  ssss::format(
    &elements,
    &ssss::FormatConfig {
      limit: 80,
      indentation: "  ".to_string(),
    },
  );
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
