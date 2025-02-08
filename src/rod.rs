use csgrs::CSG;

/// Representation of a Cylindrical Rod
#[derive(Debug, Clone)]
pub struct Rod {
    pub diameter: f64,
    pub length: f64,
}

impl Rod {
    /// Generate the rod body
    pub fn body(&self) -> CSG<()> {
        CSG::cylinder(self.diameter / 2.0, self.length)
    }

    /// Assemble the complete rod
    pub fn assemble(&self) -> CSG<()> {
        self.body()
    }
}

