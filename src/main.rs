use std::path::PathBuf;

fn get_markdown_files(
    path: impl Into<PathBuf>,
    ext: impl AsRef<str>,
) -> Result<Vec<PathBuf>, std::io::Error> {
    let path = path.into();
    let mut files = Vec::new();
    for entry in path.read_dir()? {
        let path = entry?.path();
        if path.is_file() && path.extension().map(|e| e == ext.as_ref()).unwrap_or(false) {
            files.push(path);
        }
    }
    Ok(files)
}

fn read_files(files: Vec<PathBuf>) -> Result<Vec<String>, std::io::Error> {
    let mut contents = Vec::new();
    for file in files {
        let content = std::fs::read_to_string(file)?;
        contents.push(content);
    }
    Ok(contents)
}

fn parse_markdown_to_sentences(content: String) -> Vec<String> {
    let mut sentences = Vec::new();
    let mut sentence = String::new();
    let mut in_code_block = false;
    let mut in_preamble = false;
    for line in content.lines() {
        // Remove empty lines
        if line.is_empty() {
            continue;
        }
        // Remove Headlines
        if line.starts_with('#') {
            continue;
        }
        // Remove preamble
        if line.starts_with("---") {
            in_preamble = !in_preamble;
            continue;
        }
        if in_preamble {
            continue;
        }
        // Parse code blocks
        if line.starts_with("```") {
            if in_code_block {
                sentences.push(sentence);
                sentence = String::new();
            }
            in_code_block = !in_code_block;
            continue;
        }
        // Look for "." within a line
        if line.contains(". ") && !in_code_block {
            let line = line.split(". ");
            let count = line.clone().count();
            for (idx, sentence_part) in line.enumerate() {
                if sentence_part.is_empty() {
                    continue;
                }
                sentence.push_str(sentence_part);
                if idx < count - 1 {
                    sentence.push('.');
                    sentences.push(sentence);
                    sentence = String::new();
                } else {
                    sentence.push(' ');
                }
            }
            continue;
        }
        if line.ends_with('.') {
            sentence.push_str(line);
            sentences.push(sentence);
            sentence = String::new();
        } else {
            sentence.push_str(line);
            if in_code_block {
                sentence.push('\n');
            } else {
                sentence.push(' ');
            }
        }
    }
    sentences
}

fn main() {
    let files = get_markdown_files("../../../Web/ddprrt.github.io/src/content/_posts", "md");
    let files = read_files(files.expect("No Markdown files found"));
    /*let files = files
    .expect("Error reading files")
    .into_iter()
    .flat_map(parse_markdown_to_sentences)
    .collect::<Vec<_>>();*/
    let files = files.expect("Error reading files");
    let last = files.last().expect("No files found").to_owned();
    let sentences = parse_markdown_to_sentences(last);
    for (i, sentence) in sentences.into_iter().enumerate() {
        println!("{i}, {}", sentence);
    }
}
