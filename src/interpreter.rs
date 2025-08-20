use crate::ast::{Program, Statement, Expression};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Value {
    数字(f64),
    字符串(String),
    布尔(bool),
    数组(Vec<Value>),
    函数 {
        参数: Vec<String>,
        体: Vec<Statement>,
        作用域: usize,
    },
    内置函数(String),
    空,
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::数字(n) => write!(f, "{}", n),
            Value::字符串(s) => write!(f, "\"{}\"", s),
            Value::布尔(b) => write!(f, "{}", b),
            Value::数组(元素) => {
                let elements: Vec<String> = 元素.iter().map(|elem| elem.to_string()).collect();
                write!(f, "[{}]", elements.join(", "))
            }
            Value::函数 { .. } => write!(f, "[函数]"),
            Value::内置函数(name) => write!(f, "[内置函数: {}]", name),
            Value::空 => write!(f, "空"),
        }
    }
}

#[derive(Debug)]
pub struct Environment {
    values: HashMap<String, Value>,
    parent: Option<usize>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            values: HashMap::new(),
            parent: None,
        }
    }
    
    pub fn with_parent(parent_id: usize) -> Self {
        Environment {
            values: HashMap::new(),
            parent: Some(parent_id),
        }
    }
    
    pub fn define(&mut self, name: &str, value: Value) {
        self.values.insert(name.to_string(), value);
    }
    
    pub fn get(&self, name: &str) -> Option<Value> {
        if let Some(value) = self.values.get(name) {
            return Some(value.clone());
        }
        None
    }
    
    pub fn get_with_scope_chain(&self, name: &str, environments: &[Environment]) -> Option<Value> {
        if let Some(value) = self.values.get(name) {
            return Some(value.clone());
        }
        
        if let Some(parent_id) = self.parent {
            if let Some(parent_env) = environments.get(parent_id) {
                return parent_env.get_with_scope_chain(name, environments);
            }
        }
        
        None
    }
    
    pub fn assign(&mut self, name: &str, value: Value) -> Result<(), String> {
        if self.values.contains_key(name) {
            self.values.insert(name.to_string(), value);
            Ok(())
        } else {
            Err(format!("未定义的变量: {}", name))
        }
    }
}

pub struct Interpreter {
    environments: Vec<Environment>,
    globals: usize,
    current_env: usize,
}

impl Interpreter {
    pub fn new() -> Self {
        let mut env = Environment::new();
        
        // 添加内置函数
        env.define("打印", Value::内置函数("内置打印".to_string()));
        env.define("输入", Value::内置函数("内置输入".to_string()));
        
        Interpreter {
            environments: vec![env],
            globals: 0,
            current_env: 0,
        }
    }
    
    pub fn execute(&mut self, program: &Program) -> Result<(), String> {
        for statement in &program.语句 {
            self.execute_statement(statement)?;
        }
        Ok(())
    }
    
