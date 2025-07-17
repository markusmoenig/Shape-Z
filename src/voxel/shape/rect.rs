use crate::prelude::*;

#[derive(Clone, Debug)]
pub struct Rect {
    uuid: Uuid,
    pub segments: Vec<Box<dyn Segment>>,
    pub body: Vec<NodeOp>,
}

impl Shape for Rect {
    fn new() -> Self {
        Self {
            uuid: Uuid::new_v4(),
            segments: vec![],
            body: vec![],
        }
    }

    fn id(&self) -> Uuid {
        self.uuid
    }

    fn emit(&mut self, op: &NodeOp, id: &Uuid) {
        if self.uuid == *id {
            println!("rect got something {:?}", op);
            self.body.push(op.clone());
        } else {
            for segment in self.segments.iter_mut() {
                segment.emit(op, id);
            }
        }
    }

    fn add_segment(&mut self, segment: &Box<dyn Segment>, id: &Uuid) {
        if self.uuid == *id {
            println!("rect got a segment {:?}", segment.name());
            self.segments.push(segment.clone());
        }
    }

    fn clone_box(&self) -> Box<dyn Shape> {
        Box::new(self.clone())
    }
}
