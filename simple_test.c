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

int main() {
    printf("Hello World\n");
    return 0;
}