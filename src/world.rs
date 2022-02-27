use log::{info};

const BOX_COLOR: Rgba = Rgba {r: 0x5e, g: 0x48, b: 0xe8, a: 0xff};
const BACKGROUND_COLOR: Rgba = Rgba {r: 0x48, g: 0xb2, b: 0xe8, a: 0xff};

pub enum Direction { Up, Down, Left, Right }

/// Representation of the application state. In this example, a box will bounce around the screen.
pub struct World {
    box_x: i16,
    box_y: i16,
    box_size: i16,
    width: u32,
    height: u32,
    direction: Direction
}

impl World {
    /// Create a new `World` instance that can draw a moving box.
    pub fn new(width: u32, height: u32, box_size: i16) -> Self {
        Self {
            box_x: 0,
            box_y: 0,
            box_size: box_size,
            width: width,
            height: height,
            direction: Direction::Right
        }
    }

    pub fn set_direction(&mut self, new_direction: Direction) {
        self.direction = new_direction;
    }

    /// Update the `World` internal state; bounce the box around the screen.
    pub fn update(&mut self) {

        // bouncing behavior updates velocity
        /*
        if self.box_x <= 0 || self.box_x + self.box_size > self.width as i16 {
            self.velocity_x *= -1;
        }
        if self.box_y <= 0 || self.box_y + self.box_size > self.height as i16 {
            self.velocity_y *= -1;
        }*/

        // get velocity vector based on current direction
        let (velocity_x, velocity_y) = match &self.direction {
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
            Direction::Up => (0, -1),
        };

        // apply velocity to position
        self.box_x = apply_bounds(0, self.width as i16, (velocity_x) + self.box_x);
        self.box_y = apply_bounds(0, self.height as i16, (velocity_y) + self.box_y);
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
                BOX_COLOR
            } else {
                BACKGROUND_COLOR
            };

            pixel_buffer.copy_from_slice(&rgba.get_buffer());
        }
    }
}

fn apply_bounds(min: i16, max: i16, val: i16) -> i16{
    if val < min
    {
        max - val
    }
    else if val > max
    {
        min + (val-max)
    }
    else
    {
        val
    }
}

struct Rgba {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl Rgba {
    fn get_buffer(&self) -> [u8; 4] {
        [self.r, self.g, self.b, self.a]
    }
}