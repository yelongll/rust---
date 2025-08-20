use crate::ast::{Program, Statement, Expression};
use std::fs;
use std::process::Command;

pub struct CodeGenerator {
    output_type: OutputType,
}

#[derive(Debug, Clone)]
pub enum OutputType {
    Exe,
    Dll,
    Object,
    CSource,
}

impl CodeGenerator {
    pub fn new(output_type: OutputType) -> Self {
        CodeGenerator {
            output_type,
        }
    }
    
    pub fn generate(&self, program: &Program, output_path: &str) -> Result<(), String> {
        match self.output_type {
            OutputType::CSource => {
                let c_code = self.generate_c_code(program);
                fs::write(output_path, c_code).map_err(|e| format!("写入文件失败: {}", e))?;
                Ok(())
            }
            OutputType::Exe => {
                let c_code = self.generate_c_code(program);
                let c_file = "temp_output.c";
                println!("C代码长度: {}", c_code.len());
                fs::write(c_file, c_code).map_err(|e| format!("写入临时文件失败: {}", e))?;
                println!("临时文件写入完成");
                
                // 使用mingw64中的gcc编译C代码为exe
                let gcc_path = "c:\\users\\administrator\\desktop\\rust中文版\\mingw64\\bin\\gcc.exe";
                println!("编译命令: {} -o {} {}", gcc_path, output_path, c_file);
                let output = Command::new(gcc_path)
                    .args(&["-o", output_path, c_file])
                    .output()
                    .map_err(|e| format!("编译失败: {}", e))?;
                println!("编译退出码: {}", output.status.code().unwrap_or(-1));
                    
                if !output.status.success() {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    println!("STDERR: {}", stderr);
                    println!("STDOUT: {}", stdout);
                    let error = if stderr.is_empty() { stdout } else { stderr };
                    return Err(format!("编译错误: {}", error));
                }
                
                // 清理临时文件
                let _ = fs::remove_file(c_file);
                Ok(())
            }
            OutputType::Dll => {
                let c_code = self.generate_c_code(program);
                let c_file = "temp_output.c";
                fs::write(c_file, c_code).map_err(|e| format!("写入临时文件失败: {}", e))?;
                
                // 使用mingw64中的gcc编译C代码为dll
                let gcc_path = "c:\\users\\administrator\\desktop\\rust中文版\\mingw64\\bin\\gcc.exe";
                let output = Command::new(gcc_path)
                    .args(&["-shared", "-o", output_path, c_file])
                    .output()
                    .map_err(|e| format!("编译失败: {}", e))?;
                    
                if !output.status.success() {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    let error = if stderr.is_empty() { stdout } else { stderr };
                    return Err(format!("编译错误: {}", error));
                }
                
                // 清理临时文件
                let _ = fs::remove_file(c_file);
                Ok(())
            }
            OutputType::Object => {
                let c_code = self.generate_c_code(program);
                let c_file = "temp_output.c";
                fs::write(c_file, c_code).map_err(|e| format!("写入临时文件失败: {}", e))?;
                
                // 使用mingw64中的gcc编译C代码为目标文件
                let gcc_path = "c:\\users\\administrator\\desktop\\rust中文版\\mingw64\\bin\\gcc.exe";
                let output = Command::new(gcc_path)
                    .args(&["-c", "-o", output_path, c_file])
                    .output()
                    .map_err(|e| format!("编译失败: {}", e))?;
                    
                if !output.status.success() {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    let error = if stderr.is_empty() { stdout } else { stderr };
                    return Err(format!("编译错误: {}", error));
                }
                
                // 清理临时文件
                let _ = fs::remove_file(c_file);
                Ok(())
            }
        }
    }
    
