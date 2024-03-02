use gdnative::{
    // api::{Area2D},
    // export::{
    //     hint::{EnumHint, IntHint},
    //     Export,
    // },
    prelude::*,
};

// Example MainScene class, the required "new" function, and a _ready example.
// NOTE: You will likely want something more specific than Node to inherit from.
#[derive(NativeClass)]
#[inherit(Node)]
pub struct MainScene {}

#[methods]
impl MainScene {
    fn new(_base: &Node) -> Self {
        MainScene {}
    }

    #[method]
    fn _ready(&self, #[base] _base: &Node) {
        godot_print!("Hello from MainScene!")
    }
}

// Registers all exposed classes to Godot.
fn init(handle: InitHandle) {
    handle.add_class::<MainScene>();
}

// Creates entry-points of dyn lib.
godot_init!(init);
