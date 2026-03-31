use std::{env, fs};
use std::fs::OpenOptions;
use std::io::{Write};

fn main() {
      let args: Vec<String> = env::args().collect();
      if args.len() < 2 {
        println!("Type 'help' For Usage" );
        return;
      }
      if args[1] == "help" {
          println!("TodoLIST CLI Usage:
          'add' for adding tasks
          'clear' for delete all tasks
          'list' for listing all tasks
          ");
      } else if args[1] == "add" {
          let text = args[2..].join(" ");
          addtask(&text);
      } else if args[1] == "list" {
          listtasks();
      } else if args[1] == "clear" {
          clearall();
      }

}

fn addtask(task: &str) {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("todo.txt")
        .expect("Failed to open todo.txt");
    writeln!(file, "{}", task.trim()).expect("Failed to write task");
    println!("Task added: {}", task);

}
fn listtasks() {
    let content = fs::read_to_string("todo.txt").expect("There's No Tasks Here");
    for line in content.lines() {
        println!("{}",line);
    }
}
fn clearall() {
    fs::remove_file("todo.txt").expect("Failed To Clear");
}