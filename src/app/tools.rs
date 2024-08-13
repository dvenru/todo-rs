pub trait SplitWithQuotes {
    fn split_with_quotes(&self) -> Vec<String>;
}

impl SplitWithQuotes for String {
    fn split_with_quotes(&self) -> Vec<String> {
        let mut result = Vec::new();
        let mut in_quotes = false;
        let mut current_word = String::new();

        for c in self.chars() {
            match c {
                '"' => in_quotes = !in_quotes,
                ' ' if !in_quotes => {
                    if !current_word.is_empty() {
                        result.push(current_word.clone());
                        current_word.clear();
                    }
                }
                _ => current_word.push(c),
            }
        }

        if !current_word.is_empty() {
            result.push(current_word.trim().to_string());
        }

        result
    }
}