use csgrs::CSG;
use nalgebra::{Point3, Vector3};

/// Representation of a pulley
#[derive(Debug, Clone)]
pub struct Pulley {
    pub name: String,
    pub outer_diameter: f64,
    pub inner_diameter: f64,
    pub width: f64,
    pub groove_depth: f64,
    pub groove_count: u32,
}

impl Pulley {
    /// Generate the outer cylindrical body of the pulley
    pub fn outer_body(&self) -> CSG<()> {
        CSG::cylinder(self.outer_diameter / 2.0, self.width)
    }
    
    /// Generate the inner hole of the pulley
    pub fn hole(&self) -> CSG<()> {
        CSG::cylinder(self.inner_diameter / 2.0, self.width)
    }
    
    /// Generate the grooves on the pulley
    pub fn grooves(&self) -> CSG<()> {
        let mut grooves = CSG::new();
        let spacing = self.width / (self.groove_count as f64 + 1.0);
        
        for i in 1..=self.groove_count {
            let groove = CSG::cylinder((self.outer_diameter / 2.0) - self.groove_depth, spacing / 2.0)
                .translate(0.0, 0.0, i as f64 * spacing - (self.width / 2.0));
            grooves = grooves.union(&groove);
        }
        
        grooves
    }
    
    /// Generate the full pulley model
    pub fn assemble(&self) -> CSG<()> {
        let mut pulley = self.outer_body();
        let hole = self.hole();
        let grooves = self.grooves();
        
        pulley = pulley.subtract(&hole);
        pulley = pulley.subtract(&grooves);
        
        pulley
    }
}
