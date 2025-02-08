use csgrs::CSG;

/// Representation of a Pocket Handle
#[derive(Debug, Clone)]
pub struct PocketHandle {
    pub width: f64,
    pub height: f64,
    pub depth: f64,
    pub hole_diameter: f64,
    pub hole_spacing: f64,
    pub hole_count: usize,
}

impl PocketHandle {
    /// Generate the handle body
    pub fn body(&self) -> CSG<()> {
        CSG::box_shape(self.width, self.height, self.depth)
    }

    /// Generate the mounting holes
    pub fn holes(&self) -> CSG<()> {
        let hole = CSG::cylinder(self.hole_diameter / 2.0, self.depth + 1.0);
        let mut hole_array = CSG::empty();
        let start_x = -((self.hole_count as f64 - 1.0) / 2.0) * self.hole_spacing;

        for i in 0..self.hole_count {
            let x_offset = start_x + (i as f64) * self.hole_spacing;
            hole_array = hole_array.union(&hole.translate(x_offset, 0.0, 0.0));
        }

        hole_array
    }

    /// Assemble the complete pocket handle
    pub fn assemble(&self) -> CSG<()> {
        self.body().difference(&self.holes())
    }
}

