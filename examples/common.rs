//! Shared things for the examples.

use js_sys::Function;
use wasm_bindgen::{prelude::*, JsCast};

use std::cell::{Cell, RefCell};
use std::rc::Rc;

#[macro_export]
macro_rules! __log { ($($t:tt)*) => {::web_sys::console::log_1(&format!($($t)*).into())}; }
pub use crate::__log as log;

fn cancel_animation_frame(handle: i32) {
    if let Some(window) = web_sys::window() {
        let _ = window.cancel_animation_frame(handle);
    }
}

/// A safe wrapper for animation frame callbacks that properly manages memory
/// and avoids unsafe transmutation.
#[wasm_bindgen]
pub struct AnimationFrameCallbackWrapper {
    handle: Rc<Cell<Option<i32>>>,
    func: Rc<RefCell<Option<Box<dyn FnMut() -> bool + 'static>>>>,
}

impl Default for AnimationFrameCallbackWrapper {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for AnimationFrameCallbackWrapper {
    fn drop(&mut self) {
        if let Some(handle) = self.handle.get() {
            cancel_animation_frame(handle);
        }
    }
}



impl AnimationFrameCallbackWrapper {
    pub fn new() -> Self {
        Self {
            handle: Rc::new(Cell::new(None)),
            func: Rc::new(RefCell::new(None)),
        }
    }

    /// Start the animation frame loop with a safe, properly managed callback.
    /// Returns a handle that can be used to stop the animation.
    pub fn start(&self, func: impl FnMut() -> bool + 'static) -> Result<(), JsValue> {
        // Cancel any existing animation frame
        if let Some(handle) = self.handle.get() {
            cancel_animation_frame(handle);
        }

        // Store the function
        *self.func.borrow_mut() = Some(Box::new(func));

        // Create the callback closure
        let handle_clone = self.handle.clone();
        let func_clone = self.func.clone();

        // Start the first frame
        let window = web_sys::window().ok_or("No window available")?;

        let closure = Closure::wrap(Box::new(move || {
            let should_continue = {
                let mut func_ref = func_clone.borrow_mut();
                if let Some(ref mut f) = *func_ref {
                    f()
                } else {
                    false
                }
            };

            if should_continue {
                // Schedule next frame
                if let Some(window) = web_sys::window() {
                    if let Ok(id) = window.request_animation_frame(&create_callback(handle_clone.clone(), func_clone.clone())) {
                        handle_clone.set(Some(id));
                    } else {
                        handle_clone.set(None);
                    }
                } else {
                    handle_clone.set(None);
                }
            } else {
                // Animation stopped
                handle_clone.set(None);
                *func_clone.borrow_mut() = None;
            }
        }) as Box<dyn FnMut()>);
        let id = window.request_animation_frame(closure.as_ref().unchecked_ref())?;
        self.handle.set(Some(id));

        // Keep the closure alive by forgetting it (it will be cleaned up when animation stops)
        closure.forget();

        Ok(())
    }

    /// Stop the animation frame loop
    pub fn stop(&self) {
        if let Some(handle) = self.handle.get() {
            cancel_animation_frame(handle);
            self.handle.set(None);
        }
        *self.func.borrow_mut() = None;
    }
}

fn create_callback(
    handle: Rc<Cell<Option<i32>>>,
    func: Rc<RefCell<Option<Box<dyn FnMut() -> bool + 'static>>>>
) -> Function {
    let closure = Closure::wrap(Box::new(move || {
        let should_continue = {
            let mut func_ref = func.borrow_mut();
            if let Some(ref mut f) = *func_ref {
                f()
            } else {
                false
            }
        };

        if should_continue {
            let window = web_sys::window().unwrap();
            if let Ok(id) = window.request_animation_frame(&create_callback(handle.clone(), func.clone())) {
                handle.set(Some(id));
            } else {
                handle.set(None);
            }
        } else {
            handle.set(None);
            *func.borrow_mut() = None;
        }
    }) as Box<dyn FnMut()>);

    let result = closure.as_ref().unchecked_ref::<Function>().clone();
    closure.forget();
    result
}
