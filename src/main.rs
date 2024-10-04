use std::str::FromStr;

#[derive(Debug)]
enum Token {
    Keyword(String),
    Identifier(String),
    IntNum(i64),
    FloatNum(f64),
    Plus,
    Minus,
    Star,
    Slash,
    Assign,
    LPar,
    RPar,
    LBrace,
    RBrace,
    Semi,
    Comma,
    And,
    Or,
    Not,
    BitAnd,
    BitOr,
    BitXor,
    BitNot,
    ShiftLeft,
    ShiftRight,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Equal,
    NotEqual,
    Escape(String),
    Directive(String),
}

impl Token {
    fn description(&self) -> String {
        match self {
            Token::Keyword(ref s) => format!("Token: KEYWORD \"{}\"", s),
            Token::Identifier(ref s) => format!("Token: ID \"{}\"", s),
            Token::IntNum(n) => format!("Token: INT_NUM \"{}\"", n),
            Token::FloatNum(n) => format!("Token: FLOAT_NUM \"{}\"", n),
            Token::Plus => "Token: PLUS \"+\"".to_string(),
            Token::Minus => "Token: MINUS \"-\"".to_string(),
            Token::Star => "Token: STAR \"*\"".to_string(),
            Token::Slash => "Token: SLASH \"/\"".to_string(),
            Token::Assign => "Token: ASSIGN \"=\"".to_string(),
            Token::LPar => "Token: LPAR \"(\"".to_string(),
            Token::RPar => "Token: RPAR \")\"".to_string(),
            Token::LBrace => "Token: LBRACE \"{\"".to_string(),
            Token::RBrace => "Token: RBRACE \"}\"".to_string(),
            Token::Semi => "Token: SEMI \";\"".to_string(),
            Token::Comma => "Token: COMMA \",\"".to_string(),
            Token::And => "Token: AND \"&&\"".to_string(),
            Token::Or => "Token: OR \"||\"".to_string(),
            Token::Not => "Token: NOT \"!\"".to_string(),
            Token::BitAnd => "Token: BIT_AND \"&\"".to_string(),
            Token::BitOr => "Token: BIT_OR \"|\"".to_string(),
            Token::BitXor => "Token: BIT_XOR \"^\"".to_string(),
            Token::BitNot => "Token: BIT_NOT \"~\"".to_string(),
            Token::ShiftLeft => "Token: SHIFT_LEFT \"<<\"".to_string(),
            Token::ShiftRight => "Token: SHIFT_RIGHT \">>\"".to_string(),
            Token::Greater => "Token: GREATER \">\"".to_string(),
            Token::GreaterEqual => "Token: GREATER_EQUAL \">=\"".to_string(),
            Token::Less => "Token: LESS \"<\"".to_string(),
            Token::LessEqual => "Token: LESS_EQUAL \"<=\"".to_string(),
            Token::Equal => "Token: EQUAL \"==\"".to_string(),
            Token::NotEqual => "Token: NOT_EQUAL \"!=\"".to_string(),
            Token::Escape(ref s) => format!("Token: ESCAPE \"{}\"", s),
            Token::Directive(ref s) => format!("Token: DIRECTIVE \"{}\"", s),
        }
    }
}

fn is_keyword(token: &str) -> Option<Token> {
    match token {
        "auto" | "break" | "case" | "char" | "const" | "continue" |
        "default" | "do" | "double" | "else" | "enum" | "extern" |
        "float" | "for" | "goto" | "if" | "int" | "long" |
        "register" | "return" | "short" | "signed" | "sizeof" |
        "static" | "struct" | "switch" | "typedef" | "union" |
        "unsigned" | "void" | "volatile" | "while" => 
            Some(Token::Keyword(token.to_string())),
        _ => None,
    }
}

fn is_directive(token: &str) -> Option<Token> {
    match token {
        "#define" | "#elif" | "#else" | "#endif" | "#error" |
        "#if" | "#ifdef" | "#ifndef" | "#include" | "#message" |
        "#undef" => Some(Token::Directive(token.to_string())),
        _ => None,
    }
}

