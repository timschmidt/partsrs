use csgrs::CSG;

/// Representation of a Push Button
#[derive(Debug, Clone)]
pub struct Button {
    pub body_diameter: f64,
    pub body_height: f64,
    pub cap_diameter: f64,
    pub cap_height: f64,
    pub pin_diameter: f64,
    pub pin_length: f64,
}

impl Button {
    /// Generate the button body
    pub fn body(&self) -> CSG<()> {
        CSG::cylinder_z(self.body_diameter / 2.0, self.body_height)
    }

    /// Generate the button cap
    pub fn cap(&self) -> CSG<()> {
        CSG::cylinder_z(self.cap_diameter / 2.0, self.cap_height)
            .translate(Vector3::new(0.0, 0.0, self.body_height))
    }

    /// Generate the button pins
    pub fn pins(&self) -> CSG<()> {
        let pin = CSG::cylinder_z(self.pin_diameter / 2.0, self.pin_length);
        let spacing = self.body_diameter / 3.0;
        pin.translate(Vector3::new(-spacing, 0.0, -self.pin_length))
            .union(&pin.translate(Vector3::new(spacing, 0.0, -self.pin_length)))
    }

    /// Assemble the complete button
    pub fn assemble(&self) -> CSG<()> {
        self.body().union(&self.cap()).union(&self.pins())
    }
}

