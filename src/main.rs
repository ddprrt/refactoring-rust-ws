use std::path::PathBuf;

use sentence_extractor::get_sentences;

fn main() {
    let path: PathBuf = "./fixtures".into();
    let articles = get_sentences(path);
    println!("{:?}", articles[0][0]);
}
