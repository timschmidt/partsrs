use csgrs::CSG;

/// Representation of a Linear Bearing module
#[derive(Debug, Clone)]
pub struct LinearBearing {
    pub name: String,
    pub outer_diameter: f64,
    pub inner_diameter: f64,
    pub length: f64,
    pub ball_diameter: f64,
    pub ball_spacing: f64,
    pub ball_rows: u32,
    pub ball_columns: u32,
}

impl LinearBearing {
    /// Generate the outer shell of the bearing
    pub fn outer_shell(&self) -> CSG<()> {
        CSG::cylinder(self.outer_diameter / 2.0, self.length)
    }
    
    /// Generate the inner bore of the bearing
    pub fn inner_bore(&self) -> CSG<()> {
        CSG::cylinder(self.inner_diameter / 2.0, self.length)
    }
    
    /// Generate the ball bearing rows
    pub fn balls(&self) -> CSG<()> {
        let mut balls = CSG::new();
        let start_x = -((self.ball_columns as f64 - 1.0) * self.ball_spacing) / 2.0;
        let start_z = -((self.ball_rows as f64 - 1.0) * self.ball_spacing) / 2.0;
        
        for row in 0..self.ball_rows {
            for col in 0..self.ball_columns {
                let x_offset = start_x + col as f64 * self.ball_spacing;
                let z_offset = start_z + row as f64 * self.ball_spacing;
                let ball = CSG::sphere(self.ball_diameter / 2.0)
                    .translate(Vector3::new(x_offset, self.outer_diameter / 2.0, z_offset));
                balls = balls.union(&ball);
            }
        }
        
        balls
    }
    
    /// Generate the full linear bearing model
    pub fn assemble(&self) -> CSG<()> {
        let mut bearing = self.outer_shell();
        let bore = self.inner_bore();
        let balls = self.balls();
        
        bearing = bearing.subtract(&bore);
        bearing = bearing.union(&balls);
        
        bearing
    }
}

