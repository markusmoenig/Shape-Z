pub mod undo_atom;
pub mod undo_stack;

use crate::prelude::*;
use undo_atom::*;

#[derive(Clone, Debug)]
pub struct UndoManager {
    pub max_undo: usize,

    stack: UndoStack,
}

impl Default for UndoManager {
    fn default() -> Self {
        Self::new()
    }
}

impl UndoManager {
    pub fn new() -> Self {
        Self {
            max_undo: 30,

            stack: UndoStack::default(),
        }
    }

    pub fn add_undo(&mut self, atom: UndoAtom, ctx: &mut TheContext) {
        self.stack.add(atom);
        self.stack.truncate_to_limit(self.max_undo);
        ctx.ui.set_enabled("Undo");
        self.can_save(ctx);
    }

    pub fn undo(&mut self, ui: &mut TheUI, ctx: &mut TheContext, context: &mut Context) {
        self.stack.undo(ui, ctx, context);

        if !self.stack.has_undo() {
            ctx.ui.set_disabled("Undo");
        } else {
            ctx.ui.set_enabled("Undo");
        }

        if !self.stack.has_redo() {
            ctx.ui.set_disabled("Redo");
        } else {
            ctx.ui.set_enabled("Redo");
        }
        self.can_save(ctx);
    }

    pub fn redo(&mut self, ui: &mut TheUI, ctx: &mut TheContext, context: &mut Context) {
        self.stack.redo(ui, ctx, context);

        if !self.stack.has_undo() {
            ctx.ui.set_disabled("Undo");
        } else {
            ctx.ui.set_enabled("Undo");
        }

        if !self.stack.has_redo() {
            ctx.ui.set_disabled("Redo");
        } else {
            ctx.ui.set_enabled("Redo");
        }
        self.can_save(ctx);
    }

    /// Checks if the undo manager is empty and disables the save buttons if it is.
    pub fn can_save(&self, ctx: &mut TheContext) {
        if self.has_undo() {
            ctx.ui.set_enabled("Save");
            ctx.ui.set_enabled("Save As");
        } else {
            ctx.ui.set_disabled("Save");
            ctx.ui.set_disabled("Save As");
        }
    }

    /// Checks if the undo manager has any undoable actions.
    pub fn has_undo(&self) -> bool {
        self.stack.has_undo()
    }
}
