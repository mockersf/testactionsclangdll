use euclid::vec2;
use gdnative::*;
use rand::Rng;

use helpers::stringify_fn;

/// probability there will be a star at any point
const STAR_PROBABILITY: f32 = 0.0007;

const STAR_COLORS: [(f32, Color); 4] = [
    (
        0.9,
        Color {
            r: 0.6,
            g: 0.6,
            b: 1.0,
            a: 0.6,
        },
    ),
    (
        0.8,
        Color {
            r: 1.0,
            g: 1.0,
            b: 0.6,
            a: 0.6,
        },
    ),
    (
        0.7,
        Color {
            r: 1.0,
            g: 0.6,
            b: 0.6,
            a: 0.6,
        },
    ),
    (
        0.6,
        Color {
            r: 0.7,
            g: 0.7,
            b: 0.7,
            a: 0.6,
        },
    ),
];

type OwnerNode = Control;

#[derive(NativeClass)]
#[inherit(OwnerNode)]
pub struct Menu {
    rng: rand::rngs::ThreadRng,
    star_scene: Option<PackedScene>,
}

unsafe impl Send for Menu {}

#[methods]
impl Menu {
    fn _init(_owner: OwnerNode) -> Self {
        Menu {
            rng: rand::thread_rng(),
            star_scene: helpers::load_scene("res://menu/Star.tscn"),
        }
    }

    #[export]
    fn _ready(&mut self, owner: OwnerNode) {
        unsafe {
            if let Some(mut visi) = owner.get_node("Menu/MarginContainer2/Start".into()) {
                visi.connect(
                    helpers::Signal::Pressed.into(),
                    Some(owner.to_object()),
                    stringify_fn!(Self, _start_game),
                    VariantArray::new(),
                    0,
                )
                .expect("signal connected");
            }
        }
    }

    #[export]
    fn _process(&mut self, owner: OwnerNode, _delta: f32) {
        let size = unsafe { owner.get_size() };
        let mut star_parent = unsafe {
            owner
                .get_node("Background/Stars".into())
                .expect("node Background/Stars is present")
        };
        let mut current_star_count = unsafe { star_parent.get_child_count() };
        let target_star_count: i64 = (size.x * size.y * STAR_PROBABILITY) as i64;
        while current_star_count < target_star_count {
            if let Some(mut new_star) = self
                .star_scene
                .as_ref()
                .and_then(|star_scene| (&star_scene).instance(0))
                .and_then(|new_node| unsafe { new_node.cast::<Node2D>() })
            {
                let x = self.rng.gen_range(0.0, size.x);
                let y = self.rng.gen_range(0.0, size.y);
                let color_sel = self.rng.gen_range(0.0, 1.0);
                let color = STAR_COLORS
                    .iter()
                    .filter(|(p, _)| p < &color_sel)
                    .map(|(_, color)| color)
                    .next()
                    .unwrap_or(&STAR_COLORS[STAR_COLORS.len() - 1].1);
                unsafe {
                    new_star.translate(vec2(x, y));
                    new_star
                        .get_node("Star".into())
                        .and_then(|node| node.cast::<ColorRect>())
                        .expect("ColorRect Star is present in a star")
                        .set_frame_color(color.clone());
                    star_parent.add_child(Some(new_star.to_node()), false);
                }
            }
            current_star_count += 1;
        }
    }

    #[export]
    fn _start_game(&mut self, owner: OwnerNode) {
        unsafe {
            owner
                .get_tree()
                .expect("was able to get tree from node")
                .change_scene("res://game/Game.tscn".into())
                .expect("was able to change scene");
        }
    }
}
