use minifb::{Key, Window, WindowOptions};
use image::{ImageBuffer, Rgba};
mod framebuffer;
mod bmp;
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

fn main() {
    let mut framebuffer = Framebuffer::new(80, 60);  // Asegúrate de que estas dimensiones sean las adecuadas para tus patrones
    initialize_glider(&mut framebuffer, 10, 10);  // Inicializa un glider

    // Otras inicializaciones de patrones aquí...

    let mut window = Window::new("Game of Life - Conway", 800, 600, WindowOptions::default()).unwrap();
    while window.is_open() && !window.is_key_down(Key::Escape) {
        framebuffer.update_game_of_life(); // Corrección aquí
        window.update_with_buffer(&framebuffer.buffer, framebuffer.width, framebuffer.height).unwrap();
        std::thread::sleep(std::time::Duration::from_millis(100));  // Ajusta el tiempo de espera para controlar la velocidad de animación
    }
}
