use csgrs::CSG;

/// Representation of a Printed Box
#[derive(Debug, Clone)]
pub struct PrintedBox {
    pub width: f64,
    pub height: f64,
    pub depth: f64,
    pub wall_thickness: f64,
}

impl PrintedBox {
    /// Generate the outer box body
    pub fn outer_body(&self) -> CSG<()> {
        CSG::prism(self.width, self.height, self.depth)
    }

    /// Generate the hollow interior
    pub fn inner_body(&self) -> CSG<()> {
        CSG::prism(
            self.width - 2.0 * self.wall_thickness,
            self.height - 2.0 * self.wall_thickness,
            self.depth - 2.0 * self.wall_thickness,
        )
    }

    /// Assemble the complete printed box
    pub fn assemble(&self) -> CSG<()> {
        self.outer_body().difference(&self.inner_body())
    }
}

