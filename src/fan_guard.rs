use csgrs::CSG;

/// Representation of a Fan Guard
#[derive(Debug, Clone)]
pub struct FanGuard {
    pub outer_diameter: f64,
    pub thickness: f64,
    pub vent_diameter: f64,
    pub vent_count: usize,
}

impl FanGuard {
    /// Generate the guard body
    pub fn body(&self) -> CSG<()> {
        CSG::cylinder_z(self.outer_diameter / 2.0, self.thickness)
    }

    /// Generate the vent holes
    pub fn vents(&self) -> CSG<()> {
        let vent = CSG::cylinder_z(self.vent_diameter / 2.0, self.thickness + 1.0);
        let mut holes = CSG::new();
        let angle_step = 360.0 / self.vent_count as f64;

        for i in 0..self.vent_count {
            let angle = i as f64 * angle_step;
            let x_offset = (self.outer_diameter / 3.0) * angle.to_radians().cos();
            let y_offset = (self.outer_diameter / 3.0) * angle.to_radians().sin();
            holes = holes.union(&vent.translate(Vector3::new(x_offset, y_offset, 0.0)));
        }

        holes
    }

    /// Assemble the complete fan guard
    pub fn assemble(&self) -> CSG<()> {
        self.body().difference(&self.vents())
    }
}

