use csgrs::CSG;

/// Representation of a Generic Electronic Component
#[derive(Debug, Clone)]
pub struct Component {
    pub width: f64,
    pub height: f64,
    pub depth: f64,
    pub lead_spacing: f64,
    pub lead_diameter: f64,
    pub lead_length: f64,
}

impl Component {
    /// Generate the main body of the component
    pub fn body(&self) -> CSG<()> {
        CSG::prism(self.width, self.height, self.depth)
    }

    /// Generate the leads
    pub fn leads(&self) -> CSG<()> {
        let lead = CSG::cylinder_z(self.lead_diameter / 2.0, self.lead_length);
        let offset = self.lead_spacing / 2.0;

        lead.translate(Vector3::new(offset, 0.0, -self.lead_length))
            .union(&lead.translate(Vector3::new(-offset, 0.0, -self.lead_length)))
    }

    /// Assemble the full electronic component
    pub fn assemble(&self) -> CSG<()> {
        self.body().union(&self.leads())
    }
}

