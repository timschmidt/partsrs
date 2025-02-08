use csgrs::CSG;

/// Representation of a Panel Meter module
#[derive(Debug, Clone)]
pub struct PanelMeter {
    pub name: String,
    pub body_width: f64,
    pub body_height: f64,
    pub body_depth: f64,
    pub display_width: f64,
    pub display_height: f64,
    pub display_depth: f64,
    pub mounting_hole_diameter: f64,
    pub mounting_hole_spacing: f64,
}

impl PanelMeter {
    /// Generate the rectangular body of the panel meter
    pub fn body(&self) -> CSG<()> {
        CSG::prism(self.body_width, self.body_height, self.body_depth)
    }
    
    /// Generate the display of the panel meter
    pub fn display(&self) -> CSG<()> {
        CSG::prism(self.display_width, self.display_height, self.display_depth)
            .translate(Vector3::new(0.0, 0.0, self.body_depth - self.display_depth))
    }
    
    /// Generate the mounting holes
    pub fn mounting_holes(&self) -> CSG<()> {
        let mut holes = CSG::new();
        let offsets = [
            (-self.mounting_hole_spacing / 2.0, -self.mounting_hole_spacing / 2.0),
            (self.mounting_hole_spacing / 2.0, -self.mounting_hole_spacing / 2.0),
            (self.mounting_hole_spacing / 2.0, self.mounting_hole_spacing / 2.0),
            (-self.mounting_hole_spacing / 2.0, self.mounting_hole_spacing / 2.0),
        ];
        
        for &(x, y) in &offsets {
            let hole = CSG::cylinder_z(self.mounting_hole_diameter / 2.0, self.body_depth)
                .translate(Vector3::new(x, y, 0.0));
            holes = holes.union(&hole);
        }
        
        holes
    }
    
    /// Generate the full panel meter model
    pub fn assemble(&self) -> CSG<()> {
        let mut meter = self.body();
        let display = self.display();
        let holes = self.mounting_holes();
        
        meter = meter.union(&display);
        meter = meter.subtract(&holes);
        
        meter
    }
}

