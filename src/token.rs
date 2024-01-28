type Code<'a> = std::iter::Peekable<std::str::Chars<'a>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Token {
    // Misc
    Number(i64),
    Comma,

    // Instructions
    Nop,
    DB,
    DW,
}

pub struct Tokens<'a> {
    code : Code<'a>,
}

impl<'a> Iterator for Tokens<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        get_token(&mut self.code)
    }
}

fn collect_while(code : &mut Code, predicate : impl Fn(&char) -> bool) -> String {
    let mut s = String::new();
    while let Some(c) = code.peek() {
        if !predicate(c) {
            break
        }

        s.push(*c);
        code.next();
    }
    s
}

fn collect_until(code : &mut Code, predicate : impl Fn(&char) -> bool) -> String {
    collect_while(code, |c| !predicate(c))
}

fn skip_whitespace(code : &mut Code) -> Option<char> {
    collect_while(code, |c| c.is_whitespace());
    let mut c = code.next();
    if accept_comment(c, code.peek()) {
        skip_comment(code);
        c = skip_whitespace(code);
    }
    c
}

fn accept_comment(c : Option<char>, next : Option<&char>) -> bool {
    c.is_some_and(|c| next.is_some_and(|&next| c == '/' && next == '/'))
}

fn skip_comment(code : &mut Code) {
    collect_until(code, |&c| c == '\r' || c == '\n'); 
}

fn accept_identifier(c : char) -> bool {
    c.is_alphabetic() || c == '_'
}

fn get_identifier(c : char, code : &mut Code) -> Token {
    let ident = c.to_string() + &collect_while(code, |c| c.is_alphanumeric() || c == &'_');

    match &*ident {
        // Instructions
        "nop" => Token::Nop,
        "db" => Token::DB,
        "dw" => Token::DW,

        _ => todo!("{ident}"),
    }
}

fn accept_number(c : char) -> bool {
    c.is_ascii_digit() || c == '-'
}

fn get_number(c : char, code : &mut Code) -> Token {
    let (pfx, base) = if c == '0' {
        match code.peek() {
            Some('x') => {
                code.next();
                ("".to_string(), 16)
            },
            Some('o') => {
                code.next();
                ("".to_string(), 8)
            },
            Some('b') => {
                code.next();
                ("".to_string(), 2)
            },
            Some(_) => {
                (c.to_string(), 10)
            }
            None => return Token::Number(0),
        }
    } else { (c.to_string(), 10) };

    let s = pfx.clone() + &collect_while(code, |c| c.is_ascii_hexdigit() || *c == '_').chars().filter(|c| *c != '_').collect::<String>();
    Token::Number(i64::from_str_radix(&s, base).unwrap())
}

fn get_token(code : &mut Code) -> Option<Token> {
    let c = skip_whitespace(code)?;
    if accept_identifier(c) {
        Some(get_identifier(c, code))
    } else if accept_number(c) {
        Some(get_number(c, code))
    } else {
        match c {
            ',' => Some(Token::Comma),
            _ => todo!("'{}'", c as u8),
        }
    }
}

pub fn tokenize(code : &str) -> Tokens {
    Tokens { code: code.chars().peekable() }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn number() {
        let code = "0 0x0 0o0 0b0 3273 -3274 0xF339 0o171472 0b1111001100111011 0b1111_0011_0011_1100";
        let mut toks = tokenize(code);

        assert_eq!(toks.next(), Some(Token::Number(0)));
        assert_eq!(toks.next(), Some(Token::Number(0)));
        assert_eq!(toks.next(), Some(Token::Number(0)));
        assert_eq!(toks.next(), Some(Token::Number(0)));

        assert_eq!(toks.next(), Some(Token::Number(3273)));
        assert_eq!(toks.next(), Some(Token::Number(-3274)));
        assert_eq!(toks.next(), Some(Token::Number(0xF339)));
        assert_eq!(toks.next(), Some(Token::Number(0o171472)));
        assert_eq!(toks.next(), Some(Token::Number(0b1111001100111011)));
        assert_eq!(toks.next(), Some(Token::Number(0b1111_0011_0011_1100)));
    }
}