    fn generate_c_code(&self, program: &Program) -> String {
        let mut c_code = String::new();
        
        // 添加C头文件
        c_code.push_str("#include <stdio.h>\n");
        c_code.push_str("#include <stdlib.h>\n");
        c_code.push_str("#include <string.h>\n");
        c_code.push_str("#include <stdbool.h>\n\n");
        
        // 添加值类型的定义
        c_code.push_str("typedef enum {\n");
        c_code.push_str("    VALUE_NUMBER,\n");
        c_code.push_str("    VALUE_STRING,\n");
        c_code.push_str("    VALUE_BOOLEAN,\n");
        c_code.push_str("    VALUE_ARRAY,\n");
        c_code.push_str("    VALUE_NULL\n");
        c_code.push_str("} ValueType;\n\n");
        
        // 添加前向声明
        c_code.push_str("// 前向声明\n");
        c_code.push_str("typedef struct Value Value;\n\n");
        
        c_code.push_str("struct Value {\n");
        c_code.push_str("    ValueType type;\n");
        c_code.push_str("    union {\n");
        c_code.push_str("        double number;\n");
        c_code.push_str("        char* string;\n");
        c_code.push_str("        bool boolean;\n");
        c_code.push_str("        struct {\n");
        c_code.push_str("            struct Value* values;\n");
        c_code.push_str("            int count;\n");
        c_code.push_str("            int capacity;\n");
        c_code.push_str("        } array;\n");
        c_code.push_str("    } as;\n");
        c_code.push_str("};\n\n");
        
        // 添加内置函数声明
        c_code.push_str("void builtin_print(Value value);\n");
        c_code.push_str("Value builtin_input();\n");
        c_code.push_str("Value create_array();\n");
        c_code.push_str("void array_push(Value* array, Value value);\n");
        c_code.push_str("Value array_get(Value array, int index);\n");
        c_code.push_str("int array_length(Value array);\n");
        c_code.push_str("void array_remove(Value* array, int index);\n");
        
        // 添加辅助函数声明
        c_code.push_str("bool is_truthy(Value value);\n");
        c_code.push_str("int compare_values(Value a, Value b);\n");
        c_code.push_str("Value add_values(Value a, Value b);\n");
        c_code.push_str("Value subtract_values(Value a, Value b);\n");
        c_code.push_str("Value multiply_values(Value a, Value b);\n");
        c_code.push_str("Value divide_values(Value a, Value b);\n");
        c_code.push_str("Value negate_value(Value value);\n\n");
        
        // 添加内置函数实现
        c_code.push_str(&self.generate_builtin_functions());
        
        // 分离函数声明和其他语句
        let mut function_declarations = Vec::new();
        let mut other_statements = Vec::new();
        
        for statement in &program.语句 {
            match statement {
                Statement::函数声明 { .. } => {
                    function_declarations.push(statement);
                }
                _ => {
                    other_statements.push(statement);
                }
            }
        }
        
        // 添加函数定义到main函数外部
        for func_decl in &function_declarations {
            if let Statement::函数声明 { 名字, 参数, 体 } = func_decl {
                let args: Vec<String> = 参数.iter().enumerate().map(|(i, _)| format!("Value arg{}", i)).collect();
                let args_str = args.join(", ");
                
                // 生成函数体，需要将参数名映射到arg0, arg1等
                let mut body_code = String::new();
                for stmt in 体 {
                    let stmt_code = self.generate_statement_c_code(stmt);
                    // 替换参数名
                    let mut replaced_code = stmt_code;
                    for (i, param_name) in 参数.iter().enumerate() {
                        replaced_code = replaced_code.replace(param_name, &format!("arg{}", i));
                    }
                    body_code.push_str(&replaced_code);
                    body_code.push_str("\n");
                }
                
                // 生成完整的函数定义
                c_code.push_str(&format!("Value {}({}) {{\n", 名字, args_str));
                for line in body_code.lines() {
                    c_code.push_str(&format!("    {}\n", line));
                }
                c_code.push_str("}\n\n");
            }
        }
        
        // 重新定义main函数，包含其他语句
        c_code.push_str("int main() {\n");
        c_code.push_str("    // Program entry point\n");
        for statement in &other_statements {
            let stmt_code = self.generate_statement_c_code(statement);
            // 为每行代码添加缩进
            for line in stmt_code.lines() {
                c_code.push_str(&format!("    {}\n", line));
            }
        }
        c_code.push_str("    return 0;\n");
        c_code.push_str("}\n");
        
        c_code
    }
    
