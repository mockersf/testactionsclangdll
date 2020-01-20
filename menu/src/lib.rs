//! this crate is to handle the main menu of the game

//allowing this lint for errors from gdnative macros
#![allow(clippy::not_unsafe_ptr_arg_deref, clippy::transmute_ptr_to_ptr)]
#![deny(
    warnings,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unstable_features,
    unused_import_braces,
    unused_qualifications,
    missing_docs
)]

use gdnative::*;

macro_rules! max {
    ($x: expr) => ($x);
    ($x: expr, $($z: expr),+) => {{
        let y = max!($($z),*);
        if $x > y {
            $x
        } else {
            y
        }
    }}
}

macro_rules! min {
    ($x: expr) => ($x);
    ($x: expr, $($z: expr),+) => {{
        let y = min!($($z),*);
        if $x < y {
            $x
        } else {
            y
        }
    }}
}

/// Check that a function exists in owner before stringifying it
macro_rules! stringify_fn {
    ($owner:ident, $fn:ident) => {{
        let _ = $owner::$fn;
        stringify!($fn).into()
    }};
}

/// List of known signals used from Godot
enum Signal {
    ScreenExited,
    // BodyExited,
}
impl From<Signal> for GodotString {
    fn from(signal: Signal) -> Self {
        match signal {
            Signal::ScreenExited => "screen_exited".into(),
            // Signal::BodyExited => "body_exited".into(),
        }
    }
}

mod star;

fn init(handle: gdnative::init::InitHandle) {
    handle.add_class::<star::Star>();
}

godot_gdnative_init!();
godot_nativescript_init!(init);
godot_gdnative_terminate!();

#[cfg(test)]
mod test {
    #[test]
    fn macro_max() {
        assert_eq!(max!(3.3, 5.2), 5.2);
        assert_eq!(max!(5.7, 5.2), 5.7);
    }

    #[test]
    fn macro_min() {
        assert_eq!(min!(3.3, 5.2), 3.3);
        assert_eq!(min!(5.7, 5.2), 5.2);
    }

    #[test]
    fn macro_stringify_fn() {
        struct Test;
        impl Test {
            fn zut(self) {
                unimplemented!()
            }
        }
        let stringified_fn: &str = stringify_fn!(Test, zut);
        assert_eq!(stringified_fn, "zut");
    }
}
