use crate::lexer::Token;

#[derive(Debug, Clone)]
pub enum Expression {
    数字字面量(f64),
    字符串字面量(String),
    变量(String),
    二元运算 {
        左: Box<Expression>,
        运算符: String,
        右: Box<Expression>,
    },
    一元运算 {
        运算符: String,
        操作数: Box<Expression>,
    },
    函数调用 {
        函数名: String,
        参数: Vec<Expression>,
    },
    赋值 {
        变量名: String,
        值: Box<Expression>,
    },
    数组字面量(Vec<Expression>),
    数组索引 {
        数组: Box<Expression>,
        索引: Box<Expression>,
    },
    数组长度(Box<Expression>),
    数组添加 {
        数组: Box<Expression>,
        元素: Box<Expression>,
    },
    数组删除 {
        数组: Box<Expression>,
        索引: Box<Expression>,
    },
}

#[derive(Debug, Clone)]
pub enum Statement {
    表达式语句(Expression),
    变量声明 {
        名字: String,
        初始值: Option<Expression>,
        是常量: bool,
    },
    函数声明 {
        名字: String,
        参数: Vec<String>,
        体: Vec<Statement>,
    },
    如果语句 {
        条件: Expression,
        真分支: Vec<Statement>,
        假分支: Vec<Statement>,
    },
    循环语句 {
        条件: Option<Expression>,
        体: Vec<Statement>,
    },
    当语句 {
        条件: Expression,
        体: Vec<Statement>,
    },
    对于语句 {
        变量: String,
        可迭代: Expression,
        体: Vec<Statement>,
    },
    返回语句(Option<Expression>),
    跳出语句,
    继续语句,
}

#[derive(Debug, Clone)]
pub struct Program {
    pub 语句: Vec<Statement>,
}

impl Program {
    pub fn new() -> Self {
        Program {
            语句: Vec::new(),
        }
    }
    
    pub fn 添加语句(&mut self, 语句: Statement) {
        self.语句.push(语句);
    }
}

impl std::fmt::Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::数字字面量(n) => write!(f, "{}", n),
            Expression::字符串字面量(s) => write!(f, "\"{}\"", s),
            Expression::变量(name) => write!(f, "{}", name),
            Expression::二元运算 { 左, 运算符, 右 } => write!(f, "({} {} {})", 左, 运算符, 右),
            Expression::一元运算 { 运算符, 操作数 } => write!(f, "{}{}", 运算符, 操作数),
            Expression::函数调用 { 函数名, 参数 } => {
                let args: Vec<String> = 参数.iter().map(|arg| arg.to_string()).collect();
                write!(f, "{}({})", 函数名, args.join(", "))
            }
            Expression::赋值 { 变量名, 值 } => write!(f, "{} = {}", 变量名, 值),
            Expression::数组字面量(元素) => {
                let elements: Vec<String> = 元素.iter().map(|elem| elem.to_string()).collect();
                write!(f, "[{}]", elements.join(", "))
            }
            Expression::数组索引 { 数组, 索引 } => write!(f, "{}[{}]", 数组, 索引),
            Expression::数组长度(数组) => write!(f, "{}.长度", 数组),
            Expression::数组添加 { 数组, 元素 } => write!(f, "{}.添加({})", 数组, 元素),
            Expression::数组删除 { 数组, 索引 } => write!(f, "{}.删除({})", 数组, 索引),
        }
    }
}

impl std::fmt::Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Statement::表达式语句(expr) => write!(f, "{}", expr),
            Statement::变量声明 { 名字, 初始值, 是常量 } => {
                let const_str = if *是常量 { "常量" } else { "让" };
                match 初始值 {
                    Some(value) => write!(f, "{} {} = {}", const_str, 名字, value),
                    None => write!(f, "{} {}", const_str, 名字),
                }
            }
            Statement::函数声明 { 名字, 参数, 体 } => {
                let args = 参数.join(", ");
                let body: Vec<String> = 体.iter().map(|stmt| stmt.to_string()).collect();
                write!(f, "函数 {}({}) {{\n  {}\n}}", 名字, args, body.join("\n  "))
            }
            Statement::如果语句 { 条件, 真分支, 假分支 } => {
                let true_body: Vec<String> = 真分支.iter().map(|stmt| stmt.to_string()).collect();
                let false_body: Vec<String> = 假分支.iter().map(|stmt| stmt.to_string()).collect();
                write!(f, "如果 {} {{\n  {}\n}} 否则 {{\n  {}\n}}", 条件, true_body.join("\n  "), false_body.join("\n  "))
            }
            Statement::循环语句 { 条件, 体 } => {
                let body: Vec<String> = 体.iter().map(|stmt| stmt.to_string()).collect();
                match 条件 {
                    Some(cond) => write!(f, "循环 {} {{\n  {}\n}}", cond, body.join("\n  ")),
                    None => write!(f, "循环 {{\n  {}\n}}", body.join("\n  ")),
                }
            }
            Statement::当语句 { 条件, 体 } => {
                let body: Vec<String> = 体.iter().map(|stmt| stmt.to_string()).collect();
                write!(f, "当 {} {{\n  {}\n}}", 条件, body.join("\n  "))
            }
            Statement::对于语句 { 变量, 可迭代, 体 } => {
                let body: Vec<String> = 体.iter().map(|stmt| stmt.to_string()).collect();
                write!(f, "对于 {} 在 {} {{\n  {}\n}}", 变量, 可迭代, body.join("\n  "))
            }
            Statement::返回语句(值) => {
                match 值 {
                    Some(expr) => write!(f, "返回 {}", expr),
                    None => write!(f, "返回"),
                }
            }
            Statement::跳出语句 => write!(f, "跳出"),
            Statement::继续语句 => write!(f, "继续"),
        }
    }
}

impl std::fmt::Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let statements: Vec<String> = self.语句.iter().map(|stmt| stmt.to_string()).collect();
        write!(f, "{}", statements.join("\n"))
    }
}