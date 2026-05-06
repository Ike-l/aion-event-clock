pub mod event_clock;

pub mod prelude {
    pub use super::{
        event_clock::{
            EventClock,
            current_clock::{
                CurrentClock,
                get_mut_current_clock
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
