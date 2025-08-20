use std::str::Chars;
use std::iter::Peekable;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // 关键字
    如果, // if
    否则, // else
    循环, // loop
    当, // while
    对于, // for
    在, // in
    函数, // fn
    返回, // return
    让, // let
    常量, // const
    变量, // var
    跳出, // break
    继续, // continue
    且, // and
    或, // or
    非, // not
    数组, // array
    长度, // length
    添加, // append
    删除, // remove
    索引, // index
    
    // 字面量
    数字(f64),
    字符串(String),
    布尔(bool),
    标识符(String),
    
    // 运算符
    加, // +
    减, // -
    乘, // *
    除, // /
    赋值, // =
    等于, // ==
    不等于, // !=
    大于, // >
    小于, // <
    大于等于, // >=
    小于等于, // <=
    点, // .
    
    // 分隔符
    左括号, // (
    右括号, // )
    左大括号, // {
    右大括号, // }
    左中括号, // [
    右中括号, // ]
    逗号, // ,
    分号, // ;
    冒号, // :
    
    // 特殊
    文件结束,
    错误(String),
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub line: usize,
    pub column: usize,
}

pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
    line: usize,
    column: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer {
            input: input.chars().peekable(),
            line: 1,
            column: 1,
        }
    }
    
    pub fn tokenize(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();
        
        while let Some(&c) = self.input.peek() {
            if c.is_whitespace() {
                self.advance();
                continue;
            }
            
            let token = match c {
                '+' => self.make_single_char_token(TokenType::加),
                '-' => self.make_single_char_token(TokenType::减),
                '*' => self.make_single_char_token(TokenType::乘),
                '/' => {
                    // 检查是否是注释
                    if let Some('/') = self.input.clone().nth(1) {
                        // 是注释，跳过到行尾
                        self.advance(); // 跳过第一个/
                        self.advance(); // 跳过第二个/
                        while let Some(&next_c) = self.input.peek() {
                            if next_c == '\n' {
                                break;
                            }
                            self.advance();
                        }
                        continue; // 继续下一个字符
                    } else {
                        self.make_single_char_token(TokenType::除)
                    }
                }
                '=' => self.handle_equals(),
                '>' => self.handle_greater(),
                '<' => self.handle_less(),
                '(' => self.make_single_char_token(TokenType::左括号),
                ')' => self.make_single_char_token(TokenType::右括号),
                '{' => self.make_single_char_token(TokenType::左大括号),
                '}' => self.make_single_char_token(TokenType::右大括号),
                '[' => self.make_single_char_token(TokenType::左中括号),
                ']' => self.make_single_char_token(TokenType::右中括号),
                ',' => self.make_single_char_token(TokenType::逗号),
                ';' => self.make_single_char_token(TokenType::分号),
                ':' => self.make_single_char_token(TokenType::冒号),
                '.' => self.make_single_char_token(TokenType::点),
                '"' => self.string_literal(),
                _ if c.is_digit(10) => self.number_literal(),
                _ if self.is_chinese_char(c) || c.is_alphabetic() => self.identifier_or_keyword(),
                _ => Token {
                    token_type: TokenType::错误(format!("未知字符: {}", c)),
                    line: self.line,
                    column: self.column,
                },
            };
            
            if let TokenType::错误(_) = token.token_type {
                return Err(format!("词法错误: {} (行: {}, 列: {})", 
                    match token.token_type {
                        TokenType::错误(msg) => msg,
                        _ => String::new(),
                    },
                    token.line,
                    token.column
                ));
            }
            
            tokens.push(token);
        }
        
        tokens.push(Token {
            token_type: TokenType::文件结束,
            line: self.line,
            column: self.column,
        });
        
        Ok(tokens)
    }
    
    fn advance(&mut self) {
        if let Some(c) = self.input.next() {
            if c == '\n' {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }
        }
    }
    
    fn make_single_char_token(&mut self, token_type: TokenType) -> Token {
        let token = Token {
            token_type,
            line: self.line,
            column: self.column,
        };
        self.advance();
        token
    }
    
    fn handle_equals(&mut self) -> Token {
        let start_column = self.column;
        self.advance();
        if let Some(&'=') = self.input.peek() {
            self.advance();
            Token {
                token_type: TokenType::等于,
                line: self.line,
                column: start_column,
            }
        } else {
            Token {
                token_type: TokenType::赋值,
                line: self.line,
                column: start_column,
            }
        }
    }
    
    fn handle_greater(&mut self) -> Token {
        let start_column = self.column;
        self.advance();
        if let Some(&'=') = self.input.peek() {
            self.advance();
            Token {
                token_type: TokenType::大于等于,
                line: self.line,
                column: start_column,
            }
        } else {
            Token {
                token_type: TokenType::大于,
                line: self.line,
                column: start_column,
            }
        }
    }
    
    fn handle_less(&mut self) -> Token {
        let start_column = self.column;
        self.advance();
        if let Some(&'=') = self.input.peek() {
            self.advance();
            Token {
                token_type: TokenType::小于等于,
                line: self.line,
                column: start_column,
            }
        } else {
            Token {
                token_type: TokenType::小于,
                line: self.line,
                column: start_column,
            }
        }
    }
    
    fn string_literal(&mut self) -> Token {
        self.advance(); // 跳过开始的引号
        let mut value = String::new();
        let start_line = self.line;
        let start_column = self.column;
        
        while let Some(&c) = self.input.peek() {
            if c == '"' {
                self.advance();
                return Token {
                    token_type: TokenType::字符串(value),
                    line: start_line,
                    column: start_column,
                };
            }
            
            // 处理转义字符
            if c == '\\' {
                self.advance(); // 跳过反斜杠
                if let Some(&escaped_char) = self.input.peek() {
                    let actual_char = match escaped_char {
                        'n' => '\n',
                        't' => '\t',
                        'r' => '\r',
                        '\\' => '\\',
                        '"' => '"',
                        _ => escaped_char, // 未知的转义序列，原样保留
                    };
                    value.push(actual_char);
                    self.advance();
                } else {
                    value.push('\\'); // 只有反斜杠，没有转义字符
                }
            } else {
                value.push(c);
                self.advance();
            }
        }
        
        Token {
            token_type: TokenType::错误("未闭合的字符串".to_string()),
            line: start_line,
            column: start_column,
        }
    }
    
    fn number_literal(&mut self) -> Token {
        let mut value = String::new();
        let start_line = self.line;
        let start_column = self.column;
        let mut has_decimal = false;
        
        while let Some(&c) = self.input.peek() {
            if c.is_digit(10) {
                value.push(c);
                self.advance();
            } else if c == '.' && !has_decimal {
                value.push(c);
                has_decimal = true;
                self.advance();
            } else {
                break;
            }
        }
        
        if let Ok(num) = value.parse::<f64>() {
            Token {
                token_type: TokenType::数字(num),
                line: start_line,
                column: start_column,
            }
        } else {
            Token {
                token_type: TokenType::错误("无效的数字格式".to_string()),
                line: start_line,
                column: start_column,
            }
        }
    }
    
    fn identifier_or_keyword(&mut self) -> Token {
        let mut value = String::new();
        let start_line = self.line;
        let start_column = self.column;
        
        while let Some(&c) = self.input.peek() {
            if self.is_chinese_char(c) || c.is_alphanumeric() || c == '_' {
                value.push(c);
                self.advance();
            } else {
                break;
            }
        }
        
        let token_type = match value.as_str() {
            "如果" => TokenType::如果,
            "否则" => TokenType::否则,
            "循环" => TokenType::循环,
            "当" => TokenType::当,
            "对于" => TokenType::对于,
            "在" => TokenType::在,
            "函数" => TokenType::函数,
            "返回" => TokenType::返回,
            "让" => TokenType::让,
            "常量" => TokenType::常量,
            "变量" => TokenType::变量,
            "跳出" => TokenType::跳出,
            "继续" => TokenType::继续,
            "且" => TokenType::且,
            "或" => TokenType::或,
            "非" => TokenType::非,
            // "数组"、"长度"、"添加"、"删除"、"索引" 不再作为关键字，而是作为标识符处理
            // "数组" => TokenType::数组,
            // "长度" => TokenType::长度,
            // "添加" => TokenType::添加,
            // "删除" => TokenType::删除,
            // "索引" => TokenType::索引,
            "真" => TokenType::布尔(true),
            "假" => TokenType::布尔(false),
            _ => TokenType::标识符(value),
        };
        
        Token {
            token_type,
            line: start_line,
            column: start_column,
        }
    }
    
    fn is_chinese_char(&self, c: char) -> bool {
        let code = c as u32;
        (0x4E00..=0x9FFF).contains(&code) || // CJK统一汉字
        (0x3400..=0x4DBF).contains(&code) || // CJK扩展A
        (0x20000..=0x2A6DF).contains(&code) || // CJK扩展B
        (0x2A700..=0x2B73F).contains(&code) || // CJK扩展C
        (0x2B740..=0x2B81F).contains(&code) || // CJK扩展D
        (0x2B820..=0x2CEAF).contains(&code) || // CJK扩展E
        (0xF900..=0xFAFF).contains(&code) || // CJK兼容汉字
        (0x2F800..=0x2FA1F).contains(&code) // CJK兼容补充
    }
}