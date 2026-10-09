stem_types
==========

<!-- This file has ben generated with build.rs by concatenating docs/links.md,
docs/main.md and (if available docs/end.md). Do not modify this file, instead
modify the components. -->

[`Zone`]: https://docs.rs/stem_types/0.1.0/stem_types/struct.Zone.html
[`CoilLayout`]: https://docs.rs/stem_types/0.1.0/stem_types/enum.CoilLayout.html
[`SpatialOrder`]: https://docs.rs/stem_types/0.1.0/stem_types/enum.SpatialOrder.html

[![Documentation](https://docs.rs/stem_types/badge.svg)](https://docs.rs/stem_types)

Types for describing common concepts in stem - a Simulation Toolbox for Electric Motors.

The full API documentation is available at <https://docs.rs/stem_core/0.1.0/stem_core>.

> **Feedback welcome!**  
> Found a bug, missing docs, or have a feature request?  
> Please open an issue on [GitHub](https://github.com/StefanMathis/stem_core.git).

[stem_winding]: https://crates.io/crates/stem_winding
[stem_slot]: https://crates.io/crates/stem_slot
[stem_core]: https://crates.io/crates/stem_core

This crate provides the [`CoilLayout`], [`Zone`], and [`SpatialOrder`] types,
which form basic building blocks for describing windings and their spatial
properties.

[`CoilLayout`] describes how the winding layers are arranged within a slot,
while [`Zone`] identifies a particular winding position by its slot and layer
indices. [`SpatialOrder`] represents a spatial harmonic order either in
mechanical or electrical coordinates.

The crate is intentionally small and is not intended to be used stand-alone. It
provides foundational types shared by [stem_winding], [stem_slot], and
[stem_core], without introducing a dependency on the more comprehensive slot and
core models. In particular, this separation allows [stem_winding] to be used
without [stem_slot] and [stem_core]. By default, its dependency tree i
 therefore:

```text
stem_winding
└── stem_types
```

[stem_winding] provides an optional feature that enables its [stem_slot] and
[stem_core] dependencies. This enables additional functionality that requires
the slot and core models, such as calculating resistances or inductances.

With the feature enabled, the dependency tree is:

```text
stem_winding
├── stem_types
└── stem_core
    └── stem_slot
        └── stem_types
```

This keeps the basic winding model lightweight, while allowing it to make use of
the more detailed slot and core models when required.

## Acknowledgments

The technical drawings used in the docstrings have been created using
[LibreCAD](https://librecad.org/).