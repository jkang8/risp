// TODO: handle floats
pub enum Atom {
    Symbol(String),
    Int(i32),
}

pub fn tokenize(_chars: String) -> Vec<String> {
    let res = _chars
        .replace("(", " ( ")
        .replace(")", " ) ")
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();
    res
}

//pub fn read_from_tokens(mut tokens: Vec<String>) -> Atom {
////    let token = tokens.pop();
//    let atom = Atom.new();
//    atom
//}
pub fn atom(token: String) -> Atom {
    let num = token.parse::<i32>();
    match num {
        Ok(_val) => Atom::Int(_val),
        Err(_why) => Atom::Symbol(token),
    }
//    return Atom::Symbol(token)
}

pub fn parse(program: String) ->  Vec<String> {
    let tokens = tokenize(program);
    tokens
//    let res = read_from_tokens(tokens);
//    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_atom() {
        let input1 = "asdf".to_string();
        let _res1 = atom(input1.to_string());
        match _res1 {
            Atom::Symbol(s) => assert_eq!(s, "asdf"),
            Atom::Int(i) => println!("{}", i.to_string())
        }

        let input2 = "1".to_string();
        let _res2 = atom(input2.to_string());
        match _res2 {
            Atom::Symbol(s) => println!("{}", s),
            Atom::Int(i) => assert_eq!(i, 1),
        }
    }

    #[test]
    fn test_tokenize() {
        let s = tokenize("(begin (define r 10) (* pi (* r r)))".to_string());
        assert_eq!(s, vec!["(", "begin", "(", "define", "r", "10", ")", "(", "*", "pi", "(", "*", "r", "r", ")", ")", ")"]);
    }

    #[test]
    fn test_parse() {
        let s = parse("(begin (define r 10) (* pi (* r r)))".to_string());
        assert_eq!(s, vec!["(", "begin", "(", "define", "r", "10", ")", "(", "*", "pi", "(", "*", "r", "r", ")", ")", ")"]);
    }
}

