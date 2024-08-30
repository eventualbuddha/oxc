use oxc_ast::{AstKind, Comment};

pub struct CommentWhitespace<'a> {
    start: u32,
    end: u32,
    comment: Vec<Comment>,
    leading_node: AstKind<'a>,
    trailing_node: AstKind<'a>,
    containing_node: AstKind<'a>,
}
