use crate::sensor::{Humdity, Gen};

use crate::scheduler::{Scheduler, Schedule};

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

mod sensor;
mod server;
mod scheduler;

fn main() {
   
    

    
    
    let scheduler_object = Scheduler{going: true};
   
    scheduler_object.do_it();
   

   //scheduler_object.do_it();

    //let _res = server::Server::initiate_file();


    //scheduler::Scheduler::do();
    
    //let serv = server::Server{_name: "g"};
  

    let mut test = Humdity {value: 0.0, name: "a".to_string()};

    test.aleagenval();

   // let _humid: Sensor<f32> = Sensor { value: 5.0 , ty: SensorTypes::Humdity};
   // let _temp:  Sensor<u32> = Sensor { value: 25, ty: SensorTypes::Temperature };
   // let _light: Sensor<bool> = Sensor { value: false, ty: SensorTypes::Light };
   // let _press: Sensor<u32> = Sensor { value: 7, ty: SensorTypes::Pression };
}
