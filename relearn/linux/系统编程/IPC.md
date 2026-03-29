- [管道（Pipe）](#管道pipe)
  - [匿名管道（pipe）](#匿名管道pipe)
  - [命名管道（FIFO）](#命名管道fifo)
- [消息队列（Message Queue）](#消息队列message-queue)
- [共享内存（Shared Memory）](#共享内存shared-memory)
- [信号量（Semaphore）](#信号量semaphore)
- [key\_t](#key_t)
- [IPC工具](#ipc工具)

---

# 管道（Pipe）

管道本质是内核缓冲区，以先进先出（FIFO）方式传输字节流

## 匿名管道（pipe）

* 仅用于父子进程、兄弟进程
* 半双工通信
* 字节流传输，无边界
* 生命周期随进程结束
* 内置同步互斥机制，读写阻塞（默认阻塞模式）

```c
#include <unistd.h>
// 创建匿名管道，fd[0]读端，fd[1]写端
int pipe(int pipefd[2]);
```

* 返回值：成功返回 0，失败返回 -1
* 读端关闭，写端进程会收到 SIGPIPE 信号（默认终止进程）
* 写端关闭，读端 read 返回 0

## 命名管道（FIFO）

* 无亲缘关系的进程可通信，以磁盘文件形式存在（文件类型为p）
* 不存储实际数据，仅作为通信入口，真正数据存在内核缓冲区
* 内核缓冲区进程打开时创建，最后一个进程关闭时销毁
* 半双工、字节流、阻塞特性与匿名管道一致
* 仅用于本地进程通信

```c
#include <sys/types.h>
#include <sys/stat.h>
// 创建命名管道，mode为文件权限（如0666）
int mkfifo(const char *pathname, mode_t mode);
```

```c
#include <unistd.h>
// 删除文件（包括命名管道 FIFO）
int unlink(const char *pathname);
```

* 以 O_RDONLY 打开：阻塞，直到有进程以 O_WRONLY 打开
* 以 O_WRONLY 打开：阻塞，直到有进程以 O_RDONLY 打开
* 以 O_RDWR 打开：不会阻塞（不推荐，违背半双工设计）

---

# 消息队列（Message Queue）

消息队列是内核维护的链式消息链表，以消息为单位传输数据，支持按类型读取

* 支持无亲缘关系进程通信
* 生命周期随内核（需手动删除）
* 全双工逻辑
* 消息有边界
* 可按类型筛选读取
* 异步通信（发送方不等待接收方读取）
* 数据存内核，需手动调用msgctl删除或系统重启销毁

```c
#include <sys/msg.h>
// 1. 创建/获取消息队列
int msgget(key_t key, int msgflg);
// 2. 发送消息
int msgsnd(int msqid, const void *msgp, size_t msgsz, int msgflg);
// 3. 接收消息
ssize_t msgrcv(int msqid, void *msgp, size_t msgsz, long msgtyp, int msgflg);
// 4. 控制消息队列（删除）
int msgctl(int msqid, int cmd, struct msqid_ds *buf);
```

```c
struct msgbuf {
    long mtype;       // 消息类型（必须>0，第一个字段）
    char mtext[1024]; // 消息数据
};
```

* key：通过ftok()生成，唯一标识消息队列；
* msgtyp：消息类型，设为 0 读取队列第一条，设为正数读取对应类型消息

---

# 共享内存（Shared Memory）

共享内存是将同一段物理内存映射到多个进程的虚拟地址空间，进程直接读写内存，无需内核数据拷贝，是 Linux 最快的 IPC 方式

* 无亲缘关系进程可通信
* 数据直接读写，效率极高
* 生命周期随内核，需手动删除
* 无内置同步互斥机制，必须搭配信号量 / 互斥锁使用
* 信号量和互斥锁与pthread的是两套api

```c
#include <sys/shm.h>
// 1. 创建/获取共享内存段
int shmget(key_t key, size_t size, int shmflg);
// 2. 映射共享内存到进程虚拟地址空间
void *shmat(int shmid, const void *shmaddr, int shmflg);
// 3. 解除共享内存映射
int shmdt(const void *shmaddr);
// 4. 控制共享内存（删除）
int shmctl(int shmid, int cmd, struct shmid_ds *buf);
```

* size：共享内存大小，建议按页（4KB）对齐
* shmat返回值：映射后的虚拟地址指针，失败返回(void*)-1

---

# 信号量（Semaphore）

信号量不传递数据，核心作用是控制多个进程对临界资源的互斥访问，或实现进程间同步

* 临界资源：同一时间仅允许一个进程访问的资源
* P 操作（wait）：信号量值 - 1，值 <0 时进程阻塞等待
* V 操作（post）：信号量值 + 1，值 ≤0 时唤醒阻塞进程
* 二元信号量：值为 0/1，等价于互斥锁
* 计数信号量：值 > 1，控制多资源访问

```c
#include <sys/sem.h>
// 1. 创建/获取信号量集
int semget(key_t key, int nsems, int semflg);
// 2. 信号量PV操作
int semop(int semid, struct sembuf *sops, unsigned nsops);
// 3. 控制信号量（初始化、删除）
int semctl(int semid, int semnum, int cmd, ...);
```

```c
struct sembuf {
    unsigned short sem_num; // 信号量集中的编号
    short sem_op;           // 操作值：-1=P，+1=V
    short sem_flg;          // 操作标志
};
```

# key_t

所有 System V IPC（消息队列、共享内存、信号量）都用 key_t 类型的键 来标识内核对象

```c
#include <sys/ipc.h>
key_t ftok(const char *pathname, int proj_id);
```

* pathname：必须是已存在、可访问的文件 / 目录（不能是不存在的路径），要用到文件的inode号
* proj_id：8 位整数（0~255），用于区分同一路径下不同 IPC 对象
* 返回值：成功返回key_t类型 key，失败返回-1

---

# IPC工具

```bash
ipcs -q  # 查看消息队列
ipcs -m  # 查看共享内存
ipcs -s  # 查看信号量
# 删除 IPC 资源
ipcrm -q MQID   # 消息队列
ipcrm -m SHMID  # 共享内存
ipcrm -s SEMID  # 信号量
```
---
