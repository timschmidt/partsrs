use csgrs::CSG;

/// Representation of a Screw Knob
#[derive(Debug, Clone)]
pub struct ScrewKnob {
    pub outer_diameter: f64,
    pub height: f64,
    pub screw_diameter: f64,
    pub screw_length: f64,
}

impl ScrewKnob {
    /// Generate the main knob body
    pub fn body(&self) -> CSG<()> {
        CSG::cylinder(self.outer_diameter / 2.0, self.height)
    }

    /// Generate the screw shaft
    pub fn screw(&self) -> CSG<()> {
        CSG::cylinder(self.screw_diameter / 2.0, self.screw_length)
            .translate(0.0, 0.0, -self.screw_length)
    }

    /// Assemble the complete screw knob
    pub fn assemble(&self) -> CSG<()> {
        self.body().union(&self.screw())
    }
}

