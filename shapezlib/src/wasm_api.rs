use crate::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::console;

#[wasm_bindgen]
pub struct CompileInfo {
    ok: bool,
    message: String,
}

#[wasm_bindgen]
impl CompileInfo {
    #[wasm_bindgen(getter)]
    pub fn ok(&self) -> bool {
        self.ok
    }
    #[wasm_bindgen(getter)]
    pub fn message(&self) -> String {
        self.message.clone()
    }
}

#[wasm_bindgen]
pub fn compile_check(code: &str, width: u32, height: u32) -> CompileInfo {
    console_error_panic_hook::set_once();

    let mut shapez = ShapeZ::default();
    shapez.set_resolution(width as usize, height as usize);

    match shapez.parse_str(code.to_string()) {
        Ok(module) => match shapez.compile(&module) {
            Ok(()) => CompileInfo {
                ok: true,
                message: "OK".into(),
            },
            Err(e) => CompileInfo {
                ok: false,
                message: format!("Compile error: {e}"),
            },
        },
        Err(e) => CompileInfo {
            ok: false,
            message: format!("Parse error: {e}"),
        },
    }
}

// ---------- Sample-based progressive renderer ----------
#[wasm_bindgen]
pub struct Renderer {
    width: u32,
    height: u32,
    accum: Vec<f32>,     // len = w*h*3, linear RGB sum
    sample_count: u32,   // accumulated SPP
    target_samples: u32, // desired SPP
    shapez: ShapeZ,      // your engine instance
}

#[wasm_bindgen]
impl Renderer {
    #[wasm_bindgen(constructor)]
    pub fn new(code: &str, width: u32, height: u32) -> Result<Renderer, JsValue> {
        console_error_panic_hook::set_once();

        let mut shapez = ShapeZ::default();
        shapez.set_resolution(width as usize, height as usize);

        let module = shapez
            .parse_str(code.to_string())
            .map_err(|e| JsValue::from_str(&format!("Parse error: {e}")))?;
        shapez
            .compile(&module)
            .map_err(|e| JsValue::from_str(&format!("Compile error: {e}")))?;

        // IMPORTANT: build the evaluated scene/voxel/grid before sampling
        let _t0 = shapez.get_time();
        shapez.execute();
        let _ = shapez.stats();
        let _t1 = shapez.get_time();

        let len = (width as usize) * (height as usize) * 3;
        Ok(Self {
            width,
            height,
            accum: vec![0.0; len],
            sample_count: 0,
            target_samples: 64,
            shapez,
        })
    }

    pub fn set_target_samples(&mut self, target: u32) {
        self.target_samples = target.max(1);
    }
    pub fn current_samples(&self) -> u32 {
        self.sample_count
    }
    pub fn target_samples(&self) -> u32 {
        self.target_samples
    }
    pub fn progress(&self) -> f32 {
        (self.sample_count as f32 / self.target_samples as f32).min(1.0)
    }

    #[wasm_bindgen(getter)]
    pub fn width(&self) -> u32 {
        self.width
    }
    #[wasm_bindgen(getter)]
    pub fn height(&self) -> u32 {
        self.height
    }

    /// Preferred stepping API: add N path-trace samples (SPP)
    pub fn step_samples(&mut self, n: u32) -> bool {
        let n = n.max(1);
        for _ in 0..n {
            self.shapez.sample();
            self.sample_count = self.sample_count.saturating_add(1);
            if self.sample_count >= self.target_samples {
                break;
            }
        }
        self.sample_count >= self.target_samples
    }

    /// Fallback name (the playground calls this if `step_samples` is absent)
    pub fn step(&mut self, n: u32) -> bool {
        self.step_samples(n)
    }

    /// Returns a displayable RGBA8 buffer (tonemapped + gamma)
    pub fn frame_rgba(&self) -> Box<[u8]> {
        let buf = self.shapez.write_image_to_array();
        let expected = (self.width as usize) * (self.height as usize) * 4;
        if buf.len() != expected {
            // Likely PNG-encoded bytes; return a cleared RGBA frame to prevent exceptions
            let msg = format!(
                "frame_rgba: got {} bytes, expected {} (is write_image_to_array() PNG?)",
                buf.len(),
                expected
            );
            console::log_1(&msg.into());
            return vec![0u8; expected].into_boxed_slice();
        }
        buf.into_boxed_slice()
    }
}
