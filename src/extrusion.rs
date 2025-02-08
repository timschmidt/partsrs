use csgrs::float_types::EPSILON;
use nalgebra::Vector3;

pub type CSG = csgrs::csg::CSG<()>;

/// For T‐slot extrusions, the “recess” can be `None` or `(width, depth)`.
/// The SCAD code shows either `false` or `[w, d]`.
#[derive(Debug, Clone)]
pub enum ChannelRecess {
    None,
    Some(f64, f64),
}

/// Representation of a T‐slot extrusion profile.
/// 
/// Mirrors the OpenSCAD parameters such as `E2020`, etc.
#[derive(Debug, Clone)]
pub struct ExtrusionProfile {
    pub name: &'static str,                // e.g. "E2020"
    pub width: f64,                  // overall width (X)
    pub height: f64,                 // overall height (Y)
    /// If <0, interpret as a circular hole of diameter = |center_hole_wd|.
    /// If >0, interpret as a square side = center_hole_wd.
    pub center_hole_wd: f64,
    /// If <0, interpret as circular hole in corners, diameter = |corner_hole_wd|.
    /// If >0, interpret as square side for corners, side = corner_hole_wd.
    pub corner_hole_wd: f64,
    /// The “center square” size if >0 (or tube diameter if <0).
    pub center_square_wd: f64,
    /// Channel opening width.
    pub channel_width: f64,
    /// Internal channel width (the smaller gap behind the outer slot).
    pub channel_width_internal: f64,
    /// The thickness of the “tab” that extends from the main body to the outer edges.
    pub tab_thickness: f64,
    /// The thickness of the internal spar(s).
    pub spar_thickness: f64,
    /// Radius of corner fillets (approximate).
    pub fillet_radius: f64,
    /// Recess geometry, either none or `(recess_width, recess_depth)`.
    pub recess: ChannelRecess,
}

impl ExtrusionProfile {
    /// Helper: if the parameter is negative, return a circle (diameter = |d|).
    /// If positive, return a square (side = d).
    fn circle_or_square(d: f64) -> CSG {
        if d.abs() < EPSILON {
            return CSG::new(); // degenerate
        }

        if d < 0.0 {
            // Circle with diameter = |d|
            let r = d.abs() / 2.0;
            CSG::circle(Some((r, 32))) // 32 segments for smoothness
        } else {
            // Square with side = d
            CSG::square(Some(([d, d], true))) // center it about origin
        }
    }

