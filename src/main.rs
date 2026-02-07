use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;

use dvorakjp_romantable::build_roman_table_with_emoji::RomanTableWithEmojiBuilder;
use dvorakjp_romantable::detect_duplicates::DuplicateDetector;

const DEFAULT_EMOJI_FILE: &str = "./data/emoji.txt";
const DEFAULT_INPUT_FILE: &str = "./google_japanese_input/dvorakjp_prime.txt";
const DEFAULT_OUTPUT_FILE: &str = "./google_japanese_input/dvorakjp_prime_with_emoji.txt";

#[derive(Parser)]
#[clap(name = "cargo")]
#[clap(bin_name = "cargo")]
enum Cargo {
    BuildRomanTableWithEmoji(BuildRomanTableWithEmoji),
    DetectDuplicates(DetectDuplicates),
}

#[derive(clap::Args)]
#[clap(author, version, about, long_about = None)]
struct BuildRomanTableWithEmoji {
    #[clap(long)]
    input_file: Option<PathBuf>,

    #[clap(long)]
    emoji_file: Option<PathBuf>,

    #[clap(long)]
    output_file: Option<PathBuf>,
}

#[derive(clap::Args)]
#[clap(author, version, about, long_about = None)]
struct DetectDuplicates {
    #[clap(long)]
    detect_file: PathBuf,
}

#[tokio::main]
async fn main() -> Result<()> {
    let _ = match Cargo::parse() {
        Cargo::BuildRomanTableWithEmoji(args) => {
            RomanTableWithEmojiBuilder::exec(
                args.input_file
                    .unwrap_or_else(|| PathBuf::from(DEFAULT_INPUT_FILE)),
                args.emoji_file
                    .unwrap_or_else(|| PathBuf::from(DEFAULT_EMOJI_FILE)),
                args.output_file
                    .unwrap_or_else(|| PathBuf::from(DEFAULT_OUTPUT_FILE)),
            )
            .await
        }
        Cargo::DetectDuplicates(args) => DuplicateDetector::exec(args.detect_file),
    };
    Ok(())
}
