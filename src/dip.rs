use csgrs::CSG;

/// Representation of a Dual In-line Package (DIP) module
#[derive(Debug, Clone)]
pub struct DIP {
    pub name: String,
    pub pin_count: u32,
    pub pin_diameter: f64,
    pub pin_spacing: f64,
    pub body_length: f64,
    pub body_width: f64,
    pub body_height: f64,
}

impl DIP {
    /// Generate the body of the DIP package
    pub fn body(&self) -> CSG<()> {
        CSG::box_shape(self.body_length, self.body_width, self.body_height)
    }
    
    /// Generate the pins of the DIP package
    pub fn pins(&self) -> CSG<()> {
        let mut pins = CSG::new();
        let start_x = -((self.pin_count as f64 / 2.0 - 0.5) * self.pin_spacing);
        
        for i in 0..self.pin_count {
            let x_offset = start_x + i as f64 * self.pin_spacing;
            let pin = CSG::cylinder(self.pin_diameter / 2.0, self.body_height / 2.0)
                .translate(x_offset, 0.0, -self.body_height / 2.0);
            pins = pins.union(&pin);
        }
        
        pins
    }
    
    /// Generate the full DIP model
    pub fn assemble(&self) -> CSG<()> {
        let mut dip = self.body();
        let pins = self.pins();
        
        dip = dip.union(&pins);
        
        dip
    }
}

