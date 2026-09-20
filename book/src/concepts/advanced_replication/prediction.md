# Client-side Prediction

## Introduction

Client-side prediction means that some entities are on the 'client' timeline instead of the 'server' timeline:
they are updated instantly on the client.

Predicted state is corrected using confirmed component histories on the replicated
entity. The current pipeline does not require a separate duplicate Confirmed entity.
The [advanced tutorial](../../tutorial/advanced_systems.md) shows component registration,
target selection and the application-global `PredictionManager`.

## Wrong predictions and rollback

Sometimes, the client will predict something, but the server's version won't match what the client has predicted.
For example the client moves their character by 1 unit, but the server doesn't move the character because it detects
that the character was actually stunned by another player at that time and couldn't move.
(the client could not have predicted this because the 'stun' action from the other player hasn't been replicated yet).

In those cases the client will have to perform a rollback.
Let's say the client entity is now at tick T', but the client is only receiving the server update for tick T. (T < T')
Every time the client receives an update for confirmed state at tick T, it will:

- check for each updated component if it matches what the predicted version for tick T was
- if it doesn't, it will restore all the components to the confirmed version at tick T
- then the client will replay all the systems for the predicted entity from tick T to T'

## Locally spawned entities

For entities produced on both peers, use [prespawning](prespawning.md) and stable
signatures. Client-issued state uses [client replication](client_replication.md)
and explicit authority policy; do not assume local creation grants server trust.

## Which components should I predict?

You want to predict components for entities that will live in the Predicted timeline, i.e. the timeline that will see changes immediately based on client inputs. However it doesn't mean that every component needs to be actively be predicted with `the component API (`.predict()`)`, i.e. with rollbacks enabled. A lot of components can be computed from other components and don't need to be predicted or even sent through the network. Here is a quick explanation of which components should be predicted.

As a rule of thumb, a component should be predicted with `the component API (`.predict()`)` if it meets the following criteria:

1. Cannot be calculated using the **predicted** components available within the **current** tick.
2. May be modified after creation.

As an example let's look at the `avian3d::position::Position` component provided by the physics simulator `avian`. It describes the position of a 3D object. Its value is modified at the end of each tick by the `avian` physics simulator so it meets the second criteria. If a system wants to know the value of a `Position` component during a given tick, it has no way of calculating that value. Instead, the system will query the `Position` component whose value was calculated in the **previous** tick by the `avian` physics simulator. This means that `Position` also meets the first criteria and so it should be predicted.

A more subtle example is a system that wants to calculate how quickly the length of a ray cast changes. The system would need to know the length of the ray cast in the **previous** tick in order to compare it to the length of the ray cast in then **current** tick. This previous length will have to be stored in a component and that component meets the first criteria as the value it stores cannot be calculated in the **current** tick. The system would then have to save the ray cast's current length in that component after the calculation so that it can be used in the next tick. This is a modification of the component and so it meets the second criteria as well and should be predicted. Here's how the the component and system are defined:
```rust
/// Stores the length of the ray cast from the previous tick.
#[derive(Component)]
struct RayCastPrevLength(f32)

fn calculate_ray_cast_speed(time: Res<Time>, mut query: Query<(&mut RayCastPrevLength, &Position)>) {
  for (mut ray_cast_prev_length, position) in &mut query {
    let curr_ray_cast_length = perform_ray_cast();

    // Perform calculation that relies on information from previous tick.
    let ray_cast_speed = (curr_ray_cast_length - ray_cast_prev_length.0) / time.delta_secs();

    // Do something with ray cast speed.

    // Save current ray cast length to be used in the next tick.
    ray_cast_prev_length.0 = curr_ray_cast_length;
  }
}
```

If you stored the ray cast speed in a component so that it can be used by other systems then the component does **not** need to be predicted. It is modified every tick so it meets the second criteria, however, it's value is calculated using **predicted** components available in the **current** tick (`RayCastPrevLength`) and so it does not meet the first criteria and does not need to be predicted.

## Rollback coverage

Component addition/removal, despawn, history misses and deeper later rollbacks
must preserve the [fork contracts](../../../../docs/fork/CONTRACTS.md).
The current regression suite lives in `crates/tests/src/client_server/prediction/`;
old investigation checkmarks are not evidence of a run on this revision.
