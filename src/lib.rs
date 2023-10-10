use std::path::PathBuf;

pub fn get_sentences(path: PathBuf) -> Vec<Vec<String>> {
    let mut files = Vec::new();
    for entry in path.read_dir().unwrap() {
        let path = entry.unwrap().path();
        if path.is_file() && path.extension().unwrap() == "md" {
            files.push(path);
        }
    }
    let mut contents = Vec::new();
    for file in files {
        let content = std::fs::read_to_string(file).unwrap();
        contents.push(content);
    }

    let mut parsed = Vec::new();

    for article in contents {
        let mut sentences = Vec::new();
        let mut sentence = String::new();
        let mut in_code_block = false;
        let mut in_preamble = false;
        for line in article.lines() {
            if line.is_empty() {
                continue;
            }
            if line.starts_with('#') {
                continue;
            }
            if line.starts_with("---") {
                in_preamble = !in_preamble;
                continue;
            }
            if in_preamble {
                continue;
            }
            if line.starts_with("```") {
                if in_code_block {
                    sentences.push(sentence);
                    sentence = String::new();
                }
                in_code_block = !in_code_block;
                continue;
            }
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
        parsed.push(sentences);
    }
    parsed
}
