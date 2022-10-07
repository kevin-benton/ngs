use anyhow::bail;
use clap::{builder::PossibleValuesParser, Arg, ArgMatches, Command};

use prettytable::{row, Table};

use crate::utils::genome::get_all_reference_genomes;

pub fn get_command() -> Command {
    Command::new("list")
        .about("Utility to list various supported items in this command line tool.")
        .arg(
            Arg::new("subject")
                .help("The subject which you want to list values for.")
                .value_parser(PossibleValuesParser::new(["reference-genomes"]))
                .required(true),
        )
}

pub fn list(matches: &ArgMatches) -> anyhow::Result<()> {
    let subject = matches
        .get_one::<String>("subject")
        .expect("could not parse subject");

    match subject.as_str() {
        "reference-genomes" => {
            let mut table = Table::new();

            table.add_row(row!["Name", "Source", "Basis"]);
            for reference in get_all_reference_genomes() {
                table.add_row(row![
                    reference.name(),
                    reference.source(),
                    reference.basis(),
                ]);
            }

            table.printstd();

            Ok(())
        }
        s => bail!("Unsupported subject: {}", s),
    }
}
