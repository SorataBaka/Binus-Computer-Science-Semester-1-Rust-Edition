use std::io;
fn main(){
  let mut input:String = String::new();
  io::stdin().read_line(&mut input)
    .expect("Failed to read line");
  let input = input.trim_end();
  let input:u128 = input.parse().unwrap();
  println!("{:x}", input);
}