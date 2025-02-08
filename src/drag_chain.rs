use csgrs::CSG;

/// Representation of a Drag Chain Link
#[derive(Debug, Clone)]
pub struct DragChainLink {
    pub length: f64,
    pub width: f64,
    pub height: f64,
    pub hole_diameter: f64,
}

impl DragChainLink {
    /// Generate the link body
    pub fn body(&self) -> CSG<()> {
        CSG::box_shape(self.width, self.length, self.height)
    }

    /// Generate the pivot holes
    pub fn pivot_holes(&self) -> CSG<()> {
        let hole = CSG::cylinder(self.hole_diameter / 2.0, self.width + 1.0);
        hole.translate(0.0, -self.length / 2.0, 0.0)
            .union(&hole.translate(0.0, self.length / 2.0, 0.0))
    }

    /// Assemble the complete drag chain link
    pub fn assemble(&self) -> CSG<()> {
        self.body().difference(&self.pivot_holes())
    }
}

