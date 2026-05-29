pub(crate) mod note;
use std::{env::args};
use crate::note::{Command, Note};
use std::fs::{read_to_string,write};

fn main() {
    let cmd:Vec<String> = args().collect();
    match cmd.get(1){
        Some(sub_cmd) => {
            match sub_cmd.as_str() {
                "add" =>{
                    match cmd.get(2) {
                        Some(note) => {
                            let content = read_to_string("src/notes.json").unwrap_or("[]".to_string());
                            let mut notes_vec:Vec<Note> = serde_json::from_str(&content).unwrap();
                            let new_note = Note{
                                id:( notes_vec.len()+1 ) as u32,
                             tag:"testing".to_string(),
                             name:note.to_string(),
                             created_at:"2026-05-29".to_string()
                            };
                            notes_vec.push(new_note);
                            let  json = serde_json::to_string_pretty(&notes_vec).unwrap();
                            write("src/notes.json",json).unwrap();
                                 // match content {
                                //  Ok(data) => {


                                //   }
                                  //  Err(_) => println("File does not exist !")
                                //  }




                        }
                        None => println!("write a note to ad "),
                    }
                    
                },
                "search" => {
                    match cmd.get(2){
                        Some(search_word)=> {
                            let search_cmd = Command::Search { name: search_word.to_string() };
                            print!("searching for keyword:> {:?}",search_cmd);
                        }
                        None => println!("enter the search word ")

                    }
                    
                },
                "list" => {
                    println!("This is the list of notes : ");
                }
                _ => {
                    println!("wrong arguement : cargo run <command> <sub_command>");
                }
                
            }
        }
        None => println!("unknown command")
    }

  
}
