use csgrs::CSG;
use nalgebra::{Point3, Vector3};

/// Representation of a vibration motor module
#[derive(Debug, Clone)]
pub struct VibrationMotor {
    pub name: String,
    pub body_diameter: f64,
    pub body_height: f64,
    pub eccentric_mass_diameter: f64,
    pub eccentric_mass_thickness: f64,
    pub shaft_diameter: f64,
    pub shaft_length: f64,
}

impl VibrationMotor {
    /// Generate the cylindrical body of the motor
    pub fn body(&self) -> CSG<()> {
        CSG::cylinder(self.body_diameter / 2.0, self.body_height)
    }
    
    /// Generate the eccentric mass of the motor
    pub fn eccentric_mass(&self) -> CSG<()> {
        CSG::cylinder(self.eccentric_mass_diameter / 2.0, self.eccentric_mass_thickness)
            .translate(Vector3::new(self.body_diameter / 4.0, 0.0, self.body_height / 2.0))
    }
    
    /// Generate the shaft of the motor
    pub fn shaft(&self) -> CSG<()> {
        CSG::cylinder(self.shaft_diameter / 2.0, self.shaft_length)
            .translate(Vector3::new(0.0, 0.0, self.body_height))
    }
    
    /// Generate the full vibration motor model
    pub fn assemble(&self) -> CSG<()> {
        let mut motor = self.body();
        let eccentric_mass = self.eccentric_mass();
        let shaft = self.shaft();
        
        motor = motor.union(&eccentric_mass);
        motor = motor.union(&shaft);
        
        motor
    }
}

