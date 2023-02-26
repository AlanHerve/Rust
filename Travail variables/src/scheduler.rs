use std::{thread, time};
use crate::server::{ self, ServerTrait};
use crate::sensor::{*, self};



pub struct Scheduler {
    pub going: bool
}


#[warn(while_true)]
impl Schedule for Scheduler {
    fn do_it(&self) {

        let mut humidity_object = sensor::Humdity        {value: 0.36,  name: "Humidity".to_string()};
        let mut pression_object = sensor::Pression       {value: 0,     name: "Pression".to_string()};
        let mut temperature_object = sensor::Temperature {value: 0,     name: "Temperature".to_string()};
        let mut light_object = sensor::Light             {value: false, name: "Light".to_string()};

        let interval = time::Duration::from_millis(1000);

        while self.going {

            humidity_object.aleagenval();
            pression_object.aleagenval();
            temperature_object.aleagenval();
            light_object.aleagenval();

            server::Server::console_write(humidity_object.name.clone(),    humidity_object.value.to_string());
            server::Server::console_write(pression_object.name.clone(),    pression_object.value.to_string());
            server::Server::console_write(temperature_object.name.clone(), temperature_object.value.to_string());
            server::Server::console_write(light_object.name.clone(),       light_object.value.to_string());

            println!("--------------------------------------------------------------------------------------");

            let mut _res2 = server::Server::file_write(humidity_object.name.clone(),    humidity_object.value.to_string());
                    _res2 = server::Server::file_write(pression_object.name.clone(),    pression_object.value.to_string());
                    _res2 = server::Server::file_write(temperature_object.name.clone(), temperature_object.value.to_string());
                    _res2 = server::Server::file_write(light_object.name.clone(),       light_object.value.to_string());

            

            thread::sleep(interval);
        
        }
    }

    
}

pub trait Schedule {
    fn do_it(&self);
    
}

