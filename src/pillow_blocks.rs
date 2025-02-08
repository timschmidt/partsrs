use csgrs::CSG;

/// Representation of a Pillow Block Bearing module
#[derive(Debug, Clone)]
pub struct PillowBlock {
    pub name: String,
    pub base_width: f64,
    pub base_length: f64,
    pub base_height: f64,
    pub bore_diameter: f64,
    pub housing_diameter: f64,
    pub housing_height: f64,
    pub mounting_hole_diameter: f64,
    pub mounting_hole_spacing: f64,
}

impl PillowBlock {
    /// Generate the base of the pillow block
    pub fn base(&self) -> CSG<()> {
        CSG::box_shape(self.base_length, self.base_width, self.base_height)
    }
    
    /// Generate the bearing housing
    pub fn housing(&self) -> CSG<()> {
        CSG::cylinder(self.housing_diameter / 2.0, self.housing_height)
            .translate(0.0, 0.0, self.base_height)
    }
    
    /// Generate the inner bore of the bearing
    pub fn bore(&self) -> CSG<()> {
        CSG::cylinder(self.bore_diameter / 2.0, self.housing_height + self.base_height)
    }
    
    /// Generate the mounting holes
    pub fn mounting_holes(&self) -> CSG<()> {
        let mut holes = CSG::new();
        let offsets = [
            (-self.mounting_hole_spacing / 2.0, 0.0),
            (self.mounting_hole_spacing / 2.0, 0.0),
        ];
        
        for &(x, y) in &offsets {
            let hole = CSG::cylinder(self.mounting_hole_diameter / 2.0, self.base_height)
                .translate(x, y, 0.0);
            holes = holes.union(&hole);
        }
        
        holes
    }
    
    /// Generate the full pillow block model
    pub fn assemble(&self) -> CSG<()> {
        let mut pillow_block = self.base();
        let housing = self.housing();
        let bore = self.bore();
        let holes = self.mounting_holes();
        
        pillow_block = pillow_block.union(&housing);
        pillow_block = pillow_block.subtract(&bore);
        pillow_block = pillow_block.subtract(&holes);
        
        pillow_block
    }
}

