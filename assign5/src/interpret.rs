extern crate memory;

use ast::*;

#[allow(unused_macros)]
macro_rules! fmt_state {
  ($x:ident) => (
    format!("{}: {:?}", stringify!($x), $x)
  );
  ($x:ident, $($y:ident),*) => (
    format!("{} | {}", fmt_state!($x), fmt_state!($($y),*))
  );
  ($x:expr) => {{
    let s: String = format!("{:?}", $x).chars().collect();
    let v = &s[8..s.len()-2];
    let mut r = format!("memory: [");
    let mut i = 0;
    for _c in v.chars() {
      if _c == ',' || _c == ' ' {
        continue;
      } else if _c != '0' {
        r = format!("{} {}@{} ", r, _c, i);
      }
      i += 1;
    }
    format!("{}]", r)
  }}
}

// Print elements of config state, i.e. stack, locals, instrs
// Usage ex.:
//    print_config!(stack);
//    print_config!(instrs, stack);
//    etc
#[allow(unused_macros)]
macro_rules! print_config {
  ($x:ident) => (
    println!("{:?}", fmt_state!($x));
  );
  ($x:ident, $($y:ident),*) => (
    println!("{:?}", fmt_state!($x, $($y),*));
  );
}

// Print memory layout. Format is value@index.
// Usage: print_memory!(module.memory);
#[allow(unused_macros)]
macro_rules! print_memory {
  ($x:expr) => (
    println!("{:?}", fmt_state!($x));
  )
}

fn step(module: &mut WModule, config: WConfig) -> WConfig {
  use self::WInstr::*;

  let WConfig {mut locals, mut stack, mut instrs} = config;
  let instr = instrs.remove(0);

  let new_instr = match instr {

    Unreachable => Some(Trapping("Unreachable".to_string())),
    
    Const(n) => { stack.push(n); None },

    // YOUR CODE GOES HERE
    
    Binop(binop) => {
      let x2 = stack.pop().unwrap();
      let x1 = stack.pop().unwrap();
      match binop {
        WBinop::Add => { stack.push(x1 + x2); }
        WBinop::Sub => { stack.push(x1 - x2); }
        WBinop::Mul => { stack.push(x1 * x2); }
        WBinop::DivS => { if x2 == 0 { stack.push(0); } else { stack.push(x1 / x2); }}
      }
      None
    }
    
    Relop(relop) => {
      let x2 = stack.pop().unwrap();
      let x1 = stack.pop().unwrap();
      match relop {
        WRelop::Eq => { if x1 == x2 { stack.push(1); } else { stack.push(0); }}
        WRelop::Lt => { if x1 < x2 { stack.push(1); } else { stack.push(0); }}
        WRelop::Gt => { if x1 > x2 { stack.push(1); } else { stack.push(0); }}
      }
      None
    }
    
    GetLocal(i) => {
      stack.push(locals[i as usize]);
      None
    }
    
    SetLocal(i) => {
      let n = stack.pop().unwrap();
      locals[i as usize] = n;
      None
    }
    
    BrIf(label) => {
      let n = stack.pop().unwrap();
      if n == 0 {
        None
      } else {
        Some(Br(label))
      }
    }
    
    Return => {
      let n = stack.pop().unwrap();
      Some(Returning(n))
    }
    
    Loop(instrs) => {
      Some(Label{continuation: Box::new(Some(Loop(instrs.clone()))), stack: Vec::new(), instrs: instrs})
    }
    
    Block(instrs) => {
      Some(Label{continuation: Box::new(None), stack: Vec::new(), instrs: instrs})
    }
    
    Label{continuation, stack: mut new_stack, instrs: new_instrs} => {
      let mut c = WConfig {
        locals: locals.clone(),
        stack: new_stack,
        instrs: new_instrs
      };
      while c.instrs.len() > 0 {
        if let Trapping(_err) = &c.instrs[0] {
          break;
        } else if let Returning(_n) = &c.instrs[0] {
          break;
        } else if let Br(_n) = &c.instrs[0] {
          break;
        }
        c = step(module, c);
      };
      locals = c.locals;
      if c.instrs.len() > 0 {
        match c.instrs[0].clone() {
          Trapping(s) => Some(Trapping(s)),
          Returning(n) => Some(Returning(n)),
          Br(n) => {
            if n > 0 {
              Some(Br(n - 1))
            } else {
              *continuation
            }
          },
          _ => None
        }
      } else {
        None
      }
    }
    
    Call(i) => {
      let WFunc {params, locals, body} = module.funcs[i as usize].clone();
      let c = WConfig {
        locals: {
          let len = (params + locals) as usize;
          let mut l = vec![0; len];
          for i in 0..params {
            l[(params-i-1) as usize] = stack.pop().unwrap();
          }
          l
        },
        stack: Vec::new(),
        instrs: body
      };
      Some(Frame(c))
    }
    
    Frame(mut new_config) => {
      while new_config.instrs.len() > 0 {
        if let Trapping(_err) = &new_config.instrs[0] {
          break;
        } else if let Returning(_n) = &new_config.instrs[0] {
          break;
        }
        new_config = step(module, new_config);
      };
      if new_config.instrs.len() > 0 {
        match new_config.instrs[0].clone() {
          Trapping(s) => Some(Trapping(s)),
          Returning(n) => { stack.push(n); None },
          _ => None
        }
      } else {
        stack = new_config.stack;
        None
      }
    }
    
    Load => {
      let i = stack.pop().unwrap();
      let load = (*module.memory).load(i);
      match load {
        Some(n) => { stack.push(n); None }
        None => Some(Trapping("Invalid Load".to_string()))
      }
    }
    
    Store => {
      let n = stack.pop().unwrap();
      let i = stack.pop().unwrap();
      let store = (*module.memory).store(i, n);
      if store {
        None
      } else {
        Some(Trapping("Invalid Store".to_string()))
      }
    }
    
    Size => {
      stack.push((*module.memory).size());
      None
    }
    
    Grow => {
      (*module.memory).grow();
      None
    }
    
    Returning(_n) => {
      None
    }
    
    Br(_n) => {
      None
    }
    
    Trapping(_n) => unreachable!(),

    // END YOUR CODE

  };

  if let Some(ins) = new_instr {
    instrs.insert(0, ins);
  }

  WConfig {locals, stack, instrs}
}

pub fn interpret(mut module: WModule) -> Result<i32, String> {
  let mut config = WConfig {
    locals: vec![],
    stack: vec![],
    instrs: vec![WInstr::Call(0)]
  };

  while config.instrs.len() > 0 {
    if let WInstr::Trapping(ref err) = &config.instrs[0] {
      return Err(err.clone())
    }
    config = step(&mut module, config);
  }
  Ok(config.stack.pop().unwrap())
}