    /// Build the **2D cross‐section** in the XY plane (Z=0).  
    /// `corner_holes` indicates whether we open the corner holes in the profile or not.
    ///
    /// The final shape is centered at the origin. Width extends ±(width/2) in X,
    /// height extends ±(height/2) in Y.  
    /// This replicates the logic from `extrusion_cross_section(type, cornerHole)` in SCAD.
    pub fn cross_section_2d(&self, corner_holes: bool) -> CSG {
        let w = self.width;
        let h = self.height;
        let center_sq = self.center_square_wd.abs();
        let tab_t = self.tab_thickness;
        let spar_t = self.spar_thickness;
        let chan_w = self.channel_width;
        let recess = &self.recess;

        // The SCAD logic sometimes repeats horizontal “cells” if `height` > `width`.
        // e.g. for 20×40, we might have 2 “cells.” For 20×80, 4 “cells,” etc.
        // count = (height / width) - 1
        // If the ratio is not an integer, we do floor?
        let cell_size = w; // each “cell” is as tall as the width
        let n_cells = (h / w).round() as i32;
        let num_interior = if n_cells > 1 { n_cells - 1 } else { 0 };

        // -----------
        // 1) Build a single “cell” geometry: 
        //    - The main outer shape
        //    - The center square/tube
        //    - The center hole if center_hole_wd != 0
        //    - The corner lumps with optional corner holes
        // -----------
        //
        // In SCAD, they do difference() of union() of corners, etc. Here we approximate
        // by building multiple sub‐CSG shapes and using boolean ops.

        // Helper: main “cell” outline before center hole cut
        let cell_outline = self.build_main_cell_outline_2d();

        // Subtract a center hole if any
        let center_hole_csg = Self::circle_or_square(self.center_hole_wd);
        // We place it at the origin. Subtract from the cell:
        let mut cell_2d = cell_outline.subtract(&center_hole_csg);

        // Corner holes if requested
        if corner_holes && self.corner_hole_wd.abs() > EPSILON {
            let c_hole = Self::circle_or_square(self.corner_hole_wd);
            // Each corner is offset from center. The SCAD code’s “cornerSquare” = (width - channel_width_internal)/2, etc.
            // But simpler: the corner lumps are near ±(w/2), ±(w/2). 
            // If it’s a circle, diameter=|corner_hole_wd| => radius=…
            let r = self.corner_hole_wd.abs() / 2.0;
            let corner_positions = [
                (-1.0, -1.0),
                (1.0, -1.0),
                (1.0, 1.0),
                (-1.0, 1.0),
            ];
            // The offset in x,y from the center is ~ the “inside corner” portion. 
            // By analogy with SCAD, that corner is roughly ( (w-chan_in)/2 , (w-chan_in)/2 ).
            // But to keep it consistent with the “outer corner,” we can just place it near ±(w/2 - something).
            // For a 20×20 cell, the T-slot corner lumps are around ±7-8 mm from center.
            // For simplicity, just place at ±( (w-chan_w)/2 ), or you can reference the SCAD expression:
            let corner_sq = (w - self.channel_width_internal) / 2.0;

            for &(sx, sy) in &corner_positions {
                let tx = sx * corner_sq;
                let ty = sy * corner_sq;
                // Subtract the hole from the cell:
                let hole_t = c_hole.clone().translate(Vector3::new(tx, ty, 0.0));
                cell_2d = cell_2d.subtract(&hole_t);
            }
        }

        // That’s the shape for ONE horizontal “cell.” 
        // If we have multiple cells stacked in Y, replicate them with the needed internal bridging:
        let mut full_profile = CSG::new();

        // The bottom cell is placed so that its center is at Y= (n_cells-1)*cell_size/2 - i*cell_size
        // But to keep the entire shape centered at Y=0, we shift them so that [0..n_cells-1] are symmetrical about 0.
        // We can do a start_offset = -(h/2) + w/2 to place the first cell. Then each subsequent cell is +w in Y.
        let start_offset_y = -h/2.0 + w/2.0;
        for i in 0..n_cells {
            let y_off = start_offset_y + (i as f64)*cell_size;
            let this_cell = cell_2d.clone().translate(Vector3::new(0.0, y_off, 0.0));
            full_profile = full_profile.union(&this_cell);
        }

        // Add bridging “center section(s)” if n_cells >= 2
        if num_interior > 0 {
            // Per SCAD, we fill the gap between cells with internal spar geometry.
            // Rough approach: each bridging portion is a rectangular arrangement of tabs + diagonal or horizontal spars.
            // The original SCAD code calls `extrusion_center_section(type)`.
            // For simplicity, we replicate that by unioning those bridging shapes for each gap i in [1..(n_cells-1)].
            for i in 1..n_cells {
                let y_mid = -h/2.0 + (i as f64)*cell_size;
                let bridging = self.build_center_bridging_2d();
                let bridging_t = bridging.translate(Vector3::new(0.0, y_mid, 0.0));
                full_profile = full_profile.union(&bridging_t);
            }
        }

        // Lastly, if there is a channel recess (like E2020t or E4040t with `[width, depth]`),
        // we cut little recess rectangles at each open channel. 
        // The SCAD code has 4 channels around the perimeter for each cell.
        if let ChannelRecess::Some(recess_w, recess_d) = recess {
            // For each cell i, for each of the 4 sides rotated in 90° increments. 
            // We place a small rectangle whose center is at the outer edge. 
            // The SCAD code does: 
            //   translate([ width/2, 0 ]) square([recess.y*2, recess.x], center=true); 
            //   then rotates by 0,90,180,270
            //   for each cell. 
            //
            // So the recess rectangle is: [width = recess_d *2, height = recess_w].
            // And its center is offset in X by +w/2, then rotate. 
            // We do a difference from the shape.
            let rect = CSG::square(Some(([*recess_d * 2.0, *recess_w], true)));
            for i in 0..n_cells {
                let y_off = start_offset_y + (i as f64)*cell_size;
                for rot_k in 0..4 {
                    let angle_deg = 90.0 * (rot_k as f64);
                    let r = rect.clone()
                        .rotate(0.0, 0.0, angle_deg)
                        .translate(Vector3::new(w/2.0, 0.0, 0.0))  // SCAD’s final placement
                        .translate(Vector3::new(0.0, y_off, 0.0));
                    full_profile = full_profile.subtract(&r);
                }
            }
        }

        full_profile
    }

