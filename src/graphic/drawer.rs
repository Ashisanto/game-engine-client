use std::sync::Arc;
use std::time::Duration;
use std::{fs, io, thread};

use flo_canvas::*;
use flo_draw::*;

pub fn render_2d() {
    with_2d_graphics(|| {
        let c_height = 1400.0;
        let c_width = 900.0;

        // Load a file
        let flo_bytes: &[u8] = include_bytes!["../assets/flo_drawing_on_window.png"];
        let font_space_mono =
            CanvasFontFace::from_slice(include_bytes!("../assets/font/SpaceMono.ttf"));

        let mut animate_bytes: Vec<&u8> = vec![];

        let animate_paths = fs::read_dir("./assets/animations");

        match animate_paths {
            Ok(dir_path) => {
                for animate in dir_path.into_iter() {
                    match animate {
                        Ok(file) => {
                            println!("Name: {}", file.path().display())
                        }
                        Err(_) => {}
                    }
                }
            }
            Err(_err) => {}
        }

        // Create a window
        let canvas = create_drawing_window("Game client");

        // Load the texture into it
        let mut flo_w = 0;
        let mut flo_h = 0;
        canvas.draw(|gc| {
            // Clear the canvas and set up the coordinates
            gc.clear_canvas(Color::Rgba(1.0, 1.0, 1.0, 1.0));
            gc.canvas_height(c_height);
            gc.center_region(0.0, 0.0, c_width, c_height);

            // Set up the texture
            let (w, h) = gc
                .load_texture(TextureId(0), io::Cursor::new(flo_bytes))
                .unwrap();
            flo_w = w;
            flo_h = h;

            // Load the fonts
            gc.define_font_data(FontId(1), Arc::clone(&font_space_mono));
            gc.define_font_data(FontId(2), Arc::clone(&font_space_mono));
            gc.define_font_data(FontId(3), Arc::clone(&font_space_mono));
            gc.set_font_size(FontId(1), 18.0);
            gc.set_font_size(FontId(2), 24.0);
            gc.set_font_size(FontId(3), 32.0);
        });

        let mut angle = 0.0;

        loop {
            // Render the png to the window
            canvas.draw(|gc| {
                // Redraw layer 0 rather than clearing the canvas (as clearing the canvas also clears out the textures)
                gc.layer(LayerId(0));
                gc.clear_layer();

                let ratio = (flo_w as f32) / (flo_h as f32);
                let height = c_height / ratio;
                let y_pos = (c_height - height) / 2.0;

                let mid_x = 500.0;
                let mid_y = y_pos + (height / 2.0);

                // Start with a simple text layout
                gc.fill_color(Color::Rgba(0.0, 0.0, 0.6, 1.0));
                gc.begin_line_layout(1200.0, 900.0, TextAlignment::Left);
                gc.layout_text(FontId(3), "Angel ".to_string() + &angle.to_string());
                gc.draw_text_layout();

                // Draw a circle...
                gc.new_path();
                gc.circle(mid_x, mid_y, height / 2.0);

                // Fill with the texture we just loaded
                gc.fill_texture(TextureId(0), 0.0, y_pos + height as f32, c_width, y_pos);

                gc.fill_transform(Transform2D::translate(-mid_x, -mid_y));
                gc.fill_transform(Transform2D::rotate_degrees(angle));
                gc.fill_transform(Transform2D::scale(1.0 / 3.0, 1.0 / 3.0));
                gc.fill_transform(Transform2D::translate(mid_x, mid_y));
                gc.fill();

                // Draw another couple of circles to demonstrate that it's the texture that's spinning and not the whole canvas
                gc.fill_color(Color::Rgba(0.0, 0.0, 0.0, 1.0));
            });

            // Wait for the next frame
            thread::sleep(Duration::from_nanos(1_000_000_000 / 60));

            // Rotate the texture
            angle += 1.0;
        }
    });
}
