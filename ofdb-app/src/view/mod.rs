use crate::{Mdl, Msg, Page};
use ofdb_entities::{address::*, location::*, place::*};
use seed::{prelude::*, *};
// TODO: use num_traits::ToPrimitive;

pub struct FlashMessage {
    name: String,
    msg: String,
}

impl FlashMessage {
    fn name(&self) -> &str {
        &self.name
    }
    fn msg(&self) -> &str {
        &self.msg
    }
}

pub mod dashboard;
pub mod entry;
pub mod event;
pub mod login;
pub mod page;
pub mod password;
pub mod place;
pub mod register;

pub use dashboard::*;
pub use entry::*;
pub use event::*;
pub use login::*;
use page::*;
pub use password::*;
pub use place::*;
pub use register::*;

pub fn index(email: Option<&str>) -> Node<Msg> {
    page::page(
        "OpenFairDB Search",
        email,
        None,
        None,
        div![
            class!["search"],
            h1!["OpenFairDB Search"],
            global_search_form(None)
        ],
    )
}

fn global_search_form(search_term: Option<&str>) -> Node<Msg> {
    div![
        class!["search-form"],
        form![
            attrs! {
                At::Action=> "search";
                At::Method=> "GET";
            },
            input![attrs! {
                At::Type=>"text";
                At::Name=>"q";
                At::Value => search_term.unwrap_or("");
                At::Size=> 50;
                At::MaxLength=>200;
                At::Placeholder=> "search term, empty = all";
            }],
            br![],
            input![
                class!["btn"],
                attrs! {
                    At::Type=>"submit";
                    At::Value=>"search";
                }
            ]
        ]
    ]
}

pub fn search_results(email: Option<&str>, search_term: &str, entries: &[Place]) -> Node<Msg> {
    page(
        "OpenFairDB Search Results",
        email,
        None,
        None,
        div![
            div![
                class!["search"],
                h1!["OpenFairDB Search"],
                global_search_form(Some(search_term))
            ],
            div![
                class!["results"],
                if entries.is_empty() {
                    p![
                        "We are so sorry but we could not find any entries
                                 that are related to your search term ",
                        em![format!("'{}'", search_term)]
                    ]
                } else {
                    p![ul![
                        class!["result-list"],
                        entries.iter().map(|e| li![entry_result(e)])
                    ]]
                }
            ]
        ],
    )
}

fn entry_result(e: &Place) -> Vec<Node<Msg>> {
    vec![
        h3![a![
            attrs! {
                At::Href => format!("entries/{}",e.id);
            },
            &e.title
        ]],
        p![&e.description],
    ]
}

fn address_to_html(addr: &Address) -> Vec<Node<Msg>> {
    vec![
        if let Some(ref s) = addr.street {
            span![s, br![]]
        } else {
            empty!()
        },
        if let Some(ref z) = addr.zip {
            span![z]
        } else {
            empty!()
        },
        if let Some(ref z) = addr.city {
            span![z, br![]]
        } else {
            empty!()
        },
        if let Some(ref c) = addr.country {
            span![c]
        } else {
            empty!()
        },
    ]
}

struct MapPin {
    lat: f64,
    lng: f64,
}

impl MapPin {
    fn to_js_object_string(&self) -> String {
        format!("{{lat:{},lng:{}}}", self.lat, self.lng)
    }
}

impl From<Place> for MapPin {
    fn from(e: Place) -> Self {
        e.location.into()
    }
}

impl From<&Place> for MapPin {
    fn from(e: &Place) -> Self {
        (&e.location).into()
    }
}

impl From<Location> for MapPin {
    fn from(l: Location) -> Self {
        (&l).into()
    }
}

impl From<&Location> for MapPin {
    fn from(l: &Location) -> Self {
        MapPin {
            lat: l.pos.lat().to_deg(),
            lng: l.pos.lng().to_deg(),
        }
    }
}

// TODO: fn map_scripts(pins: &[MapPin]) -> Markup {
// TODO:     let (center, zoom) = match pins.len() {
// TODO:         1 => ((pins[0].lat, pins[0].lng), 13.0),
// TODO:         _ => {
// TODO:             //TODO: calculate center & zoom
// TODO:             ((48.720, 9.152), 6.0)
// TODO:         }
// TODO:     };
// TODO:
// TODO:     let center = format!("[{},{}]", center.0, center.1);
// TODO:
// TODO:     let pins: String = pins
// TODO:         .iter()
// TODO:         .map(MapPin::to_js_object_string)
// TODO:         .collect::<Vec<_>>()
// TODO:         .join(",");
// TODO:
// TODO:     html! {
// TODO:       script{
// TODO:         (format!("window.OFDB_MAP_PINS=[{}];window.OFDB_MAP_ZOOM={};OFDB_MAP_CENTER={};",
// TODO:                  pins,
// TODO:                  zoom,
// TODO:                  center))
// TODO:       }
// TODO:       script
// TODO:         src=(LEAFLET_JS_URL)
// TODO:         integrity=(LEAFLET_JS_SHA512)
// TODO:         crossorigin="anonymous" {}
// TODO:       script src=(MAP_JS_URL){}
// TODO:     }
// TODO: }
// TODO:
// TODO: pub fn user_search_result(admin_email: &str, users: &[User]) -> Markup {
// TODO:     page(
// TODO:         "Users",
// TODO:         Some(admin_email),
// TODO:         None,
// TODO:         None,
// TODO:         html! {
// TODO:             main {
// TODO:                 h3 { "Users" }
// TODO:                 (search_users_form())
// TODO:                 @if users.is_empty() {
// TODO:                     "No users were found :("
// TODO:                 } @else {
// TODO:                     table {
// TODO:                         thead {
// TODO:                             tr {
// TODO:                               th { "Username"        }
// TODO:                               th { "eMail"           }
// TODO:                               th { "eMail confirmed" }
// TODO:                               th { "Role"            }
// TODO:                               th { "Modify role"            }
// TODO:                             }
// TODO:                         }
// TODO:                         tbody {
// TODO:                             @for u in users {
// TODO:                                 tr {
// TODO:                                     td { (u.email) }
// TODO:                                     td { (if u.email_confirmed{"yes"}else{"no"}) }
// TODO:                                     td { (format!("{:?}",u.role)) }
// TODO:                                     td {
// TODO:                                         @if u.email != admin_email {
// TODO:                                             form action="change-user-role" method="POST" {
// TODO:                                                 select name = "role" required? {
// TODO:                                                     option value="-1" {"-- please select --"}
// TODO:                                                     option value=(Role::Guest.to_u8().unwrap()) { "Guest" }
// TODO:                                                     option value=(Role::User.to_u8().unwrap())  { "User" }
// TODO:                                                     option value=(Role::Scout.to_u8().unwrap()) { "Scout" }
// TODO:                                                 }
// TODO:                                                 input type="hidden" name="email" value=(u.email);
// TODO:                                                 input type="submit" value="change";
// TODO:                                             }
// TODO:                                         }
// TODO:                                     }
// TODO:                                 }
// TODO:                             }
// TODO:                         }
// TODO:                     }
// TODO:                 }
// TODO:             }
// TODO:         },
// TODO:     )
// TODO: }

pub fn search_users_form() -> Node<Msg> {
    form![
        attrs! {
            At::Action => "search-users";
            At::Method => "GET";
        },
        input![attrs! {
            At::Type=>"email";
            At::Name=>"email";
            At::Placeholder=>"email address";
        },],
        br![],
        input![attrs! {
            At::Type=>"submit";
            At::Value =>"search";
        },]
    ]
}
