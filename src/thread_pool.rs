use std::thread::{self, JoinHandle};
use std::sync::mpsc::{Sender, Receiver, channel};
use std::sync::{Arc, Mutex};

trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>) {
        (*self)()
    }
}

type Job = Box<dyn FnBox + Send + 'static>;

enum Message {
    NewJob(Job),
    Terminate,
}

struct Worker(u8, Option<JoinHandle<()>>);

impl Worker {
    pub fn new(id: u8, rx: Arc<Mutex<Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                let job = rx.lock().unwrap().recv().unwrap();


                match job {
                    Message::NewJob(j) => {
                        println!("Worker {} got a job; executing.", id);
                        j.call_box();
                    },
                    Message::Terminate => {
                        println!("Worker {} told to terminate.", id);
                        break;
                    },
                }

            }
        });
        Worker(
            id,
            Some(thread)
        )
    }
}


pub struct ThreadPool {
    tx: Sender<Message>,
    workers: Vec<Worker>,
}

impl ThreadPool {
    pub fn new(count: u8) -> ThreadPool {
        assert!(count > 0);

        let (tx, rx) = channel();
        let receiver = Arc::new(Mutex::new(rx));

        let mut workers = Vec::with_capacity(count as usize);
        for id in 0..count {
            workers.push(Worker::new(id, receiver.clone()));
        }

        ThreadPool {
            tx: tx.clone(),
            workers,
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static {
        let job = Box::new(f);

        self.tx.send(Message::NewJob(job)).unwrap();
    }

}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate signal to all workers.");
        for _ in &mut self.workers {
            self.tx.send(Message::Terminate).unwrap();
        }

        println!("Shutting down all workers.");

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.0);


            if let Some(thread) = worker.1.take() {
                thread.join().unwrap();
            }
        }
    }
}
