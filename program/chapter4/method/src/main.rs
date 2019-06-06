struct Circle{
    x: f64,
    y: f64,
    radius: f64,
}

impl Circle{
    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }

    fn setbynew(&self,xn:f64,yn:f64,radiusn:f64) -> Circle {
        Circle{ x:xn,y:yn,radius:radiusn }
    }

    fn grow(&self, increment: f64) -> Circle{
        Circle{x: self.x, y: self.y, radius: self.radius + increment}
    }
}

struct CircleBuilder{
    x: f64,
    y: f64,
    radius: f64,
}

impl CircleBuilder{
    fn new()-> CircleBuilder{
        CircleBuilder{x:0.0,y:0.0,radius:1.0}
    }

    fn x(&mut self, coordinate: f64)-> &mut CircleBuilder {
        self.x = coordinate;
        self//return itself
    }

    fn y(&mut self, coordinate: f64)-> &mut CircleBuilder {
        self.y = coordinate;
        self//return itself
    }

    fn radius(&mut self, coordinate: f64)-> &mut CircleBuilder {
        self.radius = coordinate;
        self//return itself
    }

    fn finalize(&self)-> Circle{
        Circle{x: self.x, y:self.y,radius:self.radius}
    }
}


fn main() {
   let c = Circle { x: 0.0, y: 0.0, radius: 2.0};
   
   println!("{}",c.setbynew(3.0,3.0,3.0).area());
   //println!("{}",c.set(3.0,3.0,3.0).grow(2.0).area());

   let c2 = CircleBuilder::new().x(2.0).radius(5.0).finalize();

   println!("{}",c2.area());
}
