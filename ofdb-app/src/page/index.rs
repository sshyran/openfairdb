use crate::Msg;
use seed::prelude::*;

#[derive(Debug)]
pub struct Mdl {}

pub fn init(mut url: Url) -> Option<Mdl> {
    todo!()
}

pub fn view(mdl: &Mdl) -> Node<Msg> {
    crate::view::index(None)
}
