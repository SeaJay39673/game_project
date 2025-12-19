use std::{future::Future, sync::Arc};
use tokio::{sync::Mutex, task::JoinSet};
use tokio_util::sync::CancellationToken;

pub struct ThreadManager {
    cancel: CancellationToken,
    tasks: Mutex<JoinSet<()>>,
    children: Mutex<Vec<Arc<ThreadManager>>>,
}

impl ThreadManager {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            cancel: CancellationToken::new(),
            tasks: Mutex::new(JoinSet::new()),
            children: Mutex::new(Vec::new()),
        })
    }

    pub async fn await_cancel(self: &Arc<Self>) {
        self.cancel.cancelled().await;
    }

    pub fn is_cancelled(self: &Arc<Self>) -> bool {
        self.cancel.is_cancelled()
    }

    pub async fn child(self: &Arc<Self>) -> Arc<Self> {
        let child = Arc::new(Self {
            cancel: self.cancel.child_token(),
            tasks: Mutex::new(JoinSet::new()),
            children: Mutex::new(Vec::new()),
        });

        self.children.lock().await.push(child.clone());
        child
    }

    pub async fn spawn<F, Fut>(&self, f: F)
    where
        F: FnOnce() -> Fut + Send + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        let cancel = self.cancel.clone();

        self.tasks.lock().await.spawn(async move {
            tokio::select! {
                _ = cancel.cancelled() => {}
                _ = f() => {}
            }
        });
    }

    pub async fn spawn_loop<F, Fut>(&self, f: F)
    where
        F: Fn() -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send,
    {
        let cancel = self.cancel.clone();
        self.tasks.lock().await.spawn(async move {
            loop {
                tokio::select! {
                    _ = cancel.cancelled() => break,
                    _ = f() => {},
                }
            }
        });
    }

    pub async fn shutdown(self: &Arc<Self>) {
        self.cancel.cancel();

        let mut stack = vec![self.clone()];

        while let Some(tm) = stack.pop() {
            let children = {
                let mut lock = tm.children.lock().await;
                std::mem::take(&mut *lock)
            };

            stack.extend(children);

            tm.cancel.cancel();
        }

        let mut stack = vec![self.clone()];

        while let Some(tm) = stack.pop() {
            let children = tm.children.lock().await.clone();
            stack.extend(children);

            let mut tasks = tm.tasks.lock().await;
            while let Some(_) = tasks.join_next().await {}
        }
    }

    pub async fn abort_async(self: &Arc<Self>) {
        let mut stack = vec![self.clone()];

        while let Some(tm) = stack.pop() {
            tm.cancel.cancel();

            let children = {
                let mut lock = tm.children.lock().await;
                std::mem::take(&mut *lock)
            };

            stack.extend(children);

            let mut tasks = tm.tasks.lock().await;
            tasks.abort_all();

            while let Some(_) = tasks.join_next().await {}
        }
    }
}