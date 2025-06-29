use crate::editor::{PALETTE, RENDERBUFFER};
use crate::prelude::*;
use std::sync::Arc;

pub fn reset_render() {
    let rb = Arc::clone(&RENDERBUFFER);
    let mut buffer = rb.lock().unwrap();
    buffer.accum = 1;
}

pub fn update_palette_ui(ui: &mut TheUI, _ctx: &mut TheContext) {
    if let Some(picker) = ui.get_palette_picker("PalettePicker") {
        let mut palette = ThePalette::default();
        let mats = PALETTE.write().unwrap();
        for (index, mat) in mats.materials.iter().enumerate() {
            palette.colors[index] = Some(TheColor::from(mat.base_color));
        }
        picker.set_palette(palette);
    }
}
