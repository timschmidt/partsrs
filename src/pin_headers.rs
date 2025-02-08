use csgrs::CSG;

/// Representation of a Pin Header
#[derive(Debug, Clone)]
pub struct PinHeader {
    pub pin_count: usize,
    pub pin_diameter: f64,
    pub pin_length: f64,
    pub row_spacing: f64,
    pub base_width: f64,
    pub base_length: f64,
    pub base_thickness: f64,
}

impl PinHeader {
    /// Generate the plastic base
    pub fn base(&self) -> CSG<()> {
        CSG::prism(self.base_width, self.base_length, self.base_thickness)
    }

    /// Generate the pins
    pub fn pins(&self) -> CSG<()> {
        let pin = CSG::cylinder(self.pin_diameter / 2.0, self.pin_length);
        let mut pin_array = CSG::new();
        let spacing = self.row_spacing;

        for i in 0..self.pin_count {
            let x_offset = (i as f64 - (self.pin_count as f64 - 1.0) / 2.0) * spacing;
            pin_array = pin_array.union(&pin.translate(x_offset, 0.0, self.base_thickness));
        }

        pin_array
    }

    /// Assemble the full pin header
    pub fn assemble(&self) -> CSG<()> {
        self.base().union(&self.pins())
    }
}

