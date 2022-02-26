const BOX_PIXEL: Pixel = Pixel {r: 0x5e, g: 0x48, b: 0xe8, a: 0xff};
const BACKGROUND_PIXEL: Pixel = Pixel {r: 0x48, g: 0xb2, b: 0xe8, a: 0xff};

/// Representation of the application state. In this example, a box will bounce around the screen.
pub struct World {
    box_x: i16,
    box_y: i16,
    box_size: i16,
    width: u32,
    height: u32,
    velocity_x: i16,
    velocity_y: i16,
}

impl World {
    /// Create a new `World` instance that can draw a moving box.
    pub fn new(width: u32, height: u32, box_size: i16) -> Self {
        Self {
            box_x: 24,
            box_y: 16,
            box_size: box_size,
            width: width,
            height: height,
            velocity_x: 1,
            velocity_y: 1
        }
    }

    /// Update the `World` internal state; bounce the box around the screen.
    pub fn update(&mut self) {

        // bouncing behavior updates velocity
        if self.box_x <= 0 || self.box_x + self.box_size > self.width as i16 {
            self.velocity_x *= -1;
        }
        if self.box_y <= 0 || self.box_y + self.box_size > self.height as i16 {
            self.velocity_y *= -1;
        }

        // apply velocity to position
        self.box_x += self.velocity_x;
        self.box_y += self.velocity_y;
    }

    /// Draw the `World` state to the frame buffer.
    ///
    /// Assumes the default texture format: `wgpu::TextureFormat::Rgba8UnormSrgb`
    pub fn draw(&self, frame: &mut [u8]) {
        for (i, pixel_buffer) in frame.chunks_exact_mut(4).enumerate() {
            let x = (i % self.width as usize) as i16;
            let y = (i / self.width as usize) as i16;

            let inside_the_box = 
                x >= self.box_x
                    && x < self.box_x + self.box_size
                    && y >= self.box_y
                    && y < self.box_y + self.box_size;

            let rgba = if inside_the_box {
                BOX_PIXEL
            } else {
                BACKGROUND_PIXEL
            };

            pixel_buffer.copy_from_slice(&rgba.get_buffer());
        }
    }
}

struct Pixel {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl Pixel {
    fn get_buffer(&self) -> [u8; 4] {
        [self.r, self.g, self.b, self.a]
    }
}