fn analyze_source_code(source: &str) {
    let mut tokens: Vec<Token> = Vec::new();
    let mut chars = source.chars().peekable();
    
    while let Some(c) = chars.next() {
        if c.is_alphabetic() || c == '#' {
            let mut token = String::new();
            token.push(c);
            while let Some(&next_char) = chars.peek() {
                if next_char.is_alphanumeric() || next_char == '_' {
                    token.push(chars.next().unwrap());
                } else {
                    break;
                }
            }
            if let Some(directive) = is_directive(&token) {
                tokens.push(directive);
            } else if let Some(keyword) = is_keyword(&token) {
                tokens.push(keyword);
            } else {
                tokens.push(Token::Identifier(token));
            }
        } else if c.is_digit(10) {
            let mut num_str = String::new();
            num_str.push(c);
            while let Some(&next_char) = chars.peek() {
                if next_char.is_digit(10) {
                    num_str.push(chars.next().unwrap());
                } else {
                    break;
                }
            }
            if chars.peek() == Some(&'.') {
                num_str.push(chars.next().unwrap());
                while let Some(&next_char) = chars.peek() {
                    if next_char.is_digit(10) {
                        num_str.push(chars.next().unwrap());
                    } else {
                        break;
                    }
                }
                if let Ok(float_num) = f64::from_str(&num_str) {
                    tokens.push(Token::FloatNum(float_num));
                }
            } else {
                if let Ok(int_num) = i64::from_str(&num_str) {
                    tokens.push(Token::IntNum(int_num));
                }
            }
        } else if c == '+' {
            tokens.push(Token::Plus);
        } else if c == '-' {
            tokens.push(Token::Minus);
        } else if c == '*' {
            tokens.push(Token::Star);
        } else if c == '/' {
            tokens.push(Token::Slash);
        } else if c == '=' {
            if chars.peek() == Some(&'=') {
                chars.next();
                tokens.push(Token::Equal);
            } else {
                tokens.push(Token::Assign);
            }
        } else if c == '>' {
            if chars.peek() == Some(&'=') {
                chars.next();
                tokens.push(Token::GreaterEqual);
            } else if chars.peek() == Some(&'>') {
                chars.next();
                tokens.push(Token::ShiftRight);
            } else {
                tokens.push(Token::Greater);
            }
        } else if c == '<' {
            if chars.peek() == Some(&'=') {
                chars.next(); 
                tokens.push(Token::LessEqual);
            } else if chars.peek() == Some(&'<') {
                chars.next();
                tokens.push(Token::ShiftLeft);
            } else {
                tokens.push(Token::Less);
            }
        } else if c == '&' {
            if chars.peek() == Some(&'&') {
                chars.next();
                tokens.push(Token::And);
            } else {
                tokens.push(Token::BitAnd);
            }
        } else if c == '|' {
            if chars.peek() == Some(&'|') {
                chars.next(); 
                tokens.push(Token::Or);
            } else {
                tokens.push(Token::BitOr);
            }
        } else if c == '!' {
            if chars.peek() == Some(&'=') {
                chars.next();
                tokens.push(Token::NotEqual);
            } else {
                tokens.push(Token::Not);
            }
        } else if c == '^' {
            tokens.push(Token::BitXor);
        } else if c == '~' {
            tokens.push(Token::BitNot);
        } else if c == '(' {
            tokens.push(Token::LPar);
        } else if c == ')' {
            tokens.push(Token::RPar);
        } else if c == '{' {
            tokens.push(Token::LBrace);
        } else if c == '}' {
            tokens.push(Token::RBrace);
        } else if c == ';' {
            tokens.push(Token::Semi);
        } else if c == ',' {
            tokens.push(Token::Comma);
        } else if c == '\n' || c.is_whitespace() {
            continue; 
        } else if c == '"' {
            let mut escape_seq = String::new();
            while let Some(next_char) = chars.next() {
                if next_char == '"' {
                    break;
                } else {
                    escape_seq.push(next_char);
                }
            }
            tokens.push(Token::Escape(escape_seq));
        } else {
            println!("Unrecognized character: {}", c);
        }
    }
    
    for token in tokens {
        println!("{}", token.description());
    }
}


fn main() {
    let source_code = r#"
        int main() {
            int a;
            int b;
            a = b + 1;
            return 0;
        }
    "#;

    analyze_source_code(source_code);
}

