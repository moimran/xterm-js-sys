//! Glue for the Xterm.js types.

use super::xterm::{Disposable, Terminal, TerminalAddon, TerminalOptions};

use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
use wasm_bindgen::JsCast;

// The `object` macro and the `IntoJsInterface` trait are good candidates for
// being spun off into their own crate, along with macros that generate some of
// the boilerplate needed to mirror over JS interfaces as Rust traits.

/// Macro support: supporting items for the macros in this module; re-exported
/// here so we don't have to make any assumptions about the call-site.
#[doc(hidden)]
pub mod _m_sprt {
    pub use core::clone::Clone;
    pub use core::convert::AsRef;
    pub use core::marker::Sized;
    pub use core::{concat, stringify};
    pub use std::boxed::Box;

    pub use js_sys::{Object, Reflect};
    pub use wasm_bindgen::prelude::Closure;
    pub use wasm_bindgen::{JsCast, JsValue};
}

/// Uses the workaround detailed [here] to let us 'generate' a doc literal.
/// [here]: https://github.com/rust-lang/rust/issues/52607
#[doc(hidden)]
#[macro_export]
macro_rules! calculated_doc {
    ( $(#[doc = $doc:expr])* >>> $thing:item $(#[$metas:meta])* ) => {
        $(
            #[$metas]
        )*
        $(
            #[doc = $doc]
        )*
        $thing
    };
}

/// Creates a Rust trait to match a particular JS interface.
///
/// In addition to the actual trait, this produces:
///   - glue that lets `'static` instances of the Rust trait be "turned into"
///     instances of the JS interface
///   - a blanket impl that implements the Rust trait for all `wasm-bindgen`
///     produced types that extend the JS interface
///   - an implementation of [`IntoJsInterface`] for all things that implement
///     the Rust trait that's generated; this can be used to accept
///     implementations of the Rust or implementations of the JS interface
///     with `impl IntoJsInterface<JsInterfaceName>`
///
/// Note: if you get an error about "unconditional recursion" when using this
/// macro or an error about a trait not being in scope, it's because a method
/// that you added to the trait doesn't exist on the underlying JS interface.
/// Apologies for the cryptic error!
///
/// Also note that if you omit methods in the Rust interface you won't be
/// warned! You'll only find out when a JS user tries to call one of the methods
/// you missed at which point you'll get a runtime error 🙁 (we could use
/// [`JsCast::dyn_into`]) instead of its unchecked counterpart but we don't
/// since that's not really all that much better (still a runtime error, but
/// you'll get it on convert rather than when you try to use the method in
/// question _and_ you'll pay a performance penalty on every conversion).
///
/// Note that this currently requires that you have you `IntoJsInterface` in
/// scope. This is because this trait must be defined in the crate where this
/// macro is called in order for the blanket impl that this macro produces for
/// `IntoJsInterface` to type check. Rather than make it so that this macro only
/// works in this crate it'll instead work in other crates with the stipulation
/// that you need to mirror over the trait. Suboptimal, I know. And this
/// probably renders the little utility the `IntoJsInterface` trait had moot.
///
/// [`IntoJsInterface`]: crate::ext::IntoJsInterface
/// [`JsCast::dyn_into`]: wasm_bindgen::JsCast::dyn_into
#[macro_export]
macro_rules! interface {
    (
        $(#[$metas:meta])*
        $vis:vis trait $nom:ident
            mirrors $js_interface:ident
            $(where
                $(Self extends $ext_js:path as $ext_rs:path,)+
            )?
    {
        $(
            $(#[$fn_metas:meta])*
            // All functions that we can mirror need to take `&self` so this is
            // okay.
            fn $fn_name:ident (&self $(, $arg_name:ident: $arg_ty:ty)* $(,)?)
                $(-> $ret_ty:ty)?
                ;
            // Default impls are not supported for now.

            // This is intentionally very constrained. The idea is that this
            // just mirrors the JS interface. if you want to offer additional
            // functionality on your Rust trait, use an extension trait.
        )*
    }) => {
        $crate::calculated_doc! {
            #[doc = $crate::ext::_m_sprt::concat!(
                " Rust version of the ",
                "[`",
                    $crate::ext::_m_sprt::stringify!($js_interface),
                "`]",
                " interface.\n",
            )]
            #[doc = "\n"]
            #[doc = $crate::ext::_m_sprt::concat!(
                " See the [\"mirroring interfaces\" section]",
                "(",
                    $crate::ext::_m_sprt::stringify!(/*$*/crate),
                    "::xterm#mirroring-interfaces",
                ")",
                "\n of the `xterm` module docs for more information.",
            )]
            >>>
            $vis trait $nom
            $(where
                $(Self: $ext_rs,)+
            )?
            {
                $(
                    $crate::calculated_doc! {
                        #[doc = "\n"]
                        #[doc = $crate::ext::_m_sprt::concat!(
                            " Dual of ",
                            "[`",
                                $crate::ext::_m_sprt::stringify!($js_interface),
                                "::",
                                $crate::ext::_m_sprt::stringify!($fn_name),
                            "`].",
                        )]
                        >>>
                        fn $fn_name(
                            &self,
                            $(
                                $arg_name: $arg_ty,
                            )*
                        ) $(-> $ret_ty)?;
                        $(#[$fn_metas])*
                    }
                )*

                ////////// Internal Functions For Interface Mirroring //////////
                $crate::calculated_doc! {
                    #[doc = " Copy of [`IntoJsInterface::by_ref`].\n"]
                    #[doc = "\n"]
                    #[doc = $crate::ext::_m_sprt::concat!(
                        " [`IntoJsInterface::by_ref`]: ",
                        $crate::ext::_m_sprt::stringify!(/*$*/crate),
                        "::ext::IntoJsInterface::by_ref"
                    )]
                    >>>
                    fn into_js_by_ref(&self) -> $js_interface
                    where
                        Self: $crate::ext::_m_sprt::Clone + 'static,
                    {
                        $nom::into_js(self.clone())
                    }
                }

                $crate::calculated_doc! {
                    #[doc = " Copy of [`IntoJsInterface::to`].\n"]
                    #[doc = "\n"]
                    #[doc = $crate::ext::_m_sprt::concat!(
                        " [`IntoJsInterface::to`]: ",
                            $crate::ext::_m_sprt::stringify!(/*$*/crate),
                        "::ext::IntoJsInterface::to",
                    )]
                    >>>
                    fn into_js(self) -> $js_interface
                    where
                        Self: $crate::ext::_m_sprt::Sized + 'static,
                    {
                        use $crate::ext::_m_sprt::{Box, JsCast};
                        let b = Box::leak(Box::new(self));
                        $nom::into_js_inner(b).unchecked_into()
                    }
                }

                $crate::calculated_doc! {
                    #[doc = $crate::ext::_m_sprt::concat!(
                        " Internal version of [`into_js_by_ref`]",
                        "(",
                            $crate::ext::_m_sprt::stringify!($nom),
                            "::into_js_by_ref",
                        ")",
                        " that doesn't\n leak `self`.\n",
                    )]
                    #[doc = "\n"]
                    #[doc = " Useful for trait/interface hierarchies."]
                    >>>
                    fn into_js_inner(
                        &'static self
                    ) -> $crate::ext::_m_sprt::Object
                    where
                        Self: 'static,
                    {
                        use $crate::ext::_m_sprt::{Box, Closure, Object};
                        use $crate::ext::object;

                        let base = Object::new();

                        // First, let's verify that all the functions are
                        // actually part of the JS interface.
                        #[doc(hidden)]
                        mod __isolate_js_interface {
                            use super::*;
                            use $js_interface as __js_iface;

                            #[doc(hidden)]
                            mod __check_that_the_interface_matches_the_trait {
                                use super::__js_iface as Js;

                                $(
                                    #[allow(non_upper_case_globals, dead_code)]
                                    const $fn_name: () = {
                                        let _ = Js::$fn_name;
                                        ()
                                    };
                                )*
                            }
                        }

                        // Now we can begin. The things we extend, first:
                        $($(
                            let base = Object::assign(
                                &base,
                                &<Self as $ext_rs>::into_js_inner(self)
                            );
                        )*)?

                        // Next, the functions of the interface:
                        struct Inner {
                            $($fn_name: Closure<dyn FnMut(
                                    $($arg_ty,)*
                                ) $(-> $ret_ty)?>
                            ,)*
                        }

                        let Inner {
                            $($fn_name,)*
                        } = Inner {
                            $($fn_name: {
                                Closure::wrap(
                                    Box::new(move |$($arg_name: $arg_ty, )*| {
                                        Self::$fn_name(self $(, $arg_name)*)
                                    })
                                )
                            },)*
                        };

                        let obj = object! { (base) += {
                            $($fn_name: $fn_name),*
                        }};

                        $(Closure::forget($fn_name);)*

                        obj
                    }
                }
            }

            $(#[$metas])*
        }

        $crate::calculated_doc! {
            #[doc = $crate::ext::_m_sprt::concat!(
                " Anything that implements ",
                "[`",
                    $crate::ext::_m_sprt::stringify!($nom),
                "`]",
                " (and is `Clone + 'static`) ",
                "gets an implementation \n ",
                " of ",
                "[`IntoJsInterface<",
                    $crate::ext::_m_sprt::stringify!($js_interface),
                ">`]",
                "(",
                    $crate::ext::_m_sprt::stringify!(/*$*/crate),
                "::ext::IntoJsInterface).",
            )]
            >>>
            impl<X> IntoJsInterface<$js_interface> for X
            where
                X: $nom,
                X: $crate::ext::_m_sprt::Clone + 'static
            {
                $crate::calculated_doc! {
                    #[doc = $crate::ext::_m_sprt::concat!(
                        " Converts the ",
                        "[`",
                            $crate::ext::_m_sprt::stringify!($nom),
                        "`]",
                        " implementor into an instance of ",
                        "[`",
                            $crate::ext::_m_sprt::stringify!($js_interface),
                        "`]\n ",
                        "(the corresponding JS interface).",
                    )]
                    >>>
                    fn to(self) -> $js_interface {
                        $nom::into_js(self)
                    }
                }

                $crate::calculated_doc! {
                    #[doc = $crate::ext::_m_sprt::concat!(
                        " Converts the ",
                        "[`",
                            $crate::ext::_m_sprt::stringify!($nom),
                        "`]",
                        " implementor into an instance of ",
                        "[`",
                        $crate::ext::_m_sprt::stringify!($js_interface),
                        "`]\n ",
                        "(the corresponding JS interface) _by reference_.",
                    )]
                    >>>
                    fn by_ref(&self) -> $js_interface {
                        $nom::into_js_by_ref(self)
                    }
                }
            }
        }

        $crate::calculated_doc! {
            #[doc = $crate::ext::_m_sprt::concat!(
                " This provides an impl of the ",
                "[`",
                    $crate::ext::_m_sprt::stringify!($nom),
                "`]",
                " Rust trait for all things that 'implement'\n ",
                "the ",
                "[`",
                $crate::ext::_m_sprt::stringify!($js_interface),
                "`]",
                " JS interface the `wasm-bindgen` way.\n",
            )]
            #[doc = "\n"]
            #[doc = $crate::ext::_m_sprt::concat!(
                " See the [\"mirroring interfaces\" section]",
                "(",
                $crate::ext::_m_sprt::stringify!(/*$*/crate),
                "::xterm#mirroring-interfaces",
                ")",
                " of the\n ",
                "`xterm` module docs for more information.",
            )]
            >>>
            impl<X> $nom for X
            where
                X: $crate::ext::_m_sprt::Clone + 'static,
                $($(X: $crate::ext::_m_sprt::AsRef<$ext_js>,)*)?
                X: AsRef<$js_interface>,
            {
                $(
                    $crate::calculated_doc! {
                        #[doc = $crate::ext::_m_sprt::concat!(
                            " [`",
                                $crate::ext::_m_sprt::stringify!($fn_name),
                            "`](",
                                $crate::ext::_m_sprt::stringify!($nom),
                                "::",
                                $crate::ext::_m_sprt::stringify!($fn_name),
                            ")",
                            " for types that implement the ",
                            "[`",
                                $crate::ext::_m_sprt::stringify!($js_interface),
                            "`]",
                            " interface.",
                        )]
                        >>>
                        fn $fn_name(
                            &self
                            $(, $arg_name: $arg_ty)*
                        ) $(-> $ret_ty)? {
                            $js_interface::$fn_name(
                                $crate::ext::_m_sprt::AsRef
                                    ::<$js_interface>::as_ref(self),
                                $($arg_name,)*
                            )
                        }

                        // Unfortunately there doesn't seem to be syntax that
                        // lets us say that, when there's ambiguity between
                        // a trait method and an inherent method, use the
                        // inherent method (we can, however, say to use the
                        // trait method using FQS).
                        //
                        // That's a problem here because it means that we
                        // fall back on the trait method that we're implementing
                        // when the underlying inherent method we want to proxy
                        // doesn't exist.
                        //
                        // We can still make this a hard error by forbidding
                        // obviously infinitely recursion functions, but this
                        // isn't perfect since it produces a pretty cryptic
                        // error (we'd like to say that the method doesn't
                        // exist on the JS interface). Alas, without a proc
                        // macro or without naming the proxied methods something
                        // different (I think this'd also need a proc macro or
                        // the paste crate) this is probably as good as we can
                        // do.
                        //
                        // This is mentioned in the macro's docs.
                        //
                        // Actually, one way we can do this kind of check is
                        // to make a fake module (that's hidden) and to, within
                        // it, not import the Rust trait but to import the JS
                        // interface and then "use" the functions on the
                        // interface we're after (not directly unfortunately —
                        // structs don't work like that in Rust). We can use
                        // them by doing something like:
                        // ```rust
                        // mod __testing {
                        //   use super::$js_interface;
                        //   const $fn_name: () = {
                        //     let _ = $js_interface::$fn_name; ()
                        //   };
                        // }
                        // ```
                        // Unfortunately the `super::$js_interface` part won't
                        // work if $js_interface is an absolute path and the
                        // error message will (not so helpfully in this case)
                        // suggest that we import the Rust trait into scope to
                        // try to fix the problem which is possibly more
                        // confusing in this case.
                        //
                        // So, I think we'll just leave it for now.
                        //
                        // Actually we can get around the above by having the
                        // module above `use` $js_interface `as` something else
                        // within a scope. So, let's do it!
                        //
                        // Okay! Done!
                        //
                        // Note that if you do things like have mismatched
                        // parameters you might get the recursion error instead
                        // (I _think_ if the inherent method is there it'll
                        // resolve to it over the trait method, but I'm not
                        // 100% sure).
                        //
                        // It's possible to expand the module hack above to
                        // check for this too (you'd do something like:
                        // ```rust
                        // #[doc(hidden)]
                        // fn $fn_name($($arg_name: arg_ty),*) $(-> $ret_ty)? {
                        //   Js::$fn_name($($arg_name: arg_ty),*)
                        // }
                        // ```
                        // which should error on any type/arity mismatches), but
                        // I'll leave that for another day.
                        //
                        // Pretty sure about the inherent method thing and even
                        // if that isn't true, we've still got the unconditional
                        // recursion error.
                        //
                        #[forbid(unconditional_recursion)]
                    }
                )*

                $crate::calculated_doc! {
                    #[doc = $crate::ext::_m_sprt::concat!(
                        " [`into_js_by_ref`](",
                            $crate::ext::_m_sprt::stringify!($nom),
                        "::into_js_by_ref)",
                        " for types that implement the\n ",
                        "[`",
                            $crate::ext::_m_sprt::stringify!($js_interface),
                        "`]",
                        " interface.\n",
                    )]
                    #[doc = "\n"]
                    #[doc = $crate::ext::_m_sprt::concat!(
                        " This differs from the default impl in that it",
                        " manages to avoid a `Clone` before effectively\n",
                        " doing what ",
                        "[`into_js`](",
                        $crate::ext::_m_sprt::stringify!($nom),
                        "::into_js) does.",
                    )]
                    >>>
                    fn into_js_by_ref(&self) -> $js_interface {
                        use $crate::ext::_m_sprt::{AsRef, Clone};

                        AsRef::<$js_interface>::as_ref(self).clone()
                    }
                }

                $crate::calculated_doc! {
                    #[doc = $crate::ext::_m_sprt::concat!(
                        " [`into_js`](",
                            $crate::ext::_m_sprt::stringify!($nom),
                        "::into_js)",
                        " for types that implement the\n ",
                        "[`",
                            $crate::ext::_m_sprt::stringify!($js_interface),
                        "`]",
                        " interface.\n",
                    )]
                    #[doc = "\n"]
                    #[doc = $crate::ext::_m_sprt::concat!(
                        " This differs from the default impl in that it",
                        " manages to avoid \"double wrapping\" the methods\n",
                        " in the interface (types that impl ",
                        "[`",
                            $crate::ext::_m_sprt::stringify!($js_interface),
                        "`]",
                        " the `wasm-bindgen` way already have\n",
                        " a wrapped up",
                        " [`Object`](",
                            $crate::ext::_m_sprt::stringify!(/*$*/crate),
                        "::ext::_m_sprt::Object)",
                        " they can hand us).",
                    )]
                    >>>
                    fn into_js(self) -> $js_interface {
                        use $crate::ext::_m_sprt::{AsRef, Clone};

                        AsRef::<$js_interface>::as_ref(&self).clone()
                    }
                }
            }
        }
    };
}

