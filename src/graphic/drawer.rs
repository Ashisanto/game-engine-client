use std::sync::Arc;
use std::time::Duration;
use std::{fs, io, thread};

use flo_canvas::*;
use flo_draw::*;

pub fn render_2d() {
    with_2d_graphics(|| {
        let c_height = 1400.0;
        let c_width = 900.0;

        thread::spawn(|| {
            play::play("./assets/music/Final Boss.mp3").unwrap();
        });

        // Load a file
        let fire_bytes: &[u8] = include_bytes!["../../assets/animations/1.png"];
        let char_bytes: &[u8] = include_bytes!["../../assets/characters/1.png"];
        let font_space_mono =
            CanvasFontFace::from_slice(include_bytes!("../../assets/font/SpaceMono.ttf"));

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
        let mut fire_w = 0;
        let mut fire_h = 0;
        canvas.draw(|gc| {
            // Clear the canvas and set up
            gc.clear_canvas(Color::Rgba(0.0, 0.0, 0.0, 1.0));
            gc.canvas_height(c_height);
            gc.center_region(0.0, 0.0, c_width, c_height);

            // Set up the texture
            let (w, h) = gc
                .load_texture(TextureId(0), io::Cursor::new(fire_bytes))
                .unwrap();

            gc.load_texture(TextureId(1), io::Cursor::new(char_bytes))
                .unwrap();

            fire_w = w;
            fire_h = h;

            // Load the fonts
            gc.define_font_data(FontId(1), Arc::clone(&font_space_mono));
            gc.define_font_data(FontId(2), Arc::clone(&font_space_mono));
            gc.define_font_data(FontId(3), Arc::clone(&font_space_mono));
            gc.set_font_size(FontId(1), 18.0);
            gc.set_font_size(FontId(2), 24.0);
            gc.set_font_size(FontId(3), 32.0);
        });

        let mut angle_a = 0;
        let mut angle_c = 0;
        let mut step_a = 0;
        let mut step_c = 0;

        loop {
            // Render the png to the window
            canvas.draw(|gc| {
                let x_step = 96 * step_a;
                let x_char_step = 80 * step_c;

                // Redraw layer 0 rather than clearing the canvas (as clearing the canvas also clears out the textures)
                gc.layer(LayerId(1));
                gc.clear_layer();

                // Start with a simple text layout
                render_text_3(
                    gc,
                    "Frame ".to_string() + &angle_a.to_string(),
                    1000.0,
                    1300.0,
                );

                render_animate_sprit(gc, 0, x_step, 96.0, 96.0);
                render_char_sprit(gc, 1, x_char_step);
            });

            // Wait for the next frame
            thread::sleep(Duration::from_nanos(1_000_000_000 / 60));

            // Rotate the texture
            angle_a += 1;
            angle_c += 1;
            step_a = angle_a / 12;
            step_c = angle_c / 8;
            if angle_a > 120 {
                angle_a = 0;
            }
            if angle_c > 320 {
                angle_c = 0;
            }
        }
    });
}

fn render_animate_sprit(
    gc: &mut Vec<Draw>,
    texture_id: u64,
    x_step: i32,
    step_x: f32,
    step_y: f32,
) {
    // Draw a rect...
    gc.new_path();

    gc.rect(0.0, 480.0, step_x, 480.0 + step_y);

    // Fill with the texture we just loaded
    gc.fill_texture(
        TextureId(texture_id),
        x_step as f32,
        step_y,
        (x_step as f32) - 1152.0,
        0.0,
    );
    gc.fill();
}

fn render_char_sprit(gc: &mut Vec<Draw>, texture_id: u64, x_step: i32) {
    // Draw a rect...
    gc.new_path();
    gc.rect(0.0, 288.0, 80.0, 384.0);

    // Fill with the texture we just loaded
    gc.fill_texture(
        TextureId(texture_id),
        x_step as f32,
        380.0,
        (x_step as f32) - 320.0,
        0.0,
    );
    gc.fill();
}

fn render_text_1(gc: &mut Vec<Draw>, text: String, x: f32, y: f32) {
    gc.fill_color(Color::Rgba(1.0, 1.0, 1.0, 1.0));
    gc.begin_line_layout(x, y, TextAlignment::Left);
    gc.layout_text(FontId(1), text);
    gc.draw_text_layout();
}

fn render_text_2(gc: &mut Vec<Draw>, text: String, x: f32, y: f32) {
    gc.fill_color(Color::Rgba(1.0, 1.0, 1.0, 1.0));
    gc.begin_line_layout(x, y, TextAlignment::Left);
    gc.layout_text(FontId(2), text);
    gc.draw_text_layout();
}

fn render_text_3(gc: &mut Vec<Draw>, text: String, x: f32, y: f32) {
    gc.fill_color(Color::Rgba(1.0, 1.0, 1.0, 1.0));
    gc.begin_line_layout(x, y, TextAlignment::Left);
    gc.layout_text(FontId(3), text);
    gc.draw_text_layout();
}
