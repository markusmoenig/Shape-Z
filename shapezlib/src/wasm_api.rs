use crate::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::console;

#[wasm_bindgen]
pub fn compile(code: &str) -> Result<Box<[u8]>, JsValue> {
    console_error_panic_hook::set_once();

    let msg = format!("code length: {}", code);
    console::log_1(&msg.into());

    let mut shapez = ShapeZ::default();

    shapez.set_resolution(400, 400);

    let module = shapez
        .parse_str(code.to_string())
        .map_err(|e| JsValue::from_str(&format!("Parse error: {e}")))?;

    shapez
        .compile(&module)
        .map_err(|e| JsValue::from_str(&format!("Compile error: {e}")))?;
    // Ok(png.into_boxed_slice())

    let t0 = shapez.get_time();
    shapez.execute();
    let (voxels, mem) = shapez.stats();
    let t1 = shapez.get_time();
    println!(
        "Compiled {voxels} voxels ({mem} MB) in {:.2}s",
        (t1 - t0) as f32 / 1000.0
    );

    for i in 0..10 {
        shapez.sample();
    }

    let buf = shapez.write_image_to_array();
    let msg = format!("code length: {}", buf.len());
    console::log_1(&msg.into());

    Ok(buf.into_boxed_slice())
}
