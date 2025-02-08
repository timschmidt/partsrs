use csgrs::CSG;

/// Representation of an Extrusion Bracket
#[derive(Debug, Clone)]
pub struct ExtrusionBracket {
    pub width: f64,
    pub height: f64,
    pub thickness: f64,
    pub hole_diameter: f64,
    pub hole_spacing: f64,
}

impl ExtrusionBracket {
    /// Generate the main bracket shape
    pub fn body(&self) -> CSG<()> {
        CSG::box_shape(self.width, self.height, self.thickness)
    }

    /// Generate the mounting holes
    pub fn holes(&self) -> CSG<()> {
        let hole = CSG::cylinder(self.hole_diameter / 2.0, self.thickness);
        let offset = self.hole_spacing / 2.0;

        hole.translate(offset, 0.0, 0.0)
            .union(&hole.translate(-offset, 0.0, 0.0))
    }

    /// Assemble the extrusion bracket
    pub fn assemble(&self) -> CSG<()> {
        self.body().difference(&self.holes())
    }
}

