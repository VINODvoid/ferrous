struct User {
        name:String,
        age:u8,
        email:String,
    }

impl  User {
    fn greet(&self){
        println!("Hello {}", self.name)
    }
}
enum Direction {
    North,
    South,
    East,
    West
}
enum Message{
    Quit,
    Move(i32,i32),
    Write(String), 
}
    
fn main(){
    let mut user = User{
        name:String::from("Kalki"),
        age:26,
        email:String::from("kalki@gmail.com")
    };
    println!("{} and {} and {}",user.name,user.email,user.age);
    user.greet();
    user.age = 65;
    println!("{}",user.age);

    let direction = Direction::North;
    match direction {
        Direction::North => println!("Going North"),
        Direction::South => println!("Going South!"),
        Direction::East => println!("Going East!"),
        Direction::West => println!("Going West!"),
    }
    let msg = Message::Move(10,20);
    match msg {
        Message::Quit => println!("Quit"),
      Message::Move (x, y ) => println!("Move to {} {}", x, y),
      Message::Write(text) => println!("{}", text),
    }

}
