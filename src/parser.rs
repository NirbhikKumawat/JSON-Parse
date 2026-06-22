pub enum JSONValue {
    Object(Vec<(String,JSONValue)>),
    Array(Vec<JSONValue>),
    Bool(bool),
    Number(f64),
    Str(String),
    Null,
}
pub enum Token {
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Comma,
    Colon,
    StringToken(String),
    NumberToken(f64),
    True,
    False,
    Null,
}
pub struct Parser{
    tokens: Vec<Token>,
    pos: usize,
}
impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            pos: 0
        }
    }
    fn current(&self) -> &Token {
        if self.pos < self.tokens.len() {
            &self.tokens[self.pos]
        } else {
            panic!("Unexpected end of token stream");
        }
    }
    fn advance(&mut self) {
        self.pos += 1;
    }
    fn expect(&mut self,description:&str)->&Token {
        if self.pos < self.tokens.len() {
            let token = &self.tokens[self.pos];
            self.pos+=1;
            token
        }else{
            panic!("Expected {} but reached end of input",description);
        }
    }
    pub fn parse_value(&mut self) ->JSONValue {
        match self.current() {
            Token::LeftBrace => self.parse_object(),
            Token::LeftBracket => self.parse_array(),
            Token::True => {
                self.advance();
                JSONValue::Bool(true)
            },
            Token::False => {
                self.advance();
                JSONValue::Bool(false)
            },
            Token::Null => {
                self.advance();
                JSONValue::Null
            },
            Token::NumberToken(n) => {
                let value = *n;
                self.advance();
                JSONValue::Number(value)
            },
            Token::StringToken(s) => {
                let value = s.clone();
                self.advance();
                JSONValue::Str(value)
            },
            Token::RightBrace => {
                panic!("Unexpected ");
            },
            Token::RightBracket => {
                panic!("Unexpected ']'");
            }
            Token::Colon => {
                panic!("Unexpected ':'");
            }
            Token::Comma => {
                panic!("Unexpected ','");
            }
        }
    }
    fn parse_object(&mut self) ->JSONValue {
        self.advance();

        let mut pairs : Vec<(String,JSONValue)> = Vec::new();

        if let Token::RightBrace = self.current() {
            self.advance();
            return JSONValue::Object(pairs);
        }
        loop {
            let key = match self.expect("object key") {
                Token::StringToken(s) => s.clone(),
                _ => panic!("Expected string but got something else"),
            };

            match self.expect("colon") {
                Token::Colon => {}
                _ => panic!("Expected ':' after object key"),
            }
            let value = self.parse_value();
            pairs.push((key, value));

            match self.current() {
                Token::Comma => {
                    self.advance();
                }
                Token::RightBrace => {
                    self.advance();
                    break;
                }
                _ => panic!("Expected ',' or '}}' after object key"),
            }
        }
        JSONValue::Object(pairs)
    }
    fn parse_array(&mut self) ->JSONValue {
        self.advance();

        let mut elements : Vec<JSONValue> = Vec::new();

        if let Token::RightBracket = self.current() {
            self.advance();
            return JSONValue::Array(elements);
        }

        loop {
            let element = self.parse_value();
            elements.push(element);

            match self.current() {
                Token:: Comma => {
                    self.advance();
                }
                Token::RightBracket => {
                    self.advance();
                    break;
                }
                _ => panic!("Expected ',' or ']' after array element"),
            }
        }
        JSONValue::Array(elements)
    }
}