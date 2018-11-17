use session_bug::*;

fn sample_bug() {
  // this is only a bug **if** you comment out the Chan<Close> implementation
  // on line 63. you should get a compile error
  type Server = Close;
  let (c, _): (Chan<Server>, _) = Chan::new();
  c.close();
}

pub fn bug1() {
  type Server = Recv<i32, Close>;
  type Client = <Server as HasDual>::Dual;
  let (c1, c2): (Chan<Server>, Chan<Client>) = Chan::new();
  c2.send(0); // bug shown here; the dual of recv is wrong in the buggy code, so c2 doesn't have send defined at compile time
  c1.recv();
}

pub fn bug2() {
  type Server = Send<i32, Close>;
  type Client = <Server as HasDual>::Dual;
  let (c1, c2): (Chan<Server>, Chan<Client>) = Chan::new();
  c1.send(0).close(); // bug shown here; send doesn't return the correct Chan in the buggy code, so close is not available at compile time
}

pub fn bug3() {
  type Server = Choose<Close, Close>;
  type Client = <Server as HasDual>::Dual;
  let (c1, c2): (Chan<Server>, Chan<Client>) = Chan::new();
  c1.left();
  match c2.offer() {
  	Branch::Left(_) => {
  		assert_eq!(0, 0);
  	},
  	Branch::Right(_) => {
  		assert_eq!(0, 1); // bug shown here; left() and right() write the wrong value, so they are flipped in practice
  	}
  }
}

#[cfg(test)]
mod tests {
  #[test]
  fn bug1() {
    super::bug1();
  }

  #[test]
  fn bug2() {
    super::bug2();
  }

  #[test]
  fn bug3() {
    super::bug3();
  }
}
