


use std::fmt::{self};
use rand::Rng;
#[allow(dead_code)]
#[derive(Debug)]
pub enum SensorTypes {
    Humdity,
    Pression,
    Temperature,
    Light
}



impl fmt::Display for SensorTypes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SensorTypes::Humdity     => write!(f, "Humidity"),
            SensorTypes::Pression    => write!(f, "Pression"),
            SensorTypes::Temperature => write!(f, "Temperature"),
            SensorTypes::Light       => write!(f, "Light"),
        }
    }
}

pub struct Humdity{
    pub value: f32,
    pub name: String
}

impl Gen for Humdity{
    fn aleagenval(&mut self) {
        let mut rng = rand::thread_rng();
        self.value = rng.gen_range(0.0..100.0) / 100.0;
    }
}

pub struct Temperature{
    pub value: u32,
    pub name: String
}


impl Gen for Temperature{
    fn aleagenval(&mut self) {
        let mut rng = rand::thread_rng();
        self.value = rng.gen_range(25..=31);
    }
}

pub struct Pression{
    pub value: u32,
    pub name: String
}

impl Gen for Pression{
    fn aleagenval(&mut self) {
        let mut rng = rand::thread_rng();
        self.value = rng.gen_range(7..9);
    }
}

pub struct Light{
    pub value: bool,
    pub name: String
}

impl Gen for Light{
    fn aleagenval(&mut self) {
        let mut rng = rand::thread_rng();
        self.value = rng.gen_range(0..=20) < 10;
    }
}


pub trait Gen {
    fn aleagenval(&mut self);

} 


  
