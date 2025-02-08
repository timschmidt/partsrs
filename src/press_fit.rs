use csgrs::CSG;

/// Representation of a Power Supply Unit (PSU) Shroud
#[derive(Debug, Clone)]
pub struct PSUShroud {
    pub width: f64,
    pub height: f64,
    pub depth: f64,
    pub vent_hole_diameter: f64,
    pub vent_hole_spacing: f64,
    pub vent_hole_count: usize,
}

impl PSUShroud {
    /// Generate the shroud body
    pub fn body(&self) -> CSG<()> {
        CSG::prism(self.width, self.height, self.depth)
    }

    /// Generate vent holes
    pub fn vent_holes(&self) -> CSG<()> {
        let hole = CSG::cylinder_z(self.vent_hole_diameter / 2.0, self.depth + 1.0);
        let mut holes = CSG::new();
        let start_x = -((self.vent_hole_count as f64 - 1.0) / 2.0) * self.vent_hole_spacing;

        for i in 0..self.vent_hole_count {
            let x_offset = start_x + (i as f64) * self.vent_hole_spacing;
            holes = holes.union(&hole.translate(Vector3::new(x_offset, 0.0, self.height / 2.0)));
        }

        holes
    }

    /// Assemble the complete PSU shroud
    pub fn assemble(&self) -> CSG<()> {
        self.body().difference(&self.vent_holes())
    }
}

