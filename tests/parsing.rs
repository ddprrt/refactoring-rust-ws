use sentence_extractor::get_sentences;

#[test]
fn correct_articles() {
    let articles = get_sentences("./fixtures".into());
    assert_eq!(articles.len(), 2);
}

#[test]
fn first_sentence_correct() {
    let articles = get_sentences("./fixtures".into());
    assert_eq!(articles[0][0].as_str(), "The following piece of code takes a `PathBuf` and extracts the file name, eventually converting it to an _owned_ `String`.")
}
