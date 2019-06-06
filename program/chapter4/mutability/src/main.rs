use std::sync::Arc;

fn main() {
    let mut x = 5;//Arc::new(5);
   // {
    let y = &mut x;//x.clone();
    //println!("X is :{}",x);
    *y += 1;
    //} NO NEED NOW
    println!("X is :{}",x);
    //let mut z = x.clone();
    
}
