use std::collections::HashMap;

use crate::{name::Name, opcode::OpCode, uid::Uid};

use super::{
    comment::CommentBuilder,
    context::{BlockStackContextData, GlobalScriptingContextData, LocalScriptingContextData},
    stack::StackBuilder,
};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct BlockBuilder {
    pub opcode: OpCode,
    pub comment: Option<CommentBuilder>,
    pub inputs: HashMap<String, BlockInputBuilder>,
    pub fields: HashMap<String, BlockFieldBuilder>,
    pub mutation: Option<sb_sbity::block::BlockMutation>,
    pub shadow: bool,
    pub x: Option<f64>,
    pub y: Option<f64>,
}

impl BlockBuilder {
    pub fn new<O: Into<OpCode>>(opcode: O) -> BlockBuilder {
        BlockBuilder {
            opcode: opcode.into(),
            ..Default::default()
        }
    }

    pub fn add_input(&mut self, key: String, block_input_builder: BlockInputBuilder) -> &mut Self {
        self.inputs.insert(key, block_input_builder);
        self
    }

    pub fn add_field(&mut self, key: String, block_field_builder: BlockFieldBuilder) -> &mut Self {
        self.fields.insert(key, block_field_builder);
        self
    }

    pub fn build(
        self,
        my_uid: Uid,
        global_scripting_ctx: &GlobalScriptingContextData,
        local_scripting_ctx: &LocalScriptingContextData,
        block_stack_ctx: &mut BlockStackContextData,
    ) -> sb_sbity::block::BlockNormal {
        let BlockBuilder {
            opcode,
            comment,
            inputs,
            fields,
            shadow,
            mutation,
            x,
            y,
        } = self;
        // let mut inputs_b: HashMap<String, BlockInput> = HashMap::default();
        // for (key, input) in inputs {
        //     inputs_b.insert(key, input.build(comment_buff, final_stack, &my_uid));
        // }
        let inputs: HashMap<String, sb_sbity::block::BlockInput> = inputs
            .into_iter()
            .map(|(key, input)| {
                (
                    key,
                    input.build(
                        my_uid,
                        global_scripting_ctx,
                        local_scripting_ctx,
                        block_stack_ctx,
                    ),
                )
            })
            .collect();
        let fields: HashMap<String, sb_sbity::block::BlockField> = fields
            .into_iter()
            .map(|(key, field)| (key, field.build(global_scripting_ctx, local_scripting_ctx)))
            .collect();
        let comment = match comment {
            Some(comment) => {
                let comment_uid = Uid::generate();
                let mut comment = comment.build();
                comment.block_id = Some(my_uid.to_string());
                local_scripting_ctx
                    .comments
                    .insert(comment_uid.clone(), comment);
                Some(comment_uid.to_string())
            }
            None => None,
        };

        sb_sbity::block::BlockNormal {
            opcode: opcode.to_string(),
            comment,
            next: None,
            parent: None,
            inputs: sb_sbity::string_hashmap::StringHashMap(inputs),
            fields: sb_sbity::string_hashmap::StringHashMap(fields),
            shadow,
            top_level: false,
            mutation,
            x: x.map(|x| x.into()),
            y: y.map(|y| y.into()),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum StackOrValue {
    Value(sb_sbity::block::BlockInputValue),
    Stack(StackBuilder),
}

#[derive(Debug, Clone, PartialEq)]
pub struct BlockInputBuilder {
    pub shadow: sb_sbity::block::ShadowInputType,
    pub values: Vec<Option<StackOrValue>>,
}

impl BlockInputBuilder {
    pub fn new() -> BlockInputBuilder {
        BlockInputBuilder {
            shadow: sb_sbity::block::ShadowInputType::NoShadow,
            values: vec![],
        }
    }

    pub fn add_input(&mut self, input: Option<StackOrValue>) -> &mut Self {
        self.values.push(input);
        self
    }

    /// Shortcut for
    /// ```
    /// BlockInputBuilder::new()
    ///     .shadow(ShadowInputType::Shadow)
    ///     .input(Some(StackOrValue::Value(value)))
    /// ```
    pub fn value(value: sb_sbity::block::BlockInputValue) -> Self {
        let mut b = BlockInputBuilder::new();
        b.shadow = sb_sbity::block::ShadowInputType::Shadow;
        b.add_input(Some(StackOrValue::Value(value)));
        b
    }

    /// Shortcut for
    /// ```
    /// BlockInputBuilder::new()
    ///     .shadow(ShadowInputType::NoShadow)
    ///     .input(Some(StackOrValue::Stack(stack)))
    /// ```
    pub fn stack(stack: StackBuilder) -> Self {
        let mut b = BlockInputBuilder::new();
        b.shadow = sb_sbity::block::ShadowInputType::NoShadow;
        b.add_input(Some(StackOrValue::Stack(stack)));
        b
    }

    /// Shortcut for
    /// ```
    /// BlockInputBuilder::new()
    ///     .shadow(ShadowInputType::ShadowObscured)
    ///     .input(Some(StackOrValue::Stack(stack)))
    ///     .input(Some(StackOrValue::Value(value)))
    /// ```
    pub fn stack_with_value_obscured(
        stack: StackBuilder,
        value: sb_sbity::block::BlockInputValue,
    ) -> Self {
        let mut b = BlockInputBuilder::new();
        b.shadow = sb_sbity::block::ShadowInputType::ShadowObscured;
        b.add_input(Some(StackOrValue::Stack(stack)))
            .add_input(Some(StackOrValue::Value(value)));
        b
    }

    pub fn build(
        self,
        this_block_uid: Uid,
        // comment_buff: &mut HashMap<Uid, sb_sbity::block::Comment>,
        // final_stack: &mut HashMap<Uid, sb_sbity::block::Block>,
        global_scripting_ctx: &GlobalScriptingContextData,
        local_scripting_ctx: &LocalScriptingContextData,
        block_stack_ctx: &mut BlockStackContextData,
    ) -> sb_sbity::block::BlockInput {
        use sb_sbity::block::UidOrValue;

        let BlockInputBuilder { shadow, values } = self;
        let mut values_b: Vec<Option<UidOrValue>> = vec![];
        for value in values {
            match value {
                Some(StackOrValue::Value(value)) => values_b.push(Some(UidOrValue::Value(value))),
                Some(StackOrValue::Stack(stack)) => {
                    let first_block_uid = Uid::generate();
                    let mut builded_stack =
                        stack.build(first_block_uid, global_scripting_ctx, local_scripting_ctx);
                    let first_block = builded_stack.get_mut(&first_block_uid).unwrap();
                    match first_block {
                        sb_sbity::block::Block::Normal(n) => {
                            n.parent = Some(this_block_uid.to_string());
                            n.top_level = false;
                            n.x = None;
                            n.y = None;
                        }
                        sb_sbity::block::Block::VarList(_) => {
                            let sb_sbity::block::Block::VarList(vl) = builded_stack.remove(&first_block_uid).unwrap() else {
                                unreachable!()
                            };
                            let sb_sbity::block::BlockVarListReporterTop { kind, name, id, .. } =
                                vl;
                            values_b.push(Some(UidOrValue::Value(match kind {
                                sb_sbity::block::ListOrVariable::Variable => {
                                    sb_sbity::block::BlockInputValue::Variable { name, id }
                                }
                                sb_sbity::block::ListOrVariable::List => {
                                    sb_sbity::block::BlockInputValue::List { name, id }
                                }
                            })))
                        }
                    }
                    block_stack_ctx.blocks.extend(builded_stack);
                    values_b.push(Some(UidOrValue::Uid(first_block_uid.to_string())))
                }
                None => values_b.push(None),
            }
        }
        sb_sbity::block::BlockInput {
            shadow,
            inputs: values_b,
        }
    }
}

impl Default for BlockInputBuilder {
    fn default() -> Self {
        BlockInputBuilder {
            shadow: sb_sbity::block::ShadowInputType::NoShadow,
            values: vec![],
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum FieldKind {
    NoRef,
    #[default]
    NoRefMaybe,
    Broadcast,
    SpriteVariable,
    GlobalVariable,
    SpriteList,
    GlobalList,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct BlockFieldBuilder {
    pub value: String,
    pub kind: FieldKind,
}

impl BlockFieldBuilder {
    pub fn new_with_kind(value: String, kind: FieldKind) -> BlockFieldBuilder {
        BlockFieldBuilder { value, kind }
    }

    pub fn new(value: String) -> BlockFieldBuilder {
        BlockFieldBuilder {
            value,
            kind: FieldKind::NoRefMaybe,
        }
    }

    pub fn build(
        self,
        global_scripting_ctx: &GlobalScriptingContextData,
        local_scripting_ctx: &LocalScriptingContextData,
    ) -> sb_sbity::block::BlockField {
        let BlockFieldBuilder { value, kind } = self;
        let value = value.into();
        let sb_sbity::value::Value::Text(ref value_str) = value else {
            unreachable!()
        };
        let id = match kind {
            FieldKind::NoRef => return sb_sbity::block::BlockField::NoId { value },
            FieldKind::NoRefMaybe => {
                return sb_sbity::block::BlockField::WithId { value, id: None }
            }

            // FieldKind::Broadcast => target_context.all_broadcasts,
            // FieldKind::SpriteVariable => target_context.this_sprite_vars,
            // FieldKind::GlobalVariable => target_context.global_vars,
            // FieldKind::SpriteList => target_context.this_sprite_lists,
            // FieldKind::GlobalList => target_context.global_lists,
            FieldKind::Broadcast => global_scripting_ctx.broadcasts,
            FieldKind::SpriteVariable => local_scripting_ctx.variables,
            FieldKind::GlobalVariable => global_scripting_ctx.global_variables,
            FieldKind::SpriteList => local_scripting_ctx.lists,
            FieldKind::GlobalList => global_scripting_ctx.global_lists,
        }
        .get(&Name::from(&value_str[..]))
        .cloned()
        .expect("a valid broadcast/variable/list and their global variant");
        sb_sbity::block::BlockField::WithId {
            value,
            id: Some(id.to_string()),
        }
    }
}

// this is not included yet since it is kinda useless
// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// pub enum VarListFrom {
//     Global,
//     Sprite,
// }

// #[derive(Debug, Clone, PartialEq)]
// pub struct BlockVarListBuilder {
//     pub kind: ListOrVariable,
//     pub from: VarListFrom,
//     pub name: String,
//     pub x: f64,
//     pub y: f64,
//     pub comment: Option<CommentBuilder>,
// }

// impl BlockVarListBuilder {
//     pub fn global_var<S: Into<String>>(name: S) -> BlockVarListBuilder {
//         BlockVarListBuilder {
//             kind: ListOrVariable::Variable,
//             name: name.into(),
//             from: VarListFrom::Global,
//             x: 0.,
//             y: 0.,
//             comment: None,
//         }
//     }

//     pub fn global_list<S: Into<String>>(name: S) -> BlockVarListBuilder {
//         BlockVarListBuilder {
//             kind: ListOrVariable::List,
//             name: name.into(),
//             from: VarListFrom::Global,
//             x: 0.,
//             y: 0.,
//             comment: None,
//         }
//     }

//     pub fn sprite_var<S: Into<String>>(name: S) -> BlockVarListBuilder {
//         BlockVarListBuilder {
//             kind: ListOrVariable::Variable,
//             from: VarListFrom::Sprite,
//             name: name.into(),
//             x: 0.,
//             y: 0.,
//             comment: None,
//         }
//     }

//     pub fn sprite_list<S: Into<String>>(name: S) -> BlockVarListBuilder {
//         BlockVarListBuilder {
//             kind: ListOrVariable::List,
//             from: VarListFrom::Sprite,
//             name: name.into(),
//             x: 0.,
//             y: 0.,
//             comment: None,
//         }
//     }

//     pub fn set_kind(&mut self, kind: ListOrVariable) -> &mut Self {
//         self.kind = kind;
//         self
//     }

//     pub fn set_from(&mut self, from: VarListFrom) -> &mut Self {
//         self.from = from;
//         self
//     }

//     pub fn set_name(&mut self, name: String) -> &mut Self {
//         self.name = name;
//         self
//     }

//     pub fn set_x(&mut self, x: f64) -> &mut Self {
//         self.x = x;
//         self
//     }

//     pub fn set_y(&mut self, y: f64) -> &mut Self {
//         self.y = y;
//         self
//     }

//     pub fn set_pos(&mut self, x: f64, y: f64) -> &mut Self {
//         self.x = x;
//         self.y = y;
//         self
//     }

//     pub fn set_comment(&mut self, comment: Option<CommentBuilder>) -> &mut Self {
//         self.comment = comment;
//         self
//     }

//     pub fn build(
//         self,
//         my_uid: &Uid,
//         comment_buff: &mut HashMap<Uid, Comment>,
//         target_context: &TargetContext,
//     ) -> BlockVarListReporterTop {
//         let BlockVarListBuilder {
//             kind,
//             from,
//             name,
//             x,
//             y,
//             comment,
//         } = self;
//         let varlist_id = match (&kind, from) {
//             (ListOrVariable::Variable, VarListFrom::Global) => target_context.global_vars,
//             (ListOrVariable::Variable, VarListFrom::Sprite) => target_context.this_sprite_vars,
//             (ListOrVariable::List, VarListFrom::Global) => target_context.global_lists,
//             (ListOrVariable::List, VarListFrom::Sprite) => target_context.this_sprite_lists,
//         }
//         .get(&name)
//         .cloned()
//         .unwrap_or(Uid::new("__unknown__"));
//         if let Some(comment) = comment {
//             let comment_uid = Uid::generate();
//             let mut comment = comment.build();
//             comment.block_id = Some(my_uid.clone().into_inner());
//             comment_buff.insert(comment_uid, comment);
//         }

//         BlockVarListReporterTop {
//             kind,
//             name,
//             id: varlist_id.into_inner(),
//             x: x.into(),
//             y: y.into(),
//         }
//     }
// }

// #[derive(Debug, Clone, PartialEq)]
// pub enum BlockBuilder {
//     Normal(BlockNormalBuilder),
//     VarList(BlockVarListBuilder),
// }

// impl BlockBuilder {
//     pub fn build(
//         self,
//         my_uid: &Uid,
//         comment_buff: &mut HashMap<Uid, Comment>,
//         final_stack: &mut HashMap<Uid, Block>,
//         target_context: &TargetContext,
//     ) -> Block {
//         match self {
//             BlockBuilder::Normal(n) => {
//                 let b = n.build(my_uid, comment_buff, final_stack, target_context);
//                 Block::Normal(b)
//             }
//             BlockBuilder::VarList(vl) => {
//                 let b = vl.build(my_uid, comment_buff, target_context);
//                 Block::VarList(b)
//             }
//         }
//     }
// }
