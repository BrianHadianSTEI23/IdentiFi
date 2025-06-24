
pub struct Quadtree{
    pub top_left : Option<Box<Quadtree>>,
    pub top_right : Option<Box<Quadtree>>,
    pub bottom_left : Option<Box<Quadtree>>,
    pub bottom_right : Option<Box<Quadtree>>,
    pub x_start : u32,
    pub x_end : u32,
    pub y_start : u32,
    pub y_end : u32,
}

impl Quadtree {
    // constructor
    pub fn new( x_start_new : u32,
            x_end_new : u32,
            y_start_new : u32,
            y_end_new : u32,
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