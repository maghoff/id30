use std::process::ExitCode;

use id30::Id30;
use rand08::prelude::*;

fn help() -> ExitCode {
    println!(
        "\
Generate and interrogate id30 identifiers.

USAGE: id30 [--help] [--json] {{ID}}

--help   Show this help
--json   Format the output as JSON

{{ID}}     Optionally specify a list of IDs either in id30 format or as
         integers. These will be written back out in both canonical id30 and
         integer formats.

         If no IDs are given, one is generated randomly and printed out in both
         id30 and integer formats.
"
    );
    ExitCode::SUCCESS
}

fn unknown_option(option: &str) -> ExitCode {
    eprintln!("Unknown option {option}, try --help");
    ExitCode::FAILURE
}

fn report(id30: Id30, json: bool) {
    if !json {
        println!("Id30: {id30}\nu32: {}", u32::from(id30));
    } else {
        println!("{{\"id30\":\"{id30}\",\"u32\":{}}}", u32::from(id30));
    }
}

fn main() -> ExitCode {
    let mut errors = 0;
    let mut json = false;
    let mut query_ids = vec![];

    for arg in std::env::args().skip(1) {
        match arg.as_str() {
            "--json" => json = true,
            "--help" => return help(),
            option if option.starts_with("-") => return unknown_option(option),
            _ => query_ids.push(arg),
        }
    }

    if !query_ids.is_empty() {
        for id_str in &query_ids {
            if let Ok(id30) = id_str.parse::<Id30>() {
                report(id30, json);
                continue;
            } else if let Ok(int) = id_str.parse::<u32>() {
                if let Ok(id30) = Id30::try_from(int) {
                    report(id30, json);
                    continue;
                }
            }

            eprintln!("Unrecognized input: {id_str}");
            errors += 1;
        }
    } else {
        report(rand08::thread_rng().gen(), json);
    }

    if errors == 0 {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}
