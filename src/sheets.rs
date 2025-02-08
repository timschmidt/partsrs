use csgrs::CSG;

/// Representation of an Axial Capacitor
#[derive(Debug, Clone)]
pub struct AxialCapacitor {
    pub body_diameter: f64,
    pub body_length: f64,
    pub lead_diameter: f64,
    pub lead_length: f64,
}

impl AxialCapacitor {
    /// Generate the capacitor body
    pub fn body(&self) -> CSG<()> {
        CSG::cylinder(self.body_diameter / 2.0, self.body_length)
    }

    /// Generate the leads
    pub fn leads(&self) -> CSG<()> {
        let lead = CSG::cylinder(self.lead_diameter / 2.0, self.lead_length);
        lead.translate(0.0, 0.0, -self.lead_length)
            .union(&lead.translate(0.0, 0.0, self.body_length))
    }

    /// Assemble the complete axial capacitor
    pub fn assemble(&self) -> CSG<()> {
        self.body().union(&self.leads())
    }
}