    /// Builds the outline for a single T‐slot “cell” without holes.
    /// Roughly replicates the union/difference logic for corners + center square, etc.
    fn build_main_cell_outline_2d(&self) -> CSG {
        let w = self.width;
        let chan_w = self.channel_width;
        let corner_sq = (w - self.channel_width_internal) / 2.0;
        let center_sq = self.center_square_wd.abs();
        let tab_t = self.tab_thickness;
        let spar_t = self.spar_thickness;
        let fillet_r = self.fillet_radius;

        // 1) corners
        // In SCAD, the corners are created by rectangles + circle fillet. 
        // We’ll approximate by union of squares, then subtract a small fillet circle if fillet>0.
        // Actually modeling the curved fillet precisely can be done by Minkowski, but let's do a simpler approach:  
        // - “L‐shaped” corner = union of two rectangles plus a quarter‐circle.  
        // We'll do a direct approach: we want an “outer box” = (w/2 × w/2) minus the channel opening region.
        //
        // We can do this corner shape once, then replicate it around ±X, ±Y.
        let corner = self.make_corner_2d();
        // We place it in the top‐right quadrant, then replicate in ±X ±Y.
        let mut all_corners = CSG::new();
        let corner_positions = [
            (1.0, 1.0),
            (-1.0, 1.0),
            (-1.0, -1.0),
            (1.0, -1.0),
        ];
        for &(sx, sy) in &corner_positions {
            let tx = sx * w/2.0;
            let ty = sy * w/2.0;
            let mut rot_deg = 0.0;
            if sx > 0.0 && sy > 0.0 {
                rot_deg = 0.0;
            } else if sx < 0.0 && sy > 0.0 {
                rot_deg = 90.0;
            } else if sx < 0.0 && sy < 0.0 {
                rot_deg = 180.0;
            } else if sx > 0.0 && sy < 0.0 {
                rot_deg = 270.0;
            }
            let c2 = corner.clone().rotate(0.0, 0.0, rot_deg)
                                  .translate(Vector3::new(tx, ty, 0.0));
            all_corners = all_corners.union(&c2);
        }

        // 2) The center square/tube region
        let center_area = Self::circle_or_square(self.center_square_wd);
        // 3) Possibly add 45° bridging “spar” rectangles that connect center to corners. 
        // The original SCAD code includes small diagonal or straight segments.  
        let bridging_spars = self.make_center_spars_2d();

        // Combine corners + center square + bridging
        let shape = all_corners.union(&center_area).union(&bridging_spars);
        shape
    }

    /// The “corner piece” for a single quadrant, ignoring fillet or doing an approximate fillet.
    fn make_corner_2d(&self) -> CSG {
        let w = self.width;
        let chan_w = self.channel_width;
        let tab_t = self.tab_thickness;
        let corner_sq = (w - self.channel_width_internal) / 2.0;
        let fillet_r = self.fillet_radius;

        // We'll do an L shape = union of two rectangles plus optional corner arc. 
        // One rectangle: width=(corner_sq), height=tab_t
        // Another rectangle: width=tab_t, height=(corner_sq).
        // Both anchored at the corner = (0,0).
        let rect1 = CSG::square(Some(([corner_sq, tab_t], false)));
        let rect2 = CSG::square(Some(([tab_t, corner_sq], false)))
            .translate(Vector3::new(0.0, tab_t, 0.0));

        let union_l = rect1.union(&rect2);

        // Also the “inner block” that forms the corner from the corner_sq × corner_sq region:
        // SCAD uses 
        //   translate([fillet, fillet]) square([cornerSquare - fillet, cornerSquare - fillet]);
        let main_corner_block = CSG::square(Some(([corner_sq, corner_sq], false)))
            .translate(Vector3::new(0.0, tab_t + tab_t, 0.0)); 
        // The above line is a bit simplified. If we want to replicate the fillet cut, we can do:
        //  let fillet_circle = CSG::circle(Some((fillet_r, 16)));
        //  ... subtract from the corner?

        // For brevity, just union them all. If you want a precise fillet, do a difference with a quarter circle.
        let corner = union_l.union(&main_corner_block);
        corner
    }

