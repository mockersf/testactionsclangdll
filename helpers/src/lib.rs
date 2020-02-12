//! this crate adds helpers

//allowing this lint for errors from gdnative macros
//#![allow(clippy::not_unsafe_ptr_arg_deref, clippy::transmute_ptr_to_ptr)]
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

/// Will find the max between two values even if they don't impl Ordered
#[macro_export]
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

/// Will find the min between two values even if they don't impl Ordered
#[macro_export]
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
#[macro_export]
macro_rules! stringify_fn {
    ($owner:ident, $fn:ident) => {{
        let _ = $owner::$fn;
        stringify!($fn).into()
    }};
}

/// Load a scene from it's path
pub fn load_scene(path: &str) -> Option<PackedScene> {
    ResourceLoader::godot_singleton()
        .load(
            GodotString::from_str(path),
            GodotString::from_str("PackedScene"),
            false,
        )
        .and_then(|s| s.cast::<PackedScene>())
}

/// List of known signals used from Godot
#[derive(Debug, Clone)]
pub enum Signal {
    /// When screen is exited from a VisibilityNotifier2D
    ScreenExited,
    /// When button is pressed, from a BaseButton
    Pressed,
    /// area_entered
    AreaEntered,
}
impl Copy for Signal {}
impl From<Signal> for GodotString {
    fn from(signal: Signal) -> Self {
        match signal {
            Signal::AreaEntered => "area_entered".into(),
            Signal::ScreenExited => "screen_exited".into(),
            Signal::Pressed => "pressed".into(),
        }
    }
}

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
