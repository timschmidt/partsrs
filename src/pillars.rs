use csgrs::CSG;

/// Representation of a Pillar (Spacer)
#[derive(Debug, Clone)]
pub struct Pillar {
    pub diameter: f64,
    pub height: f64,
    pub hole_diameter: f64,
}

impl Pillar {
    /// Generate the pillar body
    pub fn body(&self) -> CSG<()> {
        CSG::cylinder(self.diameter / 2.0, self.height)
    }

    /// Generate the center hole
    pub fn hole(&self) -> CSG<()> {
        CSG::cylinder(self.hole_diameter / 2.0, self.height + 1.0)
    }

    /// Assemble the complete pillar
    pub fn assemble(&self) -> CSG<()> {
        self.body().difference(&self.hole())
    }
}