    fn execute_statement(&mut self, statement: &Statement) -> Result<Value, String> {
        match statement {
            Statement::表达式语句(expr) => {
                self.evaluate_expression(expr)?;
                Ok(Value::空)
            }
            Statement::变量声明 { 名字, 初始值, 是常量: _ } => {
                let value = match 初始值 {
                    Some(expr) => self.evaluate_expression(expr)?,
                    None => Value::空,
                };
                
                let env = self.get_current_environment_mut();
                env.define(名字, value);
                Ok(Value::空)
            }
            Statement::函数声明 { 名字, 参数, 体 } => {
                let function = Value::函数 {
                    参数: 参数.clone(),
                    体: 体.clone(),
                    作用域: self.current_env,
                };
                
                let env = self.get_current_environment_mut();
                env.define(名字, function);
                Ok(Value::空)
            }
            Statement::如果语句 { 条件, 真分支, 假分支 } => {
                let condition_value = self.evaluate_expression(条件)?;
                let should_execute = self.is_truthy(&condition_value);
                
                if should_execute {
                    for stmt in 真分支 {
                        self.execute_statement(stmt)?;
                    }
                } else {
                    for stmt in 假分支 {
                        self.execute_statement(stmt)?;
                    }
                }
                Ok(Value::空)
            }
            Statement::循环语句 { 条件, 体 } => {
                loop {
                    if let Some(condition_expr) = 条件 {
                        let condition_value = self.evaluate_expression(condition_expr)?;
                        if !self.is_truthy(&condition_value) {
                            break;
                        }
                    }
                    
                    for stmt in 体 {
                        match self.execute_statement(stmt) {
                            Ok(Value::空) => continue,
                            Ok(_) => break,
                            Err(e) if e == "跳出" => break,
                            Err(e) if e == "继续" => continue,
                            Err(e) => return Err(e),
                        }
                    }
                }
                Ok(Value::空)
            }
            Statement::当语句 { 条件, 体 } => {
                let mut condition_value = self.evaluate_expression(条件)?;
                while self.is_truthy(&condition_value) {
                    condition_value = self.evaluate_expression(条件)?;
                    for stmt in 体 {
                        match self.execute_statement(stmt) {
                            Ok(Value::空) => continue,
                            Ok(_) => break,
                            Err(e) if e == "跳出" => break,
                            Err(e) if e == "继续" => continue,
                            Err(e) => return Err(e),
                        }
                    }
                }
                Ok(Value::空)
            }
            Statement::对于语句 { 变量, 可迭代, 体 } => {
                let iterable = self.evaluate_expression(可迭代)?;
                
                match iterable {
                    Value::字符串(s) => {
                        for ch in s.chars() {
                            let env = self.get_current_environment_mut();
                            env.define(变量, Value::字符串(ch.to_string()));
                            
                            for stmt in 体 {
                                match self.execute_statement(stmt) {
                                    Ok(Value::空) => continue,
                                    Ok(_) => break,
                                    Err(e) if e == "跳出" => break,
                                    Err(e) if e == "继续" => continue,
                                    Err(e) => return Err(e),
                                }
                            }
                        }
                    }
                    _ => return Err("只能遍历字符串".to_string()),
                }
                
                Ok(Value::空)
            }
            Statement::返回语句(值) => {
                let return_value = match 值 {
                    Some(expr) => self.evaluate_expression(expr)?,
                    None => Value::空,
                };
                // 生成可解析的返回字符串
                let return_str = match return_value {
                    Value::数字(n) => n.to_string(),
                    Value::字符串(s) => format!("\"{}\"", s),
                    Value::布尔(b) => b.to_string(),
                    Value::空 => String::new(),
                    _ => String::new(),
                };
                Err(format!("返回{}", return_str))
            }
            Statement::跳出语句 => Err("跳出".to_string()),
            Statement::继续语句 => Err("继续".to_string()),
        }
    }
    
