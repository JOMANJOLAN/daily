- [std::task::Wake](#stdtaskwake)
  - [作用](#作用)
  - [代码](#代码)
- [std::task::Waker](#stdtaskwaker)
  - [作用](#作用-1)
  - [原理](#原理)
  - [依赖](#依赖)
  - [代码](#代码-1)
- [std::task::Context\<'a\>](#stdtaskcontexta)
  - [作用](#作用-2)
  - [依赖](#依赖-1)
- [std::task::Poll](#stdtaskpoll)
- [std::future::Future](#stdfuturefuture)
  - [依赖](#依赖-2)
- [示例](#示例)

---

# std::task::Wake

## 作用

* 在就绪时唤醒任务，一般的实现方法是将任务加入任务队列（这需要实现这个trait的item同时持有future、queue）

## 代码

```rust
pub trait Wake {
    // Required method
    fn wake(self: Arc<Self>);

    // Provided method
    fn wake_by_ref(self: &Arc<Self>) { ... }
}
```

---

# std::task::Waker

## 作用

* 包装实现了std::task::Wake的类型

## 原理

* 构建了唤醒机制所需函数的虚表
* 需要与系统底层交互，不能隐藏任何细节，所以选择手动构建虚表，而不使用dyn自动构建虚表

## 依赖
* [std::task::Wake](#stdtaskwake)

## 代码

```rust
pub struct Waker { /* private fields */ }

impl Waker {
    pub fn wake(self) { ... }
    pub fn wake_by_ref(&self) { ... }
}

impl<W> From<Arc<W>> for Waker
where
    W: Wake + Send + Sync + 'static
{
    fn from(waker: Arc<W>) -> Waker { ... }
}
```

---

# std::task::Context<'a>

## 作用

* 在各个层级的poll间传递Waker

## 依赖
* [std::task::Waker](#stdtaskwaker)


```rust
pub struct Context<'a> { /* private fields */ }

impl<'a> Context<'a> {
    pub const fn from_waker(waker: &'a Waker) -> Context<'a> { ... }
    pub const fn waker(&self) -> &'a Waker { ... }
}
```

---

# std::task::Poll

```rust
pub enum Poll<T> {
    Ready(T),
    Pending,
}
```

---

# std::future::Future

## 依赖
* [std::task::Context<'a>](#stdtaskcontexta)
* [std::task::Poll](#stdtaskpoll)

```rust
pub trait Future {
    type Output;

    // Required method
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}
```

---

# 示例

```rust
use std::collections::VecDeque;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Wake, Waker};

pub struct TaskInner {
    fut: Mutex<Pin<Box<dyn Future<Output = ()> + Send + Sync>>>,
    queue: Arc<Mutex<VecDeque<Task>>>,
}

impl Wake for TaskInner {
    fn wake(self: Arc<Self>) {
        self.queue
            .lock()
            .unwrap()
            .push_back(Task(Arc::clone(&self)));
    }
}

pub struct Task(Arc<TaskInner>);

impl Future for Task {
    type Output = ();

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        let waker = Waker::from(Arc::clone(&self.0));
        let mut cx = Context::from_waker(&waker);
        self.0.fut.lock().unwrap().as_mut().poll(&mut cx)
    }
}

pub struct Executor {
    queue: Arc<Mutex<VecDeque<Task>>>,
}

impl Executor {
    pub fn spawn(&self, fut: Pin<Box<dyn Future<Output = ()> + Send + Sync>>) {
        let task = Task(Arc::new(TaskInner {
            fut: Mutex::new(fut),
            queue: Arc::clone(&self.queue),
        }));
        self.queue.lock().unwrap().push_back(task);
    }

    pub fn run(self) {
        let mut cx = Context::from_waker(Waker::noop());
        loop {
            if let Some(mut task) = self.queue.lock().unwrap().pop_front() {
                let poll = Pin::new(&mut task).poll(&mut cx);
                println!("{:?}", poll);
            }
        }
    }
}
```

---
