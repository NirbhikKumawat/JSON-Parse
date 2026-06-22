use json_parser::lexer::Lexer;
use json_parser::parser::{JSONValue, Parser, Token};

fn tokenize(input: &str) -> Vec<Token>{
    let mut lexer = Lexer::new(input);
    let mut tokens = Vec::new();

    loop {
        match lexer.current() {
            None => break,
            Some(b' ')|Some(b'\t')|Some(b'\n')|Some(b'\r') => {
                lexer.advance();
            }
            Some(b'{') => {
                tokens.push(Token::LeftBrace);
                lexer.advance();
            }
            Some(b'}') => {
                tokens.push(Token::RightBrace);
                lexer.advance();
            }
            Some(b'[') => {
                tokens.push(Token::LeftBracket);
                lexer.advance();
            }
            Some(b']') => {
                tokens.push(Token::RightBracket);
                lexer.advance();
            }
            Some(b':') => {
                tokens.push(Token::Colon);
                lexer.advance();
            }
            Some(b',') => {
                tokens.push(Token::Comma);
                lexer.advance();
            }
            Some(b'"') => {
                let s = lexer.read_string();
                tokens.push(Token::StringToken(s));
            }
            Some(b't') => {
                lexer.read_keyword("true");
                tokens.push(Token::True);
            }
            Some(b'f') => {
                lexer.read_keyword("false");
                tokens.push(Token::False);
            }
            Some(b'n') => {
                lexer.read_keyword("null");
                tokens.push(Token::Null);
            }
            Some(b'-')|Some(b'0'..=b'9') => {
                let n = lexer.read_number();
                tokens.push(Token::NumberToken(n));
            }
            Some(c) => {
                panic!("Unexpected character '{}'", c as char);
            }
        }
    }
    tokens
}
fn parse(tokens: Vec<Token>) -> JSONValue {
    let mut parser = Parser::new(tokens);
    let json_value = parser.parse_value();
    json_value
}

fn display(value: &JSONValue) -> String {
    match value {
        JSONValue::Null        => String::from("null"),
        JSONValue::Bool(true)  => String::from("true"),
        JSONValue::Bool(false) => String::from("false"),
        JSONValue::Number(n)   => format!("{}", n),
        JSONValue::Str(s)      => format!("\"{}\"", s),

        JSONValue::Array(elements) => {
            let mut result = String::from("[");
            let mut first = true;
            for element in elements {
                if !first {
                    result.push_str(", ");
                }
                result.push_str(&display(element));
                first = false;
            }
            result.push(']');
            result
        }

        JSONValue::Object(pairs) => {
            let mut result = String::from("{");
            let mut first = true;
            for (key, value) in pairs {
                if !first {
                    result.push_str(", ");
                }
                result.push_str(&format!("\"{}\": {}", key, display(value)));
                first = false;
            }
            result.push('}');
            result
        }
    }
}

fn main() {
    let tests = vec![
        r#"null"#,
        r#"true"#,
        r#"42"#,
        r#"-3.14"#,
        r#""hello world""#,
        r#"[]"#,
        r#"{}"#,
        r#"[1, 2, 3]"#,
        r#"{"name": "alice", "age": 30, "active": true}"#,
        r#"{"scores": [95, 87, 100], "info": {"city": "delhi", "zip": null}}"#,
        r#"["hello\nworld", "tab\there", "\u0041\u0042\u0043"]"#,
    ];

    for input in &tests {
        let tokens = tokenize(input);
        let value = parse(tokens);
        println!("Input:  {}", input);
        println!("Output: {}", display(&value));
        println!();
    }
}
