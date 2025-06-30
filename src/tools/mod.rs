pub mod brush;
pub mod edit;

pub use crate::prelude::*;

#[derive(PartialEq, Clone, Debug)]
pub enum ToolEvent {
    Activate,
    DeActivate,

    HitHover(HitRecord),
    HitClick,
    HitDrag(HitRecord),
    HitUp,
}

#[allow(unused)]
pub trait Tool: Send + Sync {
    fn new() -> Self
    where
        Self: Sized;

    fn id(&self) -> TheId;
    fn info(&self) -> String;
    fn icon_name(&self) -> String;

    fn accel(&self) -> Option<char> {
        None
    }

    fn tool_event(
        &mut self,
        tool_event: ToolEvent,
        ui: &mut TheUI,
        ctx: &mut TheContext,
        context: &mut Context,
    ) -> bool {
        false
    }

    fn handle_event(
        &mut self,
        event: &TheEvent,
        ui: &mut TheUI,
        ctx: &mut TheContext,
        context: &mut Context,
    ) -> bool {
        false
    }
}
