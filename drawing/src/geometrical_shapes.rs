use rand::Rng;

fn random_between_0_and(max: i32) -> i32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0..max)
}

pub struct Point{
   pub x :i32,
   pub y :i32;
}

impl Point{
    pub fn random(width :i32, height :i32)->Self{ 
        Point{
            x:random_between_0_and(width),
            y:random_between_0_and(height),
        }
    }

    pub draw(&Self ,img :&mut Image){
        println!("draw point");
    }

}
/**************************************************/
pub struct Line{
    pub p1 :Point,
    pub p2 :Point,
}

impl Line{
    pub fn draw(&Self ,img :&mut Image){
         println!("draw line");
    }

    pub fn random(width :i32, height :i32)->Self{
        Line{
            p1:Point{
                x:random_between_0_and(width),
                y:random_between_0_and(height),
            },
            p2:Point{
                x:random_between_0_and(width),
                y:random_between_0_and(height),
            },
        }
    }
}

/**************************************************/
pub struct Circle{
    pub center :Point,
    pub radius :usize;
}

impl Circle{
    pub fn draw(&Self ,img :&mut Image){
         println!("draw circle");
    }

    pub fn random(width :i32, height :i32)->Self{
        Circle{
            center:Point{
                x:random_between_0_and(width),
                y:random_between_0_and(height),
            },
            radius:random_between_0_and(height.max(width)),
        }
    }
}

/**************************************************/
pub struct Triangle{
    pub p1 :Point,
    pub p2 :Point,
    pub p3 :Point,
}

impl Triangle{
    pub fn draw(&Self ,img :&mut Image){
         println!("draw Triangle");
    }

    pub fn new(p1 :&Point, p2 :&Point,p3 :&Point)->Self{
        Triangle{
            p1,
            p2,
            p3,
        }
    }
}

/**************************************************/
pub struct Rectangle{
    pub p1 :Point,
    pub p2 :Point,
}

impl Rectangle{
    pub fn draw(&Self ,img :&mut Image){
         println!("draw Rectangle");
    }

    pub fn new(p1 :&Point, p2 :&Point)->Self{
        Triangle{
            p1,
            p2,
        }
    }
}