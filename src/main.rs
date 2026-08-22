use clap::{arg, command, Command};
use sinkdir::copy;

fn main() {
    let matches = command!() // requires `cargo` feature
        .propagate_version(true)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("copy")
                .about("Copy from source to target")
                .arg(arg!([SOURCE]))
                .arg(arg!([TARGET])),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("copy", sub_matches)) => {
            let source = sub_matches
                .get_one::<String>("SOURCE")
                .expect("Must provide a source value");
            let target = sub_matches
                .get_one::<String>("TARGET")
                .expect("must provide a target value");
            println!("copying from {source:?} to {target:?}",);
            copy(source, target)
        }
        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    }
}
