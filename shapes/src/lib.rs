use std::f32::consts;

pub trait HasArea<T> {
    fn area(&self) -> T;
}

pub struct Circle<T> where T: From<f32> {
    radius: T,
}

impl<T> Circle<T> 
    where T: num::Num + From<f32>
{
    pub fn new(radius: T) -> Circle<T> {
        Circle { radius }
    }

    pub fn scale(self: &mut Self, scale: T) 
        where T: std::ops::MulAssign
    {
        self.radius *= scale;
    }

    pub fn destroy(self: Self) -> T {
        self.radius
    }
}

impl<T> HasArea<T> for Circle<T> 
    where T: num::Num + Copy + From<f32>
{
    fn area(self: &Self) -> T {
        self.radius * self.radius * consts::PI.into()
    }
}

pub struct Square<T> {
    side: T,
}

impl<T> Square<T> 
    where T: num::Num
{
    pub fn new(side: T) -> Square<T> {
        Square { side }
    }

    pub fn scale(self: &mut Self, scale: T) 
        where T: std::ops::MulAssign
    {
        self.side *= scale;
    }

    pub fn destroy(self: Self) -> T {
        self.side
    }
}

impl<T> HasArea<T> for Square<T> 
    where T: num::Num + Copy
{
    fn area(self: &Self) -> T {
        self.side * self.side 
    }
}

pub enum Shape<T> 
    where T: num::Num + From<f32>
{
    Circle(Circle<T>),
    Square(Square<T>),
}

impl<T> HasArea<T> for Shape<T> 
    where T: num::Num + Copy + From<f32>
{
    fn area(self: &Self) -> T {
        match self {
            Shape::Circle(circle) => circle.area(),
            Shape::Square(square) => square.area(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn squares() {
        let test_square = Square::new(5f64);
        assert_eq!(test_square.area(), 25f64);
        let shape = Shape::Square(test_square);
        assert_eq!(shape.area(), 25f64);
    }

    #[test]
    fn circle() {
        let test_circle = Circle::new(4.0f64);
        assert!((test_circle.area() - 50.265484).abs() < 0.001);
        let shape = Shape::Circle(test_circle);
        assert!((shape.area() - 50.265484).abs() < 0.001);
    }

    #[test]
    fn square_scale() {
        let mut test_square = Square::new(4);
        test_square.scale(2);
        assert_eq!(test_square.area(), 64);
    }

    #[test]
    fn circle_scale() {
        let mut test_circle = Circle::new(4.0f64);
        test_circle.scale(2.0);
        assert!((test_circle.area() - 201.06194).abs() < 0.001);
    }

    #[test]
    fn square_destroy() {
        let test_square = Square::new(4u32);
        assert_eq!(test_square.destroy(), 4u32);
    }

    #[test]
    fn circle_destroy() {
        let test_circle = Circle::new(4.0);
        assert_eq!(test_circle.destroy(), 4.0);
    }
}
