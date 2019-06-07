trait HasArea{
    fn area(&self) -> f64;
}

trait AreaType{
    fn areaType() -> String;
}

struct Circle {
    x : f64,
    y : f64,
    radius : f64
}

struct Rectangle {
    x1 : f64,
    y1 : f64,
    x2 : f64,
    y2 : f64
}



impl HasArea for Circle{
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

impl HasArea for Rectangle{
    fn area(&self) -> f64 {
        (self.x1 - self.x2).abs() * (self.y1 - self.y2).abs()
    }
}

impl AreaType for Circle{
    fn areaType() -> String{
        "Circle".to_string()
    }
}

impl AreaType for Rectangle{
    fn areaType() -> String{
        "Rectangle".to_string()
    }
}

fn print_area< T : HasArea + AreaType>(shape : T){
    println!("This shape is {} and it has an area of {}",T::areaType(),shape.area());
    //T::areaType() 关联函数 与T有关，与self无关
    //shape.area() 对象执行，与self有关
}

fn main() {
    let c = Circle{ x : 0.0, y: 0.0, radius: 1.0};
    let r = Rectangle{ x1 : 0.0, y1: 0.0, x2: 1.0, y2: 1.0 };


    //println!("{:?}", c.area());
    //println!("{:?}", r.area());
    print_area(c);
    print_area(r);
}
