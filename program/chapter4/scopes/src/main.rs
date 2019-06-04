struct Foo<'a>{
    x: &'a i32,
}
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


fn main() {


    f1();
    f2();
}
