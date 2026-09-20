# Interest management

Interest management is the concept of only replicating to clients the entities that they need.

For example: in a MMORPG, replicating only the entities that are "close" to the player.


There are two main advantages:
- bandwidth savings: it is pointless to replicate entities that are far away from the player, or that the player cannot interact with.
  Those bandwidth savings become especially important when you have a lot of concurrent connected clients.
- prevent cheating: if you replicate entities that the player is not supposed to see, there is a risk that clients read that data and use it to cheat.
  For example, in a RTS, you can avoid replicating units that are in fog-of-war.


## Current integration

Replication targets and Replicon visibility determine which client receives an
entity. Prediction/interpolation targets then select how receivers present it.
For room-based filtering, use the maintained
[network_visibility example](../../../../examples/network_visibility/README.md)
and its [server](../../../../examples/network_visibility/src/server.rs), which
assigns `Rooms` and `Replicate::to_clients` with prediction/interpolation targets.

The [replication send module](../../../../crates/replication/replication/src/send.rs)
owns visibility hooks, including replacement/removal behavior. Room membership is
an application policy; retaining a cached remote entity is not permission to reveal
new state. The removed `RelevanceManager`/`NetworkRelevanceMode` recipe is not a
supported way to configure this fork.
