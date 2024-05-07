use std::collections::VecDeque;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};
use futures::task;

fn main() {
    /// A MiniTokio instance is created, then an asynchronous task is launched using spawn.
    let mut mini_tokio = MiniTokio::new();

    mini_tokio.spawn(async {
        let when = Instant::now() + Duration::from_millis(10);
        let future = Delay { when };

        let out = future.await;
        assert_eq!(out, "done");
    });

    // The run method is then called, which executes all tasks in MiniTokio, including the asynchronous delay wait task
    mini_tokio.run();
}

struct MiniTokio {
    tasks: VecDeque<Task>,
}

type Task = Pin<Box<dyn Future<Output = ()> + Send>>;

impl MiniTokio {
    // A MiniTokio structure is created, which is an instance of "mini-Tokio"
    // It contains a queue of tasks that need to be completed
    fn new() -> MiniTokio {
        MiniTokio {
            tasks: VecDeque::new(),
        }
    }

    /// Spawn a future onto the mini-tokio instance.
    /// Here F is a future task that needs to be completed. 
    /// It must be Send and 'static in order to be passed between threads and have a static lifetime
    fn spawn<F>(&mut self, future: F)
    where
        F: Future<Output = ()> + Send + 'static,
    {
        self.tasks.push_back(Box::pin(future));
    }

    /// The run method executes tasks from the tasks queue. 
    fn run(&mut self) {
        /// It creates a virtual Context using noop_waker, which is an "alarm clock" that does nothing
        let waker = task::noop_waker();
        let mut cx = Context::from_waker(&waker);

        /// It then cycles through tasks from the queue and calls poll on each task.
        while let Some(mut task) = self.tasks.pop_front() {
            /// If the task is completed (is_pending() returns Poll::Ready), it is simply ignored. 
            /// Otherwise, the task is placed back at the end of the queue to be executed later.
            if task.as_mut().poll(&mut cx).is_pending() {
                self.tasks.push_back(task);
            }
        }
    }
}

struct Delay {
    when: Instant,
}

impl Future for Delay {
    type Output = &'static str;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>)
        -> Poll<&'static str>
    {
        if Instant::now() >= self.when {
            println!("Hello world");
            Poll::Ready("done")
        } else {
            // Ignore this line for now.
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}