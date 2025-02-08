use csgrs::CSG;

/// Representation of a Photo Interrupter
#[derive(Debug, Clone)]
pub struct PhotoInterrupter {
    pub body_width: f64,
    pub body_height: f64,
    pub body_depth: f64,
    pub slot_width: f64,
    pub slot_depth: f64,
    pub slot_height: f64,
    pub pin_diameter: f64,
    pub pin_length: f64,
}

impl PhotoInterrupter {
    /// Generate the main body
    pub fn body(&self) -> CSG<()> {
        CSG::prism(self.body_width, self.body_height, self.body_depth)
    }

    /// Generate the slot in the interrupter
    pub fn slot(&self) -> CSG<()> {
        CSG::prism(self.slot_width, self.slot_depth, self.slot_height)
            .translate(Vector3::new(0.0, 0.0, self.body_height / 2.0))
    }

    /// Generate the connector pins
    pub fn pins(&self) -> CSG<()> {
        let pin = CSG::cylinder(self.pin_diameter / 2.0, self.pin_length);
        pin.translate(Vector3::new(-self.body_width / 4.0, 0.0, -self.pin_length))
            .union(&pin.translate(Vector3::new(self.body_width / 4.0, 0.0, -self.pin_length)))
    }

    /// Assemble the complete photo interrupter
    pub fn assemble(&self) -> CSG<()> {
        self.body().difference(&self.slot()).union(&self.pins())
    }
}

