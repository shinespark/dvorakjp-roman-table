use anyhow::Result;
use std::fs;
use std::path::PathBuf;

/// 入力キーの各文字に対するソート優先度（五十音順ベース）。
/// 母音は「あいうえお」順、子音は五十音の行順で定義されます。
/// この配列に含まれない文字は末尾に配置されます。
const CHAR_ORDER: &[char] = &[
    'a', 'i', 'u', 'e', 'o', // あ行（母音）
    'k', // か行
    'c', // か行（代替）
    's', // さ行
    't', // た行
    'n', // な行
    'h', // は行
    'm', // ま行
    'y', // や行
    'r', // ら行
    'w', // わ行
    'g', // が行
    'z', // ざ行
    'd', // だ行
    'b', // ば行
    'p', // ぱ行
    'f', 'j', 'v', 'q', // その他の子音
    'l', // 小書き
    'x', // 小書き
];

pub struct RomanTableBuilder {}

impl RomanTableBuilder {
    pub fn build(input_dir: PathBuf, output_file: PathBuf) -> Result<()> {
        let roman_table = Self::assemble(&input_dir)?;
        fs::write(&output_file, roman_table.join("\n"))?;
        println!("ローマ字テーブルを生成しました: {}", output_file.display());

        Ok(())
    }

    fn assemble(input_dir: &PathBuf) -> Result<Vec<String>> {
        let tsv_files = Self::read_dir(input_dir)?;
        let raw_contents = Self::read_files(tsv_files)?;
        let contents = Self::remove_empty_lines(raw_contents);
        let sorted = Self::sort_lines(contents);

        Ok(sorted)
    }

    fn read_dir(input_dir: &PathBuf) -> Result<Vec<PathBuf>> {
        let tsv_files: Vec<PathBuf> = fs::read_dir(input_dir)?
            .filter_map(|entry| {
                let entry = entry.ok()?;
                let path = entry.path();
                if path.extension().and_then(|e| e.to_str()) == Some("tsv") {
                    Some(path)
                } else {
                    None
                }
            })
            .collect();

        Ok(tsv_files)
    }

    fn read_files(tsv_files: Vec<PathBuf>) -> Result<Vec<String>> {
        let lines: Vec<String> = tsv_files
            .iter()
            .map(|file| fs::read_to_string(file).map_err(Into::into))
            .collect::<Result<Vec<String>>>()?
            .into_iter()
            .flat_map(|content| content.lines().map(String::from).collect::<Vec<_>>())
            .collect();

        Ok(lines)
    }

    fn remove_empty_lines(lines: Vec<String>) -> Vec<String> {
        lines
            .into_iter()
            .filter(|line| {
                let trimmed = line.trim();
                !trimmed.is_empty() && !trimmed.starts_with('#')
            })
            .collect()
    }

    fn sort_lines(mut lines: Vec<String>) -> Vec<String> {
        lines.sort_by(|a, b| Self::sort_key(a).cmp(&Self::sort_key(b)));
        lines
    }

    fn sort_key(line: &str) -> Vec<usize> {
        let input = line.split('\t').next().unwrap_or("");
        input
            .chars()
            .map(|c| {
                let c = c.to_ascii_lowercase();
                CHAR_ORDER
                    .iter()
                    .position(|&x| x == c)
                    .unwrap_or(CHAR_ORDER.len())
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn assemble_matches_dvorakjp_prime() {
        let input_dir = PathBuf::from("./data/roman_table");
        let expected_file = PathBuf::from("./google_japanese_input/dvorakjp_prime.txt");

        let assembled = RomanTableBuilder::assemble(&input_dir).expect("assemble に失敗");
        let expected_content =
            fs::read_to_string(&expected_file).expect("dvorakjp_prime.txt の読み込みに失敗");
        let expected: Vec<&str> = expected_content
            .lines()
            .filter(|line| !line.trim().is_empty())
            .collect();

        let assembled_set: HashSet<&str> = assembled.iter().map(|s| s.as_str()).collect();
        let expected_set: HashSet<&str> = expected.into_iter().collect();

        let mut only_in_assembled: Vec<&&str> = assembled_set.difference(&expected_set).collect();
        let mut only_in_expected: Vec<&&str> = expected_set.difference(&assembled_set).collect();
        only_in_assembled.sort();
        only_in_expected.sort();

        assert!(
            only_in_assembled.is_empty() && only_in_expected.is_empty(),
            "ローマ字テーブルに差分があります:\n  assemble にのみ存在: {:?}\n  dvorakjp_prime.txt にのみ存在: {:?}",
            only_in_assembled,
            only_in_expected
        );
    }
}
