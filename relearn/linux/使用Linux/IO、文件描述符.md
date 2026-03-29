- [一切皆文件](#一切皆文件)
- [文件描述符](#文件描述符)
  - [本质](#本质)
  - [格式](#格式)
  - [内核三层结构](#内核三层结构)
- [IO](#io)
  - [阻塞 IO（Blocking IO）](#阻塞-ioblocking-io)
  - [非阻塞 IO（Non-blocking IO）](#非阻塞-ionon-blocking-io)
  - [IO 多路复用（IO Multiplexing）](#io-多路复用io-multiplexing)
  - [信号驱动 IO](#信号驱动-io)
  - [异步 IO（AIO）](#异步-ioaio)
- [命令行](#命令行)
  - [重定向](#重定向)
    - [输出重定向](#输出重定向)
    - [输入重定向](#输入重定向)
  - [管道](#管道)
  - [查看进程的fd](#查看进程的fd)
  - [命令行工具](#命令行工具)
    - [ulimit：查看 / 修改 fd 限制](#ulimit查看--修改-fd-限制)
    - [lsof：列出打开的文件（fd 关联）](#lsof列出打开的文件fd-关联)
    - [strace：跟踪系统调用（看 fd 操作）](#strace跟踪系统调用看-fd-操作)
  - [自定义 fd](#自定义-fd)

---

# 一切皆文件

文件描述符fd（file descriptor）本质上是一个小整数，进程使用文件描述符操作文件

---

# 文件描述符

## 本质

* 是进程级别的整数索引
* 指向内核中维护的 文件描述符表（file descriptor table）
* 每个进程独立拥有自己的fd表，互不干扰

## 格式

* 0：标准输入 stdin
* 1：标准输出 stdout
* 2：标准错误 stderr
* ≥3：用户打开文件 / socket / 管道时动态分配

## 内核三层结构

```
进程 → fd（整数）
文件描述符表（fd table）
文件表项（file struct，含偏移、状态、权限）
inode（真实文件，磁盘/设备唯一标识）
```

* 不同进程的同一个 fd，可以指向不同文件
* 不同 fd，可以指向同一个文件（共享偏移）

---

# IO

**进程通过 fd 与内核交互，读写数据**

## 阻塞 IO（Blocking IO）

* 数据没到，进程阻塞，直到数据就绪
* 最简单，也最不高效

## 非阻塞 IO（Non-blocking IO）

* 没数据直接返回
* 不阻塞，但需要轮询

## IO 多路复用（IO Multiplexing）

* 最核心、最高效的 Linux 网络 / 高并发模型，一次监听多个 fd
* 原理：把一堆 fd 交给内核，内核告诉你哪些 fd 可读 / 可写，你再去处理。

## 信号驱动 IO

* 内核数据就绪时，发信号通知进程

## 异步 IO（AIO）

* 直接返回，内核完成后通知你
* 真正的“不阻塞、不等待”

---

# 命令行

## 重定向

### 输出重定向

```bash
#  覆盖重定向
## 原本通过 fd=1 输出到屏幕的内容，现在输出到 file.txt
ls -l > file.txt  # 等价于 ls -l 1> file.txt

#  追加重定向
## 内容追加到 file.txt 末尾，不覆盖原有内容
echo "新内容" >> file.txt  # 等价于 echo "新内容" 1>> file.txt

#  错误重定向
## 把错误信息（fd=2）输出到 error.log，正常输出（fd=1）仍到屏幕
ls /xxx 2> error.log

#  合并重定向
## 方式1：2>&1（把 fd=2 的输出重定向到 fd=1 当前指向的位置）
ls /xxx > all.log 2>&1  # 正常+错误都写入 all.log
## 不能反着来，2>&1不是把2绑定到1，而是把2指向1

## 方式2：&>（简化写法，效果同上）
ls /xxx &> all.log

## 方式3：追加合并
ls /xxx >> all.log 2>&1
```

### 输入重定向

```bash
# read 命令从 file.txt（fd=0）读取内容，而非键盘
read name < file.txt
echo "从文件读到：$name"

# 更实用的示例：cat 读取文件本质也是 < 重定向
cat file.txt  # 等价于 cat < file.txt
```

## 管道

管道的本质是：把前一个命令的 fd=1，连接到后一个命令的 fd=0

```bash
# 示例：ls -l 的 fd=1 输出，通过管道传给 grep 的 fd=0 输入
ls -l | grep ".txt"


#  与重定向结合
## 把 ls 的错误信息也通过管道传给 grep（先合并 fd=2 到 fd=1，再走管道）
ls /xxx /home 2>&1 | grep "错误"
```

## 查看进程的fd

```bash
#  查看当前shell的fd
## 1. 先查当前 shell 的 pid
echo $$  # 输出当前 bash/zsh 的 pid，比如 12345

## 2. 查看该进程的 fd
ls -l /proc/$$/fd

#  查看其它进程的fd
## 1. 查 nginx pid
ps aux | grep nginx

## 2. 查看 fd
ls -l /proc/[nginx-pid]/fd
```

## 命令行工具

### ulimit：查看 / 修改 fd 限制

```bash
# 查看单个进程的最大 fd 数（默认通常是 1024）
ulimit -n

# 临时修改（当前 shell 有效）
ulimit -n 65535

# 永久修改（需重启）：编辑 /etc/security/limits.conf
echo "* soft nofile 65535" >> /etc/security/limits.conf
echo "* hard nofile 65535" >> /etc/security/limits.conf
```

### lsof：列出打开的文件（fd 关联）

lsof = list open files，是排查 fd 泄露的神器

```bash
# 列出所有进程打开的文件
lsof

# 列出指定进程打开的 fd
lsof -p [pid]

# 列出指定文件被哪个进程打开
lsof /var/log/syslog

# 列出所有网络 fd（socket）
lsof -i
```

### strace：跟踪系统调用（看 fd 操作）

strace 能看到进程调用了哪些 IO 系统调用，以及使用的 fd，会输出与源代码调用的操作 fd 的函数，如 openat、read、close、write

```bash
# 跟踪 ls 命令的系统调用，看它如何操作 fd
strace ls -l
```

---

## 自定义 fd

```bash
# 1. 打开 fd=3 指向 file.txt（只读）
exec 3< file.txt

# 2. 从 fd=3 读取内容
read line <&3
echo "从 fd=3 读到：$line"

# 3. 打开 fd=4 指向 new.txt（只写，追加）
exec 4>> new.txt

# 4. 往 fd=4 写入内容
echo "自定义 fd 写入" >&4

# 5. 关闭自定义 fd
exec 3<&-
exec 4>&-
```
