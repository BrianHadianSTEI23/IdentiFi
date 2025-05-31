
pub struct Quadtree{
    top_left : Option<Box<Quadtree>>,
    top_right : Option<Box<Quadtree>>,
    bottom_left : Option<Box<Quadtree>>,
    bottom_right : Option<Box<Quadtree>>,
    x_start : i32,
    x_end : i32,
    y_start : i32,
    y_end : i32,
}

impl Quadtree {
    // constructor
    pub fn new( x_start_new : i32,
            x_end_new : i32,
            y_start_new : i32,
            y_end_new : i32,
    ) -> Self {
        Self {
            top_left : None,
            top_right : None,
            bottom_left : None,
            bottom_right : None,
            x_start : x_start_new,
            x_end : x_end_new,
            y_start : y_start_new,
            y_end : y_end_new,
        }
    }
}