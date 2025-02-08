use csgrs::CSG;

/// Representation of a Circlip (Retaining Ring) model
#[derive(Debug, Clone)]
pub struct Circlip {
    pub nominal_od: f64,   // Outer Diameter (Tube diameter)
    pub groove_diameter: f64, // Groove diameter when installed
    pub relaxed_od: f64,   // Relaxed OD when not installed
    pub thickness: f64,    // Thickness
    pub lug_size: f64,     // Size of the lugs
    pub taper_width: f64,  // Widest part of the taper
    pub plier_hole_diameter: f64, // Plier hole diameter
    pub closed_angle: f64, // Closed angle (default to 25 degrees)
}

impl Circlip {
    /// Create a new circlip with default parameters
    pub fn new(
        nominal_od: f64,
        groove_diameter: f64,
        relaxed_od: f64,
        thickness: f64,
        lug_size: f64,
        taper_width: f64,
        plier_hole_diameter: f64,
        closed_angle: Option<f64>,
    ) -> Self {
        Circlip {
            nominal_od,
            groove_diameter,
            relaxed_od,
            thickness,
            lug_size,
            taper_width,
            plier_hole_diameter,
            closed_angle: closed_angle.unwrap_or(25.0),
        }
    }

    /// Generate the circlip ring shape
    pub fn ring(&self, open: i32) -> CSG<()> {
        let od = match open {
            -1 => self.nominal_od,
            0 => self.groove_diameter,
            1 => self.relaxed_od,
            _ => self.groove_diameter,
        };
        let inner_radius = self.nominal_od / 2.0;
        let outer_radius = od / 2.0;
        let csg_outer = CSG::cylinder(outer_radius, self.thickness);
        let csg_inner = CSG::cylinder(inner_radius, self.thickness);
        csg_outer.subtract(&csg_inner)
    }

    /// Generate the circlip lugs
    pub fn lugs(&self) -> CSG<()> {
        let hole = CSG::cylinder(self.plier_hole_diameter / 2.0, self.thickness);
        let lug = CSG::box_shape(self.lug_size, self.taper_width, self.thickness);
        lug.subtract(&hole)
    }

    /// Assemble the full circlip model
    pub fn assemble(&self, open: i32) -> CSG<()> {
        let mut circlip = self.ring(open);
        let lugs = self.lugs();
        circlip = circlip.union(&lugs);
        circlip
    }
}

