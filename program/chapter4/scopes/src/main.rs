struct Foo<'a>{
    x: &'a i32,
}

static STATIC: i32 = 666;

fn f1(){

    let y = &5;
    let f = Foo{ x: y };
    println!("f.x is {}",f.x);
}

fn f2(){
    
    
    let x;
    //{ if there is, it will be a scope problem
        let y = &5;
        let f = Foo{ x: y };
        x = &f.x;
    //}

    println!("f.x is {}",x);
}

impl<'a> Foo<'a>{
    fn x(self) -> &'a i32 {self.x}
    //self or &self
}

fn f3(){
    let y = &5;

    let f = Foo{ x: y};

    println!("f.x() is :{}", f.x());
    println!("static is : {}",STATIC);
}

fn main() {


    f1();
    f2();
    f3();
    
}
