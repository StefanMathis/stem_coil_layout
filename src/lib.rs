/*!
[`Zone`]: crate::Zone
[`CoilLayout`]: crate::CoilLayout

Types for specifying the positioning of coils in stem - a Simulation Toolbox for Electric Motors.

 */
#![doc = include_str!("../docs/main.md")]
#![deny(missing_docs)]

/// An index for a coil / "zone" of a winding.
///
/// The coils of a winding are placed inside "slots". Depending on the
/// [`CoilLayout`] of a winding, multiple coils may share one slot and occupy
/// different "layers" within that slot. This struct is an index to a particular
/// winding zone defined by [`slot`](Zone::slot) and [`layer`](Zone::layer)
/// index which can contain a coil. The following image shows the winding zones
/// for an air gap and a slotted winding:
#[doc = ""]
#[cfg_attr(
    feature = "doc-images",
    doc = "![Air gap and slotted winding][winding_zones]"
)]
#[cfg_attr(
    feature = "doc-images",
    embed_doc_image::embed_doc_image("winding_zones", "docs/img/winding_zones.svg")
)]
#[cfg_attr(
    not(feature = "doc-images"),
    doc = "**Doc images not enabled**. Compile docs with
    `cargo doc --features 'doc-images'` and Rust version >= 1.54."
)]
///
/// _This image was produced with `examples/winding_zone_plots.rs`._
///
/// [`Zone`] implements [`Ord`]: A zone is said to be greater than another one
/// if its [`slot`](Zone::slot) index is larger. If the [`slot`](Zone::slot)
/// indices are equal, the zone with the larger [`layer`](Zone::layer) index is
/// greater.
///
/// # Examples
///
/// ```
/// use stem_coil_layout::Zone;
///
/// let zone_a = Zone {slot: 0, layer: 0};
/// let zone_b = Zone {slot: 1, layer: 0};
/// let zone_c = Zone {slot: 0, layer: 1};
/// assert!(zone_a < zone_b);
/// assert!(zone_a < zone_c);
/// assert!(zone_c < zone_b);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Zone {
    /// Slot index of the zone.
    pub slot: u16,
    /// Layer index of the zone.
    pub layer: u16,
}

impl Zone {
    /// Returns a new [`Zone`].
    pub fn new(slot: u16, layer: u16) -> Self {
        return Self { slot, layer };
    }
}

impl From<Zone> for [u16; 2] {
    fn from(zone: Zone) -> Self {
        return [zone.slot, zone.layer];
    }
}

impl From<(u16, u16)> for Zone {
    fn from(value: (u16, u16)) -> Self {
        return Zone::new(value.0, value.1);
    }
}

impl From<[u16; 2]> for Zone {
    fn from(value: [u16; 2]) -> Self {
        return Zone::new(value[0], value[1]);
    }
}

impl PartialOrd for Zone {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        return Some(self.cmp(other));
    }
}

impl Ord for Zone {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.slot.cmp(&other.slot) {
            core::cmp::Ordering::Equal => return self.layer.cmp(&other.layer),
            ord => return ord,
        }
    }
}

impl std::fmt::Display for Zone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "zone: slot {}, layer {} ", self.slot, self.layer)
    }
}

use std::{cmp::Ordering, num::NonZeroU16};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Index of the layer in the slot bottom, left side quadrant of a quadruple
/// layer winding. Is used in the definition of that winding and hence exposed
/// here. See [`CoilLayout`].
pub const QUADRUPLE_LAYER_BOTTOM_LEFT: u16 = 0;

/// Index of the layer in the slot top, left side quadrant of a quadruple layer
/// winding. Is used in the definition of that winding and hence exposed here.
/// See [`CoilLayout`].
pub const QUADRUPLE_LAYER_TOP_LEFT: u16 = 1;

/// Index of the layer in the slot top, right side quadrant of a quadruple layer
/// winding. Is used in the definition of that winding and hence exposed here.
/// See [`CoilLayout`].
pub const QUADRUPLE_LAYER_TOP_RIGHT: u16 = 2;

/// Index of the layer in the slot bottom, right side quadrant of a quadruple
/// layer winding. Is used in the definition of that winding and hence exposed
/// here. See [`CoilLayout`].
pub const QUADRUPLE_LAYER_BOTTOM_RIGHT: u16 = 3;

