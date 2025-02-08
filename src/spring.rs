use csgrs::CSG;

/// Representation of a Spring
#[derive(Debug, Clone)]
pub struct Spring {
    pub coil_diameter: f64,
    pub wire_diameter: f64,
    pub turns: usize,
    pub height: f64,
}

impl Spring {
    /// Generate the spring coil
    pub fn coil(&self) -> CSG<()> {
        let mut coil = CSG::new();
        let step = self.height / self.turns as f64;

        for i in 0..self.turns {
            let angle = i as f64 * 360.0 / self.turns as f64;
            let z_offset = i as f64 * step;
            coil = coil.union(
                &CSG::torus(self.coil_diameter / 2.0, self.wire_diameter / 2.0)
                    .rotate(0.0, angle, 0.0)
                    .translate(Vector3::new(0.0, 0.0, z_offset)),
            );
        }

        coil
    }

    /// Assemble the complete spring
    pub fn assemble(&self) -> CSG<()> {
        self.coil()
    }
}

