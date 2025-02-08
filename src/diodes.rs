use csgrs::CSG;
use nalgebra::{Point3, Vector3};

/// Representation of a diode module
#[derive(Debug, Clone)]
pub struct Diode {
    pub name: String,
    pub body_length: f64,
    pub body_diameter: f64,
    pub lead_diameter: f64,
    pub lead_length: f64,
    pub cathode_band_width: f64,
}

impl Diode {
    /// Generate the cylindrical body of the diode
    pub fn body(&self) -> CSG<()> {
        CSG::cylinder(self.body_diameter / 2.0, self.body_length)
    }
    
    /// Generate the leads of the diode
    pub fn leads(&self) -> CSG<()> {
        let lead1 = CSG::cylinder(self.lead_diameter / 2.0, self.lead_length)
            .translate(0.0, 0.0, -self.lead_length);
        let lead2 = CSG::cylinder(self.lead_diameter / 2.0, self.lead_length)
            .translate(0.0, 0.0, self.body_length);
        
        lead1.union(&lead2)
    }
    
    /// Generate the cathode band marking
    pub fn cathode_band(&self) -> CSG<()> {
        CSG::prism(self.body_diameter, self.cathode_band_width, self.body_diameter)
            .translate(0.0, self.body_length - self.cathode_band_width / 2.0, 0.0)
    }
    
    /// Generate the full diode model
    pub fn assemble(&self) -> CSG<()> {
        let mut diode = self.body();
        let leads = self.leads();
        let band = self.cathode_band();
        
        diode = diode.union(&leads);
        diode = diode.union(&band);
        
        diode
    }
}

