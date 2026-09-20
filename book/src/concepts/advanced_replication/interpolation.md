# Interpolation

Interpolation renders remote state on a delayed timeline so two received samples
can bound the displayed state. It trades latency for smoother motion between updates.

Register replicated/interpolated components through the component API. The current
[simple_box protocol](../../../../examples/simple_box/src/protocol.rs) uses
`app.component::<PlayerPosition>().replicate().predict().add_linear_interpolation()`
and implements Bevy's `Ease` for its position component. Choose recipients with
`InterpolationTarget::to_clients`; replication and interpolation targets are separate.

The [advanced tutorial](../../tutorial/advanced_systems.md) describes the setup;
[interpolation source](../../../../crates/replication/interpolation/src/lib.rs)
owns history, timing and customization APIs. Components derived from other state
need not each maintain independent interpolation histories. For coupled values,
keep their interpolation time and ownership consistent.
