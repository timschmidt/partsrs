use csgrs::CSG;
use nalgebra::{Point3, Vector3};

/// Representation of a linear rail
#[derive(Debug, Clone)]
pub struct Rail {
    pub name: String,
    pub length: f64,
    pub width: f64,
    pub height: f64,
    pub hole_diameter: f64,
    pub hole_spacing: f64,
}

impl Rail {
    /// Generate the main rectangular body of the rail
    pub fn body(&self) -> CSG<()> {
        CSG::box_shape(self.width, self.height, self.length)
    }
    
    /// Generate the mounting holes along the rail
    pub fn holes(&self) -> CSG<()> {
        let mut holes = CSG::new();
        let num_holes = (self.length / self.hole_spacing).floor() as i32;
        
        for i in 0..num_holes {
            let hole = CSG::cylinder(self.hole_diameter / 2.0, self.height)
                .translate(0.0, 0.0, i as f64 * self.hole_spacing - (self.length / 2.0));
            holes = holes.union(&hole);
        }
        
        holes
    }
    
    /// Generate the full rail model
    pub fn assemble(&self) -> CSG<()> {
        let mut rail = self.body();
        let holes = self.holes();
        
        rail = rail.subtract(&holes);
        
        rail
    }
}

