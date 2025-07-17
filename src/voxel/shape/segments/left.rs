use crate::prelude::*;

#[derive(Clone, Debug)]
pub struct Left {
    uuid: Uuid,
    pub body: Vec<NodeOp>,
}

impl Segment for Left {
    fn new() -> Self {
        Self {
            uuid: Uuid::new_v4(),
            body: vec![],
        }
    }

    fn id(&self) -> Uuid {
        self.uuid
    }

    fn name(&self) -> &'static str {
        "left"
    }

    fn emit(&mut self, op: &NodeOp, id: &Uuid) {
        if self.uuid == *id {
            println!("left got something {:?}", op);
            self.body.push(op.clone());
        }
    }

    fn clone_box(&self) -> Box<dyn Segment> {
        Box::new(self.clone())
    }
}
