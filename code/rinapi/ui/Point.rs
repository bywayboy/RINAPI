use super::super::prelude::{
    LONG
};

#[repr(C)]
pub struct Point {
    pub x : LONG , 
    pub y : LONG ,
}

pub type POINT = Point;

impl Point {
	pub fn new(nx : LONG , ny : LONG) -> Point {
		Point {
			x : nx , 
			y : ny ,
		}
	}
}