/**
An enum defining the position of individual coils / winding layers within a slot.

This enum is used to represent the coil / layer positioning of different winding
types. For example, in a double-layer distributed winding, the two coils in a
slot are placed on top of each other (variant [`CoilLayout::DoubleVertical`]). By
contrast, a double-layer tooth-coil winding is represented by a
[`CoilLayout::DoubleHorizontal`]. The following drawing shows the layout for all
variants.
 */
#[doc = ""]
#[cfg_attr(
    feature = "doc-images",
    doc = "![Coil layout variants][cad_coil_layout]"
)]
#[cfg_attr(
    feature = "doc-images",
    embed_doc_image::embed_doc_image("cad_coil_layout", "docs/img/cad_coil_layout.svg")
)]
#[cfg_attr(
    not(feature = "doc-images"),
    doc = "**Doc images not enabled**. Compile docs with
    `cargo doc --features 'doc-images'` and Rust version >= 1.54."
)]
/**

In stem, the `Winding` trait from the
[stem_winding](https://crates.io/crates/stem_winding) crate requires the
implementation of a `coil_layout` method which returns the corresponding variant
of this enum. This is used to calculate properties like e.g. the slot leakage
inductance for motors.
 */
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum CoilLayout {
    /**
    A variant representing single-layer windings. The single coil / layer fills
    the entire winding area of the slot (but not the slot opening).
     */
    Single,
    /**
    A variant representing single-layer windings, usually casted squirrel-cage
    windings. The single coil / layer fills the slot completely (including the
    slot opening)
     */
    SingleFilled,
    /**
    A variant representing double layer windings (e.g. distributed windings).
    The coil in the first layer is placed at the slot bottom, the one in the
    second layer at the slot top.
     */
    DoubleVertical,
    /**
    A variant representing double layer windings (e.g. tooth-coil windings).
    The coil in the first layer is placed on the left side of the slot, the one
    in the second layer on the right side.
     */
    DoubleHorizontal,
    /**
    A variant representing a quadruple-layer winding, which is essentially a
    tooth-coil winding where the coils at the two slot sides are split in the
    middle again. The individual coils / layers are placed in a clockwise order,
    starting on the left side of the slot bottom.
     */
    Quadruple,
    /**
    A variant representing a winding with an arbitrary number of layers, which
    is equal to the value of the anonymous field of the variant. The individual
    layers are placed on top of each other, starting with the first layer at the
    slot bottom (see the drawing in the enum docstring). The height of the
    individual slot slices is identical.
     */
    MultiVertical(NonZeroU16),
}

impl CoilLayout {
    /**
    Returns the number of layers for `self`.

    # Examples

    ```
    use stem_coil_layout::CoilLayout;

    assert_eq!(CoilLayout::Single.layers().get(), 1);
    assert_eq!(CoilLayout::SingleFilled.layers().get(), 1);
    assert_eq!(CoilLayout::DoubleVertical.layers().get(), 2);
    assert_eq!(CoilLayout::DoubleHorizontal.layers().get(), 2);
    assert_eq!(CoilLayout::Quadruple.layers().get(), 4);
    assert_eq!(CoilLayout::MultiVertical(5.try_into().expect("not zero")).layers().get(), 5);
    ```
     */
    pub const fn layers(&self) -> NonZeroU16 {
        match self {
            CoilLayout::Single => NonZeroU16::new(1).expect("not zero"),
            CoilLayout::SingleFilled => NonZeroU16::new(1).expect("not zero"),
            CoilLayout::DoubleVertical => NonZeroU16::new(2).expect("not zero"),
            CoilLayout::DoubleHorizontal => NonZeroU16::new(2).expect("not zero"),
            CoilLayout::Quadruple => NonZeroU16::new(4).expect("not zero"),
            CoilLayout::MultiVertical(val) => *val,
        }
    }

