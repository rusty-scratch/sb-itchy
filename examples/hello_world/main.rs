use sb_itchy::{blocks::*, export::export, prelude::*};

type Bib = BlockInputBuilder;
type Biv = BlockInputValue;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut project = ProjectBuilder::default();

    // Stage ===================================================================
    {
        let mut stage = StageBuilder::default();
        stage
            .target
            .add_costume(CostumeBuilder::new(AssetBuilder::new(
                "backdrop1",
                Resource::load("examples\\hello_world\\backdrop.svg")?,
            )));

        project.set_stage(stage);
    }

    // Sprite 1 ================================================================
    {
        let mut sprite1 = SpriteBuilder::default();
        sprite1
            .target
            .set_name("sprite1")
            .set_layer_order(1)
            .add_costume(CostumeBuilder::new(AssetBuilder::new(
                "costume1",
                Resource::load("examples\\hello_world\\cat.svg")?,
            )));

        #[rustfmt::skip]
        sprite1
            .target
            .add_block_stack(
                when_flag_clicked()
                // say "hi mom"
                .next(say(Bib::value(Biv::String { value: "hi mom".to_owned().into(), })))
                // wait 1 secs
                .next(wait(Bib::value(Biv::Number { value: 1.into() })))
                // forever
                .next(forever(Some(Bib::stack(
                    // move_steps 10
                    move_steps(Bib::value(Biv::Number { value: 10.into() }))
                ))))
            );

        project.add_sprite(sprite1);
    }

    // Exporting ===============================================================
    export(
        project,
        "C:\\Users\\USER\\OneDrive\\Desktop\\itchy_project.sb3",
        false,
    )
    .unwrap();
    Ok(())
}
