use crate::lexer::{Token, TokenType};
use crate::ast::{Program, Statement, Expression};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            current: 0,
        }
    }
    
    pub fn parse(&mut self) -> Result<Program, String> {
        let mut program = Program::new();
        
        while !self.is_at_end() {
            if let Some(stmt) = self.parse_statement()? {
                program.添加语句(stmt);
            }
        }
        
        Ok(program)
    }
    
    fn parse_statement(&mut self) -> Result<Option<Statement>, String> {
        if self.match_token(&[TokenType::让, TokenType::常量, TokenType::变量]) {
            return self.parse_variable_declaration();
        }
        
        if self.match_token(&[TokenType::函数]) {
            return self.parse_function_declaration();
        }
        
        if self.match_token(&[TokenType::如果]) {
            return self.parse_if_statement();
        }
        
        if self.match_token(&[TokenType::循环]) {
            return self.parse_loop_statement();
        }
        
        if self.match_token(&[TokenType::当]) {
            return self.parse_while_statement();
        }
        
        if self.match_token(&[TokenType::对于]) {
            return self.parse_for_statement();
        }
        
        if self.match_token(&[TokenType::返回]) {
            return self.parse_return_statement();
        }
        
        if self.match_token(&[TokenType::跳出]) {
            return Ok(Some(Statement::跳出语句));
        }
        
        if self.match_token(&[TokenType::继续]) {
            return Ok(Some(Statement::继续语句));
        }
        
        // 表达式语句
        if let Some(expr) = self.parse_expression()? {
            return Ok(Some(Statement::表达式语句(expr)));
        }
        
        Ok(None)
    }
    
    fn parse_variable_declaration(&mut self) -> Result<Option<Statement>, String> {
        let is_const = self.previous().token_type == TokenType::常量;
        
        // 使用advance_if_matches来匹配标识符
        if let Some(token) = self.advance_if_matches(|t| matches!(t.token_type, TokenType::标识符(_))) {
            let name = if let TokenType::标识符(name) = token.token_type {
                name
            } else {
                return Err("期望变量名".to_string());
            };
            
            let initial_value = if self.match_token(&[TokenType::赋值]) {
                self.parse_expression()?
            } else {
                None
            };
            
            Ok(Some(Statement::变量声明 {
                名字: name,
                初始值: initial_value,
                是常量: is_const,
            }))
        } else {
            Err("期望变量名".to_string())
        }
    }
    
    fn parse_function_declaration(&mut self) -> Result<Option<Statement>, String> {
        // 使用advance_if_matches来匹配标识符
        if let Some(token) = self.advance_if_matches(|t| matches!(t.token_type, TokenType::标识符(_))) {
            let name = if let TokenType::标识符(name) = token.token_type {
                name
            } else {
                return Err("期望函数名".to_string());
            };
            
            if !self.match_token(&[TokenType::左括号]) {
                return Err("期望 '('".to_string());
            }
            
            let mut parameters = Vec::new();
            
            if !self.check(&TokenType::右括号) {
                loop {
                    // 使用advance_if_matches来匹配参数标识符
                    if let Some(param_token) = self.advance_if_matches(|t| matches!(t.token_type, TokenType::标识符(_))) {
                        let param_name = if let TokenType::标识符(name) = param_token.token_type {
                            name
                        } else {
                            return Err("期望参数名".to_string());
                        };
                        
                        parameters.push(param_name);
                    } else {
                        return Err("期望参数名".to_string());
                    }
                    
                    if !self.match_token(&[TokenType::逗号]) {
                        break;
                    }
                }
            }
            
            if !self.match_token(&[TokenType::右括号]) {
                return Err("期望 ')'".to_string());
            }
            
            if !self.match_token(&[TokenType::左大括号]) {
                return Err("期望 '{{'".to_string());
            }
            
            let mut body = Vec::new();
            
            while !self.check(&TokenType::右大括号) && !self.is_at_end() {
                if let Some(stmt) = self.parse_statement()? {
                    body.push(stmt);
                }
            }
            
            if !self.match_token(&[TokenType::右大括号]) {
                return Err("期望 '}}'".to_string());
            }
            
            Ok(Some(Statement::函数声明 {
                名字: name,
                参数: parameters,
                体: body,
            }))
        } else {
            Err("期望函数名".to_string())
        }
    }
    
    fn parse_if_statement(&mut self) -> Result<Option<Statement>, String> {
        let condition = self.parse_expression()?.ok_or_else(|| "期望条件表达式".to_string())?;
        
        if !self.match_token(&[TokenType::左大括号]) {
            return Err("期望 '{{'".to_string());
        }
        
        let mut true_branch = Vec::new();
        
        while !self.check(&TokenType::右大括号) && !self.is_at_end() {
            if let Some(stmt) = self.parse_statement()? {
                true_branch.push(stmt);
            }
        }
        
        if !self.match_token(&[TokenType::右大括号]) {
            return Err("期望 '}}'".to_string());
        }
        
        let mut false_branch = Vec::new();
        
        if self.match_token(&[TokenType::否则]) {
            if self.match_token(&[TokenType::如果]) {
                // 处理 "否则如果" - 嵌套的if语句
                let nested_if = self.parse_if_statement()?;
                if let Some(Statement::如果语句 { 条件, 真分支, 假分支: _ }) = nested_if {
                    false_branch.push(Statement::如果语句 {
                        条件,
                        真分支,
                        假分支: Vec::new(),
                    });
                }
            } else {
                // 处理普通的 "否则"
                if !self.match_token(&[TokenType::左大括号]) {
                    return Err("期望 '{{'".to_string());
                }
                
                while !self.check(&TokenType::右大括号) && !self.is_at_end() {
                    if let Some(stmt) = self.parse_statement()? {
                        false_branch.push(stmt);
                    }
                }
                
                if !self.match_token(&[TokenType::右大括号]) {
                    return Err("期望 '}}'".to_string());
                }
            }
        }
        
        Ok(Some(Statement::如果语句 {
            条件: condition,
            真分支: true_branch,
            假分支: false_branch,
        }))
    }
    
    fn parse_loop_statement(&mut self) -> Result<Option<Statement>, String> {
        let condition = if self.check(&TokenType::左大括号) {
            None
        } else {
            Some(self.parse_expression()?.ok_or_else(|| "期望条件表达式".to_string())?)
        };
        
        if !self.match_token(&[TokenType::左大括号]) {
            return Err("期望 '{{'".to_string());
        }
        
        let mut body = Vec::new();
        
        while !self.check(&TokenType::右大括号) && !self.is_at_end() {
            if let Some(stmt) = self.parse_statement()? {
                body.push(stmt);
            }
        }
        
        if !self.match_token(&[TokenType::右大括号]) {
            return Err("期望 '}}'".to_string());
        }
        
        Ok(Some(Statement::循环语句 {
            条件: condition,
            体: body,
        }))
    }
    
    fn parse_while_statement(&mut self) -> Result<Option<Statement>, String> {
        let condition = self.parse_expression()?.ok_or_else(|| "期望条件表达式".to_string())?;
        
        if !self.match_token(&[TokenType::左大括号]) {
            return Err("期望 '{{'".to_string());
        }
        
        let mut body = Vec::new();
        
        while !self.check(&TokenType::右大括号) && !self.is_at_end() {
            if let Some(stmt) = self.parse_statement()? {
                body.push(stmt);
            }
        }
        
        if !self.match_token(&[TokenType::右大括号]) {
            return Err("期望 '}}'".to_string());
        }
        
        Ok(Some(Statement::当语句 {
            条件: condition,
            体: body,
        }))
    }
    
    fn parse_for_statement(&mut self) -> Result<Option<Statement>, String> {
        // 使用advance_if_matches来匹配标识符
        if let Some(token) = self.advance_if_matches(|t| matches!(t.token_type, TokenType::标识符(_))) {
            let variable = if let TokenType::标识符(name) = token.token_type {
                name
            } else {
                return Err("期望变量名".to_string());
            };
            
            if !self.match_token(&[TokenType::在]) {
                return Err("期望 '在'".to_string());
            }
            
            let iterable = self.parse_expression()?.ok_or_else(|| "期望可迭代表达式".to_string())?;
            
            if !self.match_token(&[TokenType::左大括号]) {
                return Err("期望 '{{'".to_string());
            }
            
            let mut body = Vec::new();
            
            while !self.check(&TokenType::右大括号) && !self.is_at_end() {
                if let Some(stmt) = self.parse_statement()? {
                    body.push(stmt);
                }
            }
            
            if !self.match_token(&[TokenType::右大括号]) {
                return Err("期望 '}}'".to_string());
            }
            
            Ok(Some(Statement::对于语句 {
                变量: variable,
                可迭代: iterable,
                体: body,
            }))
        } else {
            Err("期望变量名".to_string())
        }
    }
    
    fn parse_return_statement(&mut self) -> Result<Option<Statement>, String> {
        let value = if self.check(&TokenType::分号) || self.check(&TokenType::右大括号) {
            None
        } else {
            self.parse_expression()?
        };
        
        Ok(Some(Statement::返回语句(value)))
    }
    
    fn parse_expression(&mut self) -> Result<Option<Expression>, String> {
        self.parse_assignment()
    }
    
    fn parse_assignment(&mut self) -> Result<Option<Expression>, String> {
        let expr = self.parse_logical()?;
        
        if self.match_token(&[TokenType::赋值]) {
            let value = self.parse_assignment()?;
            
            if let Some(expr) = expr {
                if let Expression::变量(name) = expr {
                    return Ok(Some(Expression::赋值 {
                        变量名: name,
                        值: Box::new(value.ok_or_else(|| "期望赋值表达式".to_string())?),
                    }));
                }
            }
            
            return Err("无效的赋值目标".to_string());
        }
        
        Ok(expr)
    }
    
    fn parse_logical(&mut self) -> Result<Option<Expression>, String> {
        let mut expr = self.parse_equality()?;
        
        while self.match_token(&[TokenType::且, TokenType::或]) {
            let operator = match self.previous().token_type {
                TokenType::且 => "&&".to_string(),
                TokenType::或 => "||".to_string(),
                _ => return Err("无效的逻辑运算符".to_string()),
            };
            
            let right = self.parse_equality()?;
            
            if let (Some(left), Some(right)) = (expr, right) {
                expr = Some(Expression::二元运算 {
                    左: Box::new(left),
                    运算符: operator,
                    右: Box::new(right),
                });
            } else {
                return Ok(None);
            }
        }
        
        Ok(expr)
    }
    
    fn parse_equality(&mut self) -> Result<Option<Expression>, String> {
        let mut expr = self.parse_comparison()?;
        
        while self.match_token(&[TokenType::等于, TokenType::不等于]) {
            let operator = match self.previous().token_type {
                TokenType::等于 => "==".to_string(),
                TokenType::不等于 => "!=".to_string(),
                _ => return Err("无效的运算符".to_string()),
            };
            
            let right = self.parse_comparison()?;
            
            if let (Some(left), Some(right)) = (expr, right) {
                expr = Some(Expression::二元运算 {
                    左: Box::new(left),
                    运算符: operator,
                    右: Box::new(right),
                });
            } else {
                return Ok(None);
            }
        }
        
        Ok(expr)
    }
    
    fn parse_comparison(&mut self) -> Result<Option<Expression>, String> {
        let mut expr = self.parse_term()?;
        
        while self.match_token(&[TokenType::大于, TokenType::大于等于, TokenType::小于, TokenType::小于等于]) {
            let operator = match self.previous().token_type {
                TokenType::大于 => ">".to_string(),
                TokenType::大于等于 => ">=".to_string(),
                TokenType::小于 => "<".to_string(),
                TokenType::小于等于 => "<=".to_string(),
                _ => return Err("无效的运算符".to_string()),
            };
            
            let right = self.parse_term()?;
            
            if let (Some(left), Some(right)) = (expr, right) {
                expr = Some(Expression::二元运算 {
                    左: Box::new(left),
                    运算符: operator,
                    右: Box::new(right),
                });
            } else {
                return Ok(None);
            }
        }
        
        Ok(expr)
    }
    
    fn parse_term(&mut self) -> Result<Option<Expression>, String> {
        let mut expr = self.parse_factor()?;
        
        while self.match_token(&[TokenType::加, TokenType::减]) {
            let operator = match self.previous().token_type {
                TokenType::加 => "+".to_string(),
                TokenType::减 => "-".to_string(),
                _ => return Err("无效的运算符".to_string()),
            };
            
            let right = self.parse_factor()?;
            
            if let (Some(left), Some(right)) = (expr, right) {
                expr = Some(Expression::二元运算 {
                    左: Box::new(left),
                    运算符: operator,
                    右: Box::new(right),
                });
            } else {
                return Ok(None);
            }
        }
        
        Ok(expr)
    }
    
    fn parse_factor(&mut self) -> Result<Option<Expression>, String> {
        let mut expr = self.parse_unary()?;
        
        while self.match_token(&[TokenType::乘, TokenType::除]) {
            let operator = match self.previous().token_type {
                TokenType::乘 => "*".to_string(),
                TokenType::除 => "/".to_string(),
                _ => return Err("无效的运算符".to_string()),
            };
            
            let right = self.parse_unary()?;
            
            if let (Some(left), Some(right)) = (expr, right) {
                expr = Some(Expression::二元运算 {
                    左: Box::new(left),
                    运算符: operator,
                    右: Box::new(right),
                });
            } else {
                return Ok(None);
            }
        }
        
        Ok(expr)
    }
    
    fn parse_unary(&mut self) -> Result<Option<Expression>, String> {
        if self.match_token(&[TokenType::减]) {
            let right = self.parse_unary()?;
            return Ok(Some(Expression::二元运算 {
                左: Box::new(Expression::数字字面量(0.0)),
                运算符: "-".to_string(),
                右: Box::new(right.ok_or_else(|| "期望一元运算表达式".to_string())?),
            }));
        }
        
        if self.match_token(&[TokenType::非]) {
            let right = self.parse_unary()?;
            return Ok(Some(Expression::一元运算 {
                运算符: "!".to_string(),
                操作数: Box::new(right.ok_or_else(|| "期望一元运算表达式".to_string())?),
            }));
        }
        
        let mut expr = self.parse_primary()?;
        
        // 处理后缀表达式（数组方法调用）
        if let Some(e) = expr {
            expr = Some(self.parse_postfix(e)?);
        }
        
        Ok(expr)
    }
    
    fn parse_postfix(&mut self, expr: Expression) -> Result<Expression, String> {
        let mut result = expr;
        
        // 检查是否有后缀操作（数组方法调用）
        while self.match_token(&[TokenType::点]) {
            // 使用advance_if_matches来匹配标识符
            if let Some(token) = self.advance_if_matches(|t| matches!(t.token_type, TokenType::标识符(_))) {
                let method_name = if let TokenType::标识符(name) = token.token_type {
                    name
                } else {
                    return Err("期望方法名".to_string());
                };
                
                match method_name.as_str() {
                    "长度" => {
                        if !self.match_token(&[TokenType::左括号]) {
                            return Err("期望 '('".to_string());
                        }
                        
                        if !self.match_token(&[TokenType::右括号]) {
                            return Err("期望 ')'".to_string());
                        }
                        
                        result = Expression::数组长度(Box::new(result));
                    },
                    "添加" => {
                        if !self.match_token(&[TokenType::左括号]) {
                            return Err("期望 '('".to_string());
                        }
                        
                        let element_expr = self.parse_expression()?;
                        if element_expr.is_none() {
                            return Err("期望元素表达式".to_string());
                        }
                        
                        if !self.match_token(&[TokenType::右括号]) {
                            return Err("期望 ')'".to_string());
                        }
                        
                        result = Expression::数组添加 {
                            数组: Box::new(result),
                            元素: Box::new(element_expr.unwrap()),
                        };
                    },
                    "删除" => {
                        if !self.match_token(&[TokenType::左括号]) {
                            return Err("期望 '('".to_string());
                        }
                        
                        let index_expr = self.parse_expression()?;
                        if index_expr.is_none() {
                            return Err("期望索引表达式".to_string());
                        }
                        
                        if !self.match_token(&[TokenType::右括号]) {
                            return Err("期望 ')'".to_string());
                        }
                        
                        result = Expression::数组删除 {
                            数组: Box::new(result),
                            索引: Box::new(index_expr.unwrap()),
                        };
                    },
                    _ => {
                        return Err(format!("未知的方法: {}", method_name));
                    }
                }
            } else {
                return Err("期望方法名".to_string());
            }
        }
        
        Ok(result)
    }
    
    fn parse_primary(&mut self) -> Result<Option<Expression>, String> {
        // 直接检查数字token
        if let TokenType::数字(value) = &self.peek().token_type {
            let token = self.advance();
            if let TokenType::数字(value) = token.token_type {
                return Ok(Some(Expression::数字字面量(value)));
            }
        }
        
        if let Some(token) = self.advance_if_matches(|t| matches!(t.token_type, TokenType::字符串(_))) {
            if let TokenType::字符串(value) = token.token_type {
                return Ok(Some(Expression::字符串字面量(value)));
            }
        }
        
        if let Some(token) = self.advance_if_matches(|t| matches!(t.token_type, TokenType::布尔(_))) {
            if let TokenType::布尔(value) = token.token_type {
                // 将布尔值转换为数字字面量（1.0表示true，0.0表示false）
                // 这样可以与现有的is_truthy函数兼容
                let num_value = if value { 1.0 } else { 0.0 };
                return Ok(Some(Expression::数字字面量(num_value)));
            }
        }
        
        // 解析数组字面量 [1, 2, 3]
        if self.match_token(&[TokenType::左中括号]) {
            let mut elements = Vec::new();
            
            if !self.check(&TokenType::右中括号) {
                loop {
                    if let Some(element) = self.parse_expression()? {
                        elements.push(element);
                    }
                    
                    if !self.match_token(&[TokenType::逗号]) {
                        break;
                    }
                }
            }
            
            if !self.match_token(&[TokenType::右中括号]) {
                return Err("期望 ']'".to_string());
            }
            
            return Ok(Some(Expression::数组字面量(elements)));
        }
        
        if let Some(token) = self.advance_if_matches(|t| matches!(t.token_type, TokenType::标识符(_))) {
            if let TokenType::标识符(name) = token.token_type {
                if self.match_token(&[TokenType::左括号]) {
                    return self.parse_function_call(name);
                }
                
                // 检查是否是数组索引访问 array[index]
                if self.match_token(&[TokenType::左中括号]) {
                    let index_expr = self.parse_expression()?;
                    if !self.match_token(&[TokenType::右中括号]) {
                        return Err("期望 ']'".to_string());
                    }
                    
                    if let Some(index) = index_expr {
                        return Ok(Some(Expression::数组索引 {
                            数组: Box::new(Expression::变量(name)),
                            索引: Box::new(index),
                        }));
                    } else {
                        return Err("期望索引表达式".to_string());
                    }
                }
                
                return Ok(Some(Expression::变量(name)));
            }
        }
        
        if self.match_token(&[TokenType::左括号]) {
            let expr = self.parse_expression()?;
            
            if !self.match_token(&[TokenType::右括号]) {
                return Err("期望 ')'".to_string());
            }
            
            return Ok(expr);
        }
        
        // 数组方法调用将在更高级别的解析方法中处理
        
        Ok(None)
    }
    
    fn parse_function_call(&mut self, function_name: String) -> Result<Option<Expression>, String> {
        let mut arguments = Vec::new();
        
        if !self.check(&TokenType::右括号) {
            loop {
                if let Some(arg) = self.parse_expression()? {
                    arguments.push(arg);
                }
                
                if !self.match_token(&[TokenType::逗号]) {
                    break;
                }
            }
        }
        
        if !self.match_token(&[TokenType::右括号]) {
            return Err("期望 ')'".to_string());
        }
        
        Ok(Some(Expression::函数调用 {
            函数名: function_name,
            参数: arguments,
        }))
    }
    

    
    fn match_token(&mut self, types: &[TokenType]) -> bool {
        for token_type in types {
            if self.check(token_type) {
                let _ = self.advance();
                return true;
            }
        }
        false
    }
    
    fn check(&self, token_type: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        
        let current_token = &self.tokens[self.current];
        
        match (token_type, &current_token.token_type) {
            (TokenType::标识符(_), TokenType::标识符(_)) => true,
            (TokenType::数字(_), TokenType::数字(_)) => true,
            (TokenType::字符串(_), TokenType::字符串(_)) => true,
            (TokenType::错误(_), TokenType::错误(_)) => true,
            _ => std::mem::discriminant(token_type) == std::mem::discriminant(&current_token.token_type),
        }
    }
    
    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous().clone()
    }
    
    fn advance_if_matches<F>(&mut self, predicate: F) -> Option<Token>
    where
        F: Fn(&Token) -> bool,
    {
        let current_token = self.peek();
        let matches = !self.is_at_end() && predicate(&current_token);
        if matches {
            Some(self.advance())
        } else {
            None
        }
    }
    
    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len() - 1
    }
    
    fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }
    
    fn peek(&self) -> Token {
        self.tokens[self.current].clone()
    }
}