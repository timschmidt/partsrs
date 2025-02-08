use csgrs::CSG;

/// Representation of an SK Bracket
#[derive(Debug, Clone)]
pub struct SKBracket {
    pub name: String,
    pub hole_diameter: f64,
    pub base_width: f64,
    pub base_length: f64,
    pub height: f64,
    pub mount_hole_spacing: f64,
}

impl SKBracket {
    /// Generate the base of the SK bracket
    pub fn base(&self) -> CSG<()> {
        CSG::prism(self.base_width, self.base_length, self.height)
    }

    /// Generate the mount holes
    pub fn holes(&self) -> CSG<()> {
        let hole = CSG::cylinder(self.hole_diameter / 2.0, self.height);
        hole.translate(Vector3::new(self.mount_hole_spacing / 2.0, 0.0, 0.0))
            .union(&hole.translate(Vector3::new(-self.mount_hole_spacing / 2.0, 0.0, 0.0)))
    }

    /// Generate the complete bracket
    pub fn assemble(&self) -> CSG<()> {
        self.base().difference(&self.holes())
    }
}

