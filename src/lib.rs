pub mod event_clock;

pub mod prelude {
    pub use super::{
        event_clock::{
            EventClock,
            clock_capture::{
                ClockCapture,
                get_mut_current_clock
            },
            timer_registry::{
                TIMER_REGISTRY_RESOURCE_ID,
                TIMER_REGISTRY_ACCESS_BUILDER,
                TimerRegistry,
                get_mut_timer_registry,
                timer::{
                    Timer
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
