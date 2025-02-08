use csgrs::CSG;
use nalgebra::{Point3, Vector3};

/// Representation of a timing belt
#[derive(Debug, Clone)]
pub struct Belt {
    pub name: String,
    pub length: f64,
    pub width: f64,
    pub thickness: f64,
    pub tooth_pitch: f64,
    pub tooth_height: f64,
}

impl Belt {
    /// Generate the base rectangular body of the belt
    pub fn body(&self) -> CSG<()> {
        CSG::box_shape(self.length, self.width, self.thickness)
    }
    
    /// Generate the teeth along the belt
    pub fn teeth(&self) -> CSG<()> {
        let mut teeth = CSG::new();
        let num_teeth = (self.length / self.tooth_pitch).floor() as i32;
        
        for i in 0..num_teeth {
            let tooth = CSG::box_shape(self.tooth_pitch, self.width, self.tooth_height)
                .translate(i as f64 * self.tooth_pitch - (self.length / 2.0), 0.0, self.thickness);
            teeth = teeth.union(&tooth);
        }
        
        teeth
    }
    
    /// Generate the full belt model
    pub fn assemble(&self) -> CSG<()> {
        let mut belt = self.body();
        let teeth = self.teeth();
        
        belt = belt.union(&teeth);
        
        belt
    }
}