    fn generate_statement_c_code(&self, statement: &Statement) -> String {
        match statement {
            Statement::表达式语句(expr) => {
                format!("{};", self.generate_expression_c_code(expr))
            }
            Statement::变量声明 { 名字, 初始值, 是常量 } => {
                let const_str = if *是常量 { "const " } else { "" };
                match 初始值 {
                    Some(expr) => {
                        if let Expression::数组字面量(元素) = expr {
                            if 元素.is_empty() {
                                format!("Value {}{} = create_array();", const_str, 名字)
                            } else {
                                let mut code = format!("Value {}{} = create_array();", const_str, 名字);
                                for element in 元素 {
                                    let element_code = self.generate_expression_c_code(element);
                                    code.push_str(&format!("\n    array_push(&{}, {});", 名字, element_code));
                                }
                                code
                            }
                        } else {
                            let expr_code = self.generate_expression_c_code(expr);
                            format!("Value {}{} = {};", const_str, 名字, expr_code)
                        }
                    }
                    None => {
                        format!("Value {}{} = create_null();", const_str, 名字)
                    }
                }
            }
            Statement::函数声明 { 名字, 参数, 体 } => {
                let args: Vec<String> = 参数.iter().map(|_| "Value".to_string()).collect();
                let args_str = args.join(", ");
                // 在C语言中，函数声明应该放在main函数外部
                format!("Value {}({});", 名字, args_str)
            }
            Statement::如果语句 { 条件, 真分支, 假分支 } => {
                let condition_code = self.generate_expression_c_code(条件);
                let mut true_body = String::new();
                for stmt in 真分支 {
                    true_body.push_str(&self.generate_statement_c_code(stmt));
                    true_body.push_str("\n");
                }
                let mut false_body = String::new();
                for stmt in 假分支 {
                    false_body.push_str(&self.generate_statement_c_code(stmt));
                    false_body.push_str("\n");
                }
                format!("if ({}) {{\n{}\n}} else {{\n{}\n}}", condition_code, true_body, false_body)
            }
            Statement::循环语句 { 条件, 体 } => {
                let mut body_code = String::new();
                for stmt in 体 {
                    body_code.push_str(&self.generate_statement_c_code(stmt));
                    body_code.push_str("\n");
                }
                match 条件 {
                    Some(cond) => {
                        let condition_code = self.generate_expression_c_code(cond);
                        format!("while ({}) {{\n{}\n}}", condition_code, body_code)
                    }
                    None => {
                        format!("while (1) {{\n{}\n}}", body_code)
                    }
                }
            }
            Statement::当语句 { 条件, 体 } => {
                let condition_code = self.generate_expression_c_code(条件);
                let mut body_code = String::new();
                for stmt in 体 {
                    body_code.push_str(&self.generate_statement_c_code(stmt));
                    body_code.push_str("\n");
                }
                format!("do {{\n{}\n}} while ({});", body_code, condition_code)
            }
            Statement::对于语句 { 变量, 可迭代, 体 } => {
                let iterable_code = self.generate_expression_c_code(可迭代);
                let mut body_code = String::new();
                for stmt in 体 {
                    body_code.push_str(&self.generate_statement_c_code(stmt));
                    body_code.push_str("\n");
                }
                format!("for (int {}_i = 0; {}_i < {}.as.array.count; {}_i++) {{\n    Value {} = {}.as.array.values[{}_i];\n{}\n}}", 
                       变量, 变量, iterable_code, 变量, 变量, iterable_code, 变量, body_code)
            }
            Statement::返回语句(值) => {
                match 值 {
                    Some(expr) => {
                        format!("return {};", self.generate_expression_c_code(expr))
                    }
                    None => {
                        "return;".to_string()
                    }
                }
            }
            Statement::跳出语句 => "break;".to_string(),
            Statement::继续语句 => "continue;".to_string(),
        }
    }
    
