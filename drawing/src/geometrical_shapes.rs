use raster::{ Image, Color };
use rand::Rng;

pub trait Drawable {
    fn draw(&self, img: &mut Image);

    fn color(&self) -> Color {
        Color {
            r: random_between_0_and(255) as u8,
            g: random_between_0_and(255) as u8,
            b: random_between_0_and(255) as u8,
            a: 255,
        }
    }
}

pub trait Displayable {
    fn display(&mut self, x: i32, y: i32, color: Color);
}

fn random_between_0_and(max: i32) -> i32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0..max)
}
/**************************************************/
// pub fn fill_image_with_color(image: &mut Image) {

//     let rgb=RGB::new();
//     let color = Color::rgb(rgb.red, rgb.green, rgb.blue);

//     for y in 0..image.height {
//         for x in 0..image.width {
//             image.set_pixel(x, y, color.clone()).unwrap();
//         }
//     }
// }

// --------------------------------------- Point impl and Drawable -------------------------------
#[derive(Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
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

impl Drawable for Point {
    fn draw(&self, img: &mut Image) {
        let color = self.color();
        img.display(self.x, self.y, color);
    }
}

// --------------------------------------- Line impl and Drawable -------------------------------
pub struct Line {
    pub p1: Point,
    pub p2: Point,
}

impl Line {
    pub fn new(new_p1: Point, new_p2: Point) -> Self {
        Self {
            p1: new_p1,
            p2: new_p2,
        }
    }
    pub fn random(width: i32, height: i32) -> Self {
        Line {
            p1: Point::random(width, height),
            p2: Point::random(width, height),
        }
    }
}

fn point_pos(x: i32, y: i32, x0: i32, y0: i32, x1: i32, y1: i32) -> i32 {
    // Signed area of the triangle; determines point orientation to line
    (x1 - x0) * (y - y0) - (y1 - y0) * (x - x0)
}

fn closest_to_zero(a: i32, b: i32, c: i32) -> i32 {
    let values = [a, b, c];
    *values
        .iter()
        .min_by_key(|x| x.abs())
        .unwrap()
}

impl Drawable for Line {
    fn draw(&self, img: &mut Image) {
        let color = Color { r: 255, g: 255, b: 255, a: 255 };
        let x0 = self.p1.x;
        let y0 = self.p1.y;
        let x1 = self.p2.x;
        let y1 = self.p2.y;

        let mut move_x = x0;
        let mut move_y = y0;
        img.display(move_x, move_y, color.clone());

        let dir_x = if x0 < x1 { 1 } else { -1 };
        let dir_y = if y0 < y1 { 1 } else { -1 };

        while move_x != x1 || move_y != y1 {
            // Calculate next possible positions
            let next_x = if move_x != x1 { move_x + dir_x } else { move_x };
            let next_y = if move_y != y1 { move_y + dir_y } else { move_y };

            // Check all 3 possible moves
            let pos1 = if move_x != x1 {
                point_pos(next_x, move_y, x0, y0, x1, y1)
            } else {
                i32::MAX
            };
            let pos2 = if move_y != y1 {
                point_pos(move_x, next_y, x0, y0, x1, y1)
            } else {
                i32::MAX
            };
            let pos3 = if move_x != x1 && move_y != y1 {
                point_pos(next_x, next_y, x0, y0, x1, y1)
            } else {
                i32::MAX
            };

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

            img.display(move_x, move_y, color.clone());
        }
    }
}

// --------------------------------------- Circle impl and Drawable -------------------------------
pub struct Circle {
    pub center: Point,
    pub radius: usize,
}

fn distance(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt()
}

