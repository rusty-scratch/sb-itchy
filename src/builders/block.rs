use std::collections::HashMap;

use sb_sbity::block::{BlockInputValue, BlockMutation, ListOrVariable, ShadowInputType};

use crate::opcode::OpCode;

use super::{comment::CommentBuilder, stack::StackBuilder};

#[derive(Debug, Clone, PartialEq)]
pub enum BlockBuilder {
    Normal(BlockNormalBuilder),
    VarListReporterTop(BlockVarListReporterTopBuilder),
}

#[derive(Debug, Clone, PartialEq)]
pub struct BlockNormalBuilder {
    opcode: OpCode,
    comment: Option<CommentBuilder>,
    inputs: HashMap<String, BlockInputBuilder>,
    fields: HashMap<String, BlockFieldBuilder>,
    mutation: Option<BlockMutation>,
    shadow: bool,
    x: Option<f64>,
    y: Option<f64>,
}
#[derive(Debug, Clone, PartialEq)]
pub struct BlockVarListReporterTopBuilder {
    kind: ListOrVariable,
    from: VarListFrom,
    name: String,
    x: f64,
    y: f64,
    comment: Option<CommentBuilder>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BlockInputBuilder {
    shadow: ShadowInputType,
    values: Vec<Option<StackOrValue>>,
}
#[derive(Debug, Clone, PartialEq)]
pub struct BlockFieldBuilder {
    value: String,
    kind: FieldKind,
}

#[derive(Debug, Clone, PartialEq)]
pub enum StackOrValue {
    Value(BlockInputValue),
    Stack(StackBuilder),
}

#[derive(Debug, Clone, PartialEq)]
pub enum VarListFrom {
    Global,
    Sprite,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FieldKind {
    NoRef,
    NoRefMaybe,
    Broadcast,
    SpriteVariable,
    GlobalVariable,
    SpriteList,
    GlobalList,
}
