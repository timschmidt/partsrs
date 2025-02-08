use csgrs::CSG;

/// Representation of a Butt Box
#[derive(Debug, Clone)]
pub struct ButtBox {
    pub width: f64,
    pub height: f64,
    pub depth: f64,
    pub wall_thickness: f64,
}

impl ButtBox {
    /// Generate the box body
    pub fn outer_body(&self) -> CSG<()> {
        CSG::box_shape(self.width, self.height, self.depth)
    }

    /// Generate the hollow inside
    pub fn inner_body(&self) -> CSG<()> {
        CSG::box_shape(
            self.width - 2.0 * self.wall_thickness,
            self.height - 2.0 * self.wall_thickness,
            self.depth - 2.0 * self.wall_thickness,
        )
    }

    /// Assemble the complete butt box
    pub fn assemble(&self) -> CSG<()> {
        self.outer_body().difference(&self.inner_body())
    }
}

