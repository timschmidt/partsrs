use csgrs::CSG;

/// Representation of an SMD (Surface Mount Device) module
#[derive(Debug, Clone)]
pub struct SMD {
    pub name: String,
    pub body_length: f64,
    pub body_width: f64,
    pub body_height: f64,
    pub pad_length: f64,
    pub pad_width: f64,
    pub pad_height: f64,
    pub pad_spacing: f64,
}

impl SMD {
    /// Generate the rectangular body of the SMD
    pub fn body(&self) -> CSG<()> {
        CSG::prism(self.body_length, self.body_width, self.body_height)
    }
    
    /// Generate the pads of the SMD
    pub fn pads(&self) -> CSG<()> {
        let mut pads = CSG::new();
        let offsets = [
            (-self.pad_spacing / 2.0, 0.0),
            (self.pad_spacing / 2.0, 0.0),
        ];
        
        for &(x, y) in &offsets {
            let pad = CSG::prism(self.pad_length, self.pad_width, self.pad_height)
                .translate(Vector3::new(x, y, -self.pad_height));
            pads = pads.union(&pad);
        }
        
        pads
    }
    
    /// Generate the full SMD model
    pub fn assemble(&self) -> CSG<()> {
        let mut smd = self.body();
        let pads = self.pads();
        
        smd = smd.union(&pads);
        
        smd
    }
}

