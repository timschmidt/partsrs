use csgrs::CSG;
use nalgebra::{Point3, Vector3};

/// Representation of a battery module
#[derive(Debug, Clone)]
pub struct Battery {
    pub name: String,
    pub body_width: f64,
    pub body_height: f64,
    pub body_depth: f64,
    pub terminal_diameter: f64,
    pub terminal_height: f64,
    pub terminal_spacing: f64,
}

impl Battery {
    /// Generate the rectangular body of the battery
    pub fn body(&self) -> CSG<()> {
        CSG::prism(self.body_width, self.body_height, self.body_depth)
    }
    
    /// Generate the terminals of the battery
    pub fn terminals(&self) -> CSG<()> {
        let mut terminals = CSG::new();
        let offsets = [
            (-self.terminal_spacing / 2.0, 0.0),
            (self.terminal_spacing / 2.0, 0.0),
        ];
        
        for &(x, y) in &offsets {
            let terminal = CSG::cylinder(self.terminal_diameter / 2.0, self.terminal_height)
                .translate(x, y, self.body_depth);
            terminals = terminals.union(&terminal);
        }
        
        terminals
    }
    
    /// Generate the full battery model
    pub fn assemble(&self) -> CSG<()> {
        let mut battery = self.body();
        let terminals = self.terminals();
        
        battery = battery.union(&terminals);
        
        battery
    }
}

