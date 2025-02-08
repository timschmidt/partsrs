use csgrs::CSG;

/// Representation of an Audio Jack
#[derive(Debug, Clone)]
pub struct AudioJack {
    pub body_diameter: f64,
    pub body_length: f64,
    pub pin_diameter: f64,
    pub pin_length: f64,
}

impl AudioJack {
    /// Generate the main body of the audio jack
    pub fn body(&self) -> CSG<()> {
        CSG::cylinder(self.body_diameter / 2.0, self.body_length)
    }

    /// Generate the pin
    pub fn pin(&self) -> CSG<()> {
        CSG::cylinder(self.pin_diameter / 2.0, self.pin_length)
            .translate(Vector3::new(0.0, 0.0, -self.pin_length))
    }

    /// Assemble the complete audio jack
    pub fn assemble(&self) -> CSG<()> {
        self.body().union(&self.pin())
    }
}