    /**
    Returns the vertical position of the `first_layer` relative to the
    `second_layer` (as seen from the slot bottom)

    # Panics
    Panics if one of the layers is equal to or larger than the total number of
    layers of `self` (see [`CoilLayout::layers`]).

    # Examples

    ## DoubleVertical

    A [`CoilLayout::DoubleVertical`] aranges the first layer at the slot bottom
    and the second at the slot top. Hence, the first layer is "lesser" compared
    to the second.
    ```
    use stem_coil_layout::CoilLayout;

    let coil_layout = CoilLayout::DoubleVertical;

    assert_eq!(std::cmp::Ordering::Less, coil_layout.ordering_vertical(0, 1));
    assert_eq!(std::cmp::Ordering::Greater, coil_layout.ordering_vertical(1, 0));
    assert_eq!(std::cmp::Ordering::Equal, coil_layout.ordering_vertical(0, 0));
    ```

    ## DoubleHorizontal

    A [`CoilLayout::DoubleHorizontal`] aranges both layers in the same vertical
    position. Hence, both layers are "equal".
    ```
    use stem_coil_layout::CoilLayout;

    let coil_layout = CoilLayout::DoubleHorizontal;

    assert_eq!(std::cmp::Ordering::Equal, coil_layout.ordering_vertical(0, 1));
    assert_eq!(std::cmp::Ordering::Equal, coil_layout.ordering_vertical(1, 0));
    ```

    ## Quadruple

    A [`CoilLayout::Quadruple`] arranges its layers as follows:
    ```ignore
      0  3  |  1  2  |     <- slot bottom
      1  2  |  0  3  |     <- slot top
            |        |     <- air gap
     slot 0 | slot 1 | ...
    ```

    Correspondingly, 0 is equal to 3, 1 is equal to 2, and both 0 and 3 are
    lesser than 1 and 2:

    ```
    use stem_coil_layout::CoilLayout;

    let coil_layout = CoilLayout::Quadruple;

    assert_eq!(std::cmp::Ordering::Equal, coil_layout.ordering_vertical(0, 3));
    assert_eq!(std::cmp::Ordering::Less, coil_layout.ordering_vertical(0, 1));
    assert_eq!(std::cmp::Ordering::Greater, coil_layout.ordering_vertical(2, 3));
    ```

    ## MultiVertical

    A [`CoilLayout::MultiVertical`] arranges the all layers on top of each
    other, starting at the slot bottom.
    ```
    use stem_coil_layout::CoilLayout;

    let coil_layout = CoilLayout::MultiVertical(3);

    assert_eq!(std::cmp::Ordering::Less, coil_layout.ordering_vertical(0, 1));
    assert_eq!(std::cmp::Ordering::Less, coil_layout.ordering_vertical(0, 2));
    assert_eq!(std::cmp::Ordering::Greater, coil_layout.ordering_vertical(2, 1));
    ```
     */
    pub fn ordering_vertical(&self, first_layer: u16, second_layer: u16) -> Ordering {
        assert!(first_layer < self.layers().get());
        assert!(second_layer < self.layers().get());
        match self {
            CoilLayout::Single => return Ordering::Equal,
            CoilLayout::SingleFilled => return Ordering::Equal,
            CoilLayout::DoubleVertical => return first_layer.cmp(&second_layer),
            CoilLayout::DoubleHorizontal => return Ordering::Equal,
            CoilLayout::Quadruple => {
                if first_layer == second_layer {
                    return Ordering::Equal;
                }
                if first_layer == QUADRUPLE_LAYER_BOTTOM_LEFT
                    || first_layer == QUADRUPLE_LAYER_BOTTOM_RIGHT
                {
                    if second_layer == QUADRUPLE_LAYER_BOTTOM_LEFT
                        || second_layer == QUADRUPLE_LAYER_BOTTOM_RIGHT
                    {
                        return Ordering::Equal;
                    } else {
                        return Ordering::Less;
                    }
                } else {
                    if second_layer == QUADRUPLE_LAYER_BOTTOM_LEFT
                        || second_layer == QUADRUPLE_LAYER_BOTTOM_RIGHT
                    {
                        return Ordering::Greater;
                    } else {
                        return Ordering::Equal;
                    }
                }
            }
            CoilLayout::MultiVertical(_) => return first_layer.cmp(&second_layer),
        }
    }

    /**
    Returns true if this [`CoilLayout`] variant uses the slot opening as space
    for conductors and false otherwise.

    # Examples

    ```
    use stem_coil_layout::CoilLayout;

    // True for these coil layouts:
    assert!(CoilLayout::SingleFilled.includes_slot_opening());

    // False for all of these:
    assert!(!CoilLayout::Single.includes_slot_opening());
    assert!(!CoilLayout::DoubleHorizontal.includes_slot_opening());
    assert!(!CoilLayout::DoubleVertical.includes_slot_opening());
    assert!(!CoilLayout::Quadruple.includes_slot_opening());
    assert!(!CoilLayout::MultiVertical(5).includes_slot_opening());
    ```
     */
    pub fn includes_slot_opening(&self) -> bool {
        match self {
            CoilLayout::SingleFilled => true,
            _ => false,
        }
    }
}
