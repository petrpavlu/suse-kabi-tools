// Copyright (C) 2024 SUSE LLC <petr.pavlu@suse.com>
// SPDX-License-Identifier: GPL-2.0-or-later

use ksyms::sym::SymCorpus;
use log::debug;
use std::{env, process};

/// Prints the global usage message on `stdout`.
fn print_usage(program: &str) {
    print!(
        concat!(
            "Usage: {} [OPTIONS] COMMAND\n",
            "\n",
            "OPTIONS\n",
            "  -h, --help   print this help\n",
            "\n",
            "COMMAND\n",
            "  compare      show differences between two symtypes corpuses\n",
        ),
        program
    );
}

/// Prints the usage message for the `compare` command on `stdout`.
fn print_compare_usage(program: &str) {
    print!(
        concat!(
            "Usage: {} compare [OPTIONS] DIR1 DIR2\n",
            "Show differences between two symtypes corpuses.\n",
            "\n",
            "OPTIONS\n",
            "  -h, --help   print this help\n",
        ),
        program
    );
}

/// Handles the `compare` command which shows differences between two symtypes corpuses.
fn do_compare<I>(program: &str, args: I) -> Result<(), ()>
where
    I: IntoIterator<Item = String>,
{
    // Parse specific command options.
    let mut maybe_dir1 = None;
    let mut maybe_dir2 = None;
    for arg in args.into_iter() {
        if arg == "-h" || arg == "--help" {
            print_compare_usage(&program);
            return Ok(());
        }
        if arg.starts_with("-") || arg.starts_with("--") {
            eprintln!("Unrecognized compare option '{}'", arg);
            return Err(());
        }
        if maybe_dir1.is_none() {
            maybe_dir1 = Some(arg);
            continue;
        }
        if maybe_dir2.is_none() {
            maybe_dir2 = Some(arg);
            continue;
        }
        eprintln!("Excess compare argument '{}' specified", arg);
        return Err(());
    }

    let dir1 = match maybe_dir1 {
        Some(dir1) => dir1,
        None => {
            eprintln!("The first compare source is missing");
            return Err(());
        }
    };
    let dir2 = match maybe_dir2 {
        Some(dir2) => dir2,
        None => {
            eprintln!("The second compare source is missing");
            return Err(());
        }
    };

    // Do the comparison.
    debug!("Compare '{}' and '{}'", dir1, dir2);

    let s1 = match SymCorpus::new(dir1.as_str()) {
        Ok(s1) => s1,
        Err(err) => {
            eprintln!("Failed to read symtypes from '{}': {}", dir1, err);
            return Err(());
        }
    };
    let s2 = match SymCorpus::new(dir2.as_str()) {
        Ok(s2) => s2,
        Err(err) => {
            eprintln!("Failed to read symtypes from '{}': {}", dir2, err);
            return Err(());
        }
    };
    s1.compare_with(&s2);

    Ok(())
}

fn main() {
    env_logger::init();

    let mut args = env::args();

    let program = match args.next() {
        Some(program) => program,
        None => {
            eprintln!("Unknown program name");
            process::exit(1);
        }
    };

    /* Handle global options and stop at the command. */
    let mut maybe_command = None;
    loop {
        let arg = match args.next() {
            Some(arg) => arg,
            None => break,
        };

        if arg == "-h" || arg == "--help" {
            print_usage(&program);
            process::exit(0);
        }
        if arg.starts_with("-") || arg.starts_with("--") {
            eprintln!("Unrecognized global option '{}'", arg);
            process::exit(1);
        }
        maybe_command = Some(arg);
        break;
    }

    let command = match maybe_command {
        Some(command) => command,
        None => {
            eprintln!("No command specified");
            process::exit(1);
        }
    };

    /* Process the specified command. */
    match command.as_str() {
        "compare" => {
            if let Err(_) = do_compare(&program, args) {
                process::exit(1);
            }
        }
        _ => {
            eprintln!("Unrecognized command '{}'", command);
            process::exit(1);
        }
    }

    process::exit(0);
}
