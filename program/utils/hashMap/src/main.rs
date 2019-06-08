use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(4,1);
    let b = 4;
    
    match map.get(&b){
        Some(index) =>{
            println!("get index is:{}",index);
        },
        None => println!("no index")
        
    }
    
    println!("get {}",map.contains_key(&b));
}