fn closest_point(r: i32, x: i32, y: i32, cx: i32, cy: i32) -> (i32, i32) {
    let a = distance(cx as f64, cy as f64, (x as f64) + 1.0, y as f64);
    let b = distance(cx as f64, cy as f64, x as f64, (y as f64) + 1.0);
    let c = distance(cx as f64, cy as f64, (x as f64) + 1.0, (y as f64) + 1.0);

    // Calculate the difference between each distance and the radius
    let diff_a = (a - (r as f64)).abs();
    let diff_b = (b - (r as f64)).abs();
    let diff_c = (c - (r as f64)).abs();

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

impl Drawable for Circle {
    fn draw(&self, img: &mut Image) {
        let color = self.color();

        let cx = self.center.x;
        let cy = self.center.y;
        let r = self.radius as i32;

        let mut x = cx;
        let mut y = cy - r;

        while y <= cy && r > 0 {
            let (new_x, new_y) = closest_point(r, x, y, cx, cy);

            if new_x == x && new_y == y {
                break;
            }
            x = new_x;
            y = new_y;

            img.display(x, y, color.clone());
            img.display(x, cy + (cy - y), color.clone());
            img.display(2 * cx - x, y, color.clone());
            img.display(2 * cx - x, cy + (cy - y), color.clone());
        }
    }
}

// --------------------------------------- Triangle impl and Drawable -------------------------------
pub struct Triangle {
    pub p1: Point,
    pub p2: Point,
    pub p3: Point,
}

impl Triangle {
    pub fn new(p1: &Point, p2: &Point, p3: &Point) -> Self {
        Triangle {
            p1: p1.clone(),
            p2: p2.clone(),
            p3: p3.clone(),
        }
    }
}

impl Drawable for Triangle {
    fn draw(&self, img: &mut Image) {
        Line::new(Point::new(self.p1.x, self.p1.y), Point::new(self.p2.x, self.p2.y)).draw(img);
        Line::new(Point::new(self.p2.x, self.p2.y), Point::new(self.p3.x, self.p3.y)).draw(img);
        Line::new(Point::new(self.p3.x, self.p3.y), Point::new(self.p1.x, self.p1.y)).draw(img);
    }
}

// --------------------------------------- Rectangle impl and Drawable -------------------------------
pub struct Rectangle {
    pub p1: Point,
    pub p2: Point,
}

impl Rectangle {
    pub fn new(p1: &Point, p2: &Point) -> Self {
        Rectangle {
            p1: p1.clone(),
            p2: p2.clone(),
        }
    }
}
impl Drawable for Rectangle {
    fn draw(&self, img: &mut Image) {
        Line::new(Point::new(self.p2.x, self.p2.y), Point::new(self.p1.x, self.p2.y)).draw(img);
        Line::new(Point::new(self.p1.x, self.p1.y), Point::new(self.p1.x, self.p2.y)).draw(img);

        Line::new(Point::new(self.p1.x, self.p1.y), Point::new(self.p2.x, self.p1.y)).draw(img);
        Line::new(Point::new(self.p2.x, self.p2.y), Point::new(self.p2.x, self.p1.y)).draw(img);
    }
}

// --------------------------------------- Cube impl and Drawable -------------------------------
pub struct Cube {
    pub rec1: Rectangle,
    pub rec2: Rectangle,
}

impl Cube {
    pub fn new(rec1: Rectangle, rec2: Rectangle) -> Self {
        Cube {
            rec1: rec1,
            rec2: rec2,
        }
    }
}

impl Drawable for Cube {
    fn draw(&self, img: &mut Image) {
        self.rec1.draw(img);
        self.rec2.draw(img);

        Line::new(
            Point::new(self.rec1.p1.x, self.rec1.p1.y),
            Point::new(self.rec2.p1.x, self.rec2.p1.y)
        ).draw(img);
        Line::new(
            Point::new(self.rec1.p2.x, self.rec1.p2.y),
            Point::new(self.rec2.p2.x, self.rec2.p2.y)
        ).draw(img);

        Line::new(
            Point::new(self.rec1.p2.x, self.rec1.p1.y),
            Point::new(self.rec2.p2.x, self.rec2.p1.y)
        ).draw(img);

        Line::new(
            Point::new(self.rec1.p1.x, self.rec1.p2.y),
            Point::new(self.rec2.p1.x, self.rec2.p2.y)
        ).draw(img);
    }
}
