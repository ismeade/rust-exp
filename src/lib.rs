use std::sync::{Arc, mpsc, Mutex};
use std::thread;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        // Arc 允许多个 Worker 同时持有 receiver，而 Mutex 可以确保一次只有一个 Worker 能从 receiver 接收消息。
        let receiver = Arc::new(Mutex::new(receiver));

        // 使用 Vec<thread::JoinHandle<()>> 来存储线程，同时设定了容量上限 with_capacity(size)，
        // 该方法还可以提前分配好内存空间，比 Vec::new 的性能要更好一点
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            // create some threads and store them in the vector
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }
    // --snip--
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.send(job).unwrap();
    }
}

// 创建一个 Worker 结构体，作为 ThreadPool 和任务线程联系的桥梁，
// 它的任务是获得将要执行的代码，然后在具体的线程中去执行。
struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();

            println!("Worker {id} got a job; executing.");

            job();
        });

        Worker { id, thread }
    }
}