    /// The bridging bars/spars between the center square and corners, to fill out the T‐slot shape.
    fn make_center_spars_2d(&self) -> CSG {
        // The SCAD code draws 4 diagonal or horizontal bars from the center square to near the corners.
        // We do a simplistic approach: 4 rectangles, each bridging from the center out. 
        let w = self.width;
        let corner_sq = (w - self.channel_width_internal) / 2.0;
        let center_sq = self.center_square_wd.abs();
        let spar_t = self.spar_thickness;

        // We place each bar at 45° or horizontally. The code in SCAD is fairly detailed, so we can approximate:
        // We'll place them at angles 45, 135, 225, 315, with length from center_sq/2 to corner_sq - fillet maybe. 
        // For simplicity, create a single rectangle that is wide = spar_t, length = (some) and rotate it.

        // We can guess the bar length from the SCAD snippet:
        //   l = cornerSquare + sparThickness * tan(22.5) - sparThickness / sqrt(2)
        // That’s a bit arcane. For a simpler approach, we’ll just do a bar from the center's outer boundary to corner's inner boundary.
        // Let's replicate the formula from SCAD:
        let l = corner_sq + spar_t * (22.5_f64.to_radians().tan()) - spar_t / std::f64::consts::SQRT_2;

        // Create a rectangle [length x spar_t], centered, then rotate 45°, 135°, etc.
        let rect = CSG::square(Some(([l, spar_t], true)));
        let mut union_spars = CSG::new();
        let angles = [45.0, 135.0, 225.0, 315.0];
        for a in angles {
            // Place so that its center is at the midpoint between centerSq/2 and cornerSq in radial direction:
            // Actually, we can just rotate in place. The rectangle is centered at origin. Then we translate outward.
            // The radial offset might be (center_sq/2 + corner_sq)/2. Let's do the SCAD approach:
            //   offset ~ cornerSquare/2 + ?
            let offset = (corner_sq + center_sq)/4.0; 
            let bar = rect.clone().rotate(0.0, 0.0, a)
                          .translate(Vector3::new(offset, 0.0, 0.0));
            union_spars = union_spars.union(&bar);
        }
        union_spars
    }

    /// For “multi‐cell” extrusions (like 20×40, 20×80), the region between cells
    /// is filled by center bridging: a pair of rectangles bridging the center squares, plus diagonal braces, etc.
    /// The SCAD code calls it `extrusion_center_section(type)`.  
    /// Here we do a simpler approach: two vertical spar strips + diagonal squares. 
    pub fn build_center_bridging_2d(&self) -> CSG {
        let w = self.width;
        let corner_sq = (w - self.channel_width_internal) / 2.0;
        let center_sq = self.center_square_wd.abs();
        let tab_t = self.tab_thickness;
        let spar_t = self.spar_thickness;

        // For each side ±X, we have a vertical strip bridging the outside corners:
        let strip = CSG::square(Some(([tab_t, w - self.channel_width], true)));
        // then shift it so its center is at x= (width/2 - tab_t/2).
        let mut union_br = CSG::new();
        for side in &[-1.0, 1.0] {
            let x_off = side * (w/2.0 - tab_t/2.0);
            let st = strip.clone().translate(Vector3::new(x_off, 0.0, 0.0));
            union_br = union_br.union(&st);
        }

        // Then add diagonal or horizontal rods connecting the center square to corners. 
        // For a simpler approach, we can do small squares of size [l × spar_t], repeated. 
        let bar_l = corner_sq + spar_t*(22.5_f64.to_radians().tan()) - spar_t/(std::f64::consts::SQRT_2);
        for side_x in &[-1.0, 1.0] {
            for side_y in &[-1.0, 1.0] {
                let bar = CSG::square(Some(([bar_l, spar_t], true)));
                // rotate if side_y > 0? etc. 
                let angle = if *side_y > 0.0 { 0.0 } else { 180.0 };
                let b2 = bar.clone().rotate(0.0, 0.0, angle)
                            .translate(Vector3::new(side_x * (w/2.0 - bar_l/2.0),
                                                    side_y*(corner_sq - spar_t/2.0),
                                                    0.0));
                union_br = union_br.union(&b2);
            }
        }

        union_br
    }

