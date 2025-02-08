use csgrs::CSG;
use nalgebra::{Point3, Vector3};

/// Representation of a push button
#[derive(Debug, Clone)]
pub struct Button {
    pub name: String,
    pub body_diameter: f64,
    pub body_height: f64,
    pub button_diameter: f64,
    pub button_height: f64,
    pub pin_diameter: f64,
    pub pin_length: f64,
    pub pin_spacing: f64,
}

impl Button {
    /// Generate the cylindrical body of the button
    pub fn body(&self) -> CSG<()> {
        CSG::cylinder(self.body_diameter / 2.0, self.body_height)
    }
    
    /// Generate the button cap
    pub fn cap(&self) -> CSG<()> {
        CSG::cylinder(self.button_diameter / 2.0, self.button_height)
            .translate(0.0, 0.0, self.body_height)
    }
    
    /// Generate the pins of the button
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
    
    /// Generate the full button model
    pub fn assemble(&self) -> CSG<()> {
        let mut button = self.body();
        let cap = self.cap();
        let pins = self.pins();
        
        button = button.union(&cap);
        button = button.union(&pins);
        
        button
    }
}
