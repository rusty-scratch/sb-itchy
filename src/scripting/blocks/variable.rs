use super::*;

pub fn move_steps<Steps>(steps: Steps) -> StackBlock
where
    Steps: IntoArg<Number>,
{
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::motion_movesteps).add_input_into_arg("STEPS", steps),
    )
}
