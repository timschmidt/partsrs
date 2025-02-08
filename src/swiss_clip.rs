use csgrs::CSG;

/// Representation of a Swiss Clip (for securing panels)
#[derive(Debug, Clone)]
pub struct SwissClip {
    pub width: f64,
    pub height: f64,
    pub thickness: f64,
    pub hole_diameter: f64,
    pub hole_spacing: f64,
}

impl SwissClip {
    /// Generate the main clip body
    pub fn body(&self) -> CSG<()> {
        CSG::prism(self.width, self.height, self.thickness)
    }

    /// Generate the mounting holes
    pub fn holes(&self) -> CSG<()> {
        let hole = CSG::cylinder(self.hole_diameter / 2.0, self.thickness);
        let offset = self.hole_spacing / 2.0;

        hole.translate(offset, 0.0, 0.0)
            .union(&hole.translate(-offset, 0.0, 0.0))
    }

    /// Assemble the complete Swiss Clip
    pub fn assemble(&self) -> CSG<()> {
        self.body().difference(&self.holes())
    }
}

