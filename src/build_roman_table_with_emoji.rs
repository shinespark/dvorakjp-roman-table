use anyhow::Result;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs::{File, read_to_string};
use std::io::Write;
use std::path::PathBuf;

const EMOJI_URL: &str =
    "https://raw.githubusercontent.com/joypixels/emoji-toolkit/master/emoji.json";

type EmojiCode = String;

#[derive(Debug, Deserialize)]
struct EmojiObj {
    // name: String,
    // unicode_version: f32,
    // category: String,
    // order: u32,
    // display: u32,
    shortname: String,
    shortname_alternates: Vec<String>,
    // ascii: Vec<String>,
    // humanform: u32,
    // diversity_base: u32,
    // diversity: Option<Vec<EmojiCode>>,
    // diversity_children: Vec<EmojiCode>,
    // gender: Vec<String>,
    // gender_children: Vec<EmojiCode>,
    code_points: CodePoints,
    // keywords: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct CodePoints {
    // base: EmojiCode,
    fully_qualified: EmojiCode,
    // decimal: String,
    // diversity_parent: Option<EmojiCode>,
    // gender_parent: Option<EmojiCode>,
}

#[derive(Debug)]
struct Emoji {
    name: String,
    name_alternates: Vec<String>,
    char: char,
}

type EmojiVec = Vec<(String, char)>;

pub struct RomanTableWithEmojiBuilder {}

impl RomanTableWithEmojiBuilder {
    pub async fn exec(
        input_file: PathBuf,
        emoji_file: PathBuf,
        output_file: PathBuf,
    ) -> Result<()> {
        let all_emojis = Self::get_emojis().await?;
        let emoji_vec = Self::build_emojis(all_emojis);
        let emoji_records = Self::build_emoji_records(emoji_vec);
        let trimmed_emoji_records = Self::trim_end_unique_name(emoji_records);

        Self::write_emoji_file(trimmed_emoji_records, &emoji_file)?;
        Self::concat_files(input_file, emoji_file, output_file)?;

        Ok(())
    }

    async fn get_emojis() -> Result<Vec<EmojiObj>> {
        let hashmap = reqwest::get(EMOJI_URL)
            .await?
            .json::<HashMap<String, EmojiObj>>()
            .await?;
        let vec = hashmap.into_values().collect::<Vec<EmojiObj>>();

        Ok(vec)
    }

    fn build_emojis(emojis: Vec<EmojiObj>) -> Vec<Emoji> {
        emojis
            .into_iter()
            .filter_map(|x| {
                let emoji_code = x.code_points.fully_qualified.clone();

                if emoji_code.contains('-') {
                    if emoji_code.split('-').count() >= 5 {
                        None
                    } else {
                        Self::build_emoji(x, emoji_code).ok()
                    }
                } else {
                    Self::build_emoji(x, emoji_code).ok()
                }
            })
            .collect::<Vec<_>>()
    }

    fn build_emoji(obj: EmojiObj, code: EmojiCode) -> Result<Emoji> {
        let char = Self::parse_unicode(&code)?;
        let name = obj.shortname.trim_matches(':').to_string();
        let name_alternates = obj
            .shortname_alternates
            .into_iter()
            .map(|x| x.trim_matches(':').to_string())
            .collect::<Vec<_>>();

        Ok(Emoji {
            name,
            name_alternates,
            char,
        })
    }

    fn parse_unicode(input: &EmojiCode) -> Result<char> {
        let unicode = u32::from_str_radix(input, 16)?;
        Ok(char::from_u32(unicode).unwrap())
    }

    fn build_emoji_records(emojis: Vec<Emoji>) -> EmojiVec {
        let mut emojis: Vec<(String, char)> = emojis
            .into_iter()
            .flat_map(|x| {
                let names = vec![vec![x.name], x.name_alternates]
                    .into_iter()
                    .flatten()
                    .collect::<Vec<_>>();

                names
                    .into_iter()
                    .map(|name| (name, x.char))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        emojis.sort_by(|a, b| a.0.cmp(&b.0));

        emojis
    }

    fn trim_end_unique_name(vec: EmojiVec) -> EmojiVec {
        let names = vec.iter().map(|(name, _)| name.clone()).collect::<Vec<_>>();

        vec.into_iter()
            .map(|(name, char)| {
                if Self::has_starts_with_same_name(&name, &names) {
                    (format!(":{}:", name), char)
                } else {
                    (format!(":{}", name), char)
                }
            })
            .collect()
    }

    fn has_starts_with_same_name(name: &str, names: &[String]) -> bool {
        names
            .iter()
            .filter(|&x| x != name) // 自身は除外
            .any(|x| x.starts_with(name))
    }

    fn write_emoji_file(emoji_vec: EmojiVec, emoji_file: &PathBuf) -> Result<()> {
        let mut buffer = File::create(emoji_file)?;
        for (name, char) in emoji_vec {
            buffer
                .write_all(format!("{}\t{}\n", name, char).as_bytes())
                .expect("Unable to write data.");
        }

        Ok(())
    }

    fn concat_files(input_file: PathBuf, emoji_file: PathBuf, output_file: PathBuf) -> Result<()> {
        let input_file_text = read_to_string(input_file)?;
        let emoji_file_text = read_to_string(emoji_file)?;
        let mut buffer = File::create(output_file)?;
        buffer
            .write_all(format!("{}{}", input_file_text, emoji_file_text).as_bytes())
            .expect("Unable to write data.");

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod has_starts_with_same_name {
        use super::*;

        #[test]
        fn any() {
            let result = RomanTableWithEmojiBuilder::has_starts_with_same_name(
                ":basketball",
                &[":basketball_player:".to_string()],
            );
            assert!(result);
        }

        #[test]
        fn not_any() {
            let result = RomanTableWithEmojiBuilder::has_starts_with_same_name(
                ":baseball:",
                &[":basketball_player:".to_string()],
            );
            assert!(!result);
        }
    }
}
