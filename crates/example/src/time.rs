// SPDX-FileCopyrightText: 2024 Schell Scivally <efsubenovex@gmail.com>>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Time constructs.
//!
//! Because Instant::now() doesn't work on arch = wasm32.
pub use std::time::Duration;
#[cfg(not(target_arch = "wasm32"))]
pub use std::time::Instant;
#[cfg(target_arch = "wasm32")]
use web_sys::window;

#[derive(Clone, Copy, Debug)]
pub struct Time {
    #[cfg(target_arch = "wasm32")]
    time: u32,

    #[cfg(not(target_arch = "wasm32"))]
    time: Instant,
}

impl Time {
    #[cfg(not(target_arch = "wasm32"))]
    pub fn now() -> Self {
        Time {
            time: Instant::now(),
        }
    }

    #[cfg(target_arch = "wasm32")]
    pub fn now() -> Self {
        Time {
            time: window().unwrap().performance().unwrap().now() as u32,
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn millis_since(&self, then: Time) -> u32 {
        self.time.duration_since(then.time).as_millis() as u32
    }

    #[cfg(target_arch = "wasm32")]
    pub fn millis_since(&self, then: Time) -> u32 {
        self.time - then.time
    }
}

pub const FPS_COUNTER_BUFFER_SIZE: usize = 60;

pub struct CounterBuffer<T> {
    buffer: [T; FPS_COUNTER_BUFFER_SIZE],
    index: usize,
}

impl CounterBuffer<f32> {
    pub fn new(init: f32) -> Self {
        CounterBuffer {
            buffer: [init; FPS_COUNTER_BUFFER_SIZE],
            index: 0,
        }
    }

    pub fn write(&mut self, val: f32) {
        self.buffer[self.index] = val;
        self.index = (self.index + 1) % self.buffer.len();
    }

    pub fn average(&self) -> f32 {
        self.buffer.iter().fold(0.0, |sum, dt| sum + dt) / self.buffer.len() as f32
    }

    pub fn current(&self) -> f32 {
        let last_index = if self.index == 0 {
            self.buffer.len() - 1
        } else {
            self.index - 1
        };
        self.buffer[last_index]
    }

    pub fn frames(&self) -> &[f32; FPS_COUNTER_BUFFER_SIZE] {
        &self.buffer
    }
}

pub struct FPSCounter {
    counter: CounterBuffer<f32>,
    last_instant: Time,
    last_dt: f32,
    averages: CounterBuffer<f32>,
}

impl Default for FPSCounter {
    fn default() -> Self {
        FPSCounter::new()
    }
}

impl FPSCounter {
    pub fn new() -> FPSCounter {
        FPSCounter {
            counter: CounterBuffer::new(0.0),
            last_instant: Time::now(),
            last_dt: 0.0,
            averages: CounterBuffer::new(0.0),
        }
    }

    pub fn restart(&mut self) {
        self.last_instant = Time::now();
    }

    pub fn next_frame(&mut self) -> f32 {
        let this_instant = Time::now();
        let delta = this_instant.millis_since(self.last_instant);
        let dt_seconds = delta as f32 / 1000.0;
        self.last_dt = dt_seconds;
        self.last_instant = this_instant;
        self.counter.write(dt_seconds);
        if self.counter.index + 1 == FPS_COUNTER_BUFFER_SIZE {
            let avg = self.counter.average();
            self.averages.write(avg);
        }
        dt_seconds
    }

    pub fn avg_frame_delta(&self) -> f32 {
        self.counter.average()
    }

    pub fn current_fps(&self) -> f32 {
        1.0 / self.avg_frame_delta()
    }

    pub fn current_fps_string(&self) -> String {
        let avg = self.averages.current();
        format!("{:.1}", 1.0 / avg)
    }

    /// Return the last frame's delta in seconds.
    pub fn last_delta(&self) -> f32 {
        self.last_dt
    }

    pub fn second_averages(&self) -> &[f32; FPS_COUNTER_BUFFER_SIZE] {
        self.averages.frames()
    }
}
