

mod note;

enum Command{
    Add { content:String, tag:String},
    List ,
    Search {keyword:String},
    Tag {name:String},
}

fn parse_args(args:Vec<String>)-> Command{
    match args[1].as_str() {
        "add" => Command::Add { content: args[2].clone(), tag: args[4].clone() },
        "list" => Command::List,
        "search" => Command::Search { keyword: args[2].clone()},
        "tag" => Command::Tag { name: args[2].clone()},
        _ => panic!("Unknown command"),
    }
}
fn main(){
    let args:Vec<String> =  std::env::args().collect();
    let results = parse_args(args);
    match results {
        results.Command::List {
            println!("Adding note:...");
        }
    }
}
