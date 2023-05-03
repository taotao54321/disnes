use std::io::BufWriter;
use std::path::PathBuf;

use anyhow::Context as _;
use clap::Parser;

use disnes::*;

#[derive(Debug, Parser)]
struct Cli {
    #[arg(long, default_value = "disnes.toml")]
    manifest: PathBuf,

    bank_name: String,
}

fn main() -> anyhow::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("warn"));

    let cli = Cli::parse();

    let manifest_toml = std::fs::read_to_string(&cli.manifest)
        .with_context(|| format!("can't read manifest '{}'", cli.manifest.display()))?;
    let manifest = Manifest::from_toml(manifest_toml)?;

    let (input, config) = manifest.into_input_config(cli.bank_name)?;

    let asm = analyze(&input, config.analysis());

    let mut wtr = BufWriter::new(std::io::stdout().lock());
    output_assembly(&mut wtr, &asm)?;

    Ok(())
}
