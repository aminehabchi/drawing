use raster::{Image,Color};
use rand::Rng;



fn random_between_0_and(max: i32) -> i32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0..max)
}

#[derive(Debug,Clone)]
pub struct RGB{
   pub red   :u8,
   pub green :u8,
   pub blue  :u8,
}

impl RGB{
    pub fn new()->RGB{
        RGB{
            red  :random_between_0_and(255) as  u8,
            blue :random_between_0_and(255) as  u8,
            green:random_between_0_and(255) as  u8,
        }
    }
}

/**************************************************/
pub fn fill_image_with_color(image: &mut Image) {

    let rgb=RGB::new();
    let color = Color::rgb(rgb.red, rgb.green, rgb.blue);

    for y in 0..image.height {
        for x in 0..image.width {
            image.set_pixel(x, y, color.clone()).unwrap();
        }
    }
}

/**************************************************/
#[derive(Debug,Clone)]
pub struct Point{
   pub x :i32,
   pub y :i32,
}

impl Point {
    pub fn new(x :i32, y :i32)->Self{
         Point {
            x,
            y,
        }
    }

    pub fn random(width: i32, height: i32) -> Self { 
        Point {
            x: random_between_0_and(width),
            y: random_between_0_and(height),
        }
    }
}
/**************************************************/
#[derive(Debug)]
pub struct Line{
    pub p1    :Point,
    pub p2    :Point,
    pub color :RGB,
}

impl Line{
    pub fn draw(&self, img: &mut Image) {
        let rgb=RGB::new();
        draw_line(self.p1.x,self.p1.y,self.p2.x,self.p2.y,rgb.clone(),img);
    }


    pub fn random(width :i32, height :i32)->Self{
        Line{
            p1:    Point::random(width,height),
            p2:    Point::random(width,height),
            color: RGB::new(),
        }
    }
}

fn point_pos(x: i32, y: i32, x0: i32, y0: i32, x1: i32, y1: i32) -> i32 {
    // Signed area of the triangle; determines point orientation to line
    (x1 - x0) * (y - y0) - (y1 - y0) * (x - x0)
}

pub fn closest_to_zero(a: i32, b: i32, c: i32) -> i32 {
    let values = [a, b, c];
    *values.iter().min_by_key(|x| x.abs()).unwrap()
}

pub fn draw_line(x0: i32, y0: i32, x1: i32, y1: i32, rgb: RGB, img: &mut Image) {
    let mut move_x = x0;
    let mut move_y = y0;
    let color = Color::rgb(rgb.red, rgb.green, rgb.blue);
    let _ = img.set_pixel(move_x, move_y, color.clone());

    let dir_x = if x0 < x1 { 1 } else { -1 };
    let dir_y = if y0 < y1 { 1 } else { -1 };

    while move_x != x1 || move_y != y1 {
        // Calculate next possible positions
        let next_x = if move_x != x1 { move_x + dir_x } else { move_x };
        let next_y = if move_y != y1 { move_y + dir_y } else { move_y };

        // Check all 3 possible moves
        let pos1 = if move_x != x1 { point_pos(next_x, move_y, x0, y0, x1, y1) } else { i32::MAX };
        let pos2 = if move_y != y1 { point_pos(move_x, next_y, x0, y0, x1, y1) } else { i32::MAX };
        let pos3 = if move_x != x1 && move_y != y1 { point_pos(next_x, next_y, x0, y0, x1, y1) } else { i32::MAX };

        let min = closest_to_zero(pos1, pos2, pos3);

        if min == pos1 {
            move_x = next_x;
        } else if min == pos2 {
            move_y = next_y;
        } else if min == pos3 {
            move_x = next_x;
            move_y = next_y;
        } else {
            // If we can't move in any direction, break to avoid infinite loop
            break;
        }

        let _ = img.set_pixel(move_x, move_y, color.clone());
    }
}

/**************************************************/

