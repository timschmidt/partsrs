use csgrs::CSG;
use nalgebra::{Point3, Vector3};

/// Representation of a crystal oscillator module
#[derive(Debug, Clone)]
pub struct Crystal {
    pub name: String,
    pub body_width: f64,
    pub body_height: f64,
    pub body_depth: f64,
    pub lead_diameter: f64,
    pub lead_length: f64,
    pub lead_spacing: f64,
}

impl Crystal {
    /// Generate the rectangular body of the crystal
    pub fn body(&self) -> CSG<()> {
        CSG::prism(self.body_width, self.body_height, self.body_depth)
    }
    
    /// Generate the leads of the crystal
    pub fn leads(&self) -> CSG<()> {
        let mut leads = CSG::new();
        let offsets = [
            (-self.lead_spacing / 2.0, 0.0),
            (self.lead_spacing / 2.0, 0.0),
        ];
        
        for &(x, y) in &offsets {
            let lead = CSG::cylinder(self.lead_diameter / 2.0, self.lead_length)
                .translate(Vector3::new(x, y, -self.lead_length));
            leads = leads.union(&lead);
        }
        
        leads
    }
    
    /// Generate the full crystal model
    pub fn assemble(&self) -> CSG<()> {
        let mut crystal = self.body();
        let leads = self.leads();
        
        crystal = crystal.union(&leads);
        
        crystal
    }
}

