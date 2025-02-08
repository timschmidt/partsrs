use csgrs::CSG;
use nalgebra::{Point3, Vector3};

/// Representation of a washer
#[derive(Debug, Clone)]
pub struct Washer {
    pub name: String,
    pub outer_diameter: f64,
    pub inner_diameter: f64,
    pub thickness: f64,
}

impl Washer {
    /// Generate the outer ring of the washer
    pub fn outer_ring(&self) -> CSG<()> {
        CSG::cylinder(self.outer_diameter / 2.0, self.thickness)
    }
    
    /// Generate the hole in the washer
    pub fn hole(&self) -> CSG<()> {
        CSG::cylinder(self.inner_diameter / 2.0, self.thickness)
    }
    
    /// Generate the full washer model
    pub fn assemble(&self) -> CSG<()> {
        let mut washer = self.outer_ring();
        let hole = self.hole();
        washer = washer.subtract(&hole);
        
        washer
    }
}
