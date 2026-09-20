# System ordering

Use the owning plugin's system sets when placing application systems. The client
input sets in [inputs/client.rs](../../../../crates/inputs/inputs/src/client.rs)
separate writing local inputs, buffering/restoring them, preparing messages and cleanup.
For native input, emit input in `InputSystems::WriteClientInputs`; Leafwing/BEI
integration supplies its own action-state path. Simulation runs in fixed schedules.

[PredictionSystems](../../../../crates/replication/prediction/src/plugin.rs)
separates rollback, snapping to confirmed state, despawn processing and history updates.
Prediction requires the application-global `PredictionManager` and a supported topology.
Frame schedules, fixed simulation and rollback replay are not interchangeable clocks.

Use the [simple_box systems](../../../../examples/simple_box/src/shared.rs) and
[advanced setup](../../tutorial/advanced_systems.md) for current integration.
Packet transport and replication bridge ordering are installed by their plugins;
an old diagram of removed BufferInputs/Main sets is not their current schedule contract.
