use csgrs::CSG;
use nalgebra::{Point3, Vector3};

/// Representation of a joystick module
#[derive(Debug, Clone)]
pub struct Joystick {
    pub name: String,
    pub base_diameter: f64,
    pub base_height: f64,
    pub stick_diameter: f64,
    pub stick_length: f64,
    pub top_diameter: f64,
    pub top_height: f64,
}

impl Joystick {
    /// Generate the cylindrical base of the joystick
    pub fn base(&self) -> CSG<()> {
        CSG::cylinder_z(self.base_diameter / 2.0, self.base_height)
    }
    
    /// Generate the stick of the joystick
    pub fn stick(&self) -> CSG<()> {
        CSG::cylinder_z(self.stick_diameter / 2.0, self.stick_length)
            .translate(Vector3::new(0.0, 0.0, self.base_height))
    }
    
    /// Generate the top handle of the joystick
    pub fn top(&self) -> CSG<()> {
        CSG::sphere(self.top_diameter / 2.0)
            .translate(Vector3::new(0.0, 0.0, self.base_height + self.stick_length + self.top_height / 2.0))
    }
    
    /// Generate the full joystick model
    pub fn assemble(&self) -> CSG<()> {
        let mut joystick = self.base();
        let stick = self.stick();
        let top = self.top();
        
        joystick = joystick.union(&stick);
        joystick = joystick.union(&top);
        
        joystick
    }
}

