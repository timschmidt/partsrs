use csgrs::CSG;
use nalgebra::{Point3, Vector3};

/// Representation of a fuse module
#[derive(Debug, Clone)]
pub struct Fuse {
    pub name: String,
    pub body_length: f64,
    pub body_diameter: f64,
    pub cap_diameter: f64,
    pub cap_height: f64,
}

impl Fuse {
    /// Generate the cylindrical body of the fuse
    pub fn body(&self) -> CSG<()> {
        CSG::cylinder(self.body_diameter / 2.0, self.body_length)
    }
    
    /// Generate the caps of the fuse
    pub fn caps(&self) -> CSG<()> {
        let cap1 = CSG::cylinder(self.cap_diameter / 2.0, self.cap_height)
            .translate(0.0, 0.0, -self.cap_height);
        let cap2 = CSG::cylinder(self.cap_diameter / 2.0, self.cap_height)
            .translate(0.0, 0.0, self.body_length);
        
        cap1.union(&cap2)
    }
    
    /// Generate the full fuse model
    pub fn assemble(&self) -> CSG<()> {
        let mut fuse = self.body();
        let caps = self.caps();
        
        fuse = fuse.union(&caps);
        
        fuse
    }
}

