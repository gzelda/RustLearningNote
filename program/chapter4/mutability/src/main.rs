use std::sync::Arc;


fn use1(a : i32){
    println!("use it and a:{}",a);
}

fn change1(a : i32) -> i32{
    
    println!("change it");
    a + 1
}


fn main() {
    let mut x = 5;//Arc::new(5);
   // {
    let y = &mut x;//x.clone();
    //println!("X is :{}",x);
    *y += 1;
    //} NO NEED NOW
    println!("X is :{}",x);
    //let mut z = x.clone();

    let a = 5;
    use1(a);
    let b = change1(a);
    println!("a is :{}",a);
    use1(b);
    let c = a;
    use1(c);
    
}
