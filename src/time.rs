use std::time::{Duration, Instant};

const FPS_INTERVAL: Duration = Duration::from_secs(3);

pub(crate) struct FrameCounter {
    last_instant: Instant,
    counter: u32,
}

impl FrameCounter {
    pub(crate) fn new(start: Instant) -> Self {
        Self {
            counter: 0,
            last_instant: start,
        }
    }

    pub(crate) fn finish_frame(&mut self, current_instant: Instant) {
        self.counter += 1;

        let time_since_last_printout = current_instant - self.last_instant;

        if time_since_last_printout > FPS_INTERVAL {
            let frames_per_second = self.counter as f32 / time_since_last_printout.as_secs_f32();
            println!("FPS: {}", frames_per_second);

            self.counter = 0;
            self.last_instant = current_instant;
        }
    }
}
