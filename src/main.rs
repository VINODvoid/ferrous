



fn main(){
    let is_even = |n| n % 2 == 0 ;
    let double = |n| n * 2 ;


    let greet = |name| format!("hello {}",name);
    println!("{}",is_even(2));
    println!("{}",double(2));
    println!("{}",greet("vinod"));

}
