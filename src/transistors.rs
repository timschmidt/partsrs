use csgrs::CSG;
use nalgebra::{Point3, Vector3};

/// Representation of a transistor module
#[derive(Debug, Clone)]
pub struct Transistor {
    pub name: String,
    pub body_diameter: f64,
    pub body_height: f64,
    pub lead_diameter: f64,
    pub lead_length: f64,
    pub lead_spacing: f64,
}

impl Transistor {
    /// Generate the cylindrical body of the transistor
    pub fn body(&self) -> CSG<()> {
        CSG::cylinder(self.body_diameter / 2.0, self.body_height)
    }
    
    /// Generate the leads of the transistor
    pub fn leads(&self) -> CSG<()> {
        let mut leads = CSG::new();
        let offsets = [
            (-self.lead_spacing, 0.0),
            (0.0, 0.0),
            (self.lead_spacing, 0.0),
        ];
        
        for &(x, y) in &offsets {
            let lead = CSG::cylinder(self.lead_diameter / 2.0, self.lead_length)
                .translate(Vector3::new(x, y, -self.lead_length));
            leads = leads.union(&lead);
        }
        
        leads
    }
    
    /// Generate the full transistor model
    pub fn assemble(&self) -> CSG<()> {
        let mut transistor = self.body();
        let leads = self.leads();
        
        transistor = transistor.union(&leads);
        
        transistor
    }
}

