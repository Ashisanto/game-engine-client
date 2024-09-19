mod graphic;

use std::thread;

use graphic::drawer::render_2d;

#[tokio::main]
async fn main() {
    render_2d();
}
