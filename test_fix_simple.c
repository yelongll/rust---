// 简化版本，移除标准库依赖
typedef enum {
    VALUE_NUMBER,
    VALUE_STRING,
    VALUE_BOOLEAN,
    VALUE_ARRAY,
    VALUE_NULL
} ValueType;

typedef struct Value {
    ValueType type;
    union {
        int number;
        char* string;
        int boolean;
        struct {
            struct Value* values;
            int count;
            int capacity;
        } array;
    } as;
} Value;

// 简单的测试函数
Value 测试函数(Value arg0, Value arg1) {
    Value result;
    result.type = VALUE_NUMBER;
    result.as.number = arg0.as.number + arg1.as.number;
    return result;
}

int main() {
    // 创建测试值
    Value 数值1 = {VALUE_NUMBER, .as.number = 1};
    Value 数值2 = {VALUE_NUMBER, .as.number = 5};
    
    // 调用测试函数
    Value 结果 = 测试函数(数值1, 数值2);
    
    // 简单的验证
    if (结果.as.number == 6) {
        return 0; // 成功
    } else {
        return 1; // 失败
    }
}