use csgrs::CSG;

/// Representation of a Handle
#[derive(Debug, Clone)]
pub struct Handle {
    pub length: f64,
    pub diameter: f64,
    pub height: f64,
    pub screw_diameter: f64,
}

impl Handle {
    /// Generate the main handle body
    pub fn body(&self) -> CSG<()> {
        CSG::cylinder_z(self.diameter / 2.0, self.length)
    }

    /// Generate the screw holes
    pub fn screw_holes(&self) -> CSG<()> {
        let hole = CSG::cylinder_z(self.screw_diameter / 2.0, self.height);
        hole.translate(Vector3::new(self.length / 2.0, 0.0, 0.0))
            .union(&hole.translate(Vector3::new(-self.length / 2.0, 0.0, 0.0)))
    }

    /// Assemble the complete handle
    pub fn assemble(&self) -> CSG<()> {
        self.body().difference(&self.screw_holes())
    }
}

