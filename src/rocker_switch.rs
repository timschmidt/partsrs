use csgrs::CSG;

/// Representation of a Rocker Switch
#[derive(Debug, Clone)]
pub struct RockerSwitch {
    pub width: f64,
    pub height: f64,
    pub depth: f64,
    pub rocker_width: f64,
    pub rocker_height: f64,
}

impl RockerSwitch {
    /// Generate the outer casing of the switch
    pub fn casing(&self) -> CSG<()> {
        CSG::box_shape(self.width, self.height, self.depth)
    }

    /// Generate the rocker part
    pub fn rocker(&self) -> CSG<()> {
        CSG::box_shape(self.rocker_width, self.rocker_height, self.depth / 2.0)
            .translate(0.0, 0.0, self.depth / 4.0)
    }

    /// Assemble the full rocker switch
    pub fn assemble(&self) -> CSG<()> {
        self.casing().union(&self.rocker())
    }
}

