#![cfg(feature = "blocks")]

use crate::{
    block::{BlockFieldBuilder, BlockInputBuilder, BlockNormalBuilder},
    // opcode::StandardOpCode,
    stack::StackBuilder,
};
// use sb_sbity::block::{BlockMutation, BlockMutationEnum};

type Bfb = BlockFieldBuilder;
type Bib = BlockInputBuilder;

include!(concat!(env!("OUT_DIR"), "/blocks/scratch-standard-blocks"));
