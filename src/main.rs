use std::collections::HashSet;

fn main() {
   let listed = ["rust", "is", "fast", "rust", "is", "great", "rust"];
   let mut unique_keys:HashSet<String> = HashSet::new();
   for list in listed {
   
    unique_keys.insert(list.to_string());
   }
   println!("{:?}",unique_keys);
   
}
