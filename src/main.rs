
mod win32;
use win32::*;

fn main() {

    let mut window = WindowBuilder::new()
        .with_title("Hello, World!".to_string())
        .with_size(800, 600)
        .build();

    window.show();

    loop {
        win32::EventLoop::poll_events();
    }

}
