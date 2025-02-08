use csgrs::CSG;
use nalgebra::{Point3, Vector3};

/// Representation of a toggle switch module
#[derive(Debug, Clone)]
pub struct ToggleSwitch {
    pub name: String,
    pub body_width: f64,
    pub body_height: f64,
    pub body_depth: f64,
    pub lever_diameter: f64,
    pub lever_length: f64,
    pub pin_diameter: f64,
    pub pin_length: f64,
    pub pin_spacing: f64,
}

impl ToggleSwitch {
    /// Generate the rectangular body of the toggle switch
    pub fn body(&self) -> CSG<()> {
        CSG::prism(self.body_width, self.body_height, self.body_depth)
    }
    
    /// Generate the lever of the toggle switch
    pub fn lever(&self) -> CSG<()> {
        CSG::cylinder(self.lever_diameter / 2.0, self.lever_length)
            .translate(Vector3::new(0.0, 0.0, self.body_depth))
    }
    
    /// Generate the pins of the toggle switch
    pub fn pins(&self) -> CSG<()> {
        let mut pins = CSG::new();
        let offsets = [
            (-self.pin_spacing, 0.0),
            (0.0, 0.0),
            (self.pin_spacing, 0.0),
        ];
        
        for &(x, y) in &offsets {
            let pin = CSG::cylinder(self.pin_diameter / 2.0, self.pin_length)
                .translate(Vector3::new(x, y, -self.pin_length));
            pins = pins.union(&pin);
        }
        
        pins
    }
    
    /// Generate the full toggle switch model
    pub fn assemble(&self) -> CSG<()> {
        let mut toggle_switch = self.body();
        let lever = self.lever();
        let pins = self.pins();
        
        toggle_switch = toggle_switch.union(&lever);
        toggle_switch = toggle_switch.union(&pins);
        
        toggle_switch
    }
}

