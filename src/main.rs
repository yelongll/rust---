use std::env;
use std::fs;
use std::process;

mod lexer;
mod parser;
mod interpreter;
mod ast;
mod code_generator;

use lexer::Lexer;
use parser::Parser;
use interpreter::Interpreter;
use code_generator::{CodeGenerator, OutputType};

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        println!("用法: cnlang <文件名.cn> [选项]");
        println!("选项:");
        println!("  --compile     编译为可执行文件");
        println!("  --dll         编译为动态链接库");
        println!("  --object      编译为目标文件");
        println!("  --c           生成C源代码");
        println!("  --output <文件>  指定输出文件名");
        process::exit(1);
    }
    
    let filename = &args[1];
    let mut compile_mode = false;
    let mut output_type = None;
    let mut output_file = None;
    
    // 解析命令行参数
    for i in 2..args.len() {
        match args[i].as_str() {
            "--compile" => {
                compile_mode = true;
                output_type = Some(OutputType::Exe);
            }
            "--dll" => {
                compile_mode = true;
                output_type = Some(OutputType::Dll);
            }
            "--object" => {
                compile_mode = true;
                output_type = Some(OutputType::Object);
            }
            "--c" => {
                compile_mode = true;
                output_type = Some(OutputType::CSource);
            }
            "--output" => {
                if i + 1 < args.len() {
                    output_file = Some(args[i + 1].clone());
                }
            }
            _ => {}
        }
    }
    
    match fs::read_to_string(filename) {
        Ok(content) => {
            if compile_mode {
                // 编译模式
                let output_path = output_file.unwrap_or_else(|| {
                    let base_name = filename.trim_end_matches(".cn");
                    match output_type {
                        Some(OutputType::Exe) => format!("{}.exe", base_name),
                        Some(OutputType::Dll) => format!("{}.dll", base_name),
                        Some(OutputType::Object) => format!("{}.o", base_name),
                        Some(OutputType::CSource) => format!("{}.c", base_name),
                        None => format!("{}.exe", base_name),
                    }
                });
                
                let output_type = output_type.unwrap_or(OutputType::Exe);
                match compile_code(&content, output_type, &output_path) {
                    Ok(_) => println!("编译完成: {}", output_path),
                    Err(e) => println!("编译错误: {}", e),
                }
            } else {
                // 解释执行模式
                match run_code(&content) {
                    Ok(_) => println!("程序执行完成"),
                    Err(e) => println!("执行错误: {}", e),
                }
            }
        }
        Err(e) => {
            println!("无法读取文件 {}: {}", filename, e);
            process::exit(1);
        }
    }
}

fn run_code(code: &str) -> Result<(), String> {
    let mut lexer = Lexer::new(code);
    let tokens = lexer.tokenize()?;
    
    let mut parser = Parser::new(tokens);
    let ast = parser.parse()?;
    
    let mut interpreter = Interpreter::new();
    interpreter.execute(&ast)?;
    
    Ok(())
}

fn compile_code(code: &str, output_type: OutputType, output_path: &str) -> Result<(), String> {
    let mut lexer = Lexer::new(code);
    let tokens = lexer.tokenize()?;
    
    let mut parser = Parser::new(tokens);
    let ast = parser.parse()?;
    
    let code_generator = CodeGenerator::new(output_type);
    code_generator.generate(&ast, output_path)?;
    
    Ok(())
}