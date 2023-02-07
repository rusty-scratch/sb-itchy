use sb_itchy::{blocks::*, prelude::*};
use sb_sbity::block::BlockInputValue;

fn main() {
    let mut project = ProjectBuilder::default();

    // Sprite 1 ================================================================
    let mut sprite1 = SpriteBuilder::default();
    sprite1.target.set_name("sprite1");
    sprite1
        .target
        .add_block_stack(say(BlockInputBuilder::value(BlockInputValue::String {
            value: "hi mom".to_owned().into(),
        })));

    project.add_sprite(sprite1);
}
