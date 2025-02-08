use csgrs::CSG;

/// Representation of an LDR (Light Dependent Resistor) module
#[derive(Debug, Clone)]
pub struct LDR {
    pub name: String,
    pub body_diameter: f64,
    pub body_height: f64,
    pub lead_diameter: f64,
    pub lead_length: f64,
    pub lead_spacing: f64,
}

impl LDR {
    /// Generate the cylindrical body of the LDR
    pub fn body(&self) -> CSG<()> {
        CSG::cylinder(self.body_diameter / 2.0, self.body_height)
    }
    
    /// Generate the leads of the LDR
    pub fn leads(&self) -> CSG<()> {
        let mut leads = CSG::new();
        let offsets = [
            (-self.lead_spacing / 2.0, 0.0),
            (self.lead_spacing / 2.0, 0.0),
        ];
        
        for &(x, y) in &offsets {
            let lead = CSG::cylinder(self.lead_diameter / 2.0, self.lead_length)
                .translate(x, y, -self.lead_length);
            leads = leads.union(&lead);
        }
        
        leads
    }
    
    /// Generate the full LDR model
    pub fn assemble(&self) -> CSG<()> {
        let mut ldr = self.body();
        let leads = self.leads();
        
        ldr = ldr.union(&leads);
        
        ldr
    }
}

