use csgrs::CSG;

/// Representation of a Foot Pad
#[derive(Debug, Clone)]
pub struct FootPad {
    pub diameter: f64,
    pub height: f64,
    pub hole_diameter: f64,
}

impl FootPad {
    /// Generate the foot body
    pub fn body(&self) -> CSG<()> {
        CSG::cylinder_z(self.diameter / 2.0, self.height)
    }

    /// Generate the mounting hole
    pub fn hole(&self) -> CSG<()> {
        CSG::cylinder_z(self.hole_diameter / 2.0, self.height + 1.0)
    }

    /// Assemble the complete foot pad
    pub fn assemble(&self) -> CSG<()> {
        self.body().difference(&self.hole())
    }
}

