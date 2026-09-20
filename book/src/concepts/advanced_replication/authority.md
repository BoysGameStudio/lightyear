# Replication authority

Authority selects the writer whose state is accepted for a replicated entity.
It is distinct from replication visibility, player input ownership and the choice
to predict or interpolate presentation. A client supplying data is not automatic
permission to change gameplay state.

The current [authority module](../../../../crates/replication/replication/src/authority.rs)
owns `Authority`, `HasAuthority`, the broker, `GiveAuthority`, `RequestAuthority`
and transfer transitions. Use that protocol and its validation; do not toggle a
local marker as a substitute for notifying the other peers.

Transfers must preserve entity mapping and writer identity across peers. In-flight
messages and disconnects can overlap a transition, so old and new writer traffic
must be interpreted against the protocol state rather than arrival order alone.
Replicon's entity mapping and Lightyear's transport remain the delivery mechanism.

Inspect the authority regression cases in `crates/tests/src/` for the supported
transition shapes. Old design sketches about duplicated Confirmed/Predicted
entities or an `AuthorityPeer` field do not define this implementation.
