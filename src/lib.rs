pub fn tokenize<'a>(_chars: String) -> Vec<String> {
    let res = _chars
        .replace("(", " ( ")
        .replace(")", " ) ")
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize() {
        let s = tokenize("(begin (define r 10) (* pi (* r r)))".to_string());
        assert_eq!(s, vec!["(", "begin", "(", "define", "r", "10", ")", "(", "*", "pi", "(", "*", "r", "r", ")", ")", ")"]);
    }
}

