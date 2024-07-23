use minifb::{Key, Window, WindowOptions};
use image::{ImageBuffer, Rgba};
mod framebuffer;
use framebuffer::Framebuffer;


fn initialize_glider(framebuffer: &mut Framebuffer, x: usize, y: usize) {
    let positions = [
        (1, 0),
        (2, 1),
        (0, 2), (1, 2), (2, 2),
    ];
    for (dx, dy) in positions.iter() {
        let px = x + dx;
        let py = y + dy;
        if px < framebuffer.width && py < framebuffer.height {
            framebuffer.point(px, py);
        }
    }
}

fn initialize_beehive(framebuffer: &mut Framebuffer, x: usize, y: usize) {
    let positions = [
        (1, 0), (2, 0),
        (0, 1), (3, 1),
        (1, 2), (2, 2),
    ];
    for (dx, dy) in positions.iter() {
        framebuffer.point(x + dx, y + dy);
    }
}

fn initialize_blinker(framebuffer: &mut Framebuffer, x: usize, y: usize) {
    let positions = [
        (1, 0),
        (1, 1),
        (1, 2),
    ];
    for (dx, dy) in positions.iter() {
        framebuffer.point(x + dx, y + dy);
    }
}

fn initialize_toad(framebuffer: &mut Framebuffer, x: usize, y: usize) {
    let positions = [
        (1, 0), (2, 0), (3, 0),
        (0, 1), (1, 1), (2, 1),
    ];
    for (dx, dy) in positions.iter() {
        framebuffer.point(x + dx, y + dy);
    }
}

fn initialize_pulsar(framebuffer: &mut Framebuffer, x: usize, y: usize) {
    let positions = [
        // First square
        (2, 0), (3, 0), (4, 0),
        (0, 2), (5, 2),
        (0, 3), (5, 3),
        (0, 4), (5, 4),
        (2, 5), (3, 5), (4, 5),
        // Second square (symmetrical)
        (2, 7), (3, 7), (4, 7),
        (0, 9), (5, 9),
        (0, 10), (5, 10),
        (0, 11), (5, 11),
        (2, 12), (3, 12), (4, 12),
    ];
    for (dx, dy) in positions.iter() {
        framebuffer.point(x + dx, y + dy);
    }
}

fn initialize_lwss(framebuffer: &mut Framebuffer, x: usize, y: usize) {
    let positions = [
        (0, 0), (3, 0),
        (4, 1),
        (0, 2), (4, 2),
        (1, 3), (2, 3), (3, 3), (4, 3),
    ];
    for (dx, dy) in positions.iter() {
        framebuffer.point(x + dx, y + dy);
    }
}

fn initialize_mwss(framebuffer: &mut Framebuffer, x: usize, y: usize) {
    let positions = [
        (1, 0), (4, 0),
        (0, 1),
        (0, 2), (4, 2),
        (0, 3), (1, 3), (2, 3), (3, 3), (4, 3),
    ];
    for (dx, dy) in positions.iter() {
        framebuffer.point(x + dx, y + dy);
    }
}

fn initialize_hwss(framebuffer: &mut Framebuffer, x: usize, y: usize) {
    let positions = [
        (1, 0), (2, 0), (3, 0), (4, 0),
        (0, 1),
        (0, 2), (4, 2),
        (0, 3), (1, 3), (2, 3), (3, 3), (4, 3),
    ];
    for (dx, dy) in positions.iter() {
        framebuffer.point(x + dx, y + dy);
    }
}

fn initialize_boat(framebuffer: &mut Framebuffer, x: usize, y: usize) {
    let positions = [
        (0, 0), (1, 0),
        (0, 1), (2, 1),
        (1, 2),
    ];
    for (dx, dy) in positions.iter() {
        framebuffer.point(x + dx, y + dy);
    }
}

fn initialize_beacon(framebuffer: &mut Framebuffer, x: usize, y: usize) {
    let positions = [
        (0, 0), (1, 0),
        (0, 1),
        (2, 2), (3, 2),
        (3, 3),
    ];
    for (dx, dy) in positions.iter() {
        framebuffer.point(x + dx, y + dy);
    }
}

fn main() {
    let mut framebuffer = Framebuffer::new(80, 60);
    initialize_glider(&mut framebuffer, 10, 10);
    initialize_beehive(&mut framebuffer, 20, 10);
    initialize_blinker(&mut framebuffer, 30, 10);
    initialize_toad(&mut framebuffer, 40, 10);
    initialize_pulsar(&mut framebuffer, 50, 10);
    initialize_lwss(&mut framebuffer, 60, 10);
    initialize_boat(&mut framebuffer, 70, 10);
    initialize_beacon(&mut framebuffer, 10, 20);
    initialize_mwss(&mut framebuffer, 20, 20);
    initialize_hwss(&mut framebuffer, 30, 20);

    let mut window = Window::new("Game of Life - Conway", 800, 600, WindowOptions::default()).unwrap();
    while window.is_open() && !window.is_key_down(Key::Escape) {
        framebuffer.update_game_of_life();
        window.update_with_buffer(&framebuffer.buffer, framebuffer.width, framebuffer.height).unwrap();
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
