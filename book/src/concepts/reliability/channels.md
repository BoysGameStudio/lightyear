# Channels


Lightyear introduces the concept of a `Channel` to handle reliability.

A `Channel` is a way to send packets with specific reliability, ordering and priority guarantees.

You can add a channel to your protocol like so:
```rust,ignore
pub struct MyChannel;

app.add_channel::<MyChannel>(ChannelSettings {
    mode: ChannelMode::OrderedReliable(ReliableSettings::default()),
    ..Default::default()
}).add_direction(NetworkDirection::Bidirectional);
```

The snippet belongs inside a protocol plugin with the Lightyear prelude imported.
See [the current protocol](../../../../examples/simple_box/src/protocol.rs) for
complete setup.


## Mode

The `mode` field of `ChannelSettings` defines the reliability/ordering guarantees of the channel.

Reliability:
- `Unreliable`: packets are not guaranteed to arrive
- `Reliable`: packets are guaranteed to arrive. We will resend the packet until we receive an acknowledgement from the remote.
  You can define how often we resend the packet via the `ReliableSettings` field.

Ordering:
- `Ordered`: packets are guaranteed to arrive in the order they were sent (*client sends 1,2,3,4,5, server receives 1,2,3,4,5*)
- `Unordered`: packets are not guaranteed to arrive in the order they were sent (*client sends 1,2,3,4,5, server receives 1,3,2,5,4*)
- `Sequenced`: packets are not guaranteed to arrive in the order they were sent, but we will discard packets that are older than the last received packet (*client sends 1,2,3,4,5, server receives 1,3,5 (2 and 4 are discarded)*)


## Direction

Use `.add_direction(NetworkDirection::...)` when registering a channel to select
client-to-server, server-to-client or bidirectional traffic.
