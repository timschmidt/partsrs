use csgrs::CSG;

/// Representation of a Ball Bearing
#[derive(Debug, Clone)]
pub struct BallBearing {
    pub outer_diameter: f64,
    pub inner_diameter: f64,
    pub width: f64,
}

impl BallBearing {
    /// Generate the outer race
    pub fn outer_race(&self) -> CSG<()> {
        CSG::cylinder(self.outer_diameter / 2.0, self.width)
    }

    /// Generate the inner race
    pub fn inner_race(&self) -> CSG<()> {
        CSG::cylinder(self.inner_diameter / 2.0, self.width + 1.0)
    }

    /// Assemble the complete ball bearing
    pub fn assemble(&self) -> CSG<()> {
        self.outer_race().difference(&self.inner_race())
    }
}

