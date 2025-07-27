use crate::prelude::*;
use js_sys::Promise;
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;
#[cfg(target_arch = "wasm32")]
use web_sys::console;

// Safe startup for both main thread and rayon workers
#[wasm_bindgen(start)]
pub fn wasm_start() {
    console_error_panic_hook::set_once();

    #[cfg(target_arch = "wasm32")]
    {
        if web_sys::window().is_none() {
            return;
        }
    }
}

#[wasm_bindgen]
pub fn main_init() {
    console_error_panic_hook::set_once();
}

// Quick environment probe for debugging from JS
#[wasm_bindgen]
pub fn is_worker() -> bool {
    #[cfg(target_arch = "wasm32")]
    {
        web_sys::window().is_none()
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        false
    }
}

// Expose current number of rayon threads for diagnostics
#[wasm_bindgen]
pub fn rayon_thread_count() -> usize {
    rayon::current_num_threads()
}

#[wasm_bindgen]
pub fn init_threads(n: usize) -> Promise {
    console_error_panic_hook::set_once();
    wasm_bindgen_rayon::init_thread_pool(n)
}

#[allow(dead_code)]
#[cfg(target_arch = "wasm32")]
fn now_ms() -> f64 {
    let g = js_sys::global();
    if let Ok(perf) = js_sys::Reflect::get(&g, &JsValue::from_str("performance")) {
        if let Some(p) = perf.dyn_ref::<web_sys::Performance>() {
            return p.now();
        }
    }
    js_sys::Date::now()
}

#[allow(dead_code)]
#[cfg(not(target_arch = "wasm32"))]
fn now_ms() -> f64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as f64
}

// One-shot compile check
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
                message: "Compiled successfully".into(),
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

// Progressive renderer: execute once, then accumulate path-trace samples
#[wasm_bindgen]
pub struct Renderer {
    width: u32,
    height: u32,
    sample_count: u32,
    target_samples: u32,
    executed: bool,
    shapez: ShapeZ,
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

        Ok(Self {
            width,
            height,
            sample_count: 0,
            target_samples: 64,
            executed: false,
            shapez,
        })
    }

    /// Prepare heavy scene evaluation once (runs ShapeZ::execute).
    pub fn prepare(&mut self) -> bool {
        if !self.executed {
            self.shapez.execute();
            let _ = self.shapez.stats();
            self.executed = true;
        }
        true
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

    /// Add N path-trace samples (SPP). Returns true if target reached.
    pub fn step_samples(&mut self, n: u32) -> bool {
        if !self.executed {
            self.prepare();
        }
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

    pub fn step(&mut self, n: u32) -> bool {
        self.step_samples(n)
    }

    /// Returns a displayable RGBA8 buffer (tonemapped + gamma) with len=w*h*4
    pub fn frame_rgba(&self) -> Box<[u8]> {
        let buf = self.shapez.write_image_to_array();
        let expected = (self.width as usize) * (self.height as usize) * 4;
        if buf.len() != expected {
            #[cfg(target_arch = "wasm32")]
            {
                let msg = format!(
                    "frame_rgba: got {} bytes, expected {} (is write_image_to_array() PNG?)",
                    buf.len(),
                    expected
                );
                console::log_1(&msg.into());
            }
            return vec![0u8; expected].into_boxed_slice();
        }
        buf.into_boxed_slice()
    }
}
