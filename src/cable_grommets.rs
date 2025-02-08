use csgrs::CSG;

/// Representation of a Cable Grommet
#[derive(Debug, Clone)]
pub struct CableGrommet {
    pub outer_diameter: f64,
    pub height: f64,
    pub inner_diameter: f64,
}

impl CableGrommet {
    /// Generate the outer body
    pub fn body(&self) -> CSG<()> {
        CSG::cylinder_z(self.outer_diameter / 2.0, self.height)
    }

    /// Generate the inner cable hole
    pub fn hole(&self) -> CSG<()> {
        CSG::cylinder_z(self.inner_diameter / 2.0, self.height + 1.0)
    }

    /// Assemble the complete cable grommet
    pub fn assemble(&self) -> CSG<()> {
        self.body().difference(&self.hole())
    }
}

