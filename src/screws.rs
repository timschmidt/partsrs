use csgrs::CSG;
use nalgebra::{Point3, Vector3};

/// Representation of a screw
#[derive(Debug, Clone)]
pub struct Screw {
    pub name: String,
    pub diameter: f64,
    pub length: f64,
    pub head_diameter: f64,
    pub head_height: f64,
    pub thread_pitch: f64,
}

impl Screw {
    /// Generate a cylindrical screw body
    pub fn body(&self) -> CSG<()> {
        CSG::cylinder(self.diameter / 2.0, self.length)
    }
    
    /// Generate the screw head
    pub fn head(&self) -> CSG<()> {
        CSG::cylinder(self.head_diameter / 2.0, self.head_height)
    }
    
    /// Generate the full screw model
    pub fn assemble(&self) -> CSG<()> {
        let mut screw = self.body();
        let head = self.head().translate(0.0, 0.0, self.length);
        screw = screw.union(&head);
        
        screw
    }
}
