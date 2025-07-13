use crate::prelude::*;

pub struct Execution {
    pub stack: Vec<Value>,
    // pub program: &'a Program,
}

impl<'a> Execution {
    pub fn new() -> Self {
        // pub fn new(program: &'a Program) -> Self {
        Self {
            stack: Vec::with_capacity(32),
            // program,
        }
    }

    pub fn execute(&mut self, code: &[NodeOp]) {
        for op in code {
            match op {
                NodeOp::Push(v) => self.stack.push(*v),
                NodeOp::Add => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(a.add(b));
                }
                NodeOp::Sub => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(a.sub(b));
                }
                NodeOp::Mul => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(a.mul(b));
                }
                NodeOp::Div => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(a.div(b));
                }
                NodeOp::Length => {
                    let a = self.stack.pop().unwrap();
                    self.stack.push(a.length());
                }
                NodeOp::Abs => {
                    let a = self.stack.pop().unwrap();
                    self.stack.push(a.abs());
                } // NodeOp::CallShape(name) => {
                  //     if let Some(shape_code) = self.program.definitons.get(name) {
                  //         self.execute(shape_code);
                  //     } else {
                  //         panic!("Undefined shape: {}", name);
                  //     }
                  // }
            }
        }
    }
}
