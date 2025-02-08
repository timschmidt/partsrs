use csgrs::CSG;

/// Representation of an LED Meter
#[derive(Debug, Clone)]
pub struct LEDMeter {
    pub width: f64,
    pub height: f64,
    pub depth: f64,
    pub led_diameter: f64,
    pub led_spacing: f64,
    pub led_count: usize,
}

impl LEDMeter {
    /// Generate the main panel body
    pub fn body(&self) -> CSG<()> {
        CSG::box_shape(self.width, self.height, self.depth)
    }

    /// Generate the LED holes
    pub fn led_holes(&self) -> CSG<()> {
        let led_hole = CSG::cylinder(self.led_diameter / 2.0, self.depth + 1.0);
        let mut holes = CSG::empty();
        let start_x = -((self.led_count as f64 - 1.0) / 2.0) * self.led_spacing;

        for i in 0..self.led_count {
            let x_offset = start_x + (i as f64) * self.led_spacing;
            holes = holes.union(&led_hole.translate(x_offset, 0.0, 0.0));
        }

        holes
    }

    /// Assemble the full LED meter
    pub fn assemble(&self) -> CSG<()> {
        self.body().difference(&self.led_holes())
    }
}

