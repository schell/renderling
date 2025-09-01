//! Request animation frame helpers, taken from [mogwai](https://crates.io/crates/mogwai).
use std::{cell::RefCell, rc::Rc};

use wasm_bindgen::{prelude::Closure, JsCast, JsValue, UnwrapThrowExt};

fn req_animation_frame(f: &Closure<dyn FnMut(JsValue)>) {
    web_sys::window()
        .expect_throw("could not get window")
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect_throw("should register `requestAnimationFrame` OK");
}

/// Sets a static rust closure to be called with `window.requestAnimationFrame`.
///
/// The static rust closure takes one parameter which is
/// a timestamp representing the number of milliseconds since the application's
/// load. See <https://developer.mozilla.org/en-US/docs/Web/API/DOMHighResTimeStamp>
/// for more info.
fn request_animation_frame(mut f: impl FnMut(JsValue) + 'static) {
    let wrapper = Rc::new(RefCell::new(None));
    let callback = Box::new({
        let wrapper = wrapper.clone();
        move |jsval| {
            f(jsval);
            wrapper.borrow_mut().take();
        }
    }) as Box<dyn FnMut(JsValue)>;
    let closure: Closure<dyn FnMut(JsValue)> = Closure::wrap(callback);
    *wrapper.borrow_mut() = Some(closure);
    req_animation_frame(wrapper.borrow().as_ref().unwrap_throw());
}

#[derive(Clone, Default)]
#[expect(clippy::type_complexity, reason = "not too complex")]
struct NextFrame {
    closure: Rc<RefCell<Option<Closure<dyn FnMut(JsValue)>>>>,
    ts: Rc<RefCell<Option<f64>>>,
    waker: Rc<RefCell<Option<std::task::Waker>>>,
}

impl std::future::Future for NextFrame {
    type Output = f64;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        if let Some(ts) = self.ts.borrow_mut().take() {
            std::task::Poll::Ready(ts)
        } else {
            *self.waker.borrow_mut() = Some(cx.waker().clone());
            std::task::Poll::Pending
        }
    }
}

/// Creates a future that will resolve on the next animation frame.
///
/// The future's output is a timestamp representing the number of
/// milliseconds since the application's load.
/// See <https://developer.mozilla.org/en-US/docs/Web/API/DOMHighResTimeStamp>
/// for more info.
pub fn next_animation_frame() -> impl std::future::Future<Output = f64> {
    // https://rustwasm.github.io/wasm-bindgen/examples/request-animation-frame.html#srclibrs
    let frame = NextFrame::default();

    *frame.closure.borrow_mut() = Some(Closure::wrap(Box::new({
        let frame = frame.clone();
        move |ts_val: JsValue| {
            *frame.ts.borrow_mut() = Some(ts_val.as_f64().unwrap_or(0.0));
            if let Some(waker) = frame.waker.borrow_mut().take() {
                waker.wake();
            }
        }
    }) as Box<dyn FnMut(JsValue)>));

    req_animation_frame(frame.closure.borrow().as_ref().unwrap_throw());

    frame
}
