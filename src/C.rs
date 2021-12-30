use std::io;
fn main(){
  let mut input = String::new();
  io::stdin().read_line(&mut input)
    .expect("Failed to read line");
  input = input.trim_end();
  let split = trimmed_input.split(" ");
  let vec: Vec<&str> = split.collect();
  println!("{} {} {}", vec[2], vec[1], vec[0]);
}