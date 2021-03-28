use std::sync::{
    mpsc::{self, Receiver, Sender},
    Arc, Mutex,
};
use std::thread;
use std::time::{Duration, Instant};

use anyhow::Result;
use log::{error, info};
use reqwest::blocking::Client;

use crate::data::{Request, Response, Scenario, Spec};
use crate::pace::{PaceState, Pacer};
use crate::validation::validate;

#[derive(Debug)]
pub struct Player {
    workers: Vec<Worker>,
    sender: Sender<Message>,
}

impl Player {
    pub fn new(worker: usize) -> Self {
        assert!(worker > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let client = Client::builder()
            .pool_idle_timeout(Duration::from_secs(10))
            .pool_max_idle_per_host(10)
            .build()
            .unwrap();
        let mut workers = Vec::with_capacity(worker);
        for id in 0..worker {
            workers.push(Worker::new(id, client.clone(), Arc::clone(&receiver)));
        }
        Player { workers, sender }
    }

    pub fn play(&self, pacer: Box<dyn Pacer>, scenario: Scenario) -> Result<()> {
        let start = Instant::now();
        for (i, req) in scenario.const_iter().enumerate() {
            let spec = scenario.spec().clone();

            let elapsed = start.elapsed();
            if let PaceState::Wait(dur) = pacer.pace(elapsed, i as u128) {
                thread::sleep(dur);
            }
            self.sender.send(Message::New(req, spec)).unwrap();
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
    New(Request, Spec),
    Terminate,
}

#[derive(Debug)]
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, client: Client, receiver: Arc<Mutex<Receiver<Message>>>) -> Self {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();

            match message {
                Message::New(req, spec) => Worker::request(&client, req, spec),
                Message::Terminate => break,
            }
        });
        Worker {
            id,
            thread: Some(thread),
        }
    }

    fn request(client: &Client, req: Request, spec: Spec) {
        let res = client.get(req.url()).send();
        let res = match res {
            Ok(r) => Response::from(r),
            Err(e) => {
                error!("[{}]: {}", req.scenario_name(), e.to_string());
                return;
            }
        };
        match validate(&spec, res) {
            Ok(_) => {
                info!(
                    "[{}:{}]: {} => OK",
                    req.scenario_name(),
                    req.seq(),
                    req.url()
                );
            }
            Err(e) => {
                error!("[{}:{}]: {}", req.scenario_name(), req.seq(), e.to_string());
            }
        }
    }
}
