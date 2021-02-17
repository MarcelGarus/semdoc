use clap::{App, AppSettings, Arg, SubCommand};
use semdoc_engine::atoms::*;

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
                ),
        )
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
    }
}

fn inspect_bytes(file: &str) {
    let bytes = std::fs::read(file).expect("File not found.");
    let atoms = (&bytes[..]).to_atom().expect("File corrupted.");
    for (index, chunk) in bytes.chunks(8).enumerate() {
        print!("{:3} |", index);
        for i in 0..8 {
            print!(" {:02x}", chunk.get(i).unwrap());
        }
        print!(" | ");
        for i in 0..8 {
            let byte = chunk.get(i).unwrap();
            print!(
                "{}",
                if (32..=126).contains(byte) {
                    *byte as char
                } else {
                    '.'
                }
            );
        }
        println!();
    }
}

fn inspect_atoms(file: &str) {
    let bytes = std::fs::read(file).expect("File not found.");
    let atoms = (&bytes[..]).to_atom().expect("File corrupted.");

    println!("Atoms: {:?}", atoms);
}

fn inspect_blocks(file: &str) {}
