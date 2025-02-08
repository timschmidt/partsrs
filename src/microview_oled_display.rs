use csgrs::CSG;

/// Representation of a MicroView OLED Display
#[derive(Debug, Clone)]
pub struct MicroView {
    pub width: f64,
    pub height: f64,
    pub depth: f64,
    pub screen_width: f64,
    pub screen_height: f64,
}

impl MicroView {
    /// Generate the main housing
    pub fn body(&self) -> CSG<()> {
        CSG::box_shape(self.width, self.height, self.depth)
    }

    /// Generate the screen cutout
    pub fn screen(&self) -> CSG<()> {
        CSG::box_shape(self.screen_width, self.screen_height, self.depth + 1.0)
            .translate(0.0, 0.0, self.depth / 2.0)
    }

    /// Assemble the full MicroView display
    pub fn assemble(&self) -> CSG<()> {
        self.body().difference(&self.screen())
    }
}

