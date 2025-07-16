use crate::prelude::*;

#[derive(Clone, Debug)]
pub struct Rect {
    pub body: Vec<NodeOp>,
}

impl Shape for Rect {
    fn new() -> Self {
        Self { body: vec![] }
    }

    fn emit(&mut self, op: NodeOp, depth: u32, rec: Vec<usize>) {
        println!("got something {} {:?}", depth, rec);
        self.body.push(op);
    }

    fn clone_box(&self) -> Box<dyn Shape> {
        Box::new(self.clone())
    }
}
