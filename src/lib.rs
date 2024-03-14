use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

pub struct ThreadPool {
    workers: Vec<Worker>, // it store the every thread individually
    sender: Option<mpsc::Sender<Job>>,// it send the job(what to do ) between individual thread
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    /// Create a new ThreadPool.
    /// The size is the number of threads in the pool.
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();// create a channel to pass data between threads

        let receiver = Arc::new(Mutex::new(receiver));// Arc- create a another referrence for receiver, and that could changable after acquiring lock

        let mut workers = Vec::with_capacity(size);// create a vector of sized thread we want

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));// creates theads and push into workers and pass the channel receiever to every threads, so that couble receive job data
        }

        ThreadPool { workers, sender: Some(sender) }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);  

        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}





impl Drop for ThreadPool {
    fn drop(&mut self) {

        drop(self.sender.take()); // sender in chaneel should be dropper first,,.//TODO: is there problem if the sender of the channel is not doppped, so what will happen

        for worker in &mut self.workers{
            println!("Shutting down worker {}", worker.id);

            if let Some(thread)= worker.thread.take(){
                thread.join().unwrap();
            }
            
        }
    }
}

struct Worker {
    id: usize,// 
    thread:Option< thread::JoinHandle<()>>,
}

// --snip--

impl Worker {// TODO: i didn't understand this new() clearly
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {// taking a reciver function from channel and get the job what to do
        let thread = thread::spawn(move || loop {// TODO: if we try to drop our ThreadPool with our current implementation of drop, the main thread will block forever waiting for the first thread to finish.
            let message= receiver.lock().unwrap().recv();

            match message{
                Ok(job)=>{                                                                                                                                                                                                                  
                    println!("Worker {} got a job; executing",id);
                    job();
                }

                Err(_)=>{// if threadpool are dropped,before then channel sender also get dropped.For that reason, channel send will return a Error in recv(), so we stop the thread.
                    println!("Worker {} disconnected; shutting down",id);
                    break;
                }
            }
        });

        Worker { id, thread: Some(thread) }
    }
}
