/*
//Assignment 3: Thread Pool Implementation
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

// Message to be sent to the workers
enum Message {
    NewJob(Job),
    Terminate,
}

// Job type is a boxed closure that can be sent across threads
type Job = Box<dyn FnOnce() + Send + 'static>;

// ThreadPool struct
struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

impl ThreadPool {
    // Create a new ThreadPool with the specified size
    fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        
        // TODO: Create a channel for sending jobs
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        
        // TODO: Create and store workers
        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        
        // TODO: Return the ThreadPool
        ThreadPool { workers, sender }

    }
    
    // Execute a job in the thread pool
    fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        // TODO: Create a job from the closure and send it to a worker
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

// Clean up resources when ThreadPool is dropped
impl Drop for ThreadPool {
    fn drop(&mut self) {
        // TODO: Send terminate message to all workers
        println!("Sending terminate signal to all workers...");

        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }
        
        // TODO: Wait for all workers to finish
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }

    }
}

// Worker struct represents a thread that can process jobs
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    // Create a new worker with the specified ID
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        // TODO: Create a thread that loops and receives jobs from the channel
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();

            match message {
                Message::NewJob(job) => {
                    println!("Worker {} got a job; executing.", id);
                    job();
                }
                Message::Terminate => {
                    println!("Worker {} was told to terminate.", id);
                    break;
                }
            }
        });

        
        // TODO: Return the Worker
        Worker {
            id,
            thread: Some(thread),
        }

    }
}

fn main() {
    // Create a new thread pool with 4 workers
    let pool = ThreadPool::new(4);
    
    // Submit 10 tasks to the pool
    for i in 1..=10 {
        pool.execute(move || {
            println!("Processing task {}", i);
            thread::sleep(std::time::Duration::from_millis(500));
            println!("Completed task {}", i);
        });
    }
    
    println!("Main thread waiting for tasks to complete...");
    // ThreadPool will be dropped when it goes out of scope, triggering the cleanup
}
*/


/*
//Assignment 4: Producer-Consumer Pattern with Termination Signal
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;
use rand::Rng;

// Define a special value that will signal termination
const TERMINATION_SIGNAL: i32 = -1;

fn main() {
    // Number of items to produce
    const ITEM_COUNT: usize = 20;

    // TODO: Create a channel for sending numbers
    let (chan0, chan1) = mpsc::channel();
    let chan1 = Arc::new(Mutex::new(chan1));

    let mut handles = vec![];

    // TODO: Create 2 producer threads
    let num_producers = 2;
    let items_per_producer = ITEM_COUNT / num_producers;

    for i in 0..num_producers {
        let chan0_clone = chan0.clone();
        handles.push(thread::spawn(move || {
            producer(i, chan0_clone, items_per_producer);
        }));
    }

    // TODO: Create 3 consumer threads
    let num_consumers = 3;
    for i in 0..num_consumers {
        let chan1_clone = Arc::clone(&chan1);
        handles.push(thread::spawn(move || {
            consumer(i, chan1_clone);
        }));
    }

    // TODO: Wait for all threads to finish
    // First, wait for all producers to finish
    for handle in handles.drain(..num_producers) {
        handle.join().unwrap();
    }

    // After producing the required number of items,
    // send a special termination value to signal consumers to exit
    for _ in 0..num_consumers {
        chan0.send(TERMINATION_SIGNAL).unwrap();
    }

    // Then, wait for all consumers to finish
    for handle in handles {
        handle.join().unwrap();
    }

    println!("All items have been produced and consumed!");
}

// TODO: Implement producer function
fn producer(id: usize, chan0: mpsc::Sender<i32>, item_count: usize) {
    // TODO: Generate random numbers and send them to the channel
    // When finished, producer should NOT send termination signal
    let mut rng = rand::thread_rng();
    for _ in 0..item_count {
        let num = rng.gen_range(1..=100);
        println!("Producer {}: produced {}", id, num);
        chan0.send(num).unwrap();
        thread::sleep(Duration::from_millis(100));
    }
    println!("Producer {} finished producing.", id);
}

// TODO: Implement consumer function
fn consumer(id: usize, chan1: Arc<Mutex<mpsc::Receiver<i32>>>) {
    // TODO: Receive numbers from the channel and process them
    // Break the loop when receiving the termination signal
    loop {
        let received = chan1.lock().unwrap().recv().unwrap();
        if received == TERMINATION_SIGNAL {
            println!("Consumer {} received termination signal.", id);
            break;
        } else {
            println!("Consumer {}: consumed {}", id, received);
            thread::sleep(Duration::from_millis(150));
        }
    }
    println!("Consumer {} exiting.", id);
}
*/