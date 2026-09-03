stem_coil_layout
================

<!-- This file has ben generated with build.rs by concatenating docs/links.md,
docs/main.md and (if available docs/end.md). Do not modify this file, instead
modify the components. -->

[`Zone`]: https://docs.rs/stem_coil_layout/0.1.0/stem_coil_layout/struct.Zone.html
[`CoilLayout`]: https://docs.rs/stem_coil_layout/0.1.0/stem_coil_layout/enum.CoilLayout.html

[![Documentation](https://docs.rs/stem_coil_layout/badge.svg)](https://docs.rs/stem_coil_layout)

Types for specifying the positioning of coils in stem - a Simulation Toolbox for Electric Motors.

The full API documentation is available at <https://docs.rs/stem_core/0.1.0/stem_core>.

> **Feedback welcome!**  
> Found a bug, missing docs, or have a feature request?  
> Please open an issue on [GitHub](https://github.com/StefanMathis/stem_core.git).

[stem_winding]: https://crates.io/crates/stem_winding
[stem_slot]: https://crates.io/crates/stem_slot
[stem_core]: https://crates.io/crates/stem_core

This crate provides the [`CoilLayout`] and [`Zone`] types, which form the basic
building blocks for describing how coils are positioned within a winding.
[`CoilLayout`] describes how the winding layers are arranged within a slot,
while [`Zone`] identifies a particular winding position by its slot and layer
indices. The crate is intentionally small and is not intended to be used
stand-alone. It provides foundational types shared by [stem_winding],
[stem_slot], and [stem_core], without introducing a dependency on the more
comprehensive slot and core models. In particular, this separation allows
[stem_winding] to be used without [stem_slot] and [stem_core]. By default, its
dependency tree is therefore:

```text
stem_winding
└── stem_coil_layout
```

[stem_winding] provides an optional feature that enables its [stem_slot] and
[stem_core] dependencies. This enables additional functionality that requires
the slot and core models, such as calculating resistances or inductances.
With the feature enabled, the dependency tree is:

```text
stem_winding
├── stem_coil_layout
└── stem_core
    └── stem_slot
        └── stem_coil_layout
```

This keeps the basic winding model lightweight, while allowing it to make use of
the more detailed slot and core models when required.

# Acknowledgments

The technical drawings used in the docstrings have been created using 
LibreCAD (<https://librecad.org/>).