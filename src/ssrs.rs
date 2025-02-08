use csgrs::CSG;

/// Representation of a Solid State Relay (SSR) module
#[derive(Debug, Clone)]
pub struct SSR {
    pub name: String,
    pub model: String,
    pub length: f64,
    pub width: f64,
    pub height: f64,
    pub hole_diameter: f64,
    pub hole_spacing: f64,
    pub slot_width: f64,
}

impl SSR {
    /// Generate the main body of the SSR
    pub fn body(&self) -> CSG<()> {
        CSG::prism(self.length, self.width, self.height)
    }
    
    /// Generate the mounting holes of the SSR
    pub fn mounting_holes(&self) -> CSG<()> {
        let mut holes = CSG::new();
        let offsets = [
            (-self.hole_spacing / 2.0, 0.0),
            (self.hole_spacing / 2.0, 0.0),
        ];
        
        for &(x, y) in &offsets {
            let hole = CSG::cylinder(self.hole_diameter / 2.0, self.height)
                .translate(x, y, 0.0);
            holes = holes.union(&hole);
        }
        
        holes
    }
    
    /// Generate the full SSR model
    pub fn assemble(&self) -> CSG<()> {
        let mut ssr = self.body();
        let holes = self.mounting_holes();
        
        ssr = ssr.subtract(&holes);
        
        ssr
    }
}

