#include <stdio.h>

int 测试函数() {
    int 变量 = 42;
    printf("测试: %d\n", 变量);
    return 变量;
}

int main() {
    return 测试函数();
}