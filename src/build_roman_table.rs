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

    #[test]
    fn test_remove_empty_lines() {
        let lines = vec![
            "ka\tか".to_string(),
            "".to_string(),
            "ki\tき".to_string(),
            "# コメント".to_string(),
            "  ".to_string(),
            "ku\tく".to_string(),
        ];
        let result = RomanTableBuilder::remove_empty_lines(lines);
        assert_eq!(result, vec!["ka\tか", "ki\tき", "ku\tく"]);
    }

    #[test]
    fn test_sort_lines() {
        let lines = vec![
            "ku\tく".to_string(),
            "ka\tか".to_string(),
            "sa\tさ".to_string(),
            "ki\tき".to_string(),
            "ai\tあい".to_string(),
        ];
        let result = RomanTableBuilder::sort_lines(lines);
        assert_eq!(
            result,
            vec!["ai\tあい", "ka\tか", "ki\tき", "ku\tく", "sa\tさ"]
        );
    }

    #[test]
    fn test_sort_key() {
        // あ行 < か行 < さ行
        let key_a = RomanTableBuilder::sort_key("a\tあ");
        let key_ka = RomanTableBuilder::sort_key("ka\tか");
        let key_sa = RomanTableBuilder::sort_key("sa\tさ");

        assert!(key_a < key_ka);
        assert!(key_ka < key_sa);

        // 同じ子音では母音順: ka < ki < ku < ke < ko
        let key_ka = RomanTableBuilder::sort_key("ka\tか");
        let key_ki = RomanTableBuilder::sort_key("ki\tき");
        let key_ku = RomanTableBuilder::sort_key("ku\tく");
        let key_ke = RomanTableBuilder::sort_key("ke\tけ");
        let key_ko = RomanTableBuilder::sort_key("ko\tこ");

        assert!(key_ka < key_ki);
        assert!(key_ki < key_ku);
        assert!(key_ku < key_ke);
        assert!(key_ke < key_ko);
    }
}
