use csgrs::CSG;

/// Representation of an OpenGrab Electro-Permanent Magnet
#[derive(Debug, Clone)]
pub struct OpenGrabMagnet {
    pub width: f64,
    pub depth: f64,
    pub magnet_height: f64,
    pub pole_width: f64,
    pub pole_length: f64,
    pub poles: usize,
}

impl OpenGrabMagnet {
    /// Generate the magnet base
    pub fn base(&self) -> CSG<()> {
        CSG::prism(self.width, self.width, self.magnet_height)
    }

    /// Generate the poles
    pub fn poles(&self) -> CSG<()> {
        let mut pole_array = CSG::new();
        let gap = (self.width - (self.poles as f64 * self.pole_width)) / ((self.poles - 1) as f64);

        for i in 0..self.poles {
            let x_offset = (i as f64) * (self.pole_width + gap) - self.width / 2.0;
            pole_array = pole_array.union(&CSG::prism(self.pole_width, self.pole_length, 1.0)
                .translate(Vector3::new(x_offset, 0.0, self.magnet_height / 2.0)));
        }

        pole_array
    }

    /// Assemble the complete OpenGrab module
    pub fn assemble(&self) -> CSG<()> {
        self.base().union(&self.poles())
    }
}

