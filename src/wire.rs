use csgrs::CSG;

/// Representation of an Electrical Wire
#[derive(Debug, Clone)]
pub struct Wire {
    pub diameter: f64,
    pub length: f64,
}

impl Wire {
    /// Generate the wire body
    pub fn body(&self) -> CSG<()> {
        CSG::cylinder_z(self.diameter / 2.0, self.length)
    }

    /// Assemble the complete wire
    pub fn assemble(&self) -> CSG<()> {
        self.body()
    }
}

