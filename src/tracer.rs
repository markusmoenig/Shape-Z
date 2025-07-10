use crate::prelude::*;
use std::sync::{Arc, Mutex};
use std::thread;
use vek::Vec2;

// use crate::editor::{CAMERA, PALETTE, RENDERBUFFER, RENDERER, VOXELGRID};

pub struct Tracer {}

#[allow(clippy::new_without_default)]
impl Tracer {
    pub fn new() -> Self {
        Self {}
    }

    /*
    pub fn draw(&mut self, ui: &mut TheUI) {
        if let Some(render_view) = ui.get_render_view("ModelView") {
            let dim = *render_view.dim();
            let surface = render_view.render_buffer_mut();
            surface.resize(dim.width, dim.height);

            let mut rb = Arc::clone(&RENDERBUFFER);

            // Resize if needed
            {
                let mut buffer = rb.lock().unwrap();
                if buffer.width != dim.width as usize || buffer.height != dim.height as usize {
                    *buffer = RenderBuffer::new(dim.width as usize, dim.height as usize);
                    buffer.accum = 1;
                }
            }

            let grid = Arc::clone(&VOXELGRID);
            let palette = Arc::clone(&PALETTE);
            let renderer = Arc::clone(&RENDERER);
            let camera = Arc::clone(&CAMERA);

            // Render
            self.render(&mut rb, &grid, &palette, &renderer, &camera);
            {
                let mut buffer = rb.lock().unwrap();
                buffer.accum += 1;
            }

            // Blit
            {
                let buffer = rb.lock().unwrap();
                buffer.to_u8_vec_gamma_buffer(surface.pixels_mut());
            }
        }
    }*/

    pub fn render(
        &self,
        buffer: &mut Arc<Mutex<RenderBuffer>>,
        grid: &Arc<RwLock<VoxelGrid>>,
        palette: &Arc<RwLock<Palette>>,
        renderer: &Arc<Box<dyn Renderer>>,
        camera: &Arc<RwLock<Box<dyn Camera>>>,
    ) {
        let tile_size = (80, 80);

        let width = buffer.lock().unwrap().width;
        let height = buffer.lock().unwrap().height;

        let tiles = self.create_tiles(width, height, tile_size.0, tile_size.1);
        let screen_size = Vec2::new(width as F, height as F);
        let tiles_mutex = Arc::new(Mutex::new(tiles));

        let num_cpus = num_cpus::get();
        let _start = self.get_time();

        // let ft_arc = Arc::clone(&ft);
        let grid_arc = Arc::clone(grid);
        let palette_arc = Arc::clone(palette);
        let renderer_arc = Arc::clone(renderer);
        // let camera_arc = Arc::clone(camera);

        // Create threads
        let mut handles = vec![];
        for _ in 0..num_cpus {
            // let ft = Arc::clone(&ft_arc);
            let grid = Arc::clone(&grid_arc);
            let palette = Arc::clone(&palette_arc);
            let renderer = Arc::clone(&renderer_arc);
            let camera = Arc::clone(camera);

            let tiles_mutex = Arc::clone(&tiles_mutex);
            let buffer_mutex = Arc::clone(buffer);

            let handle = thread::spawn(move || {
                let mut tile_buffer = RenderBuffer::new(tile_size.0, tile_size.1);
                loop {
                    // Lock mutex to access tiles
                    let mut tiles = tiles_mutex.lock().unwrap();

                    // Check if there are remaining tiles
                    if let Some(tile) = tiles.pop() {
                        // Release mutex before processing tile
                        drop(tiles);

                        let grid_guard = grid.read().unwrap();
                        let grid_ref: &VoxelGrid = &grid_guard;
                        let palette_guard = palette.read().unwrap();
                        let palette_ref: &Palette = &palette_guard;
                        let camera_guard = camera.read().unwrap();
                        let camera_ref = &camera_guard;

                        // Process tile
                        for h in 0..tile.height {
                            for w in 0..tile.width {
                                let x = tile.x + w;
                                let y = tile.y + h;

                                if x >= width || y >= height {
                                    continue;
                                }

                                let uv = Vec2::new(
                                    x as F / screen_size.x,
                                    1.0 - (y as F / screen_size.y),
                                );

                                let p = renderer.render(
                                    uv,
                                    screen_size,
                                    grid_ref,
                                    palette_ref,
                                    camera_ref,
                                );
                                tile_buffer.set(w, h, p.into_array());
                                // tile_buffer.set(w, h, [uv.x, uv.y, 0.0, 1.0]);
                            }
                        }
                        // Save the tile buffer to the main buffer
                        buffer_mutex
                            .lock()
                            .unwrap()
                            .accum_from(tile.x, tile.y, &tile_buffer);
                    } else {
                        // No remaining tiles, exit loop
                        break;
                    }
                }
            });
            handles.push(handle);
        }

        // Wait for all threads to finish
        for handle in handles {
            handle.join().unwrap();
        }

        let _stop = self.get_time();
        println!("Shader execution time: {:?} ms.", _stop - _start);
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
