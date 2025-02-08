use csgrs::CSG;

/// Representation of a Knob
#[derive(Debug, Clone)]
pub struct Knob {
    pub outer_diameter: f64,
    pub height: f64,
    pub shaft_diameter: f64,
    pub shaft_depth: f64,
}

impl Knob {
    /// Generate the main knob body
    pub fn body(&self) -> CSG<()> {
        CSG::cylinder(self.outer_diameter / 2.0, self.height)
    }

    /// Generate the shaft hole
    pub fn shaft_hole(&self) -> CSG<()> {
        CSG::cylinder(self.shaft_diameter / 2.0, self.shaft_depth)
    }

    /// Assemble the complete knob
    pub fn assemble(&self) -> CSG<()> {
        self.body().difference(&self.shaft_hole())
    }
}

