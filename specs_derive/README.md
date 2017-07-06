# Specs Procedural Derive Macros

[![Build Status][bi]][bl] [![Crates.io][ci]][cl] [![Gitter][gi]][gl] ![MIT/Apache][li] [![Docs.rs][di]][dl]

[bi]: https://travis-ci.org/slide-rs/specs.svg?branch=master
[bl]: https://travis-ci.org/slide-rs/specs

[ci]: https://img.shields.io/crates/v/specs.svg
[cl]: https://crates.io/crates/specs/

[li]: https://img.shields.io/badge/license-Apache%202.0-blue.svg

[di]: https://docs.rs/specs/badge.svg
[dl]: https://docs.rs/specs/

[gi]: https://badges.gitter.im/slide-rs/specs.svg
[gl]: https://gitter.im/slide-rs/specs


## Component Grouping

Component grouping allows for non-dynamic dispatch on multiple components without a lot of boilerplate.
Normally if you wanted to generically call a method on a bunch of components you would need to do something
similar to:

```rust
fn method<C: Component>() { ... }
method::<Component1>();
method::<Component2>();
...
```

Which can easily become tedious when it becomes necessary to do this for a changing amount of components.
A side benefit of using this approach means you do not need any dynamic dispatch to do so.

### Usage

Component groups are defined using a simple `struct` and deriving the `ComponentGroup` trait.
`group` attributes can be used to modify the components and subgroups in the group.

```rust
#[derive(ComponentGroup)]
struct ExampleGroup {
    // The group defaults to just a component.
    //
    // The field name "component1" will be used as an
    // unique identifier.
    component1: Component1,
    // If you need a subgroup, then you need to
    // designate the fields that are subgroups.
    #[group(subgroup)]
    subgroup1: Subgroup1,

    // Component grouping comes with built in support
    // for serialization and deserialization with `serde`
    // usage
    #[group(serialize)]
    serialize1: CompSerialize1,
}
```

### Attributes

All attributes used in a `ComponentGroup` derive are designated with a `group` prefix `#[group(...)]`.

`#[group(subgroup)]`

Field is a subgroup, the parent group will try to act like the subgroup's members (components and 
nested subgroups) are included in operations.

`#[group(serialize)]`

Field should be serialized. Note: This requires that all
components implement `Serialize`/`Deserialize`.

`#[group(id = "0")]`

Uses a dynamic component id for this component. Default is `0usize`.
