# Chinese Programming Language Compiler / 中文编程语言编译器

**Author / 作者：** 灵创-浪漫之夏  
**QQ Group / QQ群：** 624826263

## Project Introduction / 项目简介

This is a Chinese programming language compiler/interpreter developed in Rust, designed to enable Chinese users to program using Chinese keywords and syntax, lowering the programming learning threshold and improving code readability.

这是一个用Rust开发的中文编程语言编译器/解释器，旨在让中文用户能够使用中文关键字和语法进行编程，降低编程学习门槛，提高代码可读性。

### Rust Advantages / Rust优势

**Built with Rust for Excellence / 用Rust打造卓越性能：**

- **Memory Safety / 内存安全**: Rust's ownership system ensures memory safety without garbage collection, preventing common bugs like null pointer dereferences and buffer overflows.
  Rust的所有权系统确保内存安全，无需垃圾回收，防止空指针解引用和缓冲区溢出等常见错误。

- **High Performance / 高性能**: Rust provides zero-cost abstractions and compiles to native machine code, offering performance comparable to C and C++ while maintaining safety.
  Rust提供零成本抽象，编译为本地机器码，提供与C和C++相当的性能，同时保持安全性。

- **Concurrency Support / 并发支持**: Rust's fearless concurrency model enables safe parallel programming, making it ideal for building efficient and reliable compiler systems.
  Rust的无畏并发模型支持安全的并行编程，使其成为构建高效可靠编译器系统的理想选择。

- **Cross-Platform / 跨平台**: Rust compiles to multiple platforms (Windows, macOS, Linux) ensuring consistent behavior across different operating systems.
  Rust可编译到多个平台（Windows、macOS、Linux），确保在不同操作系统上的一致行为。

- **Modern Tooling / 现代工具链**: Rust's package manager (Cargo) and build system provide excellent dependency management and development experience.
  Rust的包管理器（Cargo）和构建系统提供出色的依赖管理和开发体验。

## LingChang Language Integration / 灵创语言集成

This compiler is an integral part of the **LingChang Language (灵创语言)** ecosystem. LingChang Language is a complete compiler + Integrated Development Environment (IDE) solution designed specifically for Chinese programming. This compiler component will continue to receive updates and improvements as part of the ongoing development of the LingChang Language platform.

本编译器是**灵创语言**生态系统的重要组成部分。灵创语言是一个完整的编译器+集成开发环境(IDE)解决方案，专为中文编程设计。作为灵创语言平台持续发展的一部分，本编译器将继续获得更新和改进。

## Features / 功能特性

### Core Features / 核心功能
- **Chinese Keyword Support / 中文关键字支持**：Use Chinese keywords like `如果` (if), `循环` (loop), `函数` (function), etc.
- **Variable System / 变量系统**：Support variable declaration and assignment (`让` keyword)
- **Data Types / 数据类型**：Support basic data types like numbers, strings, boolean values
- **Function System / 函数系统**：Support function definition, parameter passing, and recursive calls
- **Control Flow / 控制流**：Support conditional statements (`如果`/`否则`), loop statements (`循环`, `当`, `对于`)
- **String Operations / 字符串操作**：Support string concatenation, escape character handling
- **Built-in Functions / 内置函数**：Provide printing (`内置打印`) and input (`内置输入`) functions

### Advanced Features / 高级特性
- **Scope Management / 作用域管理**：Support local variables and function scope
- **Recursion Support / 递归支持**：Complete support for recursive function calls
- **Error Handling / 错误处理**：Provide friendly error messages
- **Escape Characters / 转义字符**：Support common escape sequences like `\n`, `\t`, `\\`
- **Comment Support / 注释支持**：Support single-line comments (`//`)

## Syntax Guide / 语法说明

### Variable Declaration and Assignment / 变量声明和赋值
```cn
// Declare and initialize variables / 声明并初始化变量
让 年龄 = 25
让 姓名 = "张三"

// Variable assignment / 变量赋值
年龄 = 26
```

