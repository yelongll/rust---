#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>

typedef enum {
    VALUE_NUMBER,
    VALUE_STRING,
    VALUE_BOOLEAN,
    VALUE_ARRAY,
    VALUE_NULL
} ValueType;

// 前向声明
typedef struct Value Value;

struct Value {
    ValueType type;
    union {
        double number;
        char* string;
        bool boolean;
        struct {
            struct Value* values;
            int count;
            int capacity;
        } array;
    } as;
};

void builtin_print(Value value);
Value builtin_input();
Value create_array();
void array_push(Value* array, Value value);
Value array_get(Value array, int index);
int array_length(Value array);
void array_remove(Value* array, int index);
bool is_truthy(Value value);
int compare_values(Value a, Value b);
Value add_values(Value a, Value b);
Value subtract_values(Value a, Value b);
Value multiply_values(Value a, Value b);
Value divide_values(Value a, Value b);
Value negate_value(Value value);


// 内置函数实现
void builtin_print(Value value) {
    switch (value.type) {
        case VALUE_NUMBER:
            printf("%f\n", value.as.number);
            break;
        case VALUE_STRING:
            printf("%s\n", value.as.string);
            break;
        case VALUE_BOOLEAN:
            printf("%s\n", value.as.boolean ? "true" : "false");
            break;
        case VALUE_ARRAY:
            printf("[");
            for (int i = 0; i < value.as.array.count; i++) {
                if (i > 0) printf(", ");
                builtin_print(value.as.array.values[i]);
            }
            printf("]\n");
            break;
        case VALUE_NULL:
            printf("null\n");
            break;
    }
}

Value builtin_input() {
    char buffer[1024];
    if (fgets(buffer, sizeof(buffer), stdin)) {
        buffer[strcspn(buffer, "\n")] = '\0';
        return (Value){VALUE_STRING, .as.string = strdup(buffer)};
    }
    return (Value){VALUE_NULL};
}

Value create_array() {
    Value array = {VALUE_ARRAY};
    array.as.array.values = malloc(sizeof(Value) * 8);
    array.as.array.count = 0;
    array.as.array.capacity = 8;
    return array;
}

void array_push(Value* array, Value value) {
    if (array->as.array.count >= array->as.array.capacity) {
        array->as.array.capacity *= 2;
        array->as.array.values = realloc(array->as.array.values, sizeof(Value) * array->as.array.capacity);
    }
    array->as.array.values[array->as.array.count++] = value;
}

Value array_get(Value array, int index) {
    if (index < 0 || index >= array.as.array.count) {
        return (Value){VALUE_NULL};
    }
    return array.as.array.values[index];
}

int array_length(Value array) {
    return array.as.array.count;
}


void array_remove(Value* array, int index) {
    if (index < 0 || index >= array->as.array.count) return;
    for (int i = index; i < array->as.array.count - 1; i++) {
        array->as.array.values[i] = array->as.array.values[i + 1];
    }
    array->as.array.count--;
}

// 辅助函数
bool is_truthy(Value value) {
    switch (value.type) {
        case VALUE_BOOLEAN: return value.as.boolean;
        case VALUE_NUMBER: return value.as.number != 0;
        case VALUE_STRING: return strlen(value.as.string) > 0;
        case VALUE_ARRAY: return value.as.array.count > 0;
        default: return false;
    }
}

int compare_values(Value a, Value b) {
    if (a.type != b.type) return 0;
    switch (a.type) {
        case VALUE_NUMBER: return (a.as.number > b.as.number) - (a.as.number < b.as.number);
        case VALUE_STRING: return strcmp(a.as.string, b.as.string);
        case VALUE_BOOLEAN: return (a.as.boolean > b.as.boolean) - (a.as.boolean < b.as.boolean);
        default: return 0;
    }
}

Value add_values(Value a, Value b) {
    if (a.type == VALUE_NUMBER && b.type == VALUE_NUMBER) {
        return (Value){VALUE_NUMBER, .as.number = a.as.number + b.as.number};
    }
    if (a.type == VALUE_STRING && b.type == VALUE_STRING) {
        char* result = malloc(strlen(a.as.string) + strlen(b.as.string) + 1);
        strcpy(result, a.as.string);
        strcat(result, b.as.string);
        return (Value){VALUE_STRING, .as.string = result};
    }
    return (Value){VALUE_NULL};
}

Value subtract_values(Value a, Value b) {
    if (a.type == VALUE_NUMBER && b.type == VALUE_NUMBER) {
        return (Value){VALUE_NUMBER, .as.number = a.as.number - b.as.number};
    }
    return (Value){VALUE_NULL};
}

Value multiply_values(Value a, Value b) {
    if (a.type == VALUE_NUMBER && b.type == VALUE_NUMBER) {
        return (Value){VALUE_NUMBER, .as.number = a.as.number * b.as.number};
    }
    return (Value){VALUE_NULL};
}

Value divide_values(Value a, Value b) {
    if (a.type == VALUE_NUMBER && b.type == VALUE_NUMBER) {
        if (b.as.number == 0) return (Value){VALUE_NULL};
        return (Value){VALUE_NUMBER, .as.number = a.as.number / b.as.number};
    }
    return (Value){VALUE_NULL};
}

Value negate_value(Value value) {
    if (value.type == VALUE_NUMBER) {
        return (Value){VALUE_NUMBER, .as.number = -value.as.number};
    }
    return (Value){VALUE_NULL};
}

Value 测试函数(Value arg0, Value arg1) {
    return add_values(arg0, arg1);
}

int main() {
    // Program entry point
    Value 数组1 = create_array();
        array_push(&数组1, (Value){VALUE_NUMBER, .as.number = 1});
        array_push(&数组1, (Value){VALUE_NUMBER, .as.number = 2});
        array_push(&数组1, (Value){VALUE_NUMBER, .as.number = 3});
    Value 结果 = 测试函数(array_get(数组1, (int)(Value){VALUE_NUMBER, .as.number = 0}.as.number), (Value){VALUE_NUMBER, .as.number = 5});
    builtin_print(结果);
    return 0;
}
