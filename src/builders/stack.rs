use super::block::BlockBuilder;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct StackBuilder {
    stack: Vec<BlockBuilder>,
}