### Basic Data Types / 基本数据类型
```cn
// Numbers / 数字
让 数字 = 42
让 小数 = 3.14

// Strings / 字符串
让 文本 = "你好，世界！"
让 带换行的文本 = "第一行\n第二行"

// Boolean values / 布尔值
让 是真的 = true
让 是假的 = false
```

### Conditional Statements / 条件语句
```cn
让 分数 = 85

如果 分数 >= 90 {
    打印("优秀")
} 否则 如果 分数 >= 80 {
    打印("良好")
} 否则 如果 分数 >= 60 {
    打印("及格")
} 否则 {
    打印("不及格")
}
```

### Loop Statements / 循环语句
```cn
// While loop / 当循环
让 计数器 = 0
当 计数器 < 5 {
    打印(计数器)
    计数器 = 计数器 + 1
}

// For loop / 对于循环
对于 i 在 1 到 5 {
    打印(i)
}

// String traversal / 遍历字符串
让 文本 = "你好"
对于 字符 在 文本 {
    打印("字符: " + 字符)
}
```

### Function Definition and Calling / 函数定义和调用
```cn
// Define function / 定义函数
函数 打印问候(姓名) {
    打印("你好, " + 姓名 + "!")
}

// Call function / 调用函数
打印问候("张三")

// Function with return value / 带返回值的函数
function 计算阶乘(n) {
    如果 n <= 1 {
        返回 1
    }
    返回 n * 计算阶乘(n - 1)
}

让 结果 = 计算阶乘(5)
打印("5的阶乘是: " + 结果)
```

### String Operations / 字符串操作
```cn
// String concatenation / 字符串连接
让 名字 = "张"
让 姓氏 = "三"
让 全名 = 名字 + 姓氏
打印(全名)  // Output: 张三

// String and number concatenation / 字符串和数字连接
让 年龄 = 25
打印("年龄是: " + 年龄)  // Output: 年龄是: 25
```

### Escape Character Support / 转义字符支持
```cn
打印("换行符测试\n第二行")
打印("制表符测试\t这里")
打印("引号测试\"引号内\"")
打印("反斜杠测试\\")
```

### Comments / 注释
```cn
// This is a single-line comment / 这是单行注释

让 变量 = 42  // End-of-line comment / 行尾注释

/*
Multi-line comments are not yet supported
Only single-line comments are supported
多行注释暂不支持
仅支持单行注释
*/
```

## Installation and Usage / 安装和使用

### Build the Project / 编译项目
```bash
cargo build
```

### Run the Program / 运行程序
```bash
# Run after compilation / 编译后运行
./target/debug/cnlang 程序文件.cn

# Or run directly / 或者直接运行
cargo run -- 程序文件.cn
```

### Example Programs / 示例程序
The project includes several example programs in the project root directory:
项目包含多个示例程序，位于项目根目录：
- `示例.cn` - Comprehensive example showing various features / 综合示例，展示各种功能
- `test_basic.cn` - Basic syntax test / 基础语法测试
- `test_recursion.cn` - Recursive function test / 递归函数测试
- `test_escape_chars.cn` - Escape character test / 转义字符测试

## Project Structure / 项目结构

```
├── src/
│   ├── main.rs          # Program entry point / 程序入口
│   ├── lexer.rs         # Lexical analyzer / 词法分析器
│   ├── parser.rs        # Syntax analyzer / 语法分析器
│   ├── ast.rs           # Abstract syntax tree definition / 抽象语法树定义
│   └── interpreter.rs   # Interpreter / 解释器
├── Cargo.toml           # Project configuration / 项目配置
├── README.md           # Project documentation / 项目说明
└── 示例.cn             # Example program / 示例程序
```

## Technical Architecture / 技术架构

