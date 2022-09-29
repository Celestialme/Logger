#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
use std::fs::OpenOptions;
use std::fs::File;
use std::sync::Mutex;
use std::io::{Write, BufReader,BufRead};
use chrono::{Local};
#[macro_use]
extern crate lazy_static;
lazy_static! {
  static   ref   LOG_FILE:Mutex<File> = Mutex::new(OpenOptions::new()
  .create(true)
  .append(true)
  .open(FILE_PATH)
  .unwrap());
}
const FILE_PATH:&str = "../journal.log";
#[tauri::command]
fn write_log(log:String){
  let mut log_file =  LOG_FILE.lock().unwrap();

  writeln!(log_file,"{}",log).unwrap();
}

fn is_unique(date:&str) -> bool{
  let log_file = File::open(FILE_PATH).unwrap();
  let reader = BufReader::new(log_file);

  for line in reader.lines() {
      if line.unwrap() == date{
        return false
      }
  }

return true
}





#[tauri::command]
fn read_logs() ->String{

  let contents = std::fs::read_to_string(FILE_PATH)
  .expect("Something went wrong reading the file");

  contents
}

fn main() {
 let date_time = format!("------ {} ------",Local::now().format("%d/%m/%Y"));
 { 
 let mut log_file =  LOG_FILE.lock().unwrap();
  if is_unique(&date_time){

    writeln!(log_file,"{}",date_time).unwrap();
  }
 }
  tauri::Builder::default()
  .invoke_handler(tauri::generate_handler![write_log,read_logs])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
