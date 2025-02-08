use csgrs::CSG;
use nalgebra::{Point3, Vector3};

/// Representation of an electrical switch
#[derive(Debug, Clone)]
pub struct Switch {
    pub name: String,
    pub body_width: f64,
    pub body_height: f64,
    pub body_depth: f64,
    pub button_diameter: f64,
    pub button_height: f64,
    pub terminal_diameter: f64,
    pub terminal_spacing: f64,
}

impl Switch {
    /// Generate the rectangular body of the switch
    pub fn body(&self) -> CSG<()> {
        CSG::prism(self.body_width, self.body_height, self.body_depth)
    }
    
    /// Generate the button of the switch
    pub fn button(&self) -> CSG<()> {
        CSG::cylinder(self.button_diameter / 2.0, self.button_height)
            .translate(Vector3::new(0.0, 0.0, self.body_depth))
    }
    
    /// Generate the terminals of the switch
    pub fn terminals(&self) -> CSG<()> {
        let mut terminals = CSG::new();
        let offsets = [
            (-self.terminal_spacing / 2.0, 0.0),
            (self.terminal_spacing / 2.0, 0.0),
        ];
        
        for &(x, y) in &offsets {
            let terminal = CSG::cylinder(self.terminal_diameter / 2.0, self.body_depth / 2.0)
                .translate(Vector3::new(x, y, -self.body_depth / 2.0));
            terminals = terminals.union(&terminal);
        }
        
        terminals
    }
    
    /// Generate the full switch model
    pub fn assemble(&self) -> CSG<()> {
        let mut switch = self.body();
        let button = self.button();
        let terminals = self.terminals();
        
        switch = switch.union(&button);
        switch = switch.union(&terminals);
        
        switch
    }
}
