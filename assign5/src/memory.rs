use std::fmt::Debug;
use std::{mem, ptr, alloc::{alloc, realloc, Layout}};

pub trait WMemory: Debug {
  fn load(&self, address: i32) -> Option<i32>;
  fn store(&mut self, address: i32, value: i32) -> bool;
  fn grow(&mut self);
  fn size(&self) -> i32;
}

#[derive(Debug)]
pub struct VecMem(Vec<i32>);

const PAGE_SIZE: i32 = 4096;

fn alloc_page(npages: i32) -> Vec<i32> {
  (0..npages*PAGE_SIZE).map(|_| 0).collect::<Vec<_>>()
}


impl VecMem {
  pub fn new(npages: i32) -> VecMem {
    VecMem(alloc_page(npages))
  }
}

impl WMemory for VecMem {
  fn load(&self, address: i32) -> Option<i32> {
    // YOUR CODE GOES HERE
    let idx = address as usize;
    let n = self.0.get(idx);
    match n {
      Some(val) => Some(*val),
      None => None
    }
  }

  fn store(&mut self, address: i32, value: i32) -> bool {
    // YOUR CODE GOES HERE
    let idx = address as usize;
    let x = self.0.get_mut(idx);
    match x {
      Some(x) => { *x = value; true },
      None => false
    }
  }

  fn grow(&mut self) {
    // YOUR CODE GOES HERE
    let newsize = PAGE_SIZE as usize;
    let newsize = newsize + self.0.len();
    self.0.resize(newsize, 0);
  }

  fn size(&self) -> i32 {
    // YOUR CODE GOES HERE
    self.0.len() as i32
  }
}


#[derive(Debug)]
pub struct UnsafeMem {
  data: *mut i32,
  size: i32,
}

impl UnsafeMem {
  // npages must be > 0
  pub fn new(npages: i32) -> UnsafeMem {
    let size = (npages * PAGE_SIZE) as usize;
    let data = unsafe {
      let typesize = mem::size_of::<i32>();
      let mut data = alloc(Layout::from_size_align(size * typesize, typesize).unwrap());
      ptr::write_bytes(data, 0, size * typesize);
      data
    } as *mut i32;

    UnsafeMem { data, size: size as i32 }
  }
}

impl WMemory for UnsafeMem {
  fn load(&self, address: i32) -> Option<i32> {
    // YOUR CODE GOES HERE
    if address >= 0 && address < self.size {
      unsafe {
        Some(*self.data.offset(address as isize))
      }
    } else {
      None
    }
  }

  fn store(&mut self, address: i32, value: i32) -> bool {
    // YOUR CODE GOES HERE
    if address >= 0 && address < self.size {
      unsafe {
        self.data.offset(address as isize).write(value);
        true
      }
    } else {
      false
    }
  }

  fn grow(&mut self) {
    // YOUR CODE GOES HERE
    let typesize = mem::size_of::<i32>();
    let currsize = self.size as usize;
    let newsize = (self.size + PAGE_SIZE) as usize;
    unsafe {
      self.data = realloc(self.data as *mut u8, Layout::from_size_align(currsize * typesize, typesize).unwrap(), newsize) as *mut i32;
    }
    self.size = newsize as i32;
  }

  fn size(&self) -> i32 {
    // YOUR CODE GOES HERE
    self.size
  }
}
