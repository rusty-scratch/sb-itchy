use crate::uid::Uid;
use sb_sbity::comment::Comment;

#[rustfmt::skip]
#[derive(Debug, Clone, PartialEq)]
pub struct CommentBuilder {
    block_uid: Option<Uid>,
    x:         Option<f64>,
    y:         Option<f64>,
    width:     u64,
    height:    u64,
    minimized: bool,
    content:   String,
}

impl CommentBuilder {
    pub fn new<S: Into<String>>(content: S) -> CommentBuilder {
        CommentBuilder {
            content: content.into(),
            ..Default::default()
        }
    }

    pub fn pos(mut self, x: f64, y: f64) -> Self {
        self.x = Some(x);
        self.y = Some(y);
        self
    }

    pub fn size(mut self, width: u64, height: u64) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    pub fn minimized(mut self, minimized: bool) -> Self {
        self.minimized = minimized;
        self
    }

    /// Requires:
    /// - block_uid?: To connect the block with comment
    ///
    /// Returns:
    /// - [`Uid`]: [`Uid`] of the built comment inside [`Target`]'s comment list
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
        let comment = Comment {
            block_id: block_uid.map(|u| u.into_inner()),
            x: x.map(|n| n.into()),
            y: y.map(|n| n.into()),
            width: (width as i64).into(),
            height: (height as i64).into(),
            minimized,
            text: content,
        };
        comment
    }
}

impl Default for CommentBuilder {
    #[rustfmt::skip]
    fn default() -> Self {
        CommentBuilder {
            block_uid: None,
            x:         Some(0.),
            y:         Some(0.),
            width:     200,
            height:    200,
            minimized: false,
            content:   "".to_owned(),
        }
    }
}
