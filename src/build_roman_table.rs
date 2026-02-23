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

#[derive(Debug)]
struct Romaji {
    input: String,
    output: String,
    next_input: Option<String>,
}

impl Romaji {
    fn from_line(line: &str) -> Option<Self> {
        let mut parts = line.splitn(3, '\t');
        let input = parts.next()?.to_string();
        let output = parts.next()?.to_string();
        let next_input = parts.next().map(|s| s.to_string());
        Some(Romaji {
            input,
            output,
            next_input,
        })
    }

    fn to_line(&self) -> String {
        match &self.next_input {
            Some(next) => format!("{}\t{}\t{}", self.input, self.output, next),
            None => format!("{}\t{}", self.input, self.output),
        }
    }
}

pub struct RomanTableBuilder {}

impl RomanTableBuilder {
    pub fn build(input_dirs: &[PathBuf], output_file: PathBuf) -> Result<()> {
        let roman_table = Self::assemble(input_dirs)?;
        if let Some(parent) = output_file.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(&output_file, roman_table.join("\n"))?;
        println!("ローマ字テーブルを生成しました: {}", output_file.display());

        Ok(())
    }

    fn assemble(input_dirs: &[PathBuf]) -> Result<Vec<String>> {
        let tsv_files: Vec<PathBuf> = input_dirs
            .iter()
            .map(|dir| Self::read_dir(dir))
            .collect::<Result<Vec<_>>>()?
            .into_iter()
            .flatten()
            .collect();
        let raw_lines = Self::read_files(tsv_files)?;
        let lines = Self::filter_lines(raw_lines);
        let entries = Self::to_romaji(lines);
        let sorted = Self::sort_romaji(entries);

        Ok(sorted.into_iter().map(|r| r.to_line()).collect())
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

    fn filter_lines(lines: Vec<String>) -> Vec<String> {
        lines
            .into_iter()
            .filter(|line| {
                let trimmed = line.trim();
                !trimmed.is_empty() && !trimmed.starts_with('#')
            })
            .collect()
    }

    fn to_romaji(lines: Vec<String>) -> Vec<Romaji> {
        lines
            .into_iter()
            .filter_map(|line| Romaji::from_line(&line))
            .collect()
    }

    fn sort_romaji(mut entries: Vec<Romaji>) -> Vec<Romaji> {
        entries.sort_by(|a, b| Self::sort_key(&a.input).cmp(&Self::sort_key(&b.input)));
        entries
    }

    fn sort_key(input: &str) -> Vec<usize> {
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

    #[test]
    fn test_filter_lines() {
        let lines = vec![
            "ka\tか".to_string(),
            "".to_string(),
            "ki\tき".to_string(),
            "# コメント".to_string(),
            "  ".to_string(),
            "ku\tく".to_string(),
        ];
        let result = RomanTableBuilder::filter_lines(lines);
        assert_eq!(result, vec!["ka\tか", "ki\tき", "ku\tく"]);
    }

    #[test]
    fn test_to_romaji() {
        let lines = vec![
            "ka\tか".to_string(),
            "ki\tき".to_string(),
            "ku\tく".to_string(),
        ];
        let result = RomanTableBuilder::to_romaji(lines);
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].input, "ka");
        assert_eq!(result[0].output, "か");
        assert!(result[0].next_input.is_none());
        assert_eq!(result[1].input, "ki");
        assert_eq!(result[1].output, "き");
        assert_eq!(result[2].input, "ku");
        assert_eq!(result[2].output, "く");
    }

    #[test]
    fn test_sort_romaji() {
        let entries = vec![
            Romaji {
                input: "ku".to_string(),
                output: "く".to_string(),
                next_input: None,
            },
            Romaji {
                input: "ka".to_string(),
                output: "か".to_string(),
                next_input: None,
            },
            Romaji {
                input: "sa".to_string(),
                output: "さ".to_string(),
                next_input: None,
            },
            Romaji {
                input: "ki".to_string(),
                output: "き".to_string(),
                next_input: None,
            },
            Romaji {
                input: "ai".to_string(),
                output: "あい".to_string(),
                next_input: None,
            },
        ];
        let result = RomanTableBuilder::sort_romaji(entries);
        let inputs: Vec<&str> = result.iter().map(|r| r.input.as_str()).collect();
        assert_eq!(inputs, vec!["ai", "ka", "ki", "ku", "sa"]);
    }

    #[test]
    fn test_sort_key() {
        // あ行 < か行 < さ行
        let key_a = RomanTableBuilder::sort_key("a");
        let key_ka = RomanTableBuilder::sort_key("ka");
        let key_sa = RomanTableBuilder::sort_key("sa");

        assert!(key_a < key_ka);
        assert!(key_ka < key_sa);

        // 同じ子音では母音順: ka < ki < ku < ke < ko
        let key_ka = RomanTableBuilder::sort_key("ka");
        let key_ki = RomanTableBuilder::sort_key("ki");
        let key_ku = RomanTableBuilder::sort_key("ku");
        let key_ke = RomanTableBuilder::sort_key("ke");
        let key_ko = RomanTableBuilder::sort_key("ko");

        assert!(key_ka < key_ki);
        assert!(key_ki < key_ku);
        assert!(key_ku < key_ke);
        assert!(key_ke < key_ko);
    }
}
