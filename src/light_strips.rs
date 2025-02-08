use csgrs::CSG;

/// Representation of an LED Light Strip module
#[derive(Debug, Clone)]
pub struct LightStrip {
    pub name: String,
    pub strip_length: f64,
    pub strip_width: f64,
    pub strip_thickness: f64,
    pub led_diameter: f64,
    pub led_spacing: f64,
    pub led_count: u32,
}

impl LightStrip {
    /// Generate the base strip of the light strip
    pub fn base_strip(&self) -> CSG<()> {
        CSG::prism(self.strip_length, self.strip_width, self.strip_thickness)
    }
    
    /// Generate the LEDs on the strip
    pub fn leds(&self) -> CSG<()> {
        let mut leds = CSG::new();
        let start_x = -((self.led_count as f64 - 1.0) * self.led_spacing) / 2.0;
        
        for i in 0..self.led_count {
            let x_offset = start_x + i as f64 * self.led_spacing;
            let led = CSG::cylinder_z(self.led_diameter / 2.0, self.strip_thickness)
                .translate(Vector3::new(x_offset, 0.0, self.strip_thickness / 2.0));
            leds = leds.union(&led);
        }
        
        leds
    }
    
    /// Generate the full LED light strip model
    pub fn assemble(&self) -> CSG<()> {
        let mut light_strip = self.base_strip();
        let leds = self.leds();
        
        light_strip = light_strip.union(&leds);
        
        light_strip
    }
}

