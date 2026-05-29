pub(crate) mod note;
use std::{env::args};
use crate::note::{Note};
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
                        }
                        None => println!("write a note to ad "),
                    }
                },
                "search" => {
                    match cmd.get(2){
                        Some(search_word)=> {
                            let content = read_to_string("src/notes.json").unwrap();
                           let notes_vec:Vec<Note> =  serde_json::from_str(&content).unwrap();
                           let mut found = false;
                            for note in notes_vec {
                                if note.name.contains(search_word){
                                    println!("{} | {} | {} | {}", note.id,note.name,note.tag,note.created_at);
                                    found = true 
                                }
                                
                            }
                            if !found {
                            println!(" '{}' unable to find keyword in the notes", search_word);
                            }
                        }
                        None => println!("enter the search word ")
                    }
                    
                },
                "list" => {
                    let content = read_to_string("src/notes.json").unwrap();
                    let notes_vec : Vec<Note> = serde_json::from_str(&content).unwrap();
                    for note in notes_vec {
                        println!("{} | {} | {} | {}",note.id,note.name,note.tag,note.created_at);
                    }
                    
                }
                _ => {
                    println!("wrong arguement : cargo run <command> <sub_command>");
                }
                
            }
        }
        None => println!("unknown command")
    }

  
}
