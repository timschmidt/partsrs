use csgrs::CSG;

/// Representation of a Strap Handle
#[derive(Debug, Clone)]
pub struct StrapHandle {
    pub width: f64,
    pub thickness: f64,
    pub length: f64,
    pub hole_diameter: f64,
}

impl StrapHandle {
    /// Generate the strap body
    pub fn body(&self) -> CSG<()> {
        CSG::prism(self.width, self.length, self.thickness)
    }

    /// Generate the mounting holes
    pub fn holes(&self) -> CSG<()> {
        let hole = CSG::cylinder(self.hole_diameter / 2.0, self.thickness + 1.0);
        hole.translate(self.width / 3.0, 0.0, 0.0)
            .union(&hole.translate(Vector3::new(-self.width / 3.0, 0.0, 0.0)))
    }

    /// Assemble the complete strap handle
    pub fn assemble(&self) -> CSG<()> {
        self.body().difference(&self.holes())
    }
}

