use std::{future::Future, pin::Pin, sync::{Arc, Mutex, mpsc::{Receiver, SyncSender, sync_channel}}, task::{Context, Poll}};

use futures::{FutureExt, future::BoxFuture, task::ArcWake, task::waker_ref};

struct Hello {
    state: StateHello
}

enum StateHello {
    HELLO,
    WORLD,
    END
}

impl Hello {
    fn new() -> Self {
        Hello { state: StateHello::HELLO }
    }
}

impl Future for Hello {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        match (*self).state {
            StateHello::HELLO => {
                println!("Hello, ");
                (*self).state = StateHello::WORLD;
                cx.waker().wake_by_ref();
                return Poll::Pending
            }
            StateHello::WORLD => {
                println!("World!");
                (*self).state = StateHello::END;
                cx.waker().wake_by_ref();
                return Poll::Pending
            },
            StateHello::END => return Poll::Ready(()),
        }
    }
}

struct Task {
    future: Mutex<BoxFuture<'static, ()>>,
    sender: SyncSender<Arc<Task>>,
}

impl ArcWake for Task {
    fn wake_by_ref(arc_self: &std::sync::Arc<Self>) {
        let self0 = arc_self.clone();
        arc_self.sender.send(self0).unwrap();
    }
}

struct Executor {
    sender: SyncSender<Arc<Task>>,
    receiver: Receiver<Arc<Task>>,
}

impl Executor {
    fn new() -> Self {
        let (sender, receiver) = sync_channel(1024);
        Executor {
            sender: sender.clone(), 
            receiver
        }
    }

    fn get_spawner(&self) -> Spawner {
        Spawner {
            sender: self.sender.clone(),
        }
    }

    fn run(&self) {
        while let Ok(task) = self.receiver.recv() {
            let mut future = task.future.lock().unwrap();
            let waker = waker_ref(&task);
            let mut ctx = Context::from_waker(&waker);
            let _ = future.as_mut().poll(&mut ctx);
        }
    }

}

struct Spawner {
    sender: SyncSender<Arc<Task>>,
}

impl Spawner {
    fn spawn(&self, future: impl Future<Output = ()> + 'static + Send) {
        let future = future.boxed();
        let task = Arc::new(Task {
            future: Mutex::new(future),
            sender: self.sender.clone()
        });
        self.sender.send(task).unwrap();
    }
}

fn main() {
    let executer = Executor::new();
    executer.get_spawner().spawn(Hello::new());
    executer.run();
}
