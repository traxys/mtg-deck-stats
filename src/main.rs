use std::iter::FromIterator;

mod input;
mod stats;

#[derive(Debug)]
pub struct Category {
    pub size: u64,
    pub name: String,
}

use input::InputMethod;
use stats::{GameFormat, TurnStats};

fn print_turn_stats(
    stats: &TurnStats<'_>,
    output_file: Option<std::path::PathBuf>,
) -> prettytable::csv::Result<()> {
    use prettytable::{csv::Writer, Attr, Cell, Row, Table};
    let mut table = Table::new();
    // Header
    table.add_row(Row::from_iter(
        vec![Cell::new("")].into_iter().chain(
            stats
                .iter()
                .map(|(cat, _)| Cell::new(&cat.name).with_style(Attr::Bold)),
        ),
    ));
    let until_turn = stats[0].1.len();
    table.add_row(Row::from_iter(
        vec![Cell::new("Starting Hand").with_style(Attr::Bold)]
            .into_iter()
            .chain(stats.iter().map(|(_, t)| Cell::new(&format!("{}", t[0])))),
    ));
    for turn in 1..until_turn {
        table.add_row(Row::from_iter(
            vec![Cell::new(&format!("Turn {}", turn))]
                .into_iter()
                .chain(
                    stats
                        .iter()
                        .map(|(_, t)| Cell::new(&format!("{}", t[turn]))),
                ),
        ));
    }
    table.printstd();
    if let Some(path) = output_file {
        table.to_csv_writer(Writer::from_path(path)?)?;
    }
    Ok(())
}

use structopt::StructOpt;
#[derive(Debug, StructOpt)]
#[structopt(about("Stats to help deckbuilding !"))]
struct Config {
    #[structopt(short, long, default_value = "stdin", possible_values = &["stdin", "file"], help = "Reads from stdin or from <category_file>")]
    input: InputMethod,
    #[structopt(short, long, default_value = "commander", possible_values = &["edh", "commander", "modern", "standard"], help = "Changes the type of deck to have stats on")]
    format: GameFormat,
    #[structopt(
        short,
        long,
        default_value = "15",
        help = "The number of turns to simulate"
    )]
    turns: u64,
    #[structopt(short, long, default_value = "categories", parse(from_os_str))]
    category_file: std::path::PathBuf,
    #[structopt(
        long,
        parse(from_os_str),
        help = "if set outputs a csv file at that location"
    )]
    output: Option<std::path::PathBuf>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Config::from_args();
    let categories = args.input.get_categories(&args.category_file)?;
    let result = args.format.stats(&categories, args.turns);
    print_turn_stats(&result, args.output)?;
    Ok(())
}
