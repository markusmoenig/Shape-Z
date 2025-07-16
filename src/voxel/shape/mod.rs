use crate::prelude::*;

pub mod rect;

pub trait Segment {
    fn new() -> Self
    where
        Self: Sized;

    /// Returns the name of the segment.
    fn name(&self) -> &'static str;

    /// Emit an instruction. Needed as segments can contain recursive pattern based instructions, like
    /// a bricks pattern can subdefine brick() and cement().
    fn emit(&mut self, op: NodeOp);

    /// Returns true if the segment contains the given local coordinat
    fn contains(&self, local_coord: Vec3<f32>) -> bool;

    /// Applies the local coordinate to the virtual machine, i.e. sets up all pattern uvs etc. and executes the segments byte code.
    fn exec(&self, local_coord: Vec3<f32>, program: &mut Program) -> Option<Value>;
}

pub trait Shape: std::fmt::Debug {
    fn new() -> Self
    where
        Self: Sized;

    /// Emit a bytecode
    fn emit(&mut self, op: NodeOp, depth: u32, rec: Vec<usize>);

    fn clone_box(&self) -> Box<dyn Shape>;

    /*
    /// Get the index into the segment array for the segment of the given name.
    fn get_segment_index(&self, name: String) -> Option<usize>;

    /// Sets the target segment to emit bytecode to.
    fn set_segment_target(&self, name: String);

    /// Emit an instruction to the current target segment.
    fn emit(op: NodeOp);

    /// Get the segment for the given local coordinate.
    fn segment(&self, local_coord: Vec3<f32>);
    */
}

impl Clone for Box<dyn Shape> {
    fn clone(&self) -> Box<dyn Shape> {
        self.clone_box()
    }
}
