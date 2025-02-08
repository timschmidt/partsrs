use csgrs::CSG;

/// Representation of a Corner Block
#[derive(Debug, Clone)]
pub struct CornerBlock {
    pub size: f64,
    pub hole_diameter: f64,
}

impl CornerBlock {
    /// Generate the main block
    pub fn body(&self) -> CSG<()> {
        CSG::prism(self.size, self.size, self.size)
    }

    /// Generate the screw holes
    pub fn holes(&self) -> CSG<()> {
        let hole = CSG::cylinder(self.hole_diameter / 2.0, self.size + 1.0);
        hole.translate(0.0, 0.0, self.size / 2.0)
            .union(&hole.rotate(90.0, 0.0, 0.0))
            .union(&hole.rotate(0.0, 90.0, 0.0))
    }

    /// Assemble the complete corner block
    pub fn assemble(&self) -> CSG<()> {
        self.body().difference(&self.holes())
    }
}

