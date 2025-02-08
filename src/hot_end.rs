use csgrs::CSG;

/// Representation of a Hot End module for 3D printers
#[derive(Debug, Clone)]
pub struct HotEnd {
    pub name: String,
    pub heater_block_width: f64,
    pub heater_block_height: f64,
    pub heater_block_depth: f64,
    pub nozzle_diameter: f64,
    pub nozzle_length: f64,
    pub heat_break_diameter: f64,
    pub heat_break_length: f64,
}

impl HotEnd {
    /// Generate the heater block of the hot end
    pub fn heater_block(&self) -> CSG<()> {
        CSG::prism(self.heater_block_width, self.heater_block_height, self.heater_block_depth)
    }
    
    /// Generate the nozzle of the hot end
    pub fn nozzle(&self) -> CSG<()> {
        CSG::cylinder(self.nozzle_diameter / 2.0, self.nozzle_length)
            .translate(Vector3::new(0.0, 0.0, -self.nozzle_length))
    }
    
    /// Generate the heat break of the hot end
    pub fn heat_break(&self) -> CSG<()> {
        CSG::cylinder(self.heat_break_diameter / 2.0, self.heat_break_length)
            .translate(Vector3::new(0.0, 0.0, self.heater_block_height))
    }
    
    /// Generate the full hot end model
    pub fn assemble(&self) -> CSG<()> {
        let mut hot_end = self.heater_block();
        let nozzle = self.nozzle();
        let heat_break = self.heat_break();
        
        hot_end = hot_end.union(&nozzle);
        hot_end = hot_end.union(&heat_break);
        
        hot_end
    }
}

