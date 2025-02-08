use csgrs::CSG;

/// Representation of a Printed Pulley
#[derive(Debug, Clone)]
pub struct PrintedPulley {
    pub outer_diameter: f64,
    pub width: f64,
    pub bore_diameter: f64,
    pub tooth_count: usize,
    pub tooth_depth: f64,
}

impl PrintedPulley {
    /// Generate the pulley body
    pub fn body(&self) -> CSG<()> {
        CSG::cylinder_z(self.outer_diameter / 2.0, self.width)
    }

    /// Generate the bore hole
    pub fn bore(&self) -> CSG<()> {
        CSG::cylinder_z(self.bore_diameter / 2.0, self.width + 1.0)
    }

    /// Generate the teeth
    pub fn teeth(&self) -> CSG<()> {
        let tooth = CSG::prism(self.tooth_depth, self.width, self.tooth_depth);
        let mut teeth = CSG::new();
        let angle_step = 360.0 / self.tooth_count as f64;

        for i in 0..self.tooth_count {
            let angle = i as f64 * angle_step;
            teeth = teeth.union(&tooth.rotate(0.0, 0.0, angle));
        }

        teeth
    }

    /// Assemble the complete pulley
    pub fn assemble(&self) -> CSG<()> {
        self.body().difference(&self.bore()).union(&self.teeth())
    }
}

