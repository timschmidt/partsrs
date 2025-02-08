use csgrs::CSG;

/// Representation of a Simple Box
#[derive(Debug, Clone)]
pub struct BoxModel {
    pub width: f64,
    pub height: f64,
    pub depth: f64,
    pub wall_thickness: f64,
}

impl BoxModel {
    /// Generate the outer box shape
    pub fn outer_body(&self) -> CSG<()> {
        CSG::box_shape(self.width, self.height, self.depth)
    }

    /// Generate the hollowed-out interior
    pub fn inner_body(&self) -> CSG<()> {
        CSG::box_shape(
            self.width - 2.0 * self.wall_thickness,
            self.height - 2.0 * self.wall_thickness,
            self.depth - 2.0 * self.wall_thickness,
        )
    }

    /// Assemble the complete box
    pub fn assemble(&self) -> CSG<()> {
        self.outer_body().difference(&self.inner_body())
    }
}

