use crate::prelude::*;

pub mod back;
pub mod left;

pub trait Segment: std::fmt::Debug {
    fn new() -> Self
    where
        Self: Sized;

    fn clone_box(&self) -> Box<dyn Segment>;

    /// Returns the id of the segment.
    fn id(&self) -> Uuid;

    /// Returns the name of the segment.
    fn name(&self) -> &'static str;

    /// Emit a bytecode recursively.
    fn emit(&mut self, op: &NodeOp, id: &Uuid);

    /// Add a segment to the given element.
    fn add_segment(&mut self, segment: &Box<dyn Segment>, id: &Uuid);

    /// Execute the bytecode of the segment.
    fn execute(&self, execution: &mut Execution, program: &mut Program);

    /*
    /// Emit an instruction. Needed as segments can contain recursive pattern based instructions, like
    /// a bricks pattern can subdefine brick() and cement().
    fn emit(&mut self, op: NodeOp);

    /// Returns true if the segment contains the given local coordinat
    fn contains(&self, local_coord: Vec3<f32>) -> bool;

    /// Applies the local coordinate to the virtual machine, i.e. sets up all pattern uvs etc. and executes the segments byte code.
    fn exec(&self, local_coord: Vec3<f32>, program: &mut Program) -> Option<Value>;

    */
}

impl Clone for Box<dyn Segment> {
    fn clone(&self) -> Box<dyn Segment> {
        self.clone_box()
    }
}
