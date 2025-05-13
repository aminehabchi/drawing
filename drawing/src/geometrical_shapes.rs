pub struct Point{
   pub x :i32,
   pub y :i32;
}

pub struct Line{
    pub p1 :Point,
    pub p2 :Point,
}

impl Line{
    pub fn new(p1 :Point,p2 :Point)->Self{
        Line{
            p1,
            p2,
        }
    }
    pub fn draw(img :){

    }
}

pub struct Circle{
    pub center :Point,
    pub r      :usize;
}

pub struct Triangle{
    pub p1 :Point,
    pub p2 :Point,
    pub p3 :Point,
}

pub struct Rectangle{
    pub p1 :Point,
    pub p2 :Point,
}