/// Defines a JS object with some properties.
#[macro_export]
macro_rules! object {
    (
        $($f:ident: $v:expr),* $(,)?
    ) => {{
        let obj = $crate::ext::_m_sprt::Object::new();

        let _ = $crate::ext::object! { obj += {
                $($f: $v),*
        }};

        obj
    }};

    (($base:expr) += {
        $($f:ident: $v:expr),* $(,)?
    }) => {{
        let obj = $base;

        let _ = $crate::ext::object! { obj += {
            $($f: $v),*
        }};

        obj
    }};

    ($nom:ident += {
        $($f:ident: $v:expr),* $(,)?
    }) => {{$(
        let _ = $crate::ext::_m_sprt::Reflect::set(
            &$nom,
            &$crate::ext::_m_sprt::JsValue::from_str(
                $crate::ext::_m_sprt::stringify!($f)
            ),
            ($v).as_ref(),
        ).unwrap();
    )*}};
}

/// Represents a Rust type that satisfies a JS interface and can be turned into
/// the concrete type that represents the JS interface.
///
/// See the [`disposable`] module for an example.
///
/// As mentioned in the [`xterm` module docs](crate::xterm#mirroring-interfaces)
/// we make a Rust trait dual for each JS interface (or the ones we want to make
/// instances of, anyways). Ideally we'd be able to do something like this:
/// `trait RustTrait: IntoJsInterface<JsTrait>`. The problem with that is that
/// the impl of `IntoJsInterface` _requires_ the impl of `RustTrait`; we need
/// the functions that satisfy the interface to actually make the instance of
/// the interface type.
///
/// So, instead we do the weird backwards thing that we do in [`disposable`]
/// where the Rust trait (i.e. [`Disposable`](disposable::Disposable)) ends up
/// effectively having these same functions and _then_ providing a blanket impl
/// so that [`IntoJsInterface`] is impled for all things that impl the Rust
/// trait.
///
/// It's unfortunate that we don't really have a way to encode that each Rust
/// trait can have one (and only one) interface type dual. We encode this within
/// the trait itself, but we can't seem to do this in a way that's generic over
/// traits (not until we get HKTs anyways).
///
/// So it's unclear exactly where this trait would be useful. I guess it lets
/// you be generic over the interface you're asking for? Something like this:
/// ```rust
/// # use wasm_bindgen::{convert::{FromWasmAbi, IntoWasmAbi}, JsCast};
/// # use xterm_js_sys::ext::IntoJsInterface;
/// # #[allow(dead_code)]
/// pub fn foo<I>(inst: impl IntoJsInterface<I>)
/// where
///     I: FromWasmAbi + IntoWasmAbi + JsCast,
/// {
///    inst.to();
/// }
/// ```
///
/// Combined with `AsRef` you can do things like accept Rust implementations
/// of interfaces that subclass some base class:
/// ```rust
/// # use wasm_bindgen::{convert::{FromWasmAbi, IntoWasmAbi}, JsCast};
/// # use xterm_js_sys::ext::IntoJsInterface;
/// # #[allow(dead_code)]
/// pub fn bar<I>(inst: impl IntoJsInterface<I>)
/// where
///     I: FromWasmAbi + IntoWasmAbi + JsCast,
///     I: AsRef<js_sys::Iterator>,
/// {
///    inst.to();
/// }
/// ```
///
/// But it's still unclear if/how this is useful.
pub trait IntoJsInterface<Interface: FromWasmAbi + IntoWasmAbi + JsCast> {
    /// Convert to an instance of the JS interface type.
    fn to(self) -> Interface;

    /// Produce an instance of the JS interface type without consuming the Rust
    /// instance.
    ///
    /// For Rust impls of a trait this will probably require `Self` to implement
    /// `Clone` since as part of creating the instance the instance needs to be
    /// leaked (for methods to still work), but we'll leave that up to
    /// implementors.
    fn by_ref(&self) -> Interface;
}

#[doc(inline)]
pub use super::{calculated_doc, interface, object};

pub mod addon;
pub use addon::*;

pub mod disposable;
pub use disposable::*;

pub mod event;
// pub use event::*; // Temporarily commented to fix unused import

pub mod log_level;
// pub use log_level::*; // Temporarily commented to fix unused import

pub mod marker;
// pub use marker::*; // Temporarily commented to fix unused import

pub mod options;
pub use options::*;

pub mod theme;
// pub use theme::*; // Only used internally for now

pub mod unicode;
pub use unicode::*;
