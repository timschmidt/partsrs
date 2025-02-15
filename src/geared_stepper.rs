use csgrs::CSG;

/// Representation of a Geared Stepper Motor
#[derive(Debug, Clone)]
pub struct GearedStepperMotor {
    pub body_diameter: f64,
    pub body_length: f64,
    pub gear_diameter: f64,
    pub gear_length: f64,
    pub shaft_diameter: f64,
    pub shaft_length: f64,
}

impl GearedStepperMotor {
    /// Generate the motor body
    pub fn body(&self) -> CSG<()> {
        CSG::cylinder_z(self.body_diameter / 2.0, self.body_length)
    }

    /// Generate the gear
    pub fn gear(&self) -> CSG<()> {
        CSG::cylinder_z(self.gear_diameter / 2.0, self.gear_length)
            .translate(Vector3::new(0.0, 0.0, self.body_length))
    }

    /// Generate the motor shaft
    pub fn shaft(&self) -> CSG<()> {
        CSG::cylinder_z(self.shaft_diameter / 2.0, self.shaft_length)
            .translate(Vector3::new(0.0, 0.0, self.body_length + self.gear_length))
    }

    /// Assemble the complete geared stepper motor
    pub fn assemble(&self) -> CSG<()> {
        self.body().union(&self.gear()).union(&self.shaft())
    }
}

