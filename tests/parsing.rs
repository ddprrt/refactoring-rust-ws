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

#[test]
fn only_article() {
    let article =
        get_sentences("./fixtures/2022-05-11-typescript-iterating-over-objects.md".into());
    assert_eq!(article[0][0].as_str(), "There is rarely a head-scratcher in TypeScript as prominent as trying to access an object property via iterating through its keys.")
}
