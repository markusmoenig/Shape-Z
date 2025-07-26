use crate::prelude::*;
use rayon::prelude::*;
use std::sync::{Arc, Mutex};

pub struct Tracer {}

#[allow(clippy::new_without_default)]
impl Tracer {
    pub fn new() -> Self {
        Self {}
    }

    pub fn render(
        &self,
        buffer: &mut Arc<Mutex<RenderBuffer>>,
        grid: &Arc<RwLock<VoxelGrid>>,
        materials: &Arc<RwLock<Vec<Vec<NodeOp>>>>,
        renderer: &Arc<RwLock<Box<dyn Renderer>>>,
        camera: &Arc<RwLock<Box<dyn Camera>>>,
    ) {
        let tile_size = (80, 80);

        let width = buffer.lock().unwrap().width;
        let height = buffer.lock().unwrap().height;

        let tiles = self.create_tiles(width, height, tile_size.0, tile_size.1);
        let screen_size = Vec2::new(width as F, height as F);

        let grid_guard = grid.read().unwrap();
        let material_guard = materials.read().unwrap();
        let renderer_guard = renderer.read().unwrap();
        let camera_guard = camera.read().unwrap();

        tiles.par_iter().for_each(|tile| {
            let mut tile_buffer = RenderBuffer::new(tile.width, tile.height);

            for h in 0..tile.height {
                for w in 0..tile.width {
                    let x = tile.x + w;
                    let y = tile.y + h;

                    if x >= width || y >= height {
                        continue;
                    }

                    let uv = Vec2::new(x as F / screen_size.x, 1.0 - (y as F / screen_size.y));

                    let color = renderer_guard.render(
                        uv,
                        screen_size,
                        &grid_guard,
                        &material_guard,
                        &camera_guard,
                    );
                    tile_buffer.set(w, h, color.into_array());
                }
            }

            buffer
                .lock()
                .unwrap()
                .accum_from(tile.x, tile.y, &tile_buffer);
        });

        buffer.lock().unwrap().accum += 1;
    }

    /// Get the current time
    pub fn get_time(&self) -> u128 {
        #[cfg(target_arch = "wasm32")]
        {
            web_sys::window().unwrap().performance().unwrap().now() as u128
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            let stop = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("Time went backwards");
            stop.as_millis()
        }
    }

    /// Create the tiles for the given image size.
    fn create_tiles(
        &self,
        image_width: usize,
        image_height: usize,
        tile_width: usize,
        tile_height: usize,
    ) -> Vec<Tile> {
        let mut tiles = Vec::new();
        let mut x = 0;
        let mut y = 0;
        while x < image_width && y < image_height {
            let tile = Tile {
                x,
                y,
                width: tile_width,
                height: tile_height,
            };
            tiles.push(tile);
            x += tile_width;
            if x >= image_width {
                x = 0;
                y += tile_height;
            }
        }

        tiles
    }
}

#[derive(Debug, Clone, Copy)]
struct Tile {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
}
