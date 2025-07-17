use crate::prelude::*;

pub mod segments;

pub mod rect;

pub trait Shape: std::fmt::Debug {
    fn new() -> Self
    where
        Self: Sized;

    /// Returns the id of the shape.
    fn id(&self) -> Uuid;

    /// Emit a bytecode recursively to the segments of the shape.
    fn emit(&mut self, op: &NodeOp, id: &Uuid);

    /// Add a segment to the given element.
    fn add_segment(&mut self, segment: &Box<dyn Segment>, id: &Uuid);

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
