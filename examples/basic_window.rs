
extern crate platinum;

use platinum::{Window, WindowBuilder, EventLoop};

fn main() {

    let mut window = WindowBuilder::new()
        .with_title("Hello, World!".to_string())
        .with_size(800, 600)
        .build();

    window.show();

    loop {
        EventLoop::poll_events();
    }

}
