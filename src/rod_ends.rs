use csgrs::CSG;

/// Representation of a Rod End Bearing module
#[derive(Debug, Clone)]
pub struct RodEnd {
    pub name: String,
    pub ball_diameter: f64,
    pub housing_diameter: f64,
    pub housing_thickness: f64,
    pub shank_diameter: f64,
    pub shank_length: f64,
}

impl RodEnd {
    /// Generate the spherical ball of the rod end bearing
    pub fn ball(&self) -> CSG<()> {
        CSG::sphere(self.ball_diameter / 2.0)
    }
    
    /// Generate the housing of the rod end bearing
    pub fn housing(&self) -> CSG<()> {
        CSG::cylinder(self.housing_diameter / 2.0, self.housing_thickness)
    }
    
    /// Generate the shank of the rod end bearing
    pub fn shank(&self) -> CSG<()> {
        CSG::cylinder(self.shank_diameter / 2.0, self.shank_length)
            .translate(0.0, 0.0, -self.shank_length)
    }
    
    /// Generate the full rod end bearing model
    pub fn assemble(&self) -> CSG<()> {
        let mut rod_end = self.housing();
        let ball = self.ball();
        let shank = self.shank();
        
        rod_end = rod_end.union(&ball);
        rod_end = rod_end.union(&shank);
        
        rod_end
    }
}

