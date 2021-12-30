#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
use std::io;
fn main(){
  let mut input_array:Vec<String> = Vec::new();
  let mut largest_string:usize = 0;
  for i in 0..5 {
    let mut input:String = String::new();
    match io::stdin().read_line(&mut input){
      Ok(_) => {
        input_array.push(input.trim_end().to_string());
        if input.len() > largest_string {
          largest_string = input.len();
        }
      },
      Err(_) => {
        println!("Failed to read line");
      }
    }
    input_array[i] = input_array[i].trim_end().to_string();
  }
  for i in 0..5 {
    println!("{0:>width$}|{0}", input_array[i], width = largest_string-2);
    // println!("{}", input_array[i]);
  }
  

}