    /// Create the **3D** extrusion along Z.
    ///
    /// - `length`: length along Z
    /// - `center`: if true, extrude so that Z goes ±(length/2).
    /// - `corner_holes`: if true, also subtract the corner hole cylinders from the final.
    ///
    /// The returned `CSG` is the full T‐slot solid.
    pub fn extrude_3d(&self, length: f64, center: bool, corner_holes: bool) -> CSG {
        // 1) Build the 2D cross‐section in XY
        let cross_2d = self.cross_section_2d(corner_holes);

        // 2) Extrude it in +Z or ±Z
        let solid = if center {
            cross_2d.extrude(length).translate(Vector3::new(0.0, 0.0, -length/2.0))
        } else {
            cross_2d.extrude(length)
        };

        // 3) If we want *drilled* corner holes (the code might do that in addition to the open slots),
        //    we can also subtract vertical cylinders at each corner. 
        //    However, the SCAD code only does corner holes if `corner_hole = true`, which we
        //    already included in the cross section. Typically, that’s *slots*, not “post‐extrude drilling.” 
        //
        //    If you actually want physical round holes at ±(width/2, ±(width/2)), do:
        //    (But note this might be redundant with the cross‐section logic.)
        // 
        //    if corner_holes && self.corner_hole_wd < 0.0 {
        //        let r = self.corner_hole_wd.abs()/2.0;
        //        let offsets = [
        //            ( -self.width/2.0, -self.width/2.0 ),
        //            (  self.width/2.0, -self.width/2.0 ),
        //            (  self.width/2.0,  self.width/2.0 ),
        //            ( -self.width/2.0,  self.width/2.0 ),
        //        ];
        //        for &(x, y) in &offsets {
        //            let cyl = CSG::cylinder_z(r, length);
        //            let mut c2 = cyl;
        //            if center {
        //                c2 = c2.translate(Vector3::new(0.0, 0.0, -length/2.0));
        //            }
        //            let trans = Vector3::new(x, y, 0.0);
        //            c2 = c2.translate(trans);
        //            solid = solid.subtract(&c2);
        //        }
        //    }

        solid
    }
}

/// Example static definitions akin to the SCAD `E1515`, `E2020`, etc.
pub const E1515: ExtrusionProfile = ExtrusionProfile {
    name: "E1515",
    width: 15.0,
    height: 15.0,
    center_hole_wd: -3.3, // negative => circle diameter 3.3
    corner_hole_wd: 0.0,  // 0 => no corner holes
    center_square_wd: 5.5,
    channel_width: 6.2,
    channel_width_internal: 9.5,
    tab_thickness: 1.0,
    spar_thickness: 0.9,
    fillet_radius: 0.5,
    recess: ChannelRecess::None,
};

pub const E2020: ExtrusionProfile = ExtrusionProfile {
    name: "E2020",
    width: 20.0,
    height: 20.0,
    center_hole_wd: -4.2,
    corner_hole_wd: -3.0,
    center_square_wd: 8.0,
    channel_width: 6.0,
    channel_width_internal: 12.0,
    tab_thickness: 2.0,
    spar_thickness: 2.0,
    fillet_radius: 1.0,
    recess: ChannelRecess::None,
};

pub const E2020T: ExtrusionProfile = ExtrusionProfile {
    name: "E2020t",
    width: 20.0,
    height: 20.0,
    center_hole_wd: -5.0,
    corner_hole_wd: -3.0,
    center_square_wd: 7.8,
    channel_width: 6.2,
    channel_width_internal: 11.0,
    tab_thickness: 1.8,
    spar_thickness: 1.5,
    fillet_radius: 1.5,
    recess: ChannelRecess::Some(7.2, 0.5), // from the SCAD array: [7.2, 0.5]
};

// E2040, E2060, etc.

pub const ALL_EXTRUSIONS: &[ExtrusionProfile] = &[
    E1515,
    E2020,
    E2020T,
    // E2040, E2060, ...
];
