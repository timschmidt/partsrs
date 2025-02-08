use csgrs::CSG;

/// Representation of a Bearing Block
#[derive(Debug, Clone)]
pub struct BearingBlock {
    pub width: f64,
    pub height: f64,
    pub depth: f64,
    pub bearing_diameter: f64,
    pub bearing_depth: f64,
    pub mount_hole_spacing: f64,
    pub mount_hole_diameter: f64,
}

impl BearingBlock {
    /// Generate the block body
    pub fn body(&self) -> CSG<()> {
        CSG::box_shape(self.width, self.height, self.depth)
    }

    /// Generate the bearing hole
    pub fn bearing_hole(&self) -> CSG<()> {
        CSG::cylinder(self.bearing_diameter / 2.0, self.bearing_depth)
            .translate(0.0, 0.0, self.depth / 2.0)
    }

    /// Generate the mounting holes
    pub fn mount_holes(&self) -> CSG<()> {
        let hole = CSG::cylinder(self.mount_hole_diameter / 2.0, self.depth);
        hole.translate(self.mount_hole_spacing / 2.0, 0.0, 0.0)
            .union(&hole.translate(-self.mount_hole_spacing / 2.0, 0.0, 0.0))
    }

    /// Assemble the complete bearing block
    pub fn assemble(&self) -> CSG<()> {
        self.body().difference(&self.bearing_hole()).difference(&self.mount_holes())
    }
}

