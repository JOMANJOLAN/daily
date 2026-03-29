- [指针、内存布局](#指针内存布局)
  - [Linux进程内存布局](#linux进程内存布局)
    - [栈（Stack）](#栈stack)
    - [堆（Heap）](#堆heap)
    - [数据段（Data）](#数据段data)
    - [代码段（Text）](#代码段text)
    - [内核空间](#内核空间)
  - [内存操作工具](#内存操作工具)
- [结构体、union、位段、位运算](#结构体union位段位运算)
  - [结构体（struct）](#结构体struct)
  - [共用体（union）](#共用体union)
  - [位段（位域）](#位段位域)
- [函数指针](#函数指针)
  - [基本语法](#基本语法)
  - [典型应用](#典型应用)
    - [回调函数（如信号处理）](#回调函数如信号处理)
    - [函数指针数组（内核驱动的操作集）](#函数指针数组内核驱动的操作集)
- [宏、条件编译](#宏条件编译)
  - [Linux 内核常用宏技巧](#linux-内核常用宏技巧)
  - [条件编译](#条件编译)
  - [编译时传宏](#编译时传宏)
- [编译、链接、静态库 / 动态库](#编译链接静态库--动态库)
  - [编译链接过程](#编译链接过程)
    - [预处理（-E）：展开宏、处理头文件、条件编译](#预处理-e展开宏处理头文件条件编译)
    - [编译（-S）：将预处理后的代码转为汇编代码](#编译-s将预处理后的代码转为汇编代码)
    - [汇编（-c）：将汇编代码转为二进制目标文件（.o）](#汇编-c将汇编代码转为二进制目标文件o)
    - [链接：将目标文件 + 库文件链接为可执行文件](#链接将目标文件--库文件链接为可执行文件)
  - [静态库（.a）](#静态库a)
    - [创建静态库](#创建静态库)
    - [使用静态库](#使用静态库)
  - [动态库（.so）](#动态库so)
    - [创建动态库](#创建动态库)
    - [使用动态库](#使用动态库)

---

# 指针、内存布局

## Linux进程内存布局

Linux 下 C 程序运行时的虚拟内存空间（32 位系统）分为 5 个区域：

### 栈（Stack）
* 存储局部变量、函数参数、返回值
* 自动分配 / 释放，向下增长

### 堆（Heap）
* 动态内存分配（malloc/calloc）
* 手动管理，向上增长

### 数据段（Data）
* 全局 / 静态变量、常量
* 分为只读（.rodata）和读写

### 代码段（Text）
* 程序执行的机器指令
* 只读，防止指令被篡改

### 内核空间
* 内核代码 / 数据
* 用户态不可直接访问

## 内存操作工具

```bash
size file           # 查看代码段 / 数据段 / 栈的大小；
valgrind ./program  # 检测内存泄漏、野指针等问题；
gdb                 # 调试指针相关的崩溃问题。
```

---

# 结构体、union、位段、位运算

## 结构体（struct）

* 内存对齐：默认按成员最大类型对齐，可通过__attribute__((packed))取消对齐（内核中用于硬件寄存器映射）
* 结构体嵌套：如task_struct中嵌套多个子结构体描述进程属性

## 共用体（union）

所有成员共享同一块内存空间，大小等于最大成员的大小，Linux中常用于：

* 节省内存
* 类型转换
* 硬件寄存器的位操作
* 内核中用 union 处理不同类型的地址映射

## 位段（位域）

把一个字节 / 整数拆分成多个位段

* 操作硬件寄存器
* 网络协议包（如 TCP 头）
* 需要精细控制位的场景
* Linux 驱动开发高频使用

---

# 函数指针

函数指针是指向函数的指针，存储函数的入口地址

* 实现回调
* 动态函数调用
* 框架扩展的核心

## 基本语法

```c
int (*func_ptr)(int, int);
```

## 典型应用

### 回调函数（如信号处理）

```c
void sig_handler(int signum) { ... }

signal(SIGINT, sig_handler);
```

### 函数指针数组（内核驱动的操作集）

```c
typedef struct {
    void (*open)(const char*);
    void (*read)(const char*);
    void (*write)(const char*, const char*);
} FileOps;
```

---

# 宏、条件编译

## Linux 内核常用宏技巧

```c
// 字符串化：# 将参数转为字符串
#define STR(x) #x
// 连接符：## 拼接两个标识符
#define CONCAT(a,b) a##b
```

## 条件编译

* 适配不同操作系统（如 Linux/Windows）
* 调试 / 发布版本切换
* 架构适配（x86/ARM）

```c
#define LINUX 1
#define DEBUG 1

#if LINUX
#else
#endif

#ifdef DEBUG
#else
#endif

#undef DEBUG

#ifndef DEBUG
#else
#endif
```

## 编译时传宏

```bash
# 编译时定义宏（无需在代码中#define）
gcc -o test test.c -DLINUX=1 -DDEBUG=0
```

---

# 编译、链接、静态库 / 动态库

* Linux 下 C 程序的完整构建流程是：预处理，编译，汇编，链接
* 静态库 / 动态库是代码复用的核心方式

## 编译链接过程

### 预处理（-E）：展开宏、处理头文件、条件编译

```bash
gcc -E test.c -o test.i
```

### 编译（-S）：将预处理后的代码转为汇编代码

```bash
gcc -S test.i -o test.s
```

### 汇编（-c）：将汇编代码转为二进制目标文件（.o）

```bash
gcc -c test.s -o test.o
```

### 链接：将目标文件 + 库文件链接为可执行文件
```bash
gcc test.o -o test
```

## 静态库（.a）

* 库代码被复制到可执行文件中，可执行文件独立运行
* 体积大，更新库需重新编译
* 运行速度略快

### 创建静态库

```bash
# 1. 编写库代码（math.c）
cat > math.c << EOF
int add(int a, int b) { return a + b; }
int sub(int a, int b) { return a - b; }
EOF

# 2. 编译为目标文件
gcc -c math.c -o math.o

# 3. 打包为静态库（ar：归档工具）
ar rcs libmath.a math.o
```

### 使用静态库

```bash
# 编写测试代码（main.c）
cat > main.c << EOF
#include <stdio.h>
int add(int a, int b);
int main() {
    printf("3+5=%d\n", add(3,5));
    return 0;
}
EOF

# 编译（-L指定库路径，-l指定库名（去掉lib和.a））
gcc main.c -L. -lmath -o main

# 运行
./main # 输出3+5=8
```

## 动态库（.so）

### 创建动态库

* 库代码不复制到可执行文件，运行时加载，体积小
* 多个程序共享一个库文件，节省内存
* 更新库无需重新编译程序（接口不变时）
* 运行依赖库文件，缺失会导致程序无法启动

```bash
# 1. 编译为位置无关目标文件（-fPIC）
gcc -c -fPIC math.c -o math.o

# 2. 生成动态库（-shared）
gcc -shared math.o -o libmath.so
```

### 使用动态库

```bash
# 编译（与静态库语法相同）
gcc main.c -L. -lmath -o main

# 运行（需指定动态库路径，否则报错"找不到libmath.so"）
# 方式1：临时指定
LD_LIBRARY_PATH=. ./main

# 方式2：永久添加（将库路径加入/etc/ld.so.conf，执行ldconfig）
sudo cp libmath.so /usr/lib
./main
```

---
