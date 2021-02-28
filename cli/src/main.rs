use clap::{App, AppSettings, Arg, SubCommand};
use std::fs::File;
use std::io::prelude::*;

mod inspect;
use inspect::*;

fn main() {
    let matches = App::new("SemDoc")
        .version("0.1.0")
        .author("Marcel Garus <marcel.garus@gmail.com>")
        .about("Parses Semantic Documents")
        .arg(Arg::with_name("file").required(true).index(1))
        .subcommand(
            SubCommand::with_name("inspect")
                .about("Inspects either the atoms or the blocks of a SemDoc file.")
                .setting(AppSettings::SubcommandRequiredElseHelp)
                .subcommand(
                    SubCommand::with_name("bytes").about("Inspects the bytes of the SemDoc file."),
                )
                .subcommand(
                    SubCommand::with_name("atoms").about("Inspects the atoms of the SemDoc file."),
                )
                .subcommand(
                    SubCommand::with_name("blocks")
                        .about("Inspects the blocks of the SemDoc file."),
                )
                .subcommand(
                    SubCommand::with_name("molecules")
                        .about("Inspects the molecules of the SemDoc file."),
                ),
        )
        .subcommand(SubCommand::with_name("eat"))
        .get_matches();

    let file = matches.value_of("file").unwrap();
    println!("The file passed is: {}", file);

    if let Some(ref matches) = matches.subcommand_matches("inspect") {
        if matches.subcommand_matches("bytes").is_some() {
            inspect_bytes(&file);
        }
        if matches.subcommand_matches("atoms").is_some() {
            inspect_atoms(&file);
        }
        if matches.subcommand_matches("blocks").is_some() {
            inspect_blocks(&file);
        }
        if matches.subcommand_matches("molecules").is_some() {
            inspect_molecules(&file);
        }
    }
    if matches.subcommand_matches("eat").is_some() {
        eat(file)
    }
}

fn eat(file: &str) {
    let content = std::fs::read_to_string(file).expect("File not found.");
    let doc = markdown_to_semdoc::markdown_to_semdoc(&content);

    let mut file = File::create("converted.sd").unwrap();
    file.write_all(&doc.to_bytes()).unwrap();
    inspect_blocks("converted.sd");
}
