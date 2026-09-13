struct SquareHole {
    side_length: f64,
}

impl SquareHole {
    fn new(side_length: f64) -> Self {
        SquareHole { side_length }
    }

    fn can_fit(&self, square: &dyn SquareLike) -> bool {
        self.side_length >= square.get_side_length()
    }
}

trait SquareLike {
    fn get_side_length(&self) -> f64;
}

struct Square {
    side_length: f64,
}

impl Square {
    fn new(side_length: f64) -> Self {
        Square { side_length }
    }
}

impl SquareLike for Square {
    fn get_side_length(&self) -> f64 {
        self.side_length
    }
}

struct Circle {
    radius: f64,
}

impl Circle {
    fn new(radius: f64) -> Self {
        Circle { radius }
    }

    fn get_radius(&self) -> f64 {
        self.radius
    }
}

struct CircleToSquareAdapter {
    circle: Circle,
}

impl CircleToSquareAdapter {
    
    fn new(circle: Circle) -> Self {
        CircleToSquareAdapter { circle } 
    }
}

impl SquareLike for CircleToSquareAdapter {
    fn get_side_length(&self) -> f64 {
        self.circle.get_radius() * 2.
    }
}
