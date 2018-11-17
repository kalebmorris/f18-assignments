extern crate rand;

use session::*;
use std::collections::{HashSet, HashMap};

pub struct Syn;
pub struct SynAck;
pub struct Ack;
pub struct Fin;

pub type TCPHandshake<TCPRecv> = Recv<Syn, Send<SynAck, Recv<Ack, TCPRecv>>>;

pub type TCPRecv<TCPClose> = Rec<Recv<Vec<Packet>, Send<Vec<usize>, Offer<Var<Z>, TCPClose>>>>;

pub type TCPClose = Send<Ack, Send<Fin, Recv<Ack, Close>>>;

pub type TCPServer = TCPHandshake<TCPRecv<TCPClose>>;

pub type TCPClient = <TCPServer as HasDual>::Dual;

pub fn tcp_server(c: Chan<(), TCPServer>) -> Vec<Buffer> {
  let mut packets: Vec<Packet> = vec![];
  let (c, _) = c.recv();
  let c = c.send(SynAck);
  let (c, _) = c.recv();
  let mut c = c.rec_push();
  loop {
    c = {
      let (c, mut p) = c.recv();
      let mut received: Vec<usize> = vec![];
      for packet in &p {
        received.push(packet.seqno);
      }
      packets.append(&mut p);
      let c = c.send(received);
      match c.offer() {
        Branch::Left(c) => c.rec_pop(),
        Branch::Right(c) => {
          let c = c.send(Ack);
          let c = c.send(Fin);
          let (c, _) = c.recv();
          c.close();
          break;
        }
      }
    }
  }
  packets.sort_by(|a, b| { a.seqno.cmp(&b.seqno) });
  let mut buffers: Vec<Buffer> = vec![];
  for packet in packets {
    buffers.push(packet.buf);
  }
  buffers
}

pub fn tcp_client(c: Chan<(), TCPClient>, bufs: Vec<Buffer>) {
  let c = c.send(Syn);
  let (c, _) = c.recv();
  let c = c.send(Ack);
  let mut c = c.rec_push();
  let mut received_set = HashSet::new();
  loop {
    c = {
      let mut packets: Vec<Packet> = vec![];
      for i in 0..bufs.len() {
        if !received_set.contains(&i) {
          packets.push(Packet { buf: bufs[i].clone(), seqno: i })
        }
      }
      let c = c.send(packets);
      let (c, rec) = c.recv();
      for i in rec {
        received_set.insert(i);
      }
      if received_set.len() == bufs.len() {
        let c = c.right();
        let (c, _) = c.recv();
        let (c, _) = c.recv();
        let c = c.send(Ack);
        c.close();
        break;
      } else {
        let c = c.left();
        c.rec_pop()
      }
    }
  }
}

#[cfg(test)]
mod test {
  use session::*;
  use session::NOISY;
  use std::sync::atomic::Ordering;
  use rand;
  use rand::Rng;
  use tcp::*;
  use std::marker::PhantomData;
  use std::sync::mpsc::channel;
  use std::thread;

  fn gen_bufs() -> Vec<Buffer> {
    let mut bufs: Vec<Buffer> = Vec::new();
    let mut rng = rand::thread_rng();
    for _ in 0usize..20 {
      let buf: Buffer = vec![0; rng.gen_range(1, 10)];
      let buf: Buffer = buf.into_iter().map(|x: u8| rng.gen()).collect();
      bufs.push(buf);
    }
    bufs
  }

  #[test]
  fn test_basic() {
    let bufs = gen_bufs();
    let bufs_copy = bufs.clone();
    let (s, c): ((Chan<(), TCPServer>), (Chan<(), TCPClient>)) = Chan::new();
    let thread = thread::spawn(move || { tcp_client(c, bufs); });

    let recvd = tcp_server(s);
    let res = thread.join();

    assert_eq!(recvd, bufs_copy);
  }

  #[test]
  fn test_lossy() {
    let bufs = gen_bufs();
    let bufs_copy = bufs.clone();

    NOISY.with(|noisy| {
      noisy.store(true, Ordering::SeqCst);
    });

    let (s, c): ((Chan<(), TCPServer>), (Chan<(), TCPClient>)) = Chan::new();
    let thread = thread::spawn(move || { tcp_client(c, bufs); });

    let recvd = tcp_server(s);
    let res = thread.join();

    assert_eq!(recvd, bufs_copy);
  }
}
