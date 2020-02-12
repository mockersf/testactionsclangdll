use gdnative::*;

use helpers::stringify_fn;

type OwnerNode = Area2D;

#[derive(NativeClass)]
#[inherit(OwnerNode)]
pub struct StellarObject {}

unsafe impl Send for StellarObject {}

#[methods]
impl StellarObject {
    fn _init(_owner: OwnerNode) -> Self {
        StellarObject {}
    }

    #[export]
    fn _ready(&mut self, mut owner: OwnerNode) {
        unsafe {
            let target = owner;
            owner
                .connect(
                    helpers::Signal::AreaEntered.into(),
                    Some(target.to_object()),
                    stringify_fn!(Self, _entered_stellar_object),
                    VariantArray::new(),
                    0,
                )
                .expect("signal connected");
        }
    }

    #[export]
    fn _entered_stellar_object(&mut self, _owner: OwnerNode, _entered: Area2D) {
        godot_print!("over stellar object");
    }
}
