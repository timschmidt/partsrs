use csgrs::CSG;
use nalgebra::{Point3, Vector3};

/// Representation of an integrated circuit (IC) module
#[derive(Debug, Clone)]
pub struct IC {
    pub name: String,
    pub body_width: f64,
    pub body_height: f64,
    pub body_depth: f64,
    pub pin_diameter: f64,
    pub pin_length: f64,
    pub pin_spacing: f64,
    pub pin_count: u32,
}

impl IC {
    /// Generate the rectangular body of the IC
    pub fn body(&self) -> CSG<()> {
        CSG::prism(self.body_width, self.body_height, self.body_depth)
    }
    
    /// Generate the pins of the IC
    pub fn pins(&self) -> CSG<()> {
        let mut pins = CSG::new();
        let start_x = -((self.pin_count as f64 - 1.0) * self.pin_spacing) / 2.0;
        
        for i in 0..self.pin_count {
            let x_offset = start_x + i as f64 * self.pin_spacing;
            let pin = CSG::cylinder(self.pin_diameter / 2.0, self.pin_length)
                .translate(Vector3::new(x_offset, 0.0, -self.pin_length));
            pins = pins.union(&pin);
        }
        
        pins
    }
    
    /// Generate the full IC model
    pub fn assemble(&self) -> CSG<()> {
        let mut ic = self.body();
        let pins = self.pins();
        
        ic = ic.union(&pins);
        
        ic
    }
}

