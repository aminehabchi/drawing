mod geometrical_shapes;

use geometrical_shapes::{self as gs, Triangle};
use gs::{Displayable, Drawable};
use gs::{draw_line, closest_point};
use raster::{Color, Image};

fn main() {
    let mut image = Image::blank(1000, 1000);

    // fill_image_with_color(&mut image);

    
    gs::Line::random(image.width, image.height).draw(&mut image);
    gs::Point::random(image.width, image.height).draw(&mut image);
    
    let rectangle = gs::Rectangle::new(&gs::Point::new(150, 150), &gs::Point::new(50, 50));
    rectangle.draw(&mut image);
    
    let triangle1 = gs::Triangle::new(
            &gs::Point::new(500, 500),
            &gs::Point::new(250, 700),
            &gs::Point::new(700, 800),
        );
    triangle1.draw(&mut image);
        
    for _ in 1..50 {
        gs::Circle::random(image.width, image.height).draw(&mut image);
    }

    raster::save(&image, "image.png").unwrap();
}

impl Displayable for Image {
    fn display(&mut self, x: i32, y: i32, color: Color) {
        if x >= 0 && x < self.width && y >= 0 && y < self.height {
            self.set_pixel(x, y, color).unwrap();
        }
    }
}