    fn generate_expression_c_code(&self, expr: &Expression) -> String {
        match expr {
            Expression::数字字面量(n) => {
                format!("(Value){{VALUE_NUMBER, .as.number = {}}}", n)
            }
            Expression::字符串字面量(s) => {
                format!("(Value){{VALUE_STRING, .as.string = \"{}\"}}", s)
            }
            Expression::变量(name) => name.clone(),
            Expression::二元运算 { 左, 运算符, 右 } => {
                let left_code = self.generate_expression_c_code(左);
                let right_code = self.generate_expression_c_code(右);
                match 运算符.as_str() {
                    "+" => format!("add_values({}, {})", left_code, right_code),
                    "-" => format!("subtract_values({}, {})", left_code, right_code),
                    "*" => format!("multiply_values({}, {})", left_code, right_code),
                    "/" => format!("divide_values({}, {})", left_code, right_code),
                    "==" => format!("(Value){{VALUE_BOOLEAN, .as.boolean = compare_values({}, {}) == 0}}", left_code, right_code),
                    "!=" => format!("(Value){{VALUE_BOOLEAN, .as.boolean = compare_values({}, {}) != 0}}", left_code, right_code),
                    "<" => format!("(Value){{VALUE_BOOLEAN, .as.boolean = compare_values({}, {}) < 0}}", left_code, right_code),
                    ">" => format!("(Value){{VALUE_BOOLEAN, .as.boolean = compare_values({}, {}) > 0}}", left_code, right_code),
                    "<=" => format!("(Value){{VALUE_BOOLEAN, .as.boolean = compare_values({}, {}) <= 0}}", left_code, right_code),
                    ">=" => format!("(Value){{VALUE_BOOLEAN, .as.boolean = compare_values({}, {}) >= 0}}", left_code, right_code),
                    "&&" => format!("(Value){{VALUE_BOOLEAN, .as.boolean = is_truthy({}) && is_truthy({})}}", left_code, right_code),
                    "||" => format!("(Value){{VALUE_BOOLEAN, .as.boolean = is_truthy({}) || is_truthy({})}}", left_code, right_code),
                    _ => format!("/* 未知运算符 {} */ (Value){{VALUE_NULL}}", 运算符),
                }
            }
            Expression::一元运算 { 运算符, 操作数 } => {
                let operand_code = self.generate_expression_c_code(操作数);
                match 运算符.as_str() {
                    "!" => format!("(Value){{VALUE_BOOLEAN, .as.boolean = !is_truthy({})}}", operand_code),
                    "-" => format!("negate_value({})", operand_code),
                    _ => format!("/* 未知一元运算符 {} */ (Value){{VALUE_NULL}}", 运算符),
                }
            }
            Expression::函数调用 { 函数名, 参数 } => {
                let args: Vec<String> = 参数.iter().map(|arg| self.generate_expression_c_code(arg)).collect();
                // 映射内置函数名
                let c_function_name = match 函数名.as_str() {
                    "打印" => "builtin_print",
                    "输入" => "builtin_input",
                    "创建数组" => "create_array",
                    "数组长度" => "array_length",
                    "数组添加" => "array_push",
                    "数组删除" => "array_remove",
                    "数组索引" => "array_get",
                    _ => 函数名.as_str(),
                };
                format!("{}({})", c_function_name, args.join(", "))
            }
            Expression::赋值 { 变量名, 值 } => {
                let value_code = self.generate_expression_c_code(值);
                format!("{} = {}", 变量名, value_code)
            }
            Expression::数组字面量(元素) => {
                if 元素.is_empty() {
                    "create_array()".to_string()
                } else {
                    // 生成一个临时变量来存储数组
                    let mut code = "({ ".to_string();
                    code.push_str("Value temp_array = create_array(); ");
                    for element in 元素 {
                        let element_code = self.generate_expression_c_code(element);
                        code.push_str(&format!("array_push(&temp_array, {}); ", element_code));
                    }
                    code.push_str("temp_array; })");
                    code
                }
            }
            Expression::数组索引 { 数组, 索引 } => {
                let array_code = self.generate_expression_c_code(数组);
                let index_code = self.generate_expression_c_code(索引);
                format!("array_get({}, (int){}.as.number)", array_code, index_code)
            }
            Expression::数组长度(数组) => {
                let array_code = self.generate_expression_c_code(数组);
                format!("(Value){{VALUE_NUMBER, .as.number = array_length({})}}", array_code)
            }
            Expression::数组添加 { 数组, 元素 } => {
                let array_code = self.generate_expression_c_code(数组);
                let element_code = self.generate_expression_c_code(元素);
                format!("array_push(&{}, {})", array_code, element_code)
            }
            Expression::数组删除 { 数组, 索引 } => {
                let array_code = self.generate_expression_c_code(数组);
                let index_code = self.generate_expression_c_code(索引);
                format!("array_remove(&{}, (int){}.as.number)", array_code, index_code)
            }
        }
    }
    
