use csgrs::CSG;

/// Representation of a J-Head Hot End
#[derive(Debug, Clone)]
pub struct JHead {
    pub heat_sink_diameter: f64,
    pub heat_sink_height: f64,
    pub heat_break_diameter: f64,
    pub heat_break_length: f64,
    pub nozzle_diameter: f64,
    pub nozzle_length: f64,
}

impl JHead {
    /// Generate the heat sink
    pub fn heat_sink(&self) -> CSG<()> {
        CSG::cylinder_z(self.heat_sink_diameter / 2.0, self.heat_sink_height)
    }

    /// Generate the heat break
    pub fn heat_break(&self) -> CSG<()> {
        CSG::cylinder_z(self.heat_break_diameter / 2.0, self.heat_break_length)
            .translate(Vector3::new(0.0, 0.0, -self.heat_break_length))
    }

    /// Generate the nozzle
    pub fn nozzle(&self) -> CSG<()> {
        CSG::cylinder_z(self.nozzle_diameter / 2.0, self.nozzle_length)
            .translate(Vector3::new(0.0, 0.0, -self.nozzle_length - self.heat_break_length))
    }

    /// Assemble the complete hot end
    pub fn assemble(&self) -> CSG<()> {
        self.heat_sink().union(&self.heat_break()).union(&self.nozzle())
    }
}

