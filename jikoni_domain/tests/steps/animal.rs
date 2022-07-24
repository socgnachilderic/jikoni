use cucumber::{given, then, when};

use crate::MyWorld;

#[given(expr = "a {word} cat")]
fn hungry_cat(world: &mut MyWorld, state: String) {
    match state.as_str() {
        "hungry" => world.cat.hungry = true,
        "satiated" => world.cat.hungry = false,
        _ => unreachable!(),
    }
}

#[when("I feed the cat")]
fn feed_cat(word: &mut MyWorld) {
    word.cat.feed();
}

#[then("the cat is not hungry")]
fn cat_is_feed(world: &mut MyWorld) {
    assert!(!world.cat.hungry);
}
