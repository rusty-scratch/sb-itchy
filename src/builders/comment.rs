use crate::{
    build::{Build, BuildMethod, BuildReport},
    uid::Uid,
};
use sb_sbity::comment::Comment;

#[rustfmt::skip]
#[derive(Debug, Clone, PartialEq)]
pub struct CommentBuilder {
    block_uid:    Option<Uid>,
    x:            Option<f64>,
    y:            Option<f64>,
    width:        u64,
    height:       u64,
    minimized:    bool,
    content:      String,

    build_method: BuildMethod,
}

impl CommentBuilder {
    pub fn new() -> CommentBuilder {
        CommentBuilder {
            ..Default::default()
        }
    }

    pub fn set_size(&mut self, width: u64, height: u64) -> &mut Self {
        self.width = width;
        self.height = height;
        self
    }

    pub fn set_pos(&mut self, x: Option<f64>, y: Option<f64>) -> &mut Self {
        self.x = x;
        self.y = y;
        self
    }

    pub fn set_width(&mut self, width: u64) -> &mut Self {
        self.width = width;
        self
    }

    pub fn set_height(&mut self, height: u64) -> &mut Self {
        self.height = height;
        self
    }

    pub fn set_x(&mut self, x: f64) -> &mut Self {
        self.x = Some(x);
        self
    }

    pub fn set_y(&mut self, y: f64) -> &mut Self {
        self.y = Some(y);
        self
    }

    pub fn set_content<S: Into<String>>(&mut self, content: S) -> &mut Self {
        self.content = content.into();
        self
    }

    pub fn set_minimized(&mut self, minimized: bool) -> &mut Self {
        self.minimized = minimized;
        self
    }
}

impl Build for CommentBuilder {
    type Into = Comment;
    type BuildError = void::Void;
    type Arg = ();

    fn build(self, arg: Self::Arg) -> (Result<Self::Into, Self::BuildError>, BuildReport) {
        let CommentBuilder {
            block_uid,
            x,
            y,
            width,
            height,
            minimized,
            content,
            build_method: _,
        } = self;

        let comment = Comment {
            block_id: block_uid.map(|u| u.into_string()),
            x: x.map(|n| n.into()),
            y: y.map(|n| n.into()),
            width: (width as i64).into(),
            height: (height as i64).into(),
            minimized,
            text: content,
        };
        (Ok(comment), BuildReport::new())
    }

    fn check(&self, _: crate::build::BuildMethod) -> (Option<Self::BuildError>, BuildReport) {
        (None, BuildReport::new())
    }

    fn build_method(&self) -> crate::build::BuildMethod {
        self.build_method
    }

    fn set_build_method_to(&mut self, method: crate::build::BuildMethod) {
        self.build_method = method
    }

    fn set_build_method_recursivly_to(&mut self, method: crate::build::BuildMethod) {
        self.build_method = method
    }
}

impl Default for CommentBuilder {
    #[rustfmt::skip]
    fn default() -> Self {
        CommentBuilder {
            block_uid: None,
            x:         None,
            y:         None,
            width:     200,
            height:    200,
            minimized: false,
            content:   String::new(),

            build_method: BuildMethod::default()
        }
    }
}
