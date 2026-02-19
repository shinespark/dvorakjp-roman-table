use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;

use dvorakjp_romantable::build_roman_table::RomanTableBuilder;
use dvorakjp_romantable::build_roman_table_with_emoji::RomanTableWithEmojiBuilder;
use dvorakjp_romantable::detect_duplicates::DuplicateDetector;

const DEFAULT_ROMAN_TABLE_INPUT_DIR: &str = "./data/roman_table";
const DEFAULT_ROMAN_TABLE_OUTPUT_FILE: &str = "./outputs/dvorak_jp.tsv";
const DEFAULT_EMOJI_OUTPUT_FILE: &str = "./outputs/emoji.tsv";
const DEFAULT_ROMAN_TABLE_INPUT_FILE: &str = "./outputs/dvorak_jp.tsv";
const DEFAULT_ROMAN_TABLE_WITH_EMOJI_OUTPUT_FILE: &str = "./outputs/dvorak_jp_with_emoji.tsv";

#[derive(Parser)]
#[clap(name = "cargo")]
#[clap(bin_name = "cargo")]
enum Cargo {
    Build(Build),
    Detect(Detect),
}

#[derive(clap::Args)]
#[clap(about = "ローマ字テーブルのビルド")]
struct Build {
    #[clap(subcommand)]
    command: BuildCommand,
}

#[derive(clap::Subcommand)]
enum BuildCommand {
    RomanTable(BuildRomanTable),
    RomanTableWithEmoji(BuildRomanTableWithEmoji),
}

#[derive(clap::Args)]
struct BuildRomanTable {
    #[clap(long)]
    input_dir: Option<PathBuf>,

    #[clap(long)]
    output_file: Option<PathBuf>,
}

#[derive(clap::Args)]
struct BuildRomanTableWithEmoji {
    #[clap(long)]
    input_file: Option<PathBuf>,

    #[clap(long)]
    emoji_file: Option<PathBuf>,

    #[clap(long)]
    output_file: Option<PathBuf>,
}

#[derive(clap::Args)]
#[clap(about = "ローマ字テーブルの検証")]
struct Detect {
    #[clap(subcommand)]
    command: DetectCommand,
}

#[derive(clap::Subcommand)]
enum DetectCommand {
    Duplicates(DetectDuplicates),
}

#[derive(clap::Args)]
struct DetectDuplicates {
    #[clap(long)]
    detect_file: PathBuf,
}

#[tokio::main]
async fn main() -> Result<()> {
    let _ = match Cargo::parse() {
        Cargo::Build(build) => match build.command {
            BuildCommand::RomanTable(args) => RomanTableBuilder::build(
                args.input_dir
                    .unwrap_or_else(|| PathBuf::from(DEFAULT_ROMAN_TABLE_INPUT_DIR)),
                args.output_file
                    .unwrap_or_else(|| PathBuf::from(DEFAULT_ROMAN_TABLE_OUTPUT_FILE)),
            ),
            BuildCommand::RomanTableWithEmoji(args) => {
                RomanTableWithEmojiBuilder::build(
                    args.input_file
                        .unwrap_or_else(|| PathBuf::from(DEFAULT_ROMAN_TABLE_INPUT_FILE)),
                    args.emoji_file
                        .unwrap_or_else(|| PathBuf::from(DEFAULT_EMOJI_OUTPUT_FILE)),
                    args.output_file.unwrap_or_else(|| {
                        PathBuf::from(DEFAULT_ROMAN_TABLE_WITH_EMOJI_OUTPUT_FILE)
                    }),
                )
                .await
            }
        },
        Cargo::Detect(detect) => match detect.command {
            DetectCommand::Duplicates(args) => DuplicateDetector::exec(args.detect_file),
        },
    };
    Ok(())
}