    fn generate_builtin_functions(&self) -> String {
        let mut code = String::new();
        
        code.push_str("\n// 内置函数实现\n");
        code.push_str("void builtin_print(Value value) {\n");
        code.push_str("    switch (value.type) {\n");
        code.push_str("        case VALUE_NUMBER:\n");
        code.push_str("            printf(\"%f\\n\", value.as.number);\n");
        code.push_str("            break;\n");
        code.push_str("        case VALUE_STRING:\n");
        code.push_str("            printf(\"%s\\n\", value.as.string);\n");
        code.push_str("            break;\n");
        code.push_str("        case VALUE_BOOLEAN:\n");
        code.push_str("            printf(\"%s\\n\", value.as.boolean ? \"true\" : \"false\");\n");
        code.push_str("            break;\n");
        code.push_str("        case VALUE_ARRAY:\n");
        code.push_str("            printf(\"[\");\n");
        code.push_str("            for (int i = 0; i < value.as.array.count; i++) {\n");
        code.push_str("                if (i > 0) printf(\", \");\n");
        code.push_str("                builtin_print(value.as.array.values[i]);\n");
        code.push_str("            }\n");
        code.push_str("            printf(\"]\\n\");\n");
        code.push_str("            break;\n");
        code.push_str("        case VALUE_NULL:\n");
        code.push_str("            printf(\"null\\n\");\n");
        code.push_str("            break;\n");
        code.push_str("    }\n");
        code.push_str("}\n\n");
        
        code.push_str("Value builtin_input() {\n");
        code.push_str("    char buffer[1024];\n");
        code.push_str("    if (fgets(buffer, sizeof(buffer), stdin)) {\n");
        code.push_str("        buffer[strcspn(buffer, \"\\n\")] = '\\0';\n");
        code.push_str("        return (Value){VALUE_STRING, .as.string = strdup(buffer)};\n");
        code.push_str("    }\n");
        code.push_str("    return (Value){VALUE_NULL};\n");
        code.push_str("}\n\n");
        
        code.push_str("Value create_array() {\n");
        code.push_str("    Value array = {VALUE_ARRAY};\n");
        code.push_str("    array.as.array.values = malloc(sizeof(Value) * 8);\n");
        code.push_str("    array.as.array.count = 0;\n");
        code.push_str("    array.as.array.capacity = 8;\n");
        code.push_str("    return array;\n");
        code.push_str("}\n\n");
        
        code.push_str("void array_push(Value* array, Value value) {\n");
        code.push_str("    if (array->as.array.count >= array->as.array.capacity) {\n");
        code.push_str("        array->as.array.capacity *= 2;\n");
        code.push_str("        array->as.array.values = realloc(array->as.array.values, sizeof(Value) * array->as.array.capacity);\n");
        code.push_str("    }\n");
        code.push_str("    array->as.array.values[array->as.array.count++] = value;\n");
        code.push_str("}\n\n");
        
        code.push_str("Value array_get(Value array, int index) {\n");
        code.push_str("    if (index < 0 || index >= array.as.array.count) {\n");
        code.push_str("        return (Value){VALUE_NULL};\n");
        code.push_str("    }\n");
        code.push_str("    return array.as.array.values[index];\n");
        code.push_str("}\n\n");
        
        code.push_str("int array_length(Value array) {\n");
        code.push_str("    return array.as.array.count;\n");
        code.push_str("}\n\n");
        code.push_str("\n");
        
        code.push_str("void array_remove(Value* array, int index) {\n");
        code.push_str("    if (index < 0 || index >= array->as.array.count) return;\n");
        code.push_str("    for (int i = index; i < array->as.array.count - 1; i++) {\n");
        code.push_str("        array->as.array.values[i] = array->as.array.values[i + 1];\n");
        code.push_str("    }\n");
        code.push_str("    array->as.array.count--;\n");
        code.push_str("}\n\n");
        
        code.push_str("// 辅助函数\n");
        code.push_str("bool is_truthy(Value value) {\n");
        code.push_str("    switch (value.type) {\n");
        code.push_str("        case VALUE_BOOLEAN: return value.as.boolean;\n");
        code.push_str("        case VALUE_NUMBER: return value.as.number != 0;\n");
        code.push_str("        case VALUE_STRING: return strlen(value.as.string) > 0;\n");
        code.push_str("        case VALUE_ARRAY: return value.as.array.count > 0;\n");
        code.push_str("        default: return false;\n");
        code.push_str("    }\n");
        code.push_str("}\n\n");
        
        code.push_str("int compare_values(Value a, Value b) {\n");
        code.push_str("    if (a.type != b.type) return 0;\n");
        code.push_str("    switch (a.type) {\n");
        code.push_str("        case VALUE_NUMBER: return (a.as.number > b.as.number) - (a.as.number < b.as.number);\n");
        code.push_str("        case VALUE_STRING: return strcmp(a.as.string, b.as.string);\n");
        code.push_str("        case VALUE_BOOLEAN: return (a.as.boolean > b.as.boolean) - (a.as.boolean < b.as.boolean);\n");
        code.push_str("        default: return 0;\n");
        code.push_str("    }\n");
        code.push_str("}\n\n");
        
        code.push_str("Value add_values(Value a, Value b) {\n");
        code.push_str("    if (a.type == VALUE_NUMBER && b.type == VALUE_NUMBER) {\n");
        code.push_str("        return (Value){VALUE_NUMBER, .as.number = a.as.number + b.as.number};\n");
        code.push_str("    }\n");
        code.push_str("    if (a.type == VALUE_STRING && b.type == VALUE_STRING) {\n");
        code.push_str("        char* result = malloc(strlen(a.as.string) + strlen(b.as.string) + 1);\n");
        code.push_str("        strcpy(result, a.as.string);\n");
        code.push_str("        strcat(result, b.as.string);\n");
        code.push_str("        return (Value){VALUE_STRING, .as.string = result};\n");
        code.push_str("    }\n");
        code.push_str("    return (Value){VALUE_NULL};\n");
        code.push_str("}\n\n");
        
        code.push_str("Value subtract_values(Value a, Value b) {\n");
        code.push_str("    if (a.type == VALUE_NUMBER && b.type == VALUE_NUMBER) {\n");
        code.push_str("        return (Value){VALUE_NUMBER, .as.number = a.as.number - b.as.number};\n");
        code.push_str("    }\n");
        code.push_str("    return (Value){VALUE_NULL};\n");
        code.push_str("}\n\n");
        
        code.push_str("Value multiply_values(Value a, Value b) {\n");
        code.push_str("    if (a.type == VALUE_NUMBER && b.type == VALUE_NUMBER) {\n");
        code.push_str("        return (Value){VALUE_NUMBER, .as.number = a.as.number * b.as.number};\n");
        code.push_str("    }\n");
        code.push_str("    return (Value){VALUE_NULL};\n");
        code.push_str("}\n\n");
        
        code.push_str("Value divide_values(Value a, Value b) {\n");
        code.push_str("    if (a.type == VALUE_NUMBER && b.type == VALUE_NUMBER) {\n");
        code.push_str("        if (b.as.number == 0) return (Value){VALUE_NULL};\n");
        code.push_str("        return (Value){VALUE_NUMBER, .as.number = a.as.number / b.as.number};\n");
        code.push_str("    }\n");
        code.push_str("    return (Value){VALUE_NULL};\n");
        code.push_str("}\n\n");
        
        code.push_str("Value negate_value(Value value) {\n");
        code.push_str("    if (value.type == VALUE_NUMBER) {\n");
        code.push_str("        return (Value){VALUE_NUMBER, .as.number = -value.as.number};\n");
        code.push_str("    }\n");
        code.push_str("    return (Value){VALUE_NULL};\n");
        code.push_str("}\n\n");
        
        code
    }
}