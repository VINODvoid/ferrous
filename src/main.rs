use std::collections::HashMap;

fn main() {
    let messages = ["rust","is", "fast", "rust", "is", "great", "rust"];
     let mut has_messages:HashMap<String,i32> = HashMap::new();
     for message in messages {
    //   match has_messages.get_mut(message){
    //            Some(count) => *count +=1,
     //           None => { 
                    has_messages.insert(message.to_string(), 1);
     //           }
     //    }
    
   *has_messages.entry(message.to_string()).or_insert(0) +=1;
}

     println!("{:?}",has_messages);
    }
