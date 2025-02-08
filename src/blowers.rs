use csgrs::CSG;

/// Representation of a Blower module
#[derive(Debug, Clone)]
pub struct Blower {
    pub name: String,
    pub body_diameter: f64,
    pub body_height: f64,
    pub outlet_width: f64,
    pub outlet_height: f64,
    pub fan_diameter: f64,
    pub fan_blades: u32,
}

impl Blower {
    /// Generate the cylindrical body of the blower
    pub fn body(&self) -> CSG<()> {
        CSG::cylinder(self.body_diameter / 2.0, self.body_height)
    }
    
    /// Generate the outlet of the blower
    pub fn outlet(&self) -> CSG<()> {
        CSG::prism(self.outlet_width, self.outlet_height, self.body_height)
            .translate(self.body_diameter / 2.0, 0.0, 0.0)
    }
    
    /// Generate the fan inside the blower
    pub fn fan(&self) -> CSG<()> {
        let mut fan = CSG::new();
        for i in 0..self.fan_blades {
            let angle = (i as f64) * (360.0 / self.fan_blades as f64);
            let blade = CSG::prism(self.fan_diameter / 2.0, self.body_height / 10.0, self.body_height)
                .rotate_z(angle);
            fan = fan.union(&blade);
        }
        fan
    }
    
    /// Generate the full blower model
    pub fn assemble(&self) -> CSG<()> {
        let mut blower = self.body();
        let outlet = self.outlet();
        let fan = self.fan();
        
        blower = blower.union(&outlet);
        blower = blower.union(&fan);
        
        blower
    }
}