### Compilation Process / 编译流程
1. **Lexical Analysis / 词法分析**：Convert source code to token sequence / 将源代码转换为token序列
2. **Syntax Analysis / 语法分析**：Build abstract syntax tree (AST) from token sequence / 将token序列构建为抽象语法树（AST）
3. **Semantic Analysis / 语义分析**：Verify syntax correctness and type checking / 验证语法正确性和类型检查
4. **Interpretation / 解释执行**：Traverse AST and execute corresponding operations / 遍历AST并执行相应操作

### Core Components / 核心组件

#### Lexer / 词法分析器
- Convert source code strings to token sequences / 负责将源代码字符串转换为token序列
- Support Chinese identifiers and keyword recognition / 支持中文标识符和关键字识别
- Handle escape characters and comments / 处理转义字符和注释

#### Parser / 语法分析器
- Build abstract syntax tree from token sequences / 将token序列构建为抽象语法树
- Support expression, statement, function definition syntax structures / 支持表达式、语句、函数定义等语法结构
- Handle operator precedence and associativity / 处理运算符优先级和结合性

#### Interpreter / 解释器
- Execute abstract syntax tree / 执行抽象语法树
- Manage variable scope and environment / 管理变量作用域和环境
- Handle function calls and recursion / 处理函数调用和递归

## Development Roadmap / 开发计划

### Completed Features / 已完成功能
- ✅ Basic lexical analysis / 基本词法分析
- ✅ Syntax analysis and AST construction / 语法分析和AST构建
- ✅ Variable and scope management / 变量和作用域管理
- ✅ Function definition and calling / 函数定义和调用
- ✅ Recursion support / 递归支持
- ✅ String operations / 字符串操作
- ✅ Escape character handling / 转义字符处理
- ✅ Comment support / 注释支持
- ✅ Error handling / 错误处理

### Planned Features / 计划中功能
- 🔄 Array data type / 数组数据类型
- 🔄 Object and struct support / 对象和结构体支持
- 🔄 Module system / 模块系统
- 🔄 File operations / 文件操作
- 🔄 Network programming support / 网络编程支持
- 🔄 Richer built-in function library / 更丰富的内置函数库
- 🔄 Performance optimization / 性能优化
- 🔄 Code debugging tools / 代码调试工具

## FAQ / 常见问题

### Q: Why choose Chinese keywords? / 为什么选择中文关键字？
A: Chinese keywords make code more readable and lower the programming learning threshold for Chinese users, especially suitable for programming beginners and native Chinese speakers.

中文关键字可以让代码更易读，降低中文用户的编程学习门槛，特别适合编程初学者和中文母语者。

### Q: What data types are supported? / 支持哪些数据类型？
A: Currently supports numbers (integers and floats), strings, boolean values. Planned to add composite data types like arrays, objects in future versions.

目前支持数字（整数和浮点数）、字符串、布尔值。计划在未来版本中增加数组、对象等复合数据类型。

### Q: How to contribute code? / 如何贡献代码？
A: Welcome to submit Issues and Pull Requests. Please ensure code follows project style and passes all tests.

欢迎提交Issue和Pull Request。请确保代码符合项目风格，并通过所有测试。

### Q: How to get help when encountering problems? / 遇到问题如何获取帮助？
A: You can join QQ group 624826263 for technical support and communication.

可以加入QQ群 624826263 获取技术支持和交流。

## License / 许可证

This project is licensed under the MIT License, see the LICENSE file for details.

本项目采用 MIT 许可证，详见 LICENSE 文件。

## Acknowledgments / 致谢

Thanks to all developers who have contributed code and ideas to this project. Special thanks to the Rust language community for providing excellent tools and libraries.

感谢所有为这个项目贡献代码和想法的开发者。特别感谢Rust语言社区提供的优秀工具和库。

---

**Author / 作者：** 灵创-浪漫之夏  
**QQ Group / QQ群：** 624826263  
**Project Repository / 项目地址：** [GitHub Repository]
