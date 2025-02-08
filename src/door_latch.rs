use csgrs::CSG;

/// Representation of a Door Latch
#[derive(Debug, Clone)]
pub struct DoorLatch {
    pub width: f64,
    pub height: f64,
    pub thickness: f64,
    pub latch_depth: f64,
    pub hole_diameter: f64,
    pub hole_spacing: f64,
    pub hole_count: usize,
}

impl DoorLatch {
    /// Generate the latch body
    pub fn body(&self) -> CSG<()> {
        CSG::prism(self.width, self.height, self.thickness)
    }

    /// Generate the latch cutout
    pub fn latch_cutout(&self) -> CSG<()> {
        CSG::prism(self.width / 3.0, self.latch_depth, self.thickness)
            .translate(Vector3::new(0.0, self.height / 4.0, 0.0))
    }

    /// Generate the mounting holes
    pub fn holes(&self) -> CSG<()> {
        let hole = CSG::cylinder_z(self.hole_diameter / 2.0, self.thickness + 1.0);
        let mut hole_array = CSG::new();
        let start_x = -((self.hole_count as f64 - 1.0) / 2.0) * self.hole_spacing;

        for i in 0..self.hole_count {
            let x_offset = start_x + (i as f64) * self.hole_spacing;
            hole_array = hole_array.union(&hole.translate(Vector3::new(x_offset, 0.0, 0.0)));
        }

        hole_array
    }

    /// Assemble the complete door latch
    pub fn assemble(&self) -> CSG<()> {
        self.body().difference(&self.latch_cutout()).difference(&self.holes())
    }
}

