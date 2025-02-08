use csgrs::CSG;

/// Representation of a Socket Box
#[derive(Debug, Clone)]
pub struct SocketBox {
    pub width: f64,
    pub height: f64,
    pub depth: f64,
    pub wall_thickness: f64,
    pub hole_diameter: f64,
    pub hole_spacing: f64,
    pub hole_count: usize,
}

impl SocketBox {
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

    /// Generate the socket holes
    pub fn holes(&self) -> CSG<()> {
        let hole = CSG::cylinder(self.hole_diameter / 2.0, self.depth + 1.0);
        let mut hole_array = CSG::new();
        let start_x = -((self.hole_count as f64 - 1.0) / 2.0) * self.hole_spacing;

        for i in 0..self.hole_count {
            let x_offset = start_x + (i as f64) * self.hole_spacing;
            hole_array = hole_array.union(&hole.translate(Vector3::new(x_offset, 0.0, self.height / 2.0)));
        }

        hole_array
    }

    /// Assemble the complete socket box
    pub fn assemble(&self) -> CSG<()> {
        self.outer_body().difference(&self.inner_body()).difference(&self.holes())
    }
}

