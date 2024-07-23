use minifb::{Key, Window, WindowOptions};
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

fn initialize_circle(framebuffer: &mut Framebuffer, center_x: usize, center_y: usize, radius: usize) {
    let num_points = 10;  

    for i in 0..num_points {
        let angle = 2.0 * std::f64::consts::PI * (i as f64) / (num_points as f64);
        let x = (center_x as f64 + radius as f64 * angle.cos()).round() as usize;
        let y = (center_y as f64 + radius as f64 * angle.sin()).round() as usize;

        match i % 10 {
            0 => initialize_glider(framebuffer, x, y),
            1 => initialize_beehive(framebuffer, x, y),
            2 => initialize_blinker(framebuffer, x, y),
            3 => initialize_toad(framebuffer, x, y),
            4 => initialize_pulsar(framebuffer, x, y),
            5 => initialize_lwss(framebuffer, x, y),
            6 => initialize_boat(framebuffer, x, y),
            7 => initialize_beacon(framebuffer, x, y),
            8 => initialize_mwss(framebuffer, x, y),
            9 => initialize_hwss(framebuffer, x, y),
            _ => {}
        }
    }
}

fn main() {
    let mut framebuffer = Framebuffer::new(80, 60);
    let radius = 12; 

    let center_x_left = framebuffer.width / 4;   
    let center_x_right = 3 * framebuffer.width / 4;  
    let center_y_top = framebuffer.height / 4;   
    let center_y_bottom = 3 * framebuffer.height / 4;  

    initialize_circle(&mut framebuffer, center_x_left, center_y_top, radius);
    initialize_circle(&mut framebuffer, center_x_right, center_y_top, radius);
    initialize_circle(&mut framebuffer, center_x_left, center_y_bottom, radius);
    initialize_circle(&mut framebuffer, center_x_right, center_y_bottom, radius);

    let mut window = Window::new("Game of Life - Conway", 800, 600, WindowOptions::default()).unwrap();
    while window.is_open() && !window.is_key_down(Key::Escape) {
        framebuffer.update_game_of_life();
        window.update_with_buffer(&framebuffer.buffer, framebuffer.width, framebuffer.height).unwrap();
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
