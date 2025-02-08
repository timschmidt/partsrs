use csgrs::CSG;

/// Representation of a Green Terminal Block
#[derive(Debug, Clone)]
pub struct GreenTerminal {
    pub width: f64,
    pub height: f64,
    pub depth: f64,
    pub hole_diameter: f64,
    pub hole_spacing: f64,
    pub hole_count: usize,
}

impl GreenTerminal {
    /// Generate the terminal body
    pub fn body(&self) -> CSG<()> {
        CSG::prism(self.width, self.height, self.depth)
    }

    /// Generate the wire holes
    pub fn holes(&self) -> CSG<()> {
        let hole = CSG::cylinder(self.hole_diameter / 2.0, self.depth + 1.0);
        let mut holes = CSG::new();
        let start_x = -((self.hole_count as f64 - 1.0) / 2.0) * self.hole_spacing;

        for i in 0..self.hole_count {
            let x_offset = start_x + (i as f64) * self.hole_spacing;
            holes = holes.union(&hole.translate(x_offset, 0.0, 0.0));
        }

        holes
    }

    /// Assemble the complete terminal block
    pub fn assemble(&self) -> CSG<()> {
        self.body().difference(&self.holes())
    }
}

