use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;

use dvorakjp_romantable::build_roman_table::RomanTableBuilder;
use dvorakjp_romantable::with_emoji::RomanTableWithEmojiBuilder;

const DEFAULT_ROMAN_TABLE_INPUT_DIR: &str = "./data/roman_table";

#[derive(clap::ValueEnum, Clone)]
enum ImeTarget {
    #[value(name = "azooKey")]
    AzooKey,
    #[value(name = "google-japanese-input")]
    GoogleJapaneseInput,
}

#[derive(Parser)]
#[clap(name = "cargo")]
#[clap(bin_name = "cargo")]
enum Cargo {
    Build(Build),
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
    WithEmoji(WithEmoji),
}

#[derive(clap::Args)]
#[clap(about = "ローマ字テーブルのビルド")]
struct BuildRomanTable {
    target: Option<ImeTarget>,
}

#[derive(clap::Args)]
#[clap(about = "ローマ字テーブルを絵文字付きでビルド")]
struct WithEmoji {
    target: Option<ImeTarget>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let _ = match Cargo::parse() {
        Cargo::Build(build) => match build.command {
            BuildCommand::RomanTable(args) => {
                let base_dir = PathBuf::from(DEFAULT_ROMAN_TABLE_INPUT_DIR);
                let configs: &[(&str, &str)] = match args.target {
                    Some(ImeTarget::AzooKey) => &[("azooKey", "./outputs/azooKey/dvorak_jp.tsv")],
                    Some(ImeTarget::GoogleJapaneseInput) => &[(
                        "google_japanese_input",
                        "./outputs/google_japanese_input/dvorak_jp.tsv",
                    )],
                    None => &[
                        ("azooKey", "./outputs/azooKey/dvorak_jp.tsv"),
                        (
                            "google_japanese_input",
                            "./outputs/google_japanese_input/dvorak_jp.tsv",
                        ),
                    ],
                };
                configs.iter().try_for_each(|(subdir_name, output_path)| {
                    let specific_dir = base_dir.join(subdir_name);
                    let dirs = if specific_dir.exists() {
                        vec![base_dir.clone(), specific_dir]
                    } else {
                        vec![base_dir.clone()]
                    };
                    RomanTableBuilder::build(&dirs, PathBuf::from(output_path))
                })
            }
            BuildCommand::WithEmoji(args) => match args.target {
                Some(ImeTarget::AzooKey) => {
                    anyhow::bail!("with-emoji は azooKey では未対応です")
                }
                Some(ImeTarget::GoogleJapaneseInput) | None => {
                    let configs = vec![(
                        PathBuf::from("./outputs/google_japanese_input/dvorak_jp.tsv"),
                        PathBuf::from("./outputs/google_japanese_input/dvorak_jp_with_emoji.tsv"),
                    )];
                    RomanTableWithEmojiBuilder::build(&configs).await
                }
            },
        },
    };
    Ok(())
}
