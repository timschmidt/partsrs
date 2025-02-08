use csgrs::CSG;

/// Representation of a Magnet
#[derive(Debug, Clone)]
pub struct Magnet {
    pub diameter: f64,
    pub thickness: f64,
    pub hole_diameter: f64,
}

impl Magnet {
    /// Generate the magnet body
    pub fn body(&self) -> CSG<()> {
        CSG::cylinder(self.diameter / 2.0, self.thickness)
    }

    /// Generate the center hole
    pub fn hole(&self) -> CSG<()> {
        CSG::cylinder(self.hole_diameter / 2.0, self.thickness + 1.0)
    }

    /// Assemble the complete magnet
    pub fn assemble(&self) -> CSG<()> {
        self.body().difference(&self.hole())
    }
}

