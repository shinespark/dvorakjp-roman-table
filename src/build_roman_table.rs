use anyhow::Result;
use std::fs;
use std::path::PathBuf;

pub struct RomanTableBuilder {}

impl RomanTableBuilder {
    pub fn exec(input_dir: PathBuf, output_file: PathBuf) -> Result<()> {
        let tsv_files = Self::read_dir(&input_dir)?;
        let raw_contents = Self::read_files(tsv_files)?;
        let contents = Self::remove_empty_lines(raw_contents);
        fs::write(&output_file, contents.join("\n") + "\n")?;
        println!("ローマ字テーブルを生成しました: {}", output_file.display());

        Ok(())
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
            .filter(|line| !line.trim().is_empty())
            .collect()
    }
}
