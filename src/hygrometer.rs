use csgrs::CSG;

/// Representation of a Hygrometer Module
#[derive(Debug, Clone)]
pub struct Hygrometer {
    pub width: f64,
    pub height: f64,
    pub depth: f64,
    pub sensor_diameter: f64,
}

impl Hygrometer {
    /// Generate the main casing of the hygrometer
    pub fn body(&self) -> CSG<()> {
        CSG::box_shape(self.width, self.height, self.depth)
    }

    /// Generate the sensor hole
    pub fn sensor_hole(&self) -> CSG<()> {
        CSG::cylinder(self.sensor_diameter / 2.0, self.depth)
            .translate(0.0, 0.0, self.depth / 2.0)
    }

    /// Assemble the complete hygrometer
    pub fn assemble(&self) -> CSG<()> {
        self.body().difference(&self.sensor_hole())
    }
}

