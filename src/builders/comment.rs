use crate::uid::Uid;
use sb_sbity::comment::Comment;

#[derive(Debug, Clone, PartialEq)]
pub struct CommentBuilder {
    pub block_uid: Option<Uid>,
    pub x: Option<f64>,
    pub y: Option<f64>,
    pub width: u64,
    pub height: u64,
    pub minimized: bool,
    pub content: String,
}

impl CommentBuilder {
    pub fn build(self) -> Comment {
        let CommentBuilder {
            block_uid,
            x,
            y,
            width,
            height,
            minimized,
            content,
        } = self;
        Comment {
            block_id: block_uid.map(|u| u.to_string()),
            x: x.map(|n| n.into()),
            y: y.map(|n| n.into()),
            width: (width as i64).into(),
            height: (height as i64).into(),
            minimized,
            text: content,
        }
    }
}

impl Default for CommentBuilder {
    fn default() -> Self {
        CommentBuilder {
            block_uid: None,
            x: None,
            y: None,
            width: 200,
            height: 200,
            minimized: false,
            content: "".to_owned(),
        }
    }
}
