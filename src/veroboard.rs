use csgrs::CSG;

/// Representation of a Veroboard (Stripboard)
#[derive(Debug, Clone)]
pub struct Veroboard {
    pub width: f64,
    pub height: f64,
    pub thickness: f64,
    pub hole_diameter: f64,
    pub hole_spacing: f64,
    pub hole_count_x: usize,
    pub hole_count_y: usize,
}

impl Veroboard {
    /// Generate the board body
    pub fn body(&self) -> CSG<()> {
        CSG::prism(self.width, self.height, self.thickness)
    }

    /// Generate the hole pattern
    pub fn holes(&self) -> CSG<()> {
        let hole = CSG::cylinder(self.hole_diameter / 2.0, self.thickness);
        let mut holes = CSG::new();

        let x_start = -((self.hole_count_x as f64 - 1.0) / 2.0) * self.hole_spacing;
        let y_start = -((self.hole_count_y as f64 - 1.0) / 2.0) * self.hole_spacing;

        for x in 0..self.hole_count_x {
            for y in 0..self.hole_count_y {
                let x_offset = x_start + (x as f64) * self.hole_spacing;
                let y_offset = y_start + (y as f64) * self.hole_spacing;
                holes = holes.union(&hole.translate(x_offset, y_offset, 0.0));
            }
        }

        holes
    }

    /// Assemble the complete veroboard
    pub fn assemble(&self) -> CSG<()> {
        self.body().difference(&self.holes())
    }
}

