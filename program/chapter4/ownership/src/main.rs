fn main() {
    let mut a = 2;

    let _y = double(a);//默认 copy了？？？？？？？
    println!("{}",a);//no error now 
}

fn double(x:i32)->i32{
    x * 2
}
