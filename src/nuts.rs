use csgrs::CSG;
use nalgebra::{Point3, Vector3};

/// Representation of a hexagonal nut
#[derive(Debug, Clone)]
pub struct Nut {
    pub name: String,
    pub diameter: f64,
    pub thickness: f64,
    pub hole_diameter: f64,
}

impl Nut {
    /// Generate the hexagonal shape of the nut
    pub fn hex_body(&self) -> CSG<()> {
        CSG::hex_prism(self.diameter / 2.0, self.thickness)
    }
    
    /// Generate the hole in the nut
    pub fn hole(&self) -> CSG<()> {
        CSG::cylinder_z(self.hole_diameter / 2.0, self.thickness)
    }
    
    /// Generate the full nut model
    pub fn assemble(&self) -> CSG<()> {
        let mut nut = self.hex_body();
        let hole = self.hole();
        nut = nut.subtract(&hole);
        
        nut
    }
}

