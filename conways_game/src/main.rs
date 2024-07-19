use minifb::{Key, Window, WindowOptions};
use std::time::Duration;
use image::{ImageBuffer, Rgba};
mod framebuffer;
use framebuffer::Framebuffer;

fn render(framebuffer: &mut Framebuffer, x: usize, y: usize) {
    // Establece el color de fondo y limpia el framebuffer
    framebuffer.set_background_color(0x333355);
    framebuffer.clear();

    // Establece el color actual y dibuja un punto en la posición (x, y)
    framebuffer.set_current_color(0xFFDDDD);
    framebuffer.point(x, y);
}

fn main() {
    let window_width = 800;
    let window_height = 600;

    let framebuffer_width = 80;
    let framebuffer_height = 60;

    let frame_delay = Duration::from_millis(16);

    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height);

    let mut window = Window::new(
        "Rust Graphics - Render Loop Example",
        window_width,
        window_height,
        WindowOptions::default(),
    ).unwrap();

    let mut x = 1;
    let mut speed = 1;

    while window.is_open() {
        if window.is_key_down(Key::Escape) {
            break;
        }

        if window.is_key_down(Key::S) {
            let img = ImageBuffer::<Rgba<u8>, Vec<u8>>::from_fn(framebuffer_width as u32, framebuffer_height as u32, |x, y| {
                let color = framebuffer.get_color_at(x as usize, y as usize).unwrap_or(0);
                Rgba([(color >> 16) as u8, (color >> 8) as u8, color as u8, 255])
            });
            img.save("screenshot.bmp").unwrap();
        }

        // Actualiza la posición del punto
        if x == framebuffer_width || x == 0 {
            speed *= -1;
        }
        x = (x as i32 + speed) as usize;

        // Renderiza el frame actual
        render(&mut framebuffer, x, 40);

        // Actualiza la ventana con el contenido del framebuffer
        window.update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height).unwrap();

        // Espera para mantener un framerate constante
        std::thread::sleep(frame_delay);
    }
}
