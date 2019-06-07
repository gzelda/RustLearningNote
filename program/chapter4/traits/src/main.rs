//泛型trait的用法

trait ConvertTo<T>{//ConvertTo<T> 接口名称
    fn convert(&self) -> T;
}



impl ConvertTo<i64> for i32 { //实现接口，同时实现泛型实例
    fn convert(&self) -> i64 {//实现了 目标对象为i32的对象 -> i64 的方法
        *self as i64
    }
}

impl ConvertTo<i32> for i64 { //实现接口，同时实现泛型实例
    fn convert(&self) -> i32 {//实现了 目标对象为i64的对象 -> i32 的方法
        *self as i32
    }
}


fn normal <T: ConvertTo<i64>>( x : &T ) -> i64  {
    x.convert()
}

fn inverse<T>(x:i32)-> T
    where i32: ConvertTo<T>{//i32必须实现ConvertTo<T>   
        x.convert()         //因为x为i32 所以找到了ConvertTo<i64> =》 所以T为i64？？
}

trait PrintType{
    fn type_of(&self) -> &str;
}

impl PrintType for i32{
    fn type_of(&self) -> &str{
        "i32"
    }
}

impl PrintType for i64{
    fn type_of(&self) -> &str{
        "i64"
    }
}

fn main() {
    let a : i32 = 5;
    let b : i64 = 5;

    println!("{}",a.type_of());
    //println!("{}",a.convert().type_of());

    //println!("{}",b.type_of());
    //println!("{}",b.convert().type_of());

    
    println!("{}",inverse(a).type_of());
    //这里不使用借用，直接使用a，后面没有报use moved value的错误！ 疑问

    println!("{}",normal(&a).type_of());
    //println!("a : {}",a.convert());
}
