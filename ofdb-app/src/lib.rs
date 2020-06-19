use seed::prelude::*;

mod page;
#[cfg(test)]
mod tests;
mod update;
mod view;

use page::Page;

#[derive(Debug, Default)]
pub struct Context {
    user: Option<String>,
}

#[derive(Debug)]
pub struct Mdl {
    ctx: Context,
    page: Page,
}

#[derive(Clone)]
pub enum Msg {
    UrlChanged(subs::UrlChanged),
}

fn init(url: Url, orders: &mut impl Orders<Msg>) -> Mdl {
    orders.subscribe(Msg::UrlChanged);
    Mdl {
        page: Page::init(url),
        ctx: Default::default(),
    }
}

pub fn view(mdl: &Mdl) -> Node<Msg> {
    match &mdl.page {
        Page::Home(mdl) => page::index::view(&mdl),
        Page::Login(mdl) => page::login::view(&mdl),
        Page::Register(mdl) => page::register::view(&mdl),
        Page::ResetPassword(mdl) => page::reset_password::view(&mdl),
        Page::Events(mdl) => page::events::view(&mdl),
        Page::NotFound => view::index(None),
    }
}

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update::update, view);
}
