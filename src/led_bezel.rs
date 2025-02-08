use csgrs::CSG;

/// Representation of an LED Bezel
#[derive(Debug, Clone)]
pub struct LEDBezel {
    pub outer_diameter: f64,
    pub inner_diameter: f64,
    pub height: f64,
}

impl LEDBezel {
    /// Generate the bezel body
    pub fn body(&self) -> CSG<()> {
        CSG::cylinder(self.outer_diameter / 2.0, self.height)
    }

    /// Generate the LED hole
    pub fn led_hole(&self) -> CSG<()> {
        CSG::cylinder(self.inner_diameter / 2.0, self.height + 1.0)
    }

    /// Assemble the complete LED bezel
    pub fn assemble(&self) -> CSG<()> {
        self.body().difference(&self.led_hole())
    }
}

