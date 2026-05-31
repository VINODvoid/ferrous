use std::fs::read_to_string;

fn read_note(path:&str)-> Result<String,std::io::Error>
{
    let content  = read_to_string(path)?;
    Ok(content)
        }

fn main(){

    match read_note("./sr/note.rs"){
        Ok(content) => println!("{}",content),
        Err(w) => println!("Error reading file {}",w)

    }
}
