use csgrs::CSG;
use nalgebra::{Point3, Vector3};

/// Representation of a rotary encoder module
#[derive(Debug, Clone)]
pub struct RotaryEncoder {
    pub name: String,
    pub body_diameter: f64,
    pub body_height: f64,
    pub shaft_diameter: f64,
    pub shaft_length: f64,
    pub pin_diameter: f64,
    pub pin_length: f64,
    pub pin_spacing: f64,
}

impl RotaryEncoder {
    /// Generate the cylindrical body of the rotary encoder
    pub fn body(&self) -> CSG<()> {
        CSG::cylinder_z(self.body_diameter / 2.0, self.body_height)
    }
    
    /// Generate the shaft of the rotary encoder
    pub fn shaft(&self) -> CSG<()> {
        CSG::cylinder_z(self.shaft_diameter / 2.0, self.shaft_length)
            .translate(Vector3::new(0.0, 0.0, self.body_height))
    }
    
    /// Generate the pins of the rotary encoder
    pub fn pins(&self) -> CSG<()> {
        let mut pins = CSG::new();
        let offsets = [
            (-self.pin_spacing, 0.0),
            (0.0, 0.0),
            (self.pin_spacing, 0.0),
        ];
        
        for &(x, y) in &offsets {
            let pin = CSG::cylinder_z(self.pin_diameter / 2.0, self.pin_length)
                .translate(Vector3::new(x, y, -self.pin_length));
            pins = pins.union(&pin);
        }
        
        pins
    }
    
    /// Generate the full rotary encoder model
    pub fn assemble(&self) -> CSG<()> {
        let mut encoder = self.body();
        let shaft = self.shaft();
        let pins = self.pins();
        
        encoder = encoder.union(&shaft);
        encoder = encoder.union(&pins);
        
        encoder
    }
}

