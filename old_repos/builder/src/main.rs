
use base9::{get_variables, format_variables};
use config::{Config};
use clap::{arg, Command, ArgMatches, Arg};
use std::io::{self, Read};
use std::path::PathBuf;
use std::str::FromStr;
use anyhow::{Result, anyhow};
use mustache::{compile_path, compile_str};

mod utils;
mod color_science;
mod base9;
mod config;
mod palette;
mod generator;
pub type Color = ext_palette::Srgb<u8>;

pub const N: usize = 9;

fn read_stdin() -> Result<String> {
    let mut buf = String::new();
    let mut stdin = io::stdin();
    stdin.read_to_string(&mut buf)?;
    Ok(buf)
}

fn cli() -> Command<'static> {
    let palette_arg: Arg = arg!(<PALETTE> "the palette code. Use `-` for default palette.");
    Command::new("base9-builder")
        .about("base9 CLI tool")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("render")
                .about("renders theme template")
                .arg(palette_arg.clone())
                .arg(arg!(<TEMPLATE> "path to template file. Use `-` to read from stdin."))
                .arg(
                    arg!([DEST] "path to write output to. By default output to stdout.")
                    .value_parser(clap::value_parser!(std::path::PathBuf)))
        )
        // .subcommand(
        //     Command::new("generate")
        //         .about("randomly generates a base9 palette")
        //         .arg(arg!(<INCOMPLETE_PALETTE> "Palette code but omit colors that you want to be generated. (For example: -ffffff-------"))
        // )
        .subcommand(
            Command::new("preview")
                .about("prints a table of all generated colors to preview")
                .arg(palette_arg.clone())
        )
        .subcommand(
            Command::new("list-variables")
                .hide(true)
                .about("prints all variables used by templates")
                .arg(palette_arg.clone())
        )
}

fn matches_to_formatted_variables(matches: &ArgMatches) -> Result<serde_json::Value> {
    let mut config = Config::default();
    let palette_arg: &str = matches.get_one::<String>("PALETTE").ok_or(anyhow!("missing palette!"))?;

    if palette_arg != "-" {
        config.palette = palette::Palette::from_str(palette_arg).map_err(|x| anyhow!("{}", x))?;
    }

    // Add config

    let variables = get_variables(&config)?;
    Ok(format_variables(&config, &variables))
}

fn main() -> Result<()> {
    let matches = cli().get_matches();
    match matches.subcommand() {
        Some(("render", sub_matches)) => {
            let formatted_variables = matches_to_formatted_variables(&sub_matches)?;
            let template_arg = sub_matches.get_one::<String>("TEMPLATE").unwrap();

            let template = if template_arg == "-" {
                let template_str = read_stdin()?;
                compile_str(&template_str)
            } else {
                compile_path(template_arg)
            }?;
            match sub_matches.get_one::<PathBuf>("DEST") {
                None => template.render(&mut io::stdout(), &formatted_variables)?,
                Some(dest) => {
                    let mut dest_file = utils::get_write(&dest)?;
                    template.render(&mut dest_file, &formatted_variables)?
                },
            };

            return Ok(());
        }
        Some(("preview", sub_matches)) => {
            let formatted_variables = matches_to_formatted_variables(&sub_matches)?;
            compile_str(include_str!("../templates/preview.mustache"))?.render(&mut io::stdout(), &formatted_variables)?;
        }
        Some(("list-variables", sub_matches)) => {
            let formatted_variables = matches_to_formatted_variables(&sub_matches)?;
            println!("{}", serde_json::to_string(&formatted_variables)?);
        }
        _ => unreachable!()
    }

    return Ok(());
}
