use csgrs::CSG;

/// Representation of a Stepper Motor
#[derive(Debug, Clone)]
pub struct StepperMotor {
    pub name: String,
    pub side_length: f64,
    pub body_length: f64,
    pub shaft_diameter: f64,
    pub shaft_length: f64,
    pub mount_hole_spacing: f64,
}

impl StepperMotor {
    /// Generate the motor body
    pub fn body(&self) -> CSG<()> {
        CSG::prism(self.side_length, self.side_length, self.body_length)
    }

    /// Generate the motor shaft
    pub fn shaft(&self) -> CSG<()> {
        CSG::cylinder_z(self.shaft_diameter / 2.0, self.shaft_length)
            .translate(Vector3::new(0.0, 0.0, self.body_length))
    }

    /// Generate the motor with shaft
    pub fn assemble(&self) -> CSG<()> {
        self.body().union(&self.shaft())
    }
}

