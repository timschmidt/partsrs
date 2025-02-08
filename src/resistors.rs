use csgrs::CSG;
use nalgebra::{Point3, Vector3};

/// Representation of a resistor module
#[derive(Debug, Clone)]
pub struct Resistor {
    pub name: String,
    pub body_length: f64,
    pub body_diameter: f64,
    pub lead_diameter: f64,
    pub lead_length: f64,
}

impl Resistor {
    /// Generate the cylindrical body of the resistor
    pub fn body(&self) -> CSG<()> {
        CSG::cylinder(self.body_diameter / 2.0, self.body_length)
    }
    
    /// Generate the leads of the resistor
    pub fn leads(&self) -> CSG<()> {
        let lead1 = CSG::cylinder(self.lead_diameter / 2.0, self.lead_length)
            .translate(0.0, 0.0, -self.lead_length);
        let lead2 = CSG::cylinder(self.lead_diameter / 2.0, self.lead_length)
            .translate(0.0, 0.0, self.body_length);
        
        lead1.union(&lead2)
    }
    
    /// Generate the full resistor model
    pub fn assemble(&self) -> CSG<()> {
        let mut resistor = self.body();
        let leads = self.leads();
        
        resistor = resistor.union(&leads);
        
        resistor
    }
}

