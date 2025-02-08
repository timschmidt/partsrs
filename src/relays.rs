use csgrs::CSG;
use nalgebra::{Point3, Vector3};

/// Representation of a relay module
#[derive(Debug, Clone)]
pub struct Relay {
    pub name: String,
    pub body_width: f64,
    pub body_height: f64,
    pub body_depth: f64,
    pub coil_diameter: f64,
    pub coil_height: f64,
    pub contact_diameter: f64,
    pub contact_spacing: f64,
}

impl Relay {
    /// Generate the rectangular body of the relay
    pub fn body(&self) -> CSG<()> {
        CSG::prism(self.body_width, self.body_height, self.body_depth)
    }
    
    /// Generate the coil of the relay
    pub fn coil(&self) -> CSG<()> {
        CSG::cylinder(self.coil_diameter / 2.0, self.coil_height)
            .translate(0.0, 0.0, self.body_depth)
    }
    
    /// Generate the contacts of the relay
    pub fn contacts(&self) -> CSG<()> {
        let mut contacts = CSG::new();
        let offsets = [
            (-self.contact_spacing / 2.0, 0.0),
            (self.contact_spacing / 2.0, 0.0),
        ];
        
        for &(x, y) in &offsets {
            let contact = CSG::cylinder(self.contact_diameter / 2.0, self.body_depth / 2.0)
                .translate(x, y, -self.body_depth / 2.0);
            contacts = contacts.union(&contact);
        }
        
        contacts
    }
    
    /// Generate the full relay model
    pub fn assemble(&self) -> CSG<()> {
        let mut relay = self.body();
        let coil = self.coil();
        let contacts = self.contacts();
        
        relay = relay.union(&coil);
        relay = relay.union(&contacts);
        
        relay
    }
}

