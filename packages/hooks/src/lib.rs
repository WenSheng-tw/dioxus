#![deny(missing_docs)]
//! Useful foundational hooks for Dioxus

mod usestate;
pub use usestate::{use_state, UseState};

mod useref;
pub use useref::*;

mod use_shared_state;
pub use use_shared_state::*;

mod usecoroutine;
pub use usecoroutine::*;

mod usefuture;
pub use usefuture::*;

mod usesuspense;
pub use usesuspense::*;

#[macro_export]
/// A helper macro for using hooks in async environements.
///
/// # Usage
///
///
/// ```
/// let (data) = use_ref(&cx, || {});
///
/// let handle_thing = move |_| {
///     to_owned![data]
///     cx.spawn(async move {
///         // do stuff
///     });
/// }
/// ```
macro_rules! to_owned {
    ($($es:ident),+) => {$(
        #[allow(unused_mut)]
        let mut $es = $es.to_owned();
    )*}
}
