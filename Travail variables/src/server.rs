use std::fs::OpenOptions;
use std::io::{Write, ErrorKind};
use std::{fs::File};




static PREFIX: &str = "Log.txt";

pub struct Server {
    
    
 }

pub trait ServerTrait{
    fn console_write(name: String, val:String);
    fn initiate_file()                         -> std::io::Result<()>;
    fn file_write(arg: String, val: String)    -> std::io::Result<()>;
}

impl ServerTrait for Server {
    fn console_write(name: String, val: String){
        println!("Printing {} : {}", name, val);
    }

    fn initiate_file() -> std::io::Result<()>{
        let mut _file = File::create("HumidityLog.txt").unwrap();
                _file = File::create("PressionLog.txt")?;
                _file = File::create("LightLog.txt")?;
                _file = File::create("TemperatureLog.txt")?;
                
        Ok(())
    }

    fn file_write(arg: String, val: String) -> std::io::Result<()> {

        let sufix: String = String::from(arg);
        let file_name : String = sufix + PREFIX;
        let  file = OpenOptions::new().append(true).open(file_name.clone());

        print!("hey!");

        let mut file2  = match file{
            Ok(file) => file,
            Err(error) => match error.kind() {
                ErrorKind::NotFound => match File::create(file_name) {
                Ok(fc) => fc,
                Err(e) => panic!("problem creating the file {:?}", e)
                },
                other_error=>{
                panic!("problem creating the file {:?}", other_error)
                }
            }
        };

        file2.write_all(val.as_bytes()).expect("Something went wrong opening the file");
        let _res = file2.write_all(b"\n");

    

        
        
        Ok(())
    }
}