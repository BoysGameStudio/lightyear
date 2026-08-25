use crate::action_state::LeafwingUserAction;
use bevy_app::{App, Plugin};
#[cfg(feature = "client")]
use bevy_app::{FixedPostUpdate, FixedPreUpdate};
#[cfg(feature = "client")]
use bevy_ecs::schedule::IntoScheduleConfigs;
#[cfg(feature = "client")]
use bevy_ecs::system::Query;
use leafwing_input_manager::action_state::ActionState;
#[cfg(feature = "client")]
use lightyear_core::prelude::is_in_rollback;
use lightyear_inputs::config::InputConfig;
use lightyear_inputs::input_buffer::InputBuffer;
#[cfg(feature = "client")]
use lightyear_sync::client::ClientPlugin as LightyearClientPlugin;

#[cfg(any(feature = "client", feature = "server"))]
use crate::input_message::LeafwingSequence;
#[cfg(any(feature = "client", feature = "server"))]
use leafwing_input_manager::plugin::InputManagerPlugin;
#[cfg(any(feature = "client", feature = "server"))]
use tracing::trace;

pub struct InputPlugin<A> {
    pub config: InputConfig<A>,
}

#[cfg(feature = "client")]
fn sync_fixed_update_state_after_rollback<A: LeafwingUserAction>(
    mut action_states: Query<&mut ActionState<A>>,
) {
    for mut action_state in &mut action_states {
        action_state.set_fixed_update_state_from_state();
    }
}

impl<A> Default for InputPlugin<A> {
    fn default() -> Self {
        Self {
            config: Default::default(),
        }
    }
}

impl<A: LeafwingUserAction> Plugin for InputPlugin<A> {
    fn build(&self, app: &mut App) {
        app.register_type::<InputBuffer<ActionState<A>, A>>();
        app.register_type::<ActionState<A>>();

        #[cfg(feature = "client")]
        if !app.is_plugin_added::<lightyear_inputs::plugin::InputPlugin<LeafwingSequence<A>>>() {
            // Message directions register required components on `Client`.
            // Do that before users can spawn a Client entity, while still
            // deferring Leafwing's client-only runtime systems to `finish`.
            app.add_plugins(
                lightyear_inputs::plugin::InputPlugin::<LeafwingSequence<A>>::default(),
            );
        }

        #[cfg(feature = "server")]
        {
            trace!(
                "adding server input plugin for action {:?}",
                bevy_utils::prelude::DebugName::type_name::<A>()
            );
            app.add_plugins(
                lightyear_inputs::server::ServerInputPlugin::<LeafwingSequence<A>> {
                    rebroadcast_inputs: self.config.rebroadcast_inputs,
                    marker: core::marker::PhantomData,
                },
            );
        }
    }

    fn finish(&self, app: &mut App) {
        #[cfg(feature = "client")]
        {
            use leafwing_input_manager::plugin::InputManagerSystem;

            if app.is_plugin_added::<LightyearClientPlugin>() {
                // Only install the client-side Leafwing systems when the app actually
                // contains the Lightyear client stack. Using the presence of Bevy's
                // InputPlugin alone is too broad: server-only apps (including tests)
                // often add InputPlugin, and the client Leafwing swap systems would
                // then overwrite server-reconstructed ActionState values.
                if app.is_plugin_added::<bevy_input::InputPlugin>()
                    && !app.is_plugin_added::<InputManagerPlugin<A>>()
                {
                    trace!(
                        "adding client input plugin for action {:?}",
                        bevy_utils::prelude::DebugName::type_name::<A>()
                    );
                    app.add_plugins(InputManagerPlugin::<A>::default());
                }
                app.add_plugins(lightyear_inputs::client::ClientInputPlugin::<
                    LeafwingSequence<A>,
                >::new(self.config));

                // see: https://github.com/cBournhonesque/lightyear/pull/820
                app.configure_sets(
                    FixedPreUpdate,
                    lightyear_inputs::client::InputSystems::RestoreInputs
                        .before(InputManagerSystem::Tick),
                );
                app.add_systems(
                    FixedPostUpdate,
                    sync_fixed_update_state_after_rollback::<A>
                        .after(InputManagerSystem::Tick)
                        .run_if(is_in_rollback),
                );

                return;
            }
        }

        #[cfg(feature = "server")]
        if !app.is_plugin_added::<InputManagerPlugin<A>>() {
            app.add_plugins(InputManagerPlugin::<A>::server());
        }
    }
}
