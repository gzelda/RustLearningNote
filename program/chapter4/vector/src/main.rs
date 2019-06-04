fn main() {
    let v1 = vec![1,2,3,4,5];
    let v2 = vec![1;10];

    println!("v1 {} is {} !",3,v1[2]);
    println!("v2 {} is {} !",3,v2[2]);

    for j in 0..10  {
        println!("{}",v2[j]);        
    }

}
