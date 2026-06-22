pub struct Lexer {
    input: Vec<u8>,
    pos: usize,
}
impl Lexer {
    pub fn new(input: &str) -> Self {
        Self{
            input: input.as_bytes().to_vec(),
            pos: 0
        }
    }
    pub fn current(&self) -> Option<u8> {
        if self.pos < self.input.len() {
            Some(self.input[self.pos])
        }else{
            None
        }
    }
    pub fn advance(&mut self) {
        self.pos += 1;
    }
    #[allow(dead_code)]
    fn peek(&self) -> Option<u8> {
        if self.pos+1 < self.input.len() {
            Some(self.input[self.pos+1])
        }else{
            None
        }
    }
    pub fn read_keyword(&mut self, keyword:&str){
        for expected in keyword.as_bytes(){
            match self.current(){
                Some(c) if c == *expected => self.advance(),
                Some(c) => panic!(
                    "Unexpected character '{}' while reading keyword '{}'",
                    c as char, keyword
                ),
                None => panic!("Unexpected end of input while reading keyword '{}'", keyword),
            }
        }
    }
    pub fn read_string(&mut self) -> String {
        self.advance();
        let mut result = String::new();

        loop {
            match self.current() {
                None => panic!("Unterminated string"),
                Some(b'"') => {
                    self.advance();
                    return result;
                }
                Some(b'\\') => {
                    self.advance();
                    match self.current() {
                        Some(b'"') => {
                            result.push('"');
                            self.advance();
                        }
                        Some(b'\\') => {
                            result.push('\\');
                            self.advance();
                        }
                        Some(b'/') => {
                            result.push('/');
                            self.advance();
                        }
                        Some(b'n') => {
                            result.push('\n');
                            self.advance();
                        }
                        Some(b't') => {
                            result.push('\t');
                            self.advance();
                        }
                        Some(b'r') => {
                            result.push('\r');
                            self.advance();
                        }
                        Some(b'b') => {
                            result.push('\x08');
                            self.advance();
                        }
                        Some(b'f') => {
                            result.push('\x0c');
                            self.advance();
                        }
                        Some(b'u') => {
                            self.advance();
                            let codepoint = self.read_unicode_escape();
                            let ch = char::from_u32(codepoint).unwrap_or_else(|| panic!("Invalid unicode escape"));
                            result.push(ch);
                        }
                        Some(c) => {
                            panic!("Unexpected character '\\{}'", c as char);
                        }
                        None => {
                            panic!("Unexpected end of escape sequence");
                        }
                    }
                }
                Some(c) => {
                    result.push(c as char);
                    self.advance();
                }
            }
        }
    }
    fn read_unicode_escape(&mut self) -> u32{
        let mut value: u32 = 0;
        for _ in 0..4 {
            match self.current() {
                Some(c) => {
                    let digit = match c {
                        b'0'..=b'9' => (c - b'0') as u32,
                        b'a'..=b'f' => (c - b'a' + 10) as u32,
                        b'A'..=b'F' => (c - b'A' + 10) as u32,
                        _ => panic!("Invalid hex digit in unicode escape: {}", c as char),
                    };
                    value = value*16+digit;
                    self.advance();
                }
                None => panic!("Unexpected end of unicode escape"),
            }
        }
        value
    }
    pub fn read_number(&mut self) -> f64 {
        let mut s = String::new();
        if let Some(b'-') = self.current() {
            s.push('-');
            self.advance();
        }

        loop {
            match self.current() {
                Some(c @ b'0'..=b'9') => {
                    s.push(c as char);
                    self.advance();
                }
                _ => break,
            }
        }
        if let Some(b'.') = self.current() {
            s.push('.');
            self.advance();

            loop {
                match self.current() {
                    Some(c @ b'0'..=b'9') => {
                        s.push(c as char);
                        self.advance();
                    }
                    _ => break,
                }
            }
        }
        s.parse::<f64>().unwrap_or_else(|_| panic!("Invalid number: {}", s))

    }
}