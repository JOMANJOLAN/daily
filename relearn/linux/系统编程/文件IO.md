- [一切皆文件](#一切皆文件)
- [系统调用级 IO 特点：](#系统调用级-io-特点)
- [open - 打开 / 创建文件](#open---打开--创建文件)
  - [参数](#参数)
- [read - 从文件读取数据](#read---从文件读取数据)
  - [参数](#参数-1)
- [write - 向文件写入数据](#write---向文件写入数据)
  - [参数 / 返回值](#参数--返回值)
- [close - 关闭文件描述符](#close---关闭文件描述符)
  - [注意事项](#注意事项)
- [lseek - 调整文件偏移量](#lseek---调整文件偏移量)
  - [参数 / 返回值](#参数--返回值-1)
- [stat - 获取文件属性](#stat---获取文件属性)
- [fcntl - 文件控制](#fcntl---文件控制)
  - [获取 / 设置文件状态标志（最常用）](#获取--设置文件状态标志最常用)
    - [F\_GETFL](#f_getfl)
    - [F\_SETFL](#f_setfl)
  - [复制文件描述符](#复制文件描述符)
    - [F\_DUPFD](#f_dupfd)
    - [F\_DUPFD\_CLOEXEC](#f_dupfd_cloexec)
  - [文件锁（记录锁）](#文件锁记录锁)
    - [F\_GETLK](#f_getlk)
    - [F\_SETLK](#f_setlk)
    - [F\_SETLKW](#f_setlkw)
  - [其他辅助命令（少用）](#其他辅助命令少用)
    - [F\_GETFD](#f_getfd)
    - [F\_SETFD](#f_setfd)
    - [F\_GETOWN](#f_getown)
    - [F\_SETOWN](#f_setown)

---

# 一切皆文件

* Linux 中一切皆文件
* 普通文件、设备、管道、套接字等都可通过文件 IO 接口操作

---

# 系统调用级 IO 特点：

* 无缓冲：直接和内核交互，数据不经过用户态缓冲区
* 基于文件描述符（fd）
  非负整数（0 = 标准输入，1 = 标准输出，2 = 标准错误），是内核标识文件的索引
* 原子性：系统调用由内核一次性完成，不会被中断

---

# open - 打开 / 创建文件

打开已存在的文件，或创建新文件，并返回文件描述符（fd），后续 IO 操作通过 fd 关联文件

```c
#include <sys/types.h>
#include <sys/stat.h>
#include <fcntl.h>

int open(const char *pathname, int flags);
int open(const char *pathname, int flags, mode_t mode); // 创建文件时用，mode是权限

// 真正的原型
int open(const char *pathname, int flags, ...); // 可变参数
```

## 参数

* pathname：文件路径
* flags（核心）：O_RDONLY、O_WRONLY、O_RDWR
* flags（扩展）：
  * O_CREAT：文件不存在则创建（需配合 mode 参数）
  * O_EXCL：与 O_CREAT 配合，若文件已存在则 open 失败（避免竞态）
  * O_TRUNC：打开文件时清空原有内容（只写 / 读写模式下有效）
  * O_APPEND：写操作时始终追加到文件末尾
* mode：权限参数，创建文件时的权限（如 0644），最终权限 = mode & ~umask

---

# read - 从文件读取数据

从指定文件描述符的当前偏移量读取数据到用户缓冲区，读取后偏移量自动后移

```c
#include <unistd.h>

ssize_t read(int fd, void *buf, size_t count);
```

## 参数

* fd：文件描述符
* buf：用户态缓冲区（存储读取的数据）
* count：期望读取的字节数
* 返回值：成功返回实际读取的字节数：0 表示读到文件末尾；-1 表示失败（errno 置位）

---

# write - 向文件写入数据

将用户缓冲区的数据写入指定文件描述符的当前偏移量，写入后偏移量自动后移

```c
#include <unistd.h>

ssize_t write(int fd, const void *buf, size_t count);
```

## 参数 / 返回值

* fd：文件描述符
* buf：待写入的数据缓冲区
* count：要写入的字节数
* 返回值：成功返回实际写入的字节数；-1 表示失败

---

# close - 关闭文件描述符

释放文件描述符与文件的关联，归还文件描述符资源（内核会刷新缓冲区、更新文件元信息）

```c
#include <unistd.h>

int close(int fd);
```

## 注意事项

* 返回值：0 成功，-1 失败（如 fd 已关闭）
* 必须调用：进程退出时内核会自动关闭未关闭的 fd，但显式关闭是良好编程习惯，避免资源泄漏（尤其长运行进程）
* 关闭后 fd 失效，不可再用于 read/write 等操作

---

# lseek - 调整文件偏移量

修改文件描述符对应的文件偏移量（决定下一次 read/write 的位置），类似 “移动文件指针”

```c
#include <sys/types.h>
#include <unistd.h>

off_t lseek(int fd, off_t offset, int whence);
```

## 参数 / 返回值

* offset：偏移量（可正可负）
* whence：偏移基准：
  * SEEK_SET：从文件开头（0 位置）偏移 offset 字节
  * SEEK_CUR：从当前偏移量偏移 offset 字节
  * SEEK_END：从文件末尾偏移 offset 字节
* 返回值：成功返回新的偏移量；-1 失败

---

# stat - 获取文件属性

获取文件的元信息（大小、权限、创建时间、类型等）

* 分为 stat（路径），会穿透软连接
* fstat（fd）
* lstat（不跟随软链接）

```c
#include <sys/types.h>
#include <sys/stat.h>
#include <unistd.h>

int stat(const char *pathname, struct stat *statbuf);
int fstat(int fd, struct stat *statbuf); // 通过fd获取
int lstat(const char *pathname, struct stat *statbuf); // 不解析软链接
```

```c
struct stat {
    dev_t     st_dev;     // 设备ID
    ino_t     st_ino;     // inode号
    mode_t    st_mode;    // 文件类型+权限
    nlink_t   st_nlink;   // 硬链接数
    uid_t     st_uid;     // 所有者UID
    gid_t     st_gid;     // 所有者GID
    off_t     st_size;    // 文件大小（字节）
    time_t    st_atime;   // 最后访问时间
    time_t    st_mtime;   // 最后修改时间
    time_t    st_ctime;   // 最后属性修改时间
};
```

---

# fcntl - 文件控制

对已打开的文件描述符进行 “控制操作”，是文件 IO 的 “万能接口”，支持：
* 修改 fd 属性
* 复制 fd
* 获取 / 设置文件状态标志等

```c
#include <unistd.h>
#include <fcntl.h>

int fcntl(int fd, int cmd, ... /* arg */ );
```

## 获取 / 设置文件状态标志（最常用）

### F_GETFL

* 获取文件状态标志
* 成功返回文件标志（int）；失败 -1

```c
int fcntl(int fd, int cmd);
```

### F_SETFL

* 设置文件状态标志
* 成功返回 0；失败 -1

```c
int fcntl(int fd, int cmd, int flags);
```

* flags：指 open 时指定的 O_APPEND、O_NONBLOCK（非阻塞）、O_SYNC 等（不含 O_RDONLY/O_WRONLY/O_RDWR）；

---

## 复制文件描述符

### F_DUPFD

* 复制 fd，返回≥arg 的最小可用 fd
* 成功返回新 fd；失败 - 1

```c
int fcntl(int fd, int cmd, int min_fd);
```

### F_DUPFD_CLOEXEC

* 复制 fd 并设置 CLOEXEC 标志
* 成功返回新 fd；失败 - 1

```c
int fcntl(int fd, int cmd, int min_fd);
```

---

## 文件锁（记录锁）

```c
struct flock {
    short l_type;    // 锁类型：F_RDLCK（读锁）、F_WRLCK（写锁）、F_UNLCK（解锁）
    short l_whence;  // 偏移基准：SEEK_SET/SEEK_CUR/SEEK_END
    off_t l_start;   // 锁起始偏移
    off_t l_len;     // 锁长度（0表示到文件末尾）
    pid_t l_pid;     // 持有锁的进程ID（F_GETLK时返回）
};
```

* 实现 “细粒度” 的文件锁（可以锁定文件的某一部分，而非整个文件）
* 描述锁的范围、类型、进程 ID 等

### F_GETLK

* 检查锁是否冲突
* 成功返回 0；失败 -1

```c
int fcntl(int fd, int cmd, struct flock* l);
```

### F_SETLK

* 设置 / 释放锁（非阻塞）
* 成功返回 0；失败 - 1

```c
int fcntl(int fd, int cmd, struct flock* l);
```

### F_SETLKW

* 设置 / 释放锁（阻塞，W=wait）
* 成功返回 0；失败 - 1

```c
int fcntl(int fd, int cmd, struct flock* l);
```

---

## 其他辅助命令（少用）

### F_GETFD

* 获取文件描述符标志（如 FD_CLOEXEC）

```c
int fcntl(int fd, int cmd);
```

### F_SETFD

* 设置文件描述符标志

```c
int fcntl(int fd, int cmd, int fd);
```

### F_GETOWN

* 获取接收信号（SIGIO/SIGURG）的进程 / 组 ID

```c
int fcntl(int fd, int cmd);
```

### F_SETOWN

* 设置接收信号的进程 / 组 ID

```c
int fcntl(int fd, int cmd, int id);
```

---