#[derive(Debug)]
pub struct Circle {
    pub center: Point,
    pub radius: usize,
}

fn distance(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt()
}

fn closest_point(r: i32, x: i32, y: i32, cx: i32, cy: i32) -> (i32, i32) {
    let a = distance(cx as f64, cy as f64, x as f64+1.0, y as f64);
    let b = distance(cx as f64, cy as f64, x as f64, y as f64+1.0);
    let c = distance(cx as f64, cy as f64, x as f64+1.0, y as f64+1.0);
    
    // Calculate the difference between each distance and the radius
    let diff_a = (a - r as f64).abs();
    let diff_b = (b - r as f64).abs();
    let diff_c = (c - r as f64).abs();
    
    // Return the point with the smallest difference
    if diff_a <= diff_b && diff_a <= diff_c {
        (x + 1, y)
    } else if diff_b <= diff_a && diff_b <= diff_c {
        (x, y + 1)
    } else {
        (x + 1, y + 1)
    }
}


impl Circle {
    pub fn draw(&self, img: &mut Image) {
        let rgb=RGB::new();
        let color = Color::rgb(rgb.red, rgb.green, rgb.blue);

        let cx = self.center.x;
        let cy = self.center.y;
        let mut r = self.radius as i32;

        let mut x = cx;
        let mut y = cy-r;

        while y <= cy && r>0 {

        let  (new_x,new_y)= closest_point(r,x,y,cx,cy);

        if new_x==x && new_y==y{
         break;
        }
        x=new_x;
        y=new_y;
       

        let _ = img.set_pixel(x, y, color.clone()).unwrap_or(());
        let _ = img.set_pixel(x,cy + (cy - y), color.clone()).unwrap_or(());
        let _ = img.set_pixel(2 * cx - x, y, color.clone()).unwrap_or(());
        let _ = img.set_pixel(2 * cx - x, cy + (cy - y),  color.clone()).unwrap_or(());
        }

    }

    pub fn random(width: i32, height: i32) -> Self {
        Circle {
            center: Point {
                x: random_between_0_and(width),
                y: random_between_0_and(height),
            },
            radius: random_between_0_and(width.min(height)) as usize,
        }
    }
}


/**************************************************/
#[derive(Debug,Clone)]
pub struct Triangle {
    pub p1:  Point,
    pub p2:  Point,
    pub p3:  Point,
}

impl Triangle{
    pub fn new(p1: &Point, p2: &Point, p3: &Point) -> Self {
        Triangle { 
            p1:p1.clone(), 
            p2:p2.clone(),
            p3:p3.clone(),
         }
    }

    pub fn draw(&self, img: &mut Image) {
        let rgb=RGB::new();
        draw_line(self.p1.x,self.p1.y,self.p2.x,self.p2.y,rgb.clone(),img);
        draw_line(self.p2.x,self.p2.y,self.p3.x,self.p3.y,rgb.clone(),img);
        draw_line(self.p3.x,self.p3.y,self.p1.x,self.p1.y,rgb.clone(),img);
    }
}

/**************************************************/
pub struct Rectangle{
    pub p1 :Point,
    pub p2 :Point,
}

impl Rectangle{
    pub fn draw(&self ,img :&mut Image){
        let rgb=RGB::new();
        
        draw_line(self.p2.x,self.p2.y,self.p1.x,self.p2.y,rgb.clone(),img);
        draw_line(self.p2.x,self.p1.y*2,self.p1.x,self.p1.y*2,rgb.clone(),img);

        draw_line(self.p2.x,self.p1.y*2,self.p2.x,self.p2.y,rgb.clone(),img);
        draw_line(self.p1.x,self.p1.y*2,self.p1.x,self.p2.y,rgb.clone(),img);
    }

    pub fn new(p1 :&Point, p2 :&Point)->Self{
        Rectangle{
            p1:p1.clone(),
            p2:p2.clone(),
        }
    }
}

pub trait Drawable {
    fn draw(&self, img : &mut Image);
}