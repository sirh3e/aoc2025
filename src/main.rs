use aoc::cmd::Cli;
use clap::Parser;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    cli.execute()
        .inspect(|result| println!("result: {}", result))?;

    Ok(())
}
