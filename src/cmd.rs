use crate::years::Input;
use crate::years::year_2025::day01::{solve_part1, solve_part2};
use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "aoc")]
pub struct Cli {
    #[command(subcommand)]
    year: Year,
}

impl Cli {
    pub fn execute(&self) -> anyhow::Result<usize> {
        match &self.year {
            Year::Year2025(day) => match day {
                Year2025Day::Day01 { part, file } => {
                    let input = std::fs::read_to_string(file).map(Input::from)?;
                    match part {
                        Part::One => solve_part1(&input),
                        Part::Two => solve_part2(&input),
                    }
                }
            },
        }
    }
}

#[derive(Debug, Subcommand)]
pub enum Year {
    #[command(subcommand)]
    Year2025(Year2025Day),
}

#[derive(Debug, Subcommand)]
pub enum Year2025Day {
    Day01 {
        #[clap(short, long)]
        part: Part,
        #[clap(short, long)]
        file: PathBuf,
    },
}

#[derive(Debug, Clone, ValueEnum)]
pub enum Part {
    One,
    Two,
}
