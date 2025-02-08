use csgrs::CSG;
use nalgebra::{Point3, Vector3};

/// Representation of a potentiometer module
#[derive(Debug, Clone)]
pub struct Potentiometer {
    pub name: String,
    pub body_diameter: f64,
    pub body_height: f64,
    pub shaft_diameter: f64,
    pub shaft_length: f64,
    pub pin_diameter: f64,
    pub pin_length: f64,
    pub pin_spacing: f64,
}

impl Potentiometer {
    /// Generate the cylindrical body of the potentiometer
    pub fn body(&self) -> CSG<()> {
        CSG::cylinder(self.body_diameter / 2.0, self.body_height)
    }
    
    /// Generate the shaft of the potentiometer
    pub fn shaft(&self) -> CSG<()> {
        CSG::cylinder(self.shaft_diameter / 2.0, self.shaft_length)
            .translate(0.0, 0.0, self.body_height)
    }
    
    /// Generate the pins of the potentiometer
    pub fn pins(&self) -> CSG<()> {
        let mut pins = CSG::new();
        let offsets = [
            (-self.pin_spacing, 0.0),
            (0.0, 0.0),
            (self.pin_spacing, 0.0),
        ];
        
        for &(x, y) in &offsets {
            let pin = CSG::cylinder(self.pin_diameter / 2.0, self.pin_length)
                .translate(x, y, -self.pin_length);
            pins = pins.union(&pin);
        }
        
        pins
    }
    
    /// Generate the full potentiometer model
    pub fn assemble(&self) -> CSG<()> {
        let mut potentiometer = self.body();
        let shaft = self.shaft();
        let pins = self.pins();
        
        potentiometer = potentiometer.union(&shaft);
        potentiometer = potentiometer.union(&pins);
        
        potentiometer
    }
}

