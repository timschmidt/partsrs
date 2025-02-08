use csgrs::CSG;
use nalgebra::{Point3, Vector3};

/// Representation of an electric motor
#[derive(Debug, Clone)]
pub struct Motor {
    pub name: String,
    pub body_diameter: f64,
    pub body_length: f64,
    pub shaft_diameter: f64,
    pub shaft_length: f64,
    pub mounting_hole_diameter: f64,
    pub mounting_hole_spacing: f64,
}

impl Motor {
    /// Generate the cylindrical body of the motor
    pub fn body(&self) -> CSG<()> {
        CSG::cylinder_z(self.body_diameter / 2.0, self.body_length)
    }
    
    /// Generate the shaft of the motor
    pub fn shaft(&self) -> CSG<()> {
        CSG::cylinder_z(self.shaft_diameter / 2.0, self.shaft_length)
            .translate(Vector3::new(0.0, 0.0, self.body_length))
    }
    
    /// Generate the mounting holes of the motor
    pub fn mounting_holes(&self) -> CSG<()> {
        let mut holes = CSG::new();
        let offsets = [
            (-self.mounting_hole_spacing / 2.0, -self.mounting_hole_spacing / 2.0),
            (self.mounting_hole_spacing / 2.0, -self.mounting_hole_spacing / 2.0),
            (self.mounting_hole_spacing / 2.0, self.mounting_hole_spacing / 2.0),
            (-self.mounting_hole_spacing / 2.0, self.mounting_hole_spacing / 2.0),
        ];
        
        for &(x, y) in &offsets {
            let hole = CSG::cylinder_z(self.mounting_hole_diameter / 2.0, self.body_length)
                .translate(Vector3::new(x, y, 0.0));
            holes = holes.union(&hole);
        }
        
        holes
    }
    
    /// Generate the full motor model
    pub fn assemble(&self) -> CSG<()> {
        let mut motor = self.body();
        let shaft = self.shaft();
        let holes = self.mounting_holes();
        
        motor = motor.union(&shaft);
        motor = motor.subtract(&holes);
        
        motor
    }
}
