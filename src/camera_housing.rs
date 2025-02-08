use csgrs::CSG;

/// Representation of a Camera Housing
#[derive(Debug, Clone)]
pub struct CameraHousing {
    pub width: f64,
    pub height: f64,
    pub depth: f64,
    pub lens_diameter: f64,
    pub lens_depth: f64,
}

impl CameraHousing {
    /// Generate the housing body
    pub fn body(&self) -> CSG<()> {
        CSG::prism(self.width, self.height, self.depth)
    }

    /// Generate the lens hole
    pub fn lens_hole(&self) -> CSG<()> {
        CSG::cylinder_z(self.lens_diameter / 2.0, self.lens_depth + 1.0)
            .translate(Vector3::new(0.0, 0.0, self.depth / 2.0))
    }

    /// Assemble the complete camera housing
    pub fn assemble(&self) -> CSG<()> {
        self.body().difference(&self.lens_hole())
    }
}

