use std::mem;
use std::sync::{mpsc, Mutex, Arc};
use std::thread;
use future::{Future, Poll};
use future_util::*;

/*
 * Core executor interface.
 */

pub trait Executor {
  fn spawn<F>(&mut self, f: F)
  where
    F: Future<Item = ()> + 'static;
  fn wait(&mut self);
}


/*
 * Example implementation of a naive executor that executes futures
 * in sequence.
 */

pub struct BlockingExecutor;

impl BlockingExecutor {
  pub fn new() -> BlockingExecutor {
    BlockingExecutor
  }
}

impl Executor for BlockingExecutor {
  fn spawn<F>(&mut self, mut f: F)
  where
    F: Future<Item = ()>,
  {
    loop {
      if let Poll::Ready(()) = f.poll() {
        break;
      }
    }
  }

  fn wait(&mut self) {}
}

/*
 * Part 2a - Single threaded executor
 */

pub struct SingleThreadExecutor {
  futures: Vec<Box<Future<Item = ()>>>,
}

impl SingleThreadExecutor {
  pub fn new() -> SingleThreadExecutor {
    SingleThreadExecutor { futures: vec![] }
  }
}

impl Executor for SingleThreadExecutor {
  fn spawn<F>(&mut self, mut f: F)
  where
    F: Future<Item = ()> + 'static,
  {
    match f.poll() {
      Poll::Ready(()) => (),
      Poll::NotReady => self.futures.push(Box::new(f))
    }
  }

  fn wait(&mut self) {
    let mut i = 0;
    while self.futures.len() > 0 {
      i = (i + 1) % self.futures.len();
      match self.futures[i].poll() {
        Poll::NotReady => {
          self.futures.remove(i);
          i -= 1;
        },
        _ => ()
      }
    }
  }
}

pub struct MultiThreadExecutor {
  sender: mpsc::Sender<Option<Box<Future<Item = ()>>>>,
  threads: Vec<thread::JoinHandle<()>>,
}

impl MultiThreadExecutor {
  pub fn new(num_threads: i32) -> MultiThreadExecutor {
    let (sender, receiver): (mpsc::Sender<Option<Box<Future<Item = ()>>>>, 
                             mpsc::Receiver<Option<Box<Future<Item = ()>>>>)
                          = mpsc::channel();
    let safe_rec = Arc::new(Mutex::new(receiver));
    let threads = (0..num_threads).map(|_| {
      let safe_rec_copy = safe_rec.clone();
      thread::spawn(move || {
        let mut executor = SingleThreadExecutor::new();
        loop {
          match safe_rec_copy.lock().unwrap().recv().unwrap() {
            Some(f) => { executor.spawn(f); },
            None => { executor.wait(); break; }
          }
        }
      })
    }).collect::<Vec<_>>();
    MultiThreadExecutor {sender, threads}
  }
}

impl Executor for MultiThreadExecutor {
  fn spawn<F>(&mut self, f: F)
  where
    F: Future<Item = ()> + 'static,
  {
    self.sender.send(Some(Box::new(f))).unwrap();
  }

  fn wait(&mut self) {
    let len = self.threads.len();
    for _ in 0..len {
      self.sender.send(None).unwrap();
    }
    take_mut::take(&mut self.threads, |th| {
      for thread in th {
        thread.join().unwrap();
      }
      vec![]
    });
  }
}
