use csgrs::CSG;
use nalgebra::{Point3, Vector3};

/// Representation of a buzzer module
#[derive(Debug, Clone)]
pub struct Buzzer {
    pub name: String,
    pub body_diameter: f64,
    pub body_height: f64,
    pub sound_hole_diameter: f64,
    pub pin_diameter: f64,
    pub pin_length: f64,
    pub pin_spacing: f64,
}

impl Buzzer {
    /// Generate the cylindrical body of the buzzer
    pub fn body(&self) -> CSG<()> {
        CSG::cylinder(self.body_diameter / 2.0, self.body_height)
    }
    
    /// Generate the sound hole of the buzzer
    pub fn sound_hole(&self) -> CSG<()> {
        CSG::cylinder(self.sound_hole_diameter / 2.0, self.body_height)
    }
    
    /// Generate the pins of the buzzer
    pub fn pins(&self) -> CSG<()> {
        let mut pins = CSG::new();
        let offsets = [
            (-self.pin_spacing / 2.0, 0.0),
            (self.pin_spacing / 2.0, 0.0),
        ];
        
        for &(x, y) in &offsets {
            let pin = CSG::cylinder(self.pin_diameter / 2.0, self.pin_length)
                .translate(Vector3::new(x, y, -self.pin_length));
            pins = pins.union(&pin);
        }
        
        pins
    }
    
    /// Generate the full buzzer model
    pub fn assemble(&self) -> CSG<()> {
        let mut buzzer = self.body();
        let sound_hole = self.sound_hole();
        let pins = self.pins();
        
        buzzer = buzzer.subtract(&sound_hole);
        buzzer = buzzer.union(&pins);
        
        buzzer
    }
}

