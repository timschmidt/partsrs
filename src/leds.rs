use csgrs::CSG;
use nalgebra::{Point3, Vector3};

/// Representation of an LED component
#[derive(Debug, Clone)]
pub struct LED {
    pub name: String,
    pub body_diameter: f64,
    pub body_height: f64,
    pub pin_diameter: f64,
    pub pin_length: f64,
    pub pin_spacing: f64,
}

impl LED {
    /// Generate the cylindrical body of the LED
    pub fn body(&self) -> CSG<()> {
        CSG::cylinder(self.body_diameter / 2.0, self.body_height)
    }
    
    /// Generate the pins of the LED
    pub fn pins(&self) -> CSG<()> {
        let mut pins = CSG::new();
        let offsets = [
            (-self.pin_spacing / 2.0, 0.0),
            (self.pin_spacing / 2.0, 0.0),
        ];
        
        for &(x, y) in &offsets {
            let pin = CSG::cylinder(self.pin_diameter / 2.0, self.pin_length)
                .translate(x, y, -self.pin_length);
            pins = pins.union(&pin);
        }
        
        pins
    }
    
    /// Generate the full LED model
    pub fn assemble(&self) -> CSG<()> {
        let mut led = self.body();
        let pins = self.pins();
        
        led = led.union(&pins);
        
        led
    }
}

