use crate::{view::page::page, Mdl, Msg, Page};
use ofdb_entities::{
    event::Event,
    event::RegistrationType,
    user::{Role, User},
};
use seed::{prelude::*, *};

pub fn event(user: Option<User>, ev: Event) -> Node<Msg> {
    page(
        &ev.title,
        user.as_ref().map(|u| &*u.email),
        None,
        None,
        div![
            C!["details", "event"],
            div![ C!["entity-type"], "Event"],
            h2![ &ev.title ],
            p![ C!["time"],
                ev.start.format("%d.%m.%Y %H:%M").to_string(),
                ev.end.map(|e| e.format(" - %d.%m.%Y %H:%M").to_string())
            ],
            p![
                C!["description"],
                ev.description.unwrap_or_default()
            ],

            ev.location.map(|location|{
                nodes![
                    h4![ "Ort" ],
                    location.address.as_ref().map(|addr|{
                        if !addr.is_empty(){
                            nodes![
                                addr.street.as_ref().map(|s| nodes![ Node::new_text(s.clone()), br![] ]),
                                addr.zip.as_ref().map(|z| Node::new_text(format!("{} ", z)))
                                addr.city.as_ref().map(|c| nodes![ Node::new_text(c.clone()), br![] ])
                                addr.country.clone().map(Node::new_text)
                            ]
                        } else {
                            nodes![]
                        }
                    }),
                    h4! [ "Koordinaten" ],
                    p! [
                        format!("{:.2} / {:.2}",location.pos.lat().to_deg(), location.pos.lng().to_deg())
                    ]
                ]
            }),
            ev.organizer.map(|org|
                nodes![
                    h4!["Veranstalter"],
                    p![org]
                ]
            ),
            ev.contact.map(|contact|{
                if !contact.is_empty(){
                    nodes![
                        h4![ "Kontakt" ],
                        contact.email.map(|email| nodes![
                            Node::new_text(email.to_string()),
                            br![]
                        ]),
                        contact.phone.map(Node::new_text)
                    ]
                } else {
                    nodes![]
                }
            }),
            ev.homepage.map(|url|
                nodes![
                    h4!["Webseite"]
                    p![
                        a![
                            attrs!{
                                At::Href => url;
                            },
                            url.to_string()
                        ]
                    ]
                ]
            ),
            ev.registration.map(|reg|{
                nodes![
                h4![ "Anmeldung" ],
                p![
                    match reg{
                    RegistrationType::Email => "eMail" ,
                    RegistrationType::Phone => "Telefon",
                    RegistrationType::Homepage => "Webseite",
                    }
                ]
                ]
            }),
            p![
                h4![ "Tags" ],
                ul![
                    C!["tags"],
                    ev.tags.iter().map(|t|
                        li![ format!("#{}",t) ]
                    )
                ]
            ],
            if let Some(user) = &user {
                match user.role {
                    Role::Admin | Role::Scout => {
                        form![
                            attrs!{
                                At::Action => format!("/events/{}/archive", ev.id);
                                At::Method => "POST";
                            },
                            input![
                                attrs!{
                                    At::Type => "submit";
                                    At::Value => "archive event";
                                }
                            ]
                        ]
                    }
                    _=> empty!()
                }
            } else {
                empty!()
            }
            // TODO: @if let Some(l) = ev.location {
            // TODO:     div id="map" style="height:100vh;" { }
            // TODO:     (map_scripts(&[l.into()]))
            // TODO: }
        ]
    )
}

pub fn events(email: Option<&str>, events: &[Event]) -> Node<Msg> {
    // TODO: let locations: Vec<_> = events
    // TODO:     .iter()
    // TODO:     .filter_map(|e| e.location.as_ref())
    // TODO:     .map(|l| l.into())
    // TODO:     .collect();
    let event_lis = events.iter().map(|e| {
        li![a![
            attrs! {At::Href => format!("/events/{}", e.id)},
            div![
                h4![
                    span![C!["title"], &e.title],
                    " ",
                    span![C!["date"], e.start.format("%d.%m.%y").to_string()]
                ],
                p![
                    e.location
                        .as_ref()
                        .and_then(|l| l.address.as_ref().and_then(|a| a
                            .city
                            .as_ref()
                            .map(|city| nodes![span![class!["city"], city], br![]]))),
                    e.organizer.as_ref().map(|o| span![C!["organizer"], o])
                ]
            ]
        ]]
    });

    page(
        "List of Events",
        email,
        None,
        None,
        div![
            C!["events"],
            h3!["Events"],
            if events.is_empty() {
                p![C!["no-results"], "Es konnten keine Events gefunden werden."]
            } else {
                ul![C!["event-list"], event_lis,]
            }
        ], // TODO: div id="map" style="height:100vh;" { }
           // TODO: (map_scripts(&locations))
    )
}
