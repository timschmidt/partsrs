use csgrs::CSG;

/// Representation of a Leadnut model
#[derive(Debug, Clone)]
pub struct Leadnut {
    pub outer_diameter: f64,
    pub inner_diameter: f64,
    pub length: f64,
    pub flange_diameter: f64,
    pub flange_thickness: f64,
    pub mounting_hole_diameter: f64,
    pub mounting_hole_spacing: f64,
}

impl Leadnut {
    /// Generate the cylindrical body of the leadnut
    pub fn body(&self) -> CSG<()> {
        CSG::cylinder_z(self.outer_diameter / 2.0, self.length)
    }
    
    /// Generate the inner bore of the leadnut
    pub fn bore(&self) -> CSG<()> {
        CSG::cylinder_z(self.inner_diameter / 2.0, self.length)
    }
    
    /// Generate the flange of the leadnut
    pub fn flange(&self) -> CSG<()> {
        CSG::cylinder_z(self.flange_diameter / 2.0, self.flange_thickness)
            .translate(Vector3::new(0.0, 0.0, self.length))
    }
    
    /// Generate the mounting holes of the leadnut
    pub fn mounting_holes(&self) -> CSG<()> {
        let mut holes = CSG::new();
        let offsets = [
            (-self.mounting_hole_spacing / 2.0, 0.0),
            (self.mounting_hole_spacing / 2.0, 0.0),
        ];
        
        for &(x, y) in &offsets {
            let hole = CSG::cylinder_z(self.mounting_hole_diameter / 2.0, self.flange_thickness)
                .translate(Vector3::new(x, y, self.length));
            holes = holes.union(&hole);
        }
        
        holes
    }
    
    /// Generate the full leadnut model
    pub fn assemble(&self) -> CSG<()> {
        let mut leadnut = self.body();
        let bore = self.bore();
        let flange = self.flange();
        let holes = self.mounting_holes();
        
        leadnut = leadnut.subtract(&bore);
        leadnut = leadnut.union(&flange);
        leadnut = leadnut.subtract(&holes);
        
        leadnut
    }
}

