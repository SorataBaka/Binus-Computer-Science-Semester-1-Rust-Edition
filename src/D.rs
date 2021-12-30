use std::io;
fn main(){
  let mut nim = String::new();
  let mut name = String::new();
  let mut class_num = String::new();
  io::stdin().read_line(&mut nim)
    .expect("Failed to read line");
  io::stdin().read_line(&mut name)
    .expect("Failed to read line");
  io::stdin().read_line(&mut class_num)
    .expect("Failed to read line");
  let nim = nim.trim_end();
  let name = name.trim_end();
  let class_num = class_num.trim_end();
  let class_num:Vec<&str> = class_num.split(" ").collect();
  println!("Id    : {}", nim);
  println!("Name  : {}", name);
  println!("Class : {}", class_num[0]);
  println!("Num   : {}", class_num[1]);

}