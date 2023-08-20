use std::collections::HashMap;

use super::{
    block::BlockBuilder,
    context::{BlockStackContextData, GlobalScriptingContextData, LocalScriptingContextData},
};
use crate::uid::Uid;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct StackBuilder {
    pub stack: Vec<BlockBuilder>,
}

impl StackBuilder {
    pub fn new() -> StackBuilder {
        StackBuilder { stack: Vec::new() }
    }

    // pub fn start(block: BlockNormalBuilder) -> StackBuilder {
    //     StackBuilder::start_with_capacity(1, BlockBuilder::Normal(block))
    // }

    // /// Varlist is a reporter. You shouldn't continue after this... but nothing disllowed you.
    // pub fn start_varlist(block: BlockVarListBuilder) -> StackBuilder {
    //     StackBuilder::start_with_capacity(1, BlockBuilder::VarList(block))
    // }

    // pub fn start_with_capacity(capacity: usize, block: BlockBuilder) -> StackBuilder {
    //     let mut stack = Vec::with_capacity(capacity);
    //     stack.push(block);
    //     StackBuilder { stack }
    // }

    // pub fn with_capacity(capacity: usize) -> StackBuilder {
    //     let stack = Vec::with_capacity(capacity);
    //     StackBuilder { stack }
    // }

    pub fn next(mut self, mut next_stack: StackBuilder) -> StackBuilder {
        self.stack.append(&mut next_stack.stack);
        self
    }

    // pub fn set_top_block_position(&mut self, x: f64, y: f64) -> &mut Self {
    //     match &mut self.stack[0] {
    //         BlockBuilder::Normal(n) => {
    //             n.set_pos(Some(x), Some(y));
    //         }
    //         BlockBuilder::VarList(vl) => {
    //             vl.set_pos(x, y);
    //         }
    //     }
    //     self
    // }

    pub fn build(
        self,
        first_block_uid: Uid,
        // comment_buff: &mut HashMap<Uid, Comment>,
        // target_context: &TargetContext,
        global_scripting_ctx: &GlobalScriptingContextData,
        local_scripting_ctx: &LocalScriptingContextData,
    ) -> HashMap<Uid, sb_sbity::block::Block> {
        let mut builded_stack: HashMap<Uid, sb_sbity::block::Block> = HashMap::default();
        let mut block_stack_ctx_data = BlockStackContextData::default();
        let mut stack_iter = self.stack.into_iter();
        let first_builded_block = stack_iter.next().unwrap().build(
            first_block_uid,
            global_scripting_ctx,
            local_scripting_ctx,
            &mut block_stack_ctx_data,
        );

        first_builded_block.top_level = true;
        first_builded_block.x = Some(0.into());
        first_builded_block.y = Some(0.into());

        let mut previous_builded_block = (first_builded_block, first_block_uid);
        for block2 in stack_iter {
            let (mut builded_block1, block1_uid) = previous_builded_block;
            let block2_uid = Uid::generate();
            let builded_block2 = block2.build(
                block2_uid,
                global_scripting_ctx,
                local_scripting_ctx,
                &mut block_stack_ctx_data,
            );
            let sb_sbity::block::Block::Normal(mut builded_block2) =
                 builded_block2 else {
                unreachable!("BlockVarList shouldn't exist here")
            };

            builded_block1.next = Some(block2_uid.to_string());
            builded_block2.parent = Some(block1_uid.to_string());

            previous_builded_block = (builded_block2, block2_uid);

            builded_stack.insert(block1_uid, sb_sbity::block::Block::Normal(builded_block1));
        }
        builded_stack.insert(
            previous_builded_block.1,
            sb_sbity::block::Block::Normal(previous_builded_block.0),
        );
        builded_stack
    }
}
