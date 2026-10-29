pub struct Parser<'a> {
    input: &'a str,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { input }
    }

    pub fn next_word(&mut self) -> Option<&'a str> {
        self.input = self.input.trim_start();
        if self.input.is_empty() {
            return None;
        }
        let end = self
            .input
            .find(|c: char| c.is_whitespace())
            .unwrap_or(self.input.len());
        let word = &self.input[..end];
        self.input = &self.input[end..];
        Some(word)
    }

    pub fn words(self) -> Words<'a> {
        Words { parser: self }
    }
}

pub struct Words<'a> {
    parser: Parser<'a>,
}

impl<'a> Iterator for Words<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        self.parser.next_word()
    }
}

pub fn greeting() -> &'static str {
    "hello shop"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_words() {
        let mut parser = Parser::new("mug bowl");
        assert_eq!(parser.next_word(), Some("mug"));
        assert_eq!(parser.next_word(), Some("bowl"));
        assert_eq!(parser.next_word(), None);
    }

    #[test]
    fn iterator_borrows_input() {
        let text = String::from("teapot mug");
        let words: Vec<&str> = Parser::new(&text).words().collect();
        assert_eq!(words, vec!["teapot", "mug"]);
    }

    #[test]
    fn static_greeting() {
        assert_eq!(greeting(), "hello shop");
    }
}