    fn evaluate_expression(&mut self, expression: &Expression) -> Result<Value, String> {
        match expression {
            Expression::数字字面量(n) => Ok(Value::数字(*n)),
            Expression::字符串字面量(s) => Ok(Value::字符串(s.clone())),
            Expression::变量(name) => {
                let env = self.get_current_environment();
                match env.get_with_scope_chain(name, &self.environments) {
                    Some(value) => Ok(value),
                    None => Err(format!("未定义的变量: {}", name)),
                }
            }
            Expression::二元运算 { 左, 运算符, 右 } => {
                let left = self.evaluate_expression(左)?;
                let right = self.evaluate_expression(右)?;
                
                match 运算符.as_str() {
                    "+" => self.add(left, right),
                    "-" => self.subtract(left, right),
                    "*" => self.multiply(left, right),
                    "/" => self.divide(left, right),
                    "==" => Ok(Value::布尔(self.is_equal(&left, &right))),
                    "!=" => Ok(Value::布尔(!self.is_equal(&left, &right))),
                    ">" => self.greater_than(left, right),
                    "<" => self.less_than(left, right),
                    ">=" => self.greater_equal(left, right),
                    "<=" => self.less_equal(left, right),
                    "&&" => Ok(Value::布尔(self.is_truthy(&left) && self.is_truthy(&right))),
                    "||" => Ok(Value::布尔(self.is_truthy(&left) || self.is_truthy(&right))),
                    "=" => {
                        if let Expression::变量(name) = &**左 {
                            let env = self.get_current_environment_mut();
                            env.assign(name, right.clone())?;
                            Ok(right)
                        } else {
                            Err("无效的赋值目标".to_string())
                        }
                    }
                    _ => Err(format!("未知运算符: {}", 运算符)),
                }
            }
            Expression::函数调用 { 函数名, 参数 } => {
                let function_value = {
                    let env = self.get_current_environment();
                    env.get_with_scope_chain(函数名, &self.environments).ok_or_else(|| format!("未定义的函数: {}", 函数名))?
                };
                
                match function_value {
                    Value::内置函数(name) => {
                        match name.as_str() {
                            "内置打印" => {
                                if 参数.len() != 1 {
                                    return Err("打印函数需要一个参数".to_string());
                                }
                                let value = self.evaluate_expression(&参数[0])?;
                                println!("{}", value);
                                Ok(Value::空)
                            }
                            "内置输入" => {
                                if !参数.is_empty() {
                                    return Err("输入函数不需要参数".to_string());
                                }
                                use std::io;
                                let mut input = String::new();
                                io::stdin().read_line(&mut input).unwrap();
                                Ok(Value::字符串(input.trim().to_string()))
                            }
                            _ => Err(format!("未知内置函数: {}", name)),
                        }
                    }
                    Value::函数 { 参数: params, 体: body, 作用域: scope } => {
                        if params.len() != 参数.len() {
                            return Err(format!("参数数量不匹配: 期望 {}, 得到 {}", params.len(), 参数.len()));
                        }
                        
                        // 先计算所有参数的值，在旧的作用域中
                        let mut arg_values = Vec::new();
                        for arg_expr in 参数 {
                            let arg_value = self.evaluate_expression(arg_expr)?;
                            arg_values.push(arg_value);
                        }
                        
                        // 创建新的作用域
                        let new_env_id = self.environments.len();
                        self.environments.push(Environment::with_parent(scope));
                        let old_env = self.current_env;
                        self.current_env = new_env_id;
                        
                        // 绑定参数
                        for (i, param) in params.iter().enumerate() {
                            let env = self.get_current_environment_mut();
                            env.define(param, arg_values[i].clone());
                        }
                        
                        // 执行函数体
                        let mut result = Value::空;
                        for stmt in &body {
                            match self.execute_statement(stmt) {
                                Ok(Value::空) => continue,
                                Ok(value) => {
                                    result = value;
                                    break;
                                }
                                Err(e) if e.starts_with("返回") => {
                                    // 简单处理返回值
                                    let return_str = e.trim_start_matches("返回");
                                    if !return_str.is_empty() {
                                        // 尝试解析数字
                                        if let Ok(num) = return_str.parse::<f64>() {
                                            result = Value::数字(num);
                                        } else if return_str == "true" || return_str == "false" {
                                            result = Value::布尔(return_str == "true");
                                        } else if return_str.starts_with('"') && return_str.ends_with('"') {
                                            result = Value::字符串(return_str[1..return_str.len()-1].to_string());
                                        } else {
                                            result = Value::字符串(return_str.to_string());
                                        }
                                    }
                                    break;
                                }
                                Err(e) => {
                                    self.current_env = old_env;
                                    return Err(e);
                                }
                            }
                        }
                        
                        // 恢复旧的作用域
                        self.current_env = old_env;
                        Ok(result)
                    }
                    _ => Err(format!("{} 不是函数", 函数名)),
                }
            }
            Expression::一元运算 { 运算符, 操作数 } => {
                let operand = self.evaluate_expression(操作数)?;
                match 运算符.as_str() {
                    "!" => Ok(Value::布尔(!self.is_truthy(&operand))),
                    _ => Err(format!("未知一元运算符: {}", 运算符)),
                }
            }
            Expression::赋值 { 变量名, 值 } => {
                let value = self.evaluate_expression(值)?;
                let env = self.get_current_environment_mut();
                env.assign(变量名, value.clone())?;
                Ok(value)
            }
            Expression::数组字面量(元素) => {
                let mut array_values = Vec::new();
                for element in 元素 {
                    array_values.push(self.evaluate_expression(element)?);
                }
                Ok(Value::数组(array_values))
            }
            Expression::数组索引 { 数组, 索引 } => {
                let array_value = self.evaluate_expression(数组)?;
                let index_value = self.evaluate_expression(索引)?;
                
                match (array_value, index_value) {
                    (Value::数组(元素), Value::数字(idx)) => {
                        let idx = idx as usize;
                        if idx < 元素.len() {
                            Ok(元素[idx].clone())
                        } else {
                            Err(format!("数组索引越界: 索引 {}, 长度 {}", idx, 元素.len()))
                        }
                    }
                    _ => Err("索引操作需要数组和数字索引".to_string()),
                }
            }
            Expression::数组长度(数组) => {
                let array_value = self.evaluate_expression(数组)?;
                match array_value {
                    Value::数组(元素) => Ok(Value::数字(元素.len() as f64)),
                    _ => Err("长度操作需要数组".to_string()),
                }
            }
            Expression::数组添加 { 数组, 元素 } => {
                let array_value = self.evaluate_expression(数组)?;
                let element_value = self.evaluate_expression(元素)?;
                
                match array_value {
                    Value::数组(mut 元素) => {
                        元素.push(element_value);
                        Ok(Value::数组(元素))
                    }
                    _ => Err("添加操作需要数组".to_string()),
                }
            }
            Expression::数组删除 { 数组, 索引 } => {
                let array_value = self.evaluate_expression(数组)?;
                let index_value = self.evaluate_expression(索引)?;
                
                match (array_value, index_value) {
                    (Value::数组(mut 元素), Value::数字(idx)) => {
                        let idx = idx as usize;
                        if idx < 元素.len() {
                            元素.remove(idx);
                            Ok(Value::数组(元素))
                        } else {
                            Err(format!("数组索引越界: 索引 {}, 长度 {}", idx, 元素.len()))
                        }
                    }
                    _ => Err("删除操作需要数组和数字索引".to_string()),
                }
            }
        }
    }
    
