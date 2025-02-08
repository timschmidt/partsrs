use csgrs::CSG;

/// Representation of an O-Ring
#[derive(Debug, Clone)]
pub struct ORing {
    pub outer_diameter: f64,
    pub thickness: f64,
}

impl ORing {
    /// Generate the toroidal body of the O-Ring
    pub fn body(&self) -> CSG<()> {
        CSG::torus(self.outer_diameter / 2.0, self.thickness / 2.0)
    }

    /// Assemble the complete O-Ring
    pub fn assemble(&self) -> CSG<()> {
        self.body()
    }
}

