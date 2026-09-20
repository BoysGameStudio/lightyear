# Client-to-server replication

An application can send client-owned state to the server through a replication
link. `ReplicationSender` enables outgoing replication on the link;
`Replicate::to_server()` selects client-to-server delivery for an entity.
The [send module](../../../../crates/replication/replication/src/send.rs) owns
current target and link semantics. Components still require protocol registration.

Receiving an entity does not authorize its gameplay meaning. The application
must decide which client may create or change it and validate that input before
rebroadcast. Prediction and interpolation targets select presentation at receivers;
they are not permission checks or proof of authoritative state.

## Matching and ownership

Do not transmit a local Bevy entity ID as another peer's entity identity. Use the
replication mapper and declared entity-bearing component/message contracts.
For entities simulated independently on both peers, [prespawning](prespawning.md)
matches them through a shared signature before component updates. For changing
writers, follow [authority](authority.md), not removal of a replication component
at a guessed schedule point. The old `PrePredicted`/duplicate-Confirmed-entity
recipe is not this fork's supported API.
