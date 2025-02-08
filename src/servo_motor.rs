use csgrs::CSG;

/// Representation of a Servo Motor
#[derive(Debug, Clone)]
pub struct ServoMotor {
    pub body_width: f64,
    pub body_height: f64,
    pub body_depth: f64,
    pub horn_diameter: f64,
    pub shaft_diameter: f64,
    pub shaft_length: f64,
}

impl ServoMotor {
    /// Generate the main servo body
    pub fn body(&self) -> CSG<()> {
        CSG::prism(self.body_width, self.body_height, self.body_depth)
    }

    /// Generate the rotating shaft
    pub fn shaft(&self) -> CSG<()> {
        CSG::cylinder(self.shaft_diameter / 2.0, self.shaft_length)
            .translate(Vector3::new(0.0, 0.0, self.body_height / 2.0))
    }

    /// Generate the horn
    pub fn horn(&self) -> CSG<()> {
        CSG::cylinder(self.horn_diameter / 2.0, self.shaft_length / 2.0)
            .translate(Vector3::new(0.0, 0.0, self.body_height / 2.0 + self.shaft_length))
    }

    /// Assemble the complete servo motor
    pub fn assemble(&self) -> CSG<()> {
        self.body().union(&self.shaft()).union(&self.horn())
    }
}

