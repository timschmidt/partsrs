use csgrs::CSG;

/// Representation of a Microswitch module
#[derive(Debug, Clone)]
pub struct Microswitch {
    pub name: String,
    pub body_width: f64,
    pub body_height: f64,
    pub body_depth: f64,
    pub lever_length: f64,
    pub lever_width: f64,
    pub lever_thickness: f64,
    pub mounting_hole_diameter: f64,
    pub mounting_hole_spacing: f64,
}

impl Microswitch {
    /// Generate the rectangular body of the microswitch
    pub fn body(&self) -> CSG<()> {
        CSG::box_shape(self.body_width, self.body_height, self.body_depth)
    }
    
    /// Generate the lever of the microswitch
    pub fn lever(&self) -> CSG<()> {
        CSG::box_shape(self.lever_length, self.lever_width, self.lever_thickness)
            .translate(0.0, self.body_height / 2.0, self.body_depth / 2.0)
    }
    
    /// Generate the mounting holes
    pub fn mounting_holes(&self) -> CSG<()> {
        let mut holes = CSG::new();
        let offsets = [
            (-self.mounting_hole_spacing / 2.0, 0.0),
            (self.mounting_hole_spacing / 2.0, 0.0),
        ];
        
        for &(x, y) in &offsets {
            let hole = CSG::cylinder(self.mounting_hole_diameter / 2.0, self.body_depth)
                .translate(x, y, 0.0);
            holes = holes.union(&hole);
        }
        
        holes
    }
    
    /// Generate the full microswitch model
    pub fn assemble(&self) -> CSG<()> {
        let mut microswitch = self.body();
        let lever = self.lever();
        let holes = self.mounting_holes();
        
        microswitch = microswitch.union(&lever);
        microswitch = microswitch.subtract(&holes);
        
        microswitch
    }
}

