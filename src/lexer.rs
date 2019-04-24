use std::str::CharIndices;

#[derive(Debug, PartialEq, Eq)]
pub enum Token<'a> {
    LParen,
    RParen,
    Quote,
    Float(&'a str),
    Int(&'a str),
    Symbol(&'a str),
    StrLit(&'a str),
    Comment(&'a str),
}

#[derive(Debug)]
pub struct Lexer<'a> {
    source: &'a str,
    chars: CharIndices<'a>,
    lookahead: Option<char>,
    pos: usize,
    line_number: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Lexer<'a> {
        let mut lex = Self {
            source,
            chars: source.char_indices(),
            lookahead: None,
            pos: 0,
            line_number: 1,
        };

        lex.next_char();

        // lex.
        lex
    }

    pub fn iter(&'a mut self) -> Iter<'a> {
        Iter {
            inner: self
        }
    }


    fn next_char(&mut self) -> Option<char> {
        // If we hit a line, count it.
        if let Some('\n') = self.lookahead {
            self.line_number += 1;
        }

        match self.chars.next() {
            // If another char exists, update the lexer.
            Some((idx, ch)) => {
                self.pos = idx;
                self.lookahead = Some(ch);
            },

            // We're finished
            None => {
                self.pos = self.source.len();
                self.lookahead = None;
            }
        }

        self.lookahead
    }

    fn scan_char(&mut self, token: Token<'a>) -> Token<'a> {
        self.next_char();
        token
    }

    fn looking_at(&self, prefix: &'a str) -> bool {
        self.source[self.pos..].starts_with(prefix)
    }

    fn looking_at_str(&self) -> bool {
        match self.lookahead {
            Some('"') => true,
            _ => false,
        }
    }

    fn rest_of_line(&mut self) -> &'a str {
        let start = self.pos;

        loop {
            match self.next_char() {
                None | Some('\n') => return &self.source[start..self.pos],
                _ => {}
            }
        }
    }

    fn scan_comment(&mut self) -> Token<'a> {
        // Consume the ';'
        self.next_char();

        let text = self.rest_of_line();
        Token::Comment(text)
    }

    fn scan_string(&mut self) -> Token<'a> {
        let start = self.pos;
        
        assert!(self.looking_at_str());
        
        loop {
            match self.next_char() {
                Some('"') => break,
                Some(_) if self.looking_at("\\\"") => {
                    // We're looking at a \, and know that the next
                    // char is a ", so consume both, the " will be
                    // consumed on the next iteration;
                    assert_eq!(self.next_char(), Some('"'));
                },
                None => panic!("Malformed string"),
                _ => {},    
            }
        }

        // Consume the end quote
        self.next_char();

        let pre_text = &self.source[start + 1..self.pos - 1];

        Token::StrLit(pre_text)
    }

    fn scan_number(&mut self) -> Token<'a> {
        let start = self.pos;
        let mut is_float = false;

        match self.lookahead {
            Some('-') => {
                self.next_char();
                if !self.looking_at_numeric() {
                    return Token::Symbol("-");
                }
            }
            Some('+') => {
                self.next_char();
                if !self.looking_at_numeric() {
                    return Token::Symbol("+");
                }
            }
            _ => {}
        }

        if let Some('.') = self.lookahead {
            is_float = true;
            
            match self.next_char() {
                Some(c) if c.is_numeric() => {},
                _ => panic!(),
            }
        }

        loop {
            match self.next_char() {
                Some('.') => is_float = true,
                Some(ch) if ch.is_numeric() => {}
                _ => break,
            }
        }

        let text = &self.source[start..self.pos];

        if is_float {
            Token::Float(text)
        } else {
            Token::Int(text)
        }
    }


    fn looking_at_numeric(&self) -> bool {
        if let Some(c) = self.lookahead {
            if c.is_digit(10) {
                return true;
            }
            match c {
                '-' | '+' | '.' => return true,
                _ => {}
            }
        }

        false
    }

    fn allowed_in_symbol(ch: &char) -> bool {
        if ch.is_ascii_whitespace() {
            return false;
        }

        match ch {
            '\'' | '\"' | ';' | '(' | ')' => false,
            _ => true
        }
    }

    pub fn looking_at_symbol(&self) -> bool {
        if let Some(c) = self.lookahead {
            match c {
                '.' | _ if c.is_numeric() => false,
                _ => true,
            }
        } else {
            false
        }
    }

    fn scan_symbol(&mut self) -> Token<'a> {
        let start = self.pos;

        assert!(self.looking_at_symbol());

        loop {
            match self.next_char() {
                Some(c) if Lexer::allowed_in_symbol(&c) => {}
                _ => break
            }
        }

        let text = &self.source[start..self.pos];

        Token::Symbol(text)
    }

    pub fn next(&mut self) -> Option<Token<'a>> {
        loop {
            return match self.lookahead {
                None => None,
                Some(c) if c.is_whitespace() => {
                    self.next_char();
                    continue;
                },
                Some('\'') => Some(self.scan_char(Token::Quote)),
                Some(';') => Some(self.scan_comment()),
                Some('(') => Some(self.scan_char(Token::LParen)),
                Some(')') => Some(self.scan_char(Token::RParen)),
                Some('+') | Some('-') => Some(self.scan_number()),
                Some(c) if c.is_digit(10) => Some(self.scan_number()),
                Some('"') => Some(self.scan_string()),
                Some(_) if self.looking_at_symbol() => Some(self.scan_symbol()),
                _ => {
                    panic!("Invalid character");
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct Iter<'lex> {
    inner: &'lex mut Lexer<'lex>,
}

impl<'lex> Iterator for Iter<'lex> {
    type Item = Token<'lex>;

    fn next(&mut self) -> Option<Token<'lex>> {
        self.inner.next()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lexer<'a>(code: &'a str) -> Lexer<'a> {
        Lexer::new(code)
    }

    #[test]
    fn scan_parens() {
        let code = "()";

        let mut lex = Lexer::new(code);

        assert_eq!(lex.next(), Some(Token::LParen));
        assert_eq!(lex.next(), Some(Token::RParen));
    }

    #[test]
    fn scan_int() {
        let mut lex = lexer("123 456 +89 -0001");

        assert_eq!(lex.next(), Some(Token::Int("123")));
        assert_eq!(lex.next(), Some(Token::Int("456")));
        assert_eq!(lex.next(), Some(Token::Int("+89")));
        assert_eq!(lex.next(), Some(Token::Int("-0001")));
    }

    #[test]
    fn scan_float() {
        let mut lex = lexer("0.0 5.1 123.456 -.32 +.0");

        assert_eq!(lex.next(), Some(Token::Float("0.0")));
        assert_eq!(lex.next(), Some(Token::Float("5.1")));
        assert_eq!(lex.next(), Some(Token::Float("123.456")));
        assert_eq!(lex.next(), Some(Token::Float("-.32")));
        assert_eq!(lex.next(), Some(Token::Float("+.0")));    
    }

    #[test]
    fn scan_symbol() {
        let mut lex = lexer("(name n._.ame r-_-^ee?)");

        assert_eq!(lex.next(), Some(Token::LParen));
        assert_eq!(lex.next(), Some(Token::Symbol("name")));
        assert_eq!(lex.next(), Some(Token::Symbol("n._.ame")));
        assert_eq!(lex.next(), Some(Token::Symbol("r-_-^ee?")));
        assert_eq!(lex.next(), Some(Token::RParen));
    }

    #[test]
    fn scan_plus_minus() {
        let mut lex = lexer("+1.2 -3 + -");

        assert_eq!(lex.next(), Some(Token::Float("+1.2")));
        assert_eq!(lex.next(), Some(Token::Int("-3")));
        assert_eq!(lex.next(), Some(Token::Symbol("+")));
        assert_eq!(lex.next(), Some(Token::Symbol("-")));
    }

    #[test]

    fn scan_comment() {
        let mut lex = lexer("+1.2 ;comment\nsymb\n;reee");

        assert_eq!(lex.next(), Some(Token::Float("+1.2")));
        assert_eq!(lex.next(), Some(Token::Comment("comment")));
        assert_eq!(lex.next(), Some(Token::Symbol("symb")));
        assert_eq!(lex.next(), Some(Token::Comment("reee")));
    }

    #[test]
    fn scan_str_lit() {
        let mut lex = lexer(r#" "" "hello world!" "\"mem\es" "\"\"" """#);
        
        assert_eq!(lex.next(), Some(Token::StrLit("")));
        assert_eq!(lex.next(), Some(Token::StrLit("hello world!")));
        assert_eq!(lex.next(), Some(Token::StrLit("\\\"mem\\es")));
        assert_eq!(lex.next(), Some(Token::StrLit("\\\"\\\"")));
    }
}