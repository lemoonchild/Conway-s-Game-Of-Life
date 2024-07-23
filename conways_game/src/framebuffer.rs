pub struct Framebuffer {
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<u32>,
    background_color: u32,
    current_color: u32,
}


impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Framebuffer {
        let background_color = 0x000000; // Negro por defecto
        let current_color = 0x008892c6; 
        let buffer = vec![background_color; width * height];
        Framebuffer {
            width,
            height,
            buffer,
            background_color,
            current_color,
        }
    }

    pub fn clear(&mut self) {
        self.buffer.fill(self.background_color);
    }

    pub fn point(&mut self, x: usize, y: usize) {
        if x < self.width && y < self.height {
            let index = y * self.width + x;
            self.buffer[index] = self.current_color;
        }
    }

    pub fn get_color_at(&self, x: usize, y: usize) -> Option<u32> {
        if x < self.width && y < self.height {
            let index = y * self.width + x;
            Some(self.buffer[index])
        } else {
            None
        }
    }

    pub fn set_background_color(&mut self, color: u32) {
        self.background_color = color;
    }

    pub fn set_current_color(&mut self, color: u32) {
        self.current_color = color;
    }

    fn is_alive(&self, x: usize, y: usize) -> bool {
        self.get_color_at(x, y).unwrap_or(0) == self.current_color
    }

    fn live_neighbor_count(&self, x: usize, y: usize) -> usize {
        let mut count = 0;
        for i in -1..=1 {
            for j in -1..=1 {
                if !(i == 0 && j == 0) {
                    let nx = x as i32 + i;
                    let ny = y as i32 + j;
                    if nx >= 0 && nx < self.width as i32 && ny >= 0 && ny < self.height as i32 {
                        if self.is_alive(nx as usize, ny as usize) {
                            count += 1;
                        }
                    }
                }
            }
        }
        count
    }

    pub fn update_game_of_life(&mut self) {
        let mut new_buffer = self.buffer.clone();
        for y in 0..self.height {
            for x in 0..self.width {
                let alive = self.is_alive(x, y);
                let count = self.live_neighbor_count(x, y);

                let index = y * self.width + x;
                new_buffer[index] = match (alive, count) {
                    (true, 2) | (true, 3) => self.current_color, // Sobrevive
                    (false, 3) => self.current_color,           // Nace
                    _ => self.background_color,                 // Muere
                };
            }
        }
        self.buffer = new_buffer;
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_framebuffer_initialization() {
        let fb = Framebuffer::new(100, 100);
        assert_eq!(fb.width, 100);
        assert_eq!(fb.height, 100);
        assert_eq!(fb.background_color, 0x000000);
        assert_eq!(fb.current_color, 0xFFFFFF);
        assert!(fb.buffer.iter().all(|&color| color == 0x000000));
    }

    #[test]
    fn test_clear() {
        let mut fb = Framebuffer::new(10, 10);
        fb.set_background_color(0xFF0000); 
        fb.clear();
        assert!(fb.buffer.iter().all(|&color| color == 0xFF0000));
    }

    #[test]
    fn test_point() {
        let mut fb = Framebuffer::new(10, 10);
        fb.set_current_color(0x00FF00); 
        fb.point(5, 5);
        assert_eq!(fb.get_color_at(5, 5).unwrap(), 0x00FF00);
        assert_eq!(fb.get_color_at(0, 0).unwrap(), 0x000000); 
    }

    #[test]
    fn test_get_color_at() {
        let fb = Framebuffer::new(10, 10);
        assert!(fb.get_color_at(10, 10).is_none());
        assert!(fb.get_color_at(0, 0).is_some());
    }

    #[test]
    fn test_set_background_color() {
        let mut fb = Framebuffer::new(10, 10);
        fb.set_background_color(0x123456);
        assert_eq!(fb.background_color, 0x123456);
    }

    #[test]
    fn test_set_current_color() {
        let mut fb = Framebuffer::new(10, 10);
        fb.set_current_color(0x654321);
        assert_eq!(fb.current_color, 0x654321);
    }
}
