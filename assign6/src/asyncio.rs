use future::*;
use std::path::PathBuf;
use std::thread;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::fs;
use std::io;

pub struct FileReader {
  path: PathBuf,
  thread: Option<thread::JoinHandle<io::Result<String>>>,
  done_flag: Arc<AtomicBool>,
}

impl FileReader {
  pub fn new(path: PathBuf) -> FileReader {
    let thread = None;
    let done_flag = Arc::new(AtomicBool::new(false));
    FileReader { path: path, thread: thread, done_flag: done_flag }
  }
}

impl Future for FileReader {
  type Item = io::Result<String>;

  fn poll(&mut self) -> Poll<Self::Item> {
    let mut result: Option<io::Result<String>> = None;
    let mut done = false;
    take_mut::take(self, |fr| {
      match fr.thread {
        Some(thread) => {
          if fr.done_flag.load(Ordering::Relaxed) {
            result = Some(thread.join().unwrap());
            done = true;
            FileReader { path: fr.path, thread: None, done_flag: fr.done_flag }
          } else {
            FileReader { path: fr.path, thread: Some(thread), done_flag: fr.done_flag }
          }
        },
        None => {
          let mut done_flag = fr.done_flag.clone();
          let mut path = fr.path.clone();
          let thread = Some(thread::spawn(move || {
            let res = fs::read_to_string(path.as_path());
            done_flag.store(true, Ordering::Relaxed);
            res
          }));
          FileReader { path: fr.path, thread: thread, done_flag: fr.done_flag }
        }
      }
    });
    if done {
      Poll::Ready(result.unwrap())
    } else {
      Poll::NotReady
    }
  }
}
