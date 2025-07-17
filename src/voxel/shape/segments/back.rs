use crate::prelude::*;

#[derive(Clone, Debug)]
pub struct Back {
    uuid: Uuid,
    pub body: Vec<NodeOp>,

    pub childs: Vec<Box<dyn Segment>>,
}

impl Segment for Back {
    fn new() -> Self {
        Self {
            uuid: Uuid::new_v4(),
            body: vec![],
            childs: vec![],
        }
    }

    fn id(&self) -> Uuid {
        self.uuid
    }

    fn name(&self) -> &'static str {
        "back"
    }

    fn emit(&mut self, op: &NodeOp, id: &Uuid) {
        if self.uuid == *id {
            println!("back got something {:?}", op);
            self.body.push(op.clone());
        }
    }

    fn add_segment(&mut self, segment: &Box<dyn Segment>, id: &Uuid) {
        if self.uuid == *id {
            println!("rect got a segment {:?}", segment.name());
            self.childs.push(segment.clone());
        }
    }

    fn execute(&self, execution: &mut Execution, program: &mut Program) {
        let old_max_z = execution.bbox.max.z;

        let local = execution.local.as_vec3();

        let thickness = 0.1;
        execution.bbox.max.z = execution.bbox.min.z + thickness;

        if execution.bbox.contains_point(local) {
            execution.u = Value::from_float(local.x - execution.bbox.min.x); // u = X
            execution.v = Value::from_float(local.y - execution.bbox.min.y); // v = Y
            execution.d = Value::from_float(local.z - execution.bbox.min.z); // d = Z (depth)

            execution.execute(&self.body, program);
            for child in self.childs.iter() {
                child.execute(execution, program);
            }
        }

        execution.bbox.max.z = old_max_z;
    }

    fn clone_box(&self) -> Box<dyn Segment> {
        Box::new(self.clone())
    }
}
