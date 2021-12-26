use std::{collections::HashMap, f64::consts::FRAC_PI_2};

use fj::prelude::*;

#[no_mangle]
pub extern "C" fn model(_args: &HashMap<String, String>) -> fj::Shape {
    // # Prusa Mini Enclosure
    //
    // A friend offered to build an enclosure for my Prusa Mini out of plywood,
    // as long as I design it. This file presents the design and documents the
    // design intent. It is intentionally vague in some areas, as many decisions
    // are left up to my friend.
    //
    // All units are in millimeters.
    //
    //
    // ## Internal Dimensions
    //
    // Let's take a look at the internal dimensions of the enclosure, how I came
    // up with them, and the constraints they are driven by. The design intent
    // here is to define dimensions that fit the printer, allow me to handle the
    // printer, but are otherwise as small as practical, to not make the
    // enclosure overly large.
    //
    // ### Width
    //
    // The approximate width of the printer:
    let printer_width = 325.;

    // We need some extra wiggle room to take the printer into or out of the
    // enclosure. Here's a nice value for the left side:
    let margin_left = 30.;

    // The right side needs a larger margin. You'd typically lift the printer by
    // grasping the Z axis extrusion from the right. The following margin should
    // allow me to do that comfortably:
    let margin_right = 60.;

    // The final width is the sum of those numbers:
    let inner_width = printer_width + margin_left + margin_right;

    // ### Depth
    //
    // Measuring the depth of the printer is complicated by the fact that the Y
    // axis is moving front-to-back.
    //
    // Let's start with the length of the Y axis assembly's base:
    let y_assembly_base_depth = 285.;

    // If the Y axis is in its front-most position, it overhangs this much on
    // the front:
    let print_bed_overhang_front = 55.;

    // With a bit of additional margin, this results in the front margin:
    let margin_front = print_bed_overhang_front + 20.;

    // If the Y axis is in its back-most position, it overhangs this much on the
    // back:
    let print_bed_overhang_back = 50.;

    // We can ignore the electronics enclosure. It protrudes behind the Y axis
    // base, but is completely covered by the back overhang.
    //
    // In addition to the overhang, we need to consider the cable going to the
    // heated bed. This should provide enough clearance for the plug and the
    // cable, without bending it too much:
    let margin_heat_bed_cable = 60.;

    // Together, this results in the back margin:
    let margin_back = print_bed_overhang_back + margin_heat_bed_cable;

    // Inner depth is the sum of all of these:
    let inner_depth = y_assembly_base_depth + margin_front + margin_back;

    // ### Height
    //
    // Now the height. This one is the most straight-forward. First, the printer
    // height:
    let printer_height = 385.;

    // Next, a bit of margin on top to take it into or out of the enclosure:
    let margin_top = 30.;

    // Sum it up to get the total height:
    let inner_height = printer_height + margin_top;

    // These are the values for the inner dimensions:
    assert_eq!(inner_width, 415.);
    assert_eq!(inner_depth, 470.);
    assert_eq!(inner_height, 415.);

    // ## Outer Dimensions
    //
    // To compute the outer dimensions, we need to know the material strength.
    // The following is my current assumption:
    let material_strength = 12.;

    // This gives us the following outer dimensions:
    let outer_width = inner_width + material_strength * 2.;
    let outer_depth = inner_depth + material_strength * 2.;
    let outer_height = inner_height + material_strength * 2.;

    // These are the values for the outer dimensions:
    assert_eq!(outer_width, 439.);
    assert_eq!(outer_depth, 494.);
    assert_eq!(outer_height, 439.);

    // ## Tolerances
    //
    // Since all dimensions are based on guesstimated margins, there is mostly
    // some wiggle room. For width and depth, a few mm less or a few cm more
    // won't matter.
    //
    // However, height is a *critical dimension*. A few mm less won't matter
    // here either, but the height where I want to place the enclosure is
    // limited. As planned, there are only going to be a few mm of space left
    // above the enclosure.
    //
    // To be on the safe side, the height should be limited to this value:
    assert!(outer_height < 440.);

    // ## Structure
    //
    // Now that we got the dimensions, let's think about the structure of the
    // enclosure. I figure, it's best for the stability of the construction, if
    // there is a base piece where everything else rests on.
    #[rustfmt::skip]
    let base = [
        [         0.,          0.],
        [outer_width,          0.],
        [outer_width, outer_depth],
        [         0., outer_depth],
    ];
    let base = base.sketch().sweep(material_strength);

    // Left and right walls rest on the base and reach from front to back. They
    // don't reach to the outer height, to leave room for the top.
    #[rustfmt::skip]
    let side = [
        [          0.,          0.],
        [inner_height,          0.],
        [inner_height, outer_depth],
        [          0., outer_depth],
    ];
    let side = side
        .sketch()
        .sweep(material_strength)
        .rotate([0., 1., 0.], -FRAC_PI_2)
        .translate([0., 0., material_strength]);

    let left = side.translate([material_strength, 0., 0.]);
    let right = side.translate([outer_width, 0., 0.]);

    // The top rests on the left and right walls.
    let top = base.translate([0., 0., outer_height - material_strength]);

    // The back fills in the room left by the other parts.
    #[rustfmt::skip]
    let back = [
        [         0.,           0.],
        [inner_width,           0.],
        [inner_width, inner_height],
        [         0., inner_height],
    ];
    let back = back
        .sketch()
        .sweep(material_strength)
        .rotate([1., 0., 0.], FRAC_PI_2)
        .translate([material_strength, outer_depth, material_strength]);

    // We've only defined the walls here, but not how to join them together.
    // This is left for the builder to decide.

    // ## Door
    //
    // The previous definition leaves out the door. How that's going to look
    // exactly is going to be left to the builder, but here are a few thoughts:
    // - There should be a window in there that's as large as possible, to watch
    //   ongoing prints.
    // - Hinges should be placed on the left side. When putting the printer in
    //   or taking it out, the door is much more likely to be in the way on the
    //   right side.
    // - I can 3D print a handle, so if none is at hand during construction,
    //   that's not a problem
    //
    // ### Magnets
    //
    // To hold the door closed, I think magnets are a good solution that's also
    // easy to implement.
    //
    // I'm not sure how many would be appropriate, and where exactly to place
    // them. But they should be closed as close to the edge of the door as
    // practical, so their counterpart is not in the way when taking the printer
    // into or out of the enclosure.
    //
    // I can easily print magnet holders that I can screw to the enclosure. This
    // shouldn't be a problem and can easily be done after the enclosure has
    // been built.

    // ## Access Ports
    //
    // The printer needs to interface with the world outside of the enclosure in
    // various ways:
    // - Power cable
    // - Network cable
    // - USB ports
    // - Filament
    // - Power switch
    //
    // To accommodate these, the enclosure needs two openings. One on the back
    // side, one on the right side.
    //
    // In addition to providing the means to guide cables/filament through,
    // those openings need to be big enough to allow access to the printer's
    // ports. Cables need to be connected/disconnected, filament needs to be
    // loaded/unloaded, and the USB port takes a flash drive.
    //
    // I think the most practical way to address this, is to make the openings
    // rather large, so it is easy to access the inside. To prevent this from
    // causing an unwanted draft during printing, I can later print a panel that
    // covers the openings, still lets cables and filament through, and can be
    // removed whenever anything needs to be plugged/unplugged/loaded/unloaded.
    //
    // Since both openings must allow for the same kind of access, they can both
    // have the same height:
    let opening_height = 70.;

    // In addition, the lower boundaries of both openings are flush with the
    // upper surface of the base, i.e. the surface the printer stands on.
    //
    // Please note that the the position of those openings is specified in terms
    // of the distances from the _inner_ surfaces of the back and right walls.

    // ### Back Opening
    //
    // The back opening needs to accommodate the power and network ports. It
    // requires the following width:
    let back_opening_width = 130.;

    // Here's the distance from the inner surface of the right wall to the
    // boundary of the opening:
    let back_opening_to_right_wall = margin_right + 30.;
    assert_eq!(back_opening_to_right_wall, 90.);

    // ### Right Opening
    //
    // The right needs to accommodate the USB ports, filament, and power switch.
    // It requires the following width:
    let right_opening_width = 120.;

    // The distance from the inner surface of the back wall to the boundary of
    // the opening:
    let right_opening_to_back_wall = margin_back;
    assert_eq!(right_opening_to_back_wall, 110.);

    // Let's model the back opening.
    #[rustfmt::skip]
    let back_opening = [
        [                0.,             0.],
        [back_opening_width,             0.],
        [back_opening_width, opening_height],
        [                0., opening_height],
    ];
    let back_opening = back_opening
        .sketch()
        .sweep(material_strength)
        .rotate([1., 0., 0.], FRAC_PI_2)
        .translate([
            outer_width
                - material_strength
                - back_opening_width
                - back_opening_to_right_wall,
            outer_depth,
            material_strength,
        ]);

    // And the right opening.
    #[rustfmt::skip]
    let right_opening = [
        [            0.,                  0.],
        [opening_height,                  0.],
        [opening_height, right_opening_width],
        [            0., right_opening_width],
    ];
    let right_opening = right_opening
        .sketch()
        .sweep(material_strength)
        .rotate([0., 1., 0.], -FRAC_PI_2)
        .translate([
            outer_width,
            outer_depth
                - material_strength
                - right_opening_width
                - right_opening_to_back_wall,
            material_strength,
        ]);

    // Finally, let's put all of it together.
    let enclosure = base
        .union(&left)
        .union(&right)
        .union(&top)
        .union(&back)
        // TASK: Change to `difference`.
        .union(&back_opening)
        // TASK: Change to `difference`.
        .union(&right_opening);

    enclosure.into()
}