    fn add(&self, left: Value, right: Value) -> Result<Value, String> {
        match (left, right) {
            (Value::数字(a), Value::数字(b)) => Ok(Value::数字(a + b)),
            (Value::字符串(a), Value::字符串(b)) => Ok(Value::字符串(a + &b)),
            (Value::字符串(a), Value::数字(b)) => Ok(Value::字符串(a + &b.to_string())),
            (Value::数字(a), Value::字符串(b)) => Ok(Value::字符串(a.to_string() + &b)),
            (Value::字符串(a), Value::布尔(b)) => Ok(Value::字符串(a + &b.to_string())),
            (Value::布尔(a), Value::字符串(b)) => Ok(Value::字符串(a.to_string() + &b)),
            (Value::字符串(a), Value::空) => Ok(Value::字符串(a + "空")),
            (Value::空, Value::字符串(b)) => Ok(Value::字符串("空".to_string() + &b)),
            // 处理数组与字符串的相加
            (Value::字符串(a), Value::数组(elements)) => {
                let array_str = format!("{:?}", elements);
                Ok(Value::字符串(a + &array_str))
            }
            (Value::数组(elements), Value::字符串(b)) => {
                let array_str = format!("{:?}", elements);
                Ok(Value::字符串(array_str + &b))
            }
            _ => Err("类型不匹配: 无法相加".to_string()),
        }
    }
    
    fn subtract(&self, left: Value, right: Value) -> Result<Value, String> {
        match (left, right) {
            (Value::数字(a), Value::数字(b)) => Ok(Value::数字(a - b)),
            _ => Err("类型不匹配: 无法相减".to_string()),
        }
    }
    
    fn multiply(&self, left: Value, right: Value) -> Result<Value, String> {
        match (left, right) {
            (Value::数字(a), Value::数字(b)) => Ok(Value::数字(a * b)),
            _ => Err("类型不匹配: 无法相乘".to_string()),
        }
    }
    
    fn divide(&self, left: Value, right: Value) -> Result<Value, String> {
        match (left, right) {
            (Value::数字(a), Value::数字(b)) => {
                if b == 0.0 {
                    Err("除零错误".to_string())
                } else {
                    Ok(Value::数字(a / b))
                }
            }
            _ => Err("类型不匹配: 无法相除".to_string()),
        }
    }
    
    fn greater_than(&self, left: Value, right: Value) -> Result<Value, String> {
        match (left, right) {
            (Value::数字(a), Value::数字(b)) => Ok(Value::布尔(a > b)),
            _ => Err("类型不匹配: 无法比较".to_string()),
        }
    }
    
    fn less_than(&self, left: Value, right: Value) -> Result<Value, String> {
        match (left, right) {
            (Value::数字(a), Value::数字(b)) => Ok(Value::布尔(a < b)),
            _ => Err("类型不匹配: 无法比较".to_string()),
        }
    }
    
    fn greater_equal(&self, left: Value, right: Value) -> Result<Value, String> {
        match (left, right) {
            (Value::数字(a), Value::数字(b)) => Ok(Value::布尔(a >= b)),
            _ => Err("类型不匹配: 无法比较".to_string()),
        }
    }
    
    fn less_equal(&self, left: Value, right: Value) -> Result<Value, String> {
        match (left, right) {
            (Value::数字(a), Value::数字(b)) => Ok(Value::布尔(a <= b)),
            _ => Err("类型不匹配: 无法比较".to_string()),
        }
    }
    
    fn is_equal(&self, left: &Value, right: &Value) -> bool {
        match (left, right) {
            (Value::数字(a), Value::数字(b)) => a == b,
            (Value::字符串(a), Value::字符串(b)) => a == b,
            (Value::布尔(a), Value::布尔(b)) => a == b,
            (Value::空, Value::空) => true,
            _ => false,
        }
    }
    
    fn is_truthy(&self, value: &Value) -> bool {
        match value {
            Value::布尔(b) => *b,
            Value::数字(n) => *n != 0.0,
            Value::字符串(s) => !s.is_empty(),
            Value::空 => false,
            _ => true,
        }
    }
    
    fn get_current_environment(&self) -> &Environment {
        &self.environments[self.current_env]
    }
    
    fn get_current_environment_mut(&mut self) -> &mut Environment {
        &mut self.environments[self.current_env]
    }
}