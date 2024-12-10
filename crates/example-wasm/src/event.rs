// SPDX-FileCopyrightText: 2024 Schell Scivally <efsubenovex@gmail.com>>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! A light abstraction over UI event callbacks.
//!
//! This uses [`futures-lite::Stream`] to send events to downstream listeners.
use std::{
    pin::Pin,
    sync::{Arc, Mutex},
    task::Waker,
};

use futures_lite::Stream;
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use web_sys::EventTarget;

type MaybeCallback = Option<Arc<Closure<dyn FnMut(JsValue)>>>;

struct WebCallback {
    target: EventTarget,
    name: String,
    closure: MaybeCallback,
    waker: Arc<Mutex<Option<Waker>>>,
    event: Arc<Mutex<Option<web_sys::Event>>>,
}

impl Drop for WebCallback {
    fn drop(&mut self) {
        if let Some(arc) = self.closure.take() {
            if let Ok(closure) = Arc::try_unwrap(arc) {
                self.target
                    .remove_event_listener_with_callback(
                        self.name.as_str(),
                        closure.as_ref().unchecked_ref(),
                    )
                    .unwrap();
                log::trace!("dropping event {}", self.name);
            }
        }
    }
}

impl Stream for WebCallback {
    type Item = web_sys::Event;

    fn poll_next(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        let data = self.get_mut();
        *data.waker.lock().unwrap() = Some(cx.waker().clone());

        if let Some(event) = data.event.lock().unwrap().take() {
            std::task::Poll::Ready(Some(event))
        } else {
            std::task::Poll::Pending
        }
    }
}

/// Listen for events of the given name on the given target.
/// All events will be sent downstream until the stream is
/// dropped.
pub fn event_stream(
    ev_name: &str,
    target: &web_sys::EventTarget,
) -> impl Stream<Item = web_sys::Event> {
    let waker: Arc<Mutex<Option<Waker>>> = Default::default();
    let waker_here = waker.clone();

    let event: Arc<Mutex<Option<web_sys::Event>>> = Default::default();
    let event_here = event.clone();

    let closure = Closure::wrap(Box::new(move |val: JsValue| {
        let ev = val.unchecked_into();
        *event.lock().unwrap() = Some(ev);
        if let Some(waker) = waker.lock().unwrap().take() {
            waker.wake()
        }
    }) as Box<dyn FnMut(JsValue)>);

    target
        .add_event_listener_with_callback(ev_name, closure.as_ref().unchecked_ref())
        .unwrap();

    WebCallback {
        target: target.clone(),
        name: ev_name.to_string(),
        closure: Some(closure.into()),
        event: event_here,
        waker: waker_here,
    }
}
