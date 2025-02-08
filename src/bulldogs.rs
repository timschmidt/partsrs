use csgrs::CSG;

/// Representation of a Bulldog Clamp module
#[derive(Debug, Clone)]
pub struct BulldogClamp {
    pub name: String,
    pub body_width: f64,
    pub body_height: f64,
    pub body_depth: f64,
    pub jaw_width: f64,
    pub jaw_depth: f64,
    pub spring_diameter: f64,
    pub spring_length: f64,
}

impl BulldogClamp {
    /// Generate the rectangular body of the bulldog clamp
    pub fn body(&self) -> CSG<()> {
        CSG::box_shape(self.body_width, self.body_height, self.body_depth)
    }
    
    /// Generate the jaws of the bulldog clamp
    pub fn jaws(&self) -> CSG<()> {
        CSG::box_shape(self.jaw_width, self.body_height, self.jaw_depth)
            .translate(0.0, 0.0, self.body_depth / 2.0)
    }
    
    /// Generate the spring mechanism
    pub fn spring(&self) -> CSG<()> {
        CSG::cylinder(self.spring_diameter / 2.0, self.spring_length)
            .translate(0.0, 0.0, self.body_depth / 2.0)
    }
    
    /// Generate the full bulldog clamp model
    pub fn assemble(&self) -> CSG<()> {
        let mut clamp = self.body();
        let jaws = self.jaws();
        let spring = self.spring();
        
        clamp = clamp.union(&jaws);
        clamp = clamp.union(&spring);
        
        clamp
    }
}

