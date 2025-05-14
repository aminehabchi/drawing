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

impl Drawable for gs::Point {
    fn draw(&self, img: &mut Image) {
        let color = self.color();
        img.set_pixel(self.x, self.y, color.clone()).unwrap_or(());
    }
}

impl Drawable for gs::Line {
    fn draw(&self, img: &mut Image) {
        let rgb = Color{r: 255, g: 255, b:255,a:255};
        draw_line(self.p1.x, self.p1.y, self.p2.x, self.p2.y, rgb.clone(), img);
    }
}

impl Drawable for gs::Circle {
    fn draw(&self, img: &mut Image) {
        let rgb = self.color();

        let cx = self.center.x;
        let cy = self.center.y;
        let mut r = self.radius as i32;

        let mut x = cx;
        let mut y = cy - r;

        while y <= cy && r > 0 {
            let (new_x, new_y) = closest_point(r, x, y, cx, cy);

            if new_x == x && new_y == y {
                break;
            }
            x = new_x;
            y = new_y;

            let _ = img.set_pixel(x, y, rgb.clone()).unwrap_or(());
            let _ = img.set_pixel(x, cy + (cy - y), rgb.clone()).unwrap_or(());
            let _ = img.set_pixel(2 * cx - x, y, rgb.clone()).unwrap_or(());
            let _ = img
                .set_pixel(2 * cx - x, cy + (cy - y), rgb.clone())
                .unwrap_or(());
        }
    }
}

impl Drawable for gs::Triangle {
    fn draw(&self, img : &mut Image) {
        // let rgb= self.color();
        let rgb = Color{r: 255, g: 255, b:255,a:255};
        draw_line(self.p1.x,self.p1.y,self.p2.x,self.p2.y,rgb.clone(),img);
        draw_line(self.p2.x,self.p2.y,self.p3.x,self.p3.y,rgb.clone(),img);
        draw_line(self.p3.x,self.p3.y,self.p1.x,self.p1.y,rgb.clone(),img);
    }
}

impl Drawable for gs::Rectangle {
    fn draw(&self, img : &mut Image) {
        // let rgb= self.color();
                let rgb = Color{r: 255, g: 255, b:255,a:255};

        
        draw_line(self.p2.x,self.p2.y,self.p1.x,self.p2.y,rgb.clone(),img);
        draw_line(self.p2.x,self.p1.y*2,self.p1.x,self.p1.y*2,rgb.clone(),img);

        draw_line(self.p2.x,self.p1.y*2,self.p2.x,self.p2.y,rgb.clone(),img);
        draw_line(self.p1.x,self.p1.y*2,self.p1.x,self.p2.y,rgb.clone(),img);
    }
}