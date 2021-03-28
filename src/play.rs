use std::time::{Duration, Instant};

use anyhow::Result;
use reqwest::blocking;

use crate::data::{Request, Response, Scenario};
use crate::pace::{PaceState, Pacer};
use crate::validation::validate;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

#[derive(Debug)]
pub struct Player {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

fn request(client: &blocking::Client, scenario: &Scenario, req: Request) -> Result<()> {
    let spec = scenario.spec();
    let res = Response::from(client.get(req.url()).send()?);
    match validate(spec, res) {
        Ok(_) => {
            println!("[{}]: {} => OK", scenario.name(), scenario.url());
        }
        Err(e) => {
            eprintln!("{}", e.to_string());
        }
    }
    Ok(())
}

impl Player {
    pub fn new(worker: usize) -> Self {
        assert!(worker > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(worker);
        for id in 0..worker {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        Player { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
    }

    pub fn play(&self, pacer: impl Pacer, scenario: Scenario) -> Result<()> {
        let client = blocking::Client::builder()
            .pool_idle_timeout(Duration::from_secs(10))
            .pool_max_idle_per_host(10)
            .build()?;
        let start = Instant::now();
        for (i, req) in scenario.const_iter().enumerate() {
            let client = client.clone();
            let scenario = scenario.clone();
            let job = Box::new(move || {
                request(&client, &scenario, req).unwrap();
            });

            let elapsed = start.elapsed();

            if let PaceState::Wait(dur) = pacer.pace(elapsed, i as u128) {
                thread::sleep(dur);
            }
            self.sender.send(Message::NewJob(job)).unwrap();
        }
        Ok(())
    }
}

impl Drop for Player {
    fn drop(&mut self) {
        for _ in &mut self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

enum Message {
    NewJob(Job),
    Terminate,
}

trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<Self>) {
        (*self)()
    }
}

type Job = Box<dyn FnBox + Send + 'static>;

#[derive(Debug)]
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Self {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();
            match message {
                Message::NewJob(job) => {
                    job.call_box();
                }
                Message::Terminate => {
                    break;
                }
            }
        });
        Worker {
            id,
            thread: Some(thread),
        }
    }
}
