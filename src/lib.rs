pub mod event_clock;

pub mod prelude {
    pub use super::{
        event_clock::{
            EventClock,
            clock_capture::{
                ClockCapture,
                get_mut_current_clock
            },
            active_clock_registry::{
                ACTIVE_CLOCK_REGISTRY_RESOURCE_ID,
                ACTIVE_CLOCK_REGISTRY_ACCESS_BUILDER,
                ActiveClockRegistry,
                get_mut_active_clock_registry,
                active_clock::{
                    ActiveClock
                }
            },
            clock_registry::{
                CLOCK_REGISTRY_ACCESS_BUILDER,
                CLOCK_REGISTRY_RESOURCE_ID,
                ClockRegistry,
                get_clock_registry
            },
            clock::{
                Clock,
                tick::{
                    Tick
                },
                clock_duration::{
                    ClockDuration
                },
                clock_finish::{
                    ClockFinish
                },
                clock_instant::{
                    ClockInstant
                },
                clock_interval::{
                    ClockInterval
                }
            }
        }
    };
}
