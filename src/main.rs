use std::path::PathBuf;

use sentence_extractor::get_sentences;

fn main() {
    let path: PathBuf = "./fixtures".into();
    let articles = get_sentences(path);
    println!("{:?}", articles[0][0]);

    let article =
        get_sentences("./fixtures/2022-05-11-typescript-iterating-over-objects.md".into());

    println!("{:?}", article[0][0]);
}
