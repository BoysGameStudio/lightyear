# Prespawning

Prespawning lets the client simulate a locally created entity before the server's
spawn arrives. Matching must reuse that candidate rather than duplicate it.
The current bridge uses Replicon's `Signature`; it does not create a separate
Confirmed entity plus a Predicted twin.

[`PreSpawned`](../../../../crates/replication/replication/src/prespawn.rs) accepts
an explicit shared hash via `new`, or the default archetype/tick hash with an
optional `user_salt`. Distinguish same-tick spawns by stable semantic identity,
not a peer-local Bevy entity index. Both peers must derive the same signature.

`for_client` scopes the sender's signature mapping to a client link; replication
visibility is independent. `for_receiver` associates a P2P input target with its
receiving peer link. `PreSpawnedReceiver` is application-global bookkeeping for
timeout cleanup, timeline shifts and rollback, not a per-link simulation clock.

See the [prespawn regressions](../../../../crates/tests/src/client_server/prediction/prespawn.rs) and
[fork contracts](../../../../docs/fork/CONTRACTS.md). Duplicates, mismatches,
timeouts and timeline shifts need the owning regression coverage; a matching
signature alone does not prove general rollback or game-spawn acceptance.
