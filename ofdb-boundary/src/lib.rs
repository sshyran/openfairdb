//#![deny(missing_docs)] // TODO: Complete missing documentation and enable this option
#![cfg_attr(features = "extra-derive", deny(missing_debug_implementations))]
#![deny(broken_intra_doc_links)]
#![cfg_attr(test, deny(warnings))]

//! # ofdb-boundary
//!
//! Serializable, anemic data structures for accessing the OpenFairDB API in a type-safe manner.
//!
//! Only supposed to be used as short-lived, transitional instances for (de-)serializing entities!

use ofdb_entities as e;

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::convert::{TryFrom, TryInto};
use url::Url;

#[rustfmt::skip]
#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "extra-derive", derive(Debug, Clone, PartialEq))]
pub struct Entry {
    pub id             : String,
    pub created        : i64,
    pub version        : u64,
    pub title          : String,
    pub description    : String,
    pub lat            : f64,
    pub lng            : f64,
    pub street         : Option<String>,
    pub zip            : Option<String>,
    pub city           : Option<String>,
    pub country        : Option<String>,
    pub state          : Option<String>,
    pub contact_name   : Option<String>,
    pub email          : Option<String>,
    pub telephone      : Option<String>,
    pub homepage       : Option<String>,
    pub opening_hours  : Option<String>,
    pub founded_on     : Option<NaiveDate>,
    pub categories     : Vec<String>,
    pub tags           : Vec<String>,
    pub ratings        : Vec<String>,
    pub license        : Option<String>,
    pub image_url      : Option<String>,
    pub image_link_url : Option<String>,

    #[serde(rename = "custom", skip_serializing_if = "Vec::is_empty", default = "Default::default")]
    pub custom_links   : Vec<CustomLink>,
}

#[rustfmt::skip]
#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "extra-derive", derive(Debug, Clone, PartialEq, Eq))]
pub struct CustomLink {
    pub url            : String,
    pub title          : Option<String>,
    pub description    : Option<String>,
}

impl From<e::links::CustomLink> for CustomLink {
    fn from(from: e::links::CustomLink) -> Self {
        let e::links::CustomLink {
            url,
            title,
            description,
        } = from;
        Self {
            url: url.to_string(),
            title,
            description,
        }
    }
}

// TODO: use TryFrom
impl From<CustomLink> for e::links::CustomLink {
    fn from(from: CustomLink) -> Self {
        let CustomLink {
            url,
            title,
            description,
        } = from;
        Self {
            url: url.parse().unwrap(),
            title,
            description,
        }
    }
}

#[rustfmt::skip]
#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "extra-derive", derive(Debug, Clone, PartialEq))]
pub struct NewPlace {
    pub title          : String,
    pub description    : String,
    pub lat            : f64,
    pub lng            : f64,
    pub street         : Option<String>,
    pub zip            : Option<String>,
    pub city           : Option<String>,
    pub country        : Option<String>,
    pub state          : Option<String>,
    pub contact_name   : Option<String>,
    pub email          : Option<String>,
    pub telephone      : Option<String>,
    pub homepage       : Option<String>,
    pub opening_hours  : Option<String>,
    pub founded_on     : Option<NaiveDate>,
    pub categories     : Vec<String>,
    pub tags           : Vec<String>,
    pub license        : String,
    pub image_url      : Option<String>,
    pub image_link_url : Option<String>,

    #[serde(skip_serializing_if = "Vec::is_empty", default = "Default::default")]
    pub links          : Vec<CustomLink>,
}

#[rustfmt::skip]
#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "extra-derive", derive(Debug, Clone, PartialEq))]
pub struct UpdatePlace {
    pub version        : u64,
    pub title          : String,
    pub description    : String,
    pub lat            : f64,
    pub lng            : f64,
    pub street         : Option<String>,
    pub zip            : Option<String>,
    pub city           : Option<String>,
    pub country        : Option<String>,
    pub state          : Option<String>,
    pub contact_name   : Option<String>,
    pub email          : Option<String>,
    pub telephone      : Option<String>,
    pub homepage       : Option<String>,
    pub opening_hours  : Option<String>,
    pub founded_on     : Option<NaiveDate>,
    pub categories     : Vec<String>,
    pub tags           : Vec<String>,
    pub image_url      : Option<String>,
    pub image_link_url : Option<String>,

    #[serde(skip_serializing_if = "Vec::is_empty", default = "Default::default")]
    pub links          : Vec<CustomLink>,
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "extra-derive", derive(Debug, Clone))]
pub struct Event {
    pub id: String,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub start: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lat: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lng: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub street: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub telephone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub homepage: Option<String>,
    pub tags: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registration: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organizer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_link_url: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "extra-derive", derive(Debug, Clone, Copy, PartialEq))]
pub struct Coordinate {
    pub lat: f64,
    pub lng: f64,
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "extra-derive", derive(Debug, Clone))]
pub struct User {
    pub email: String,
    pub email_confirmed: bool,
    pub role: UserRole,
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "extra-derive", derive(Debug, Clone, Copy))]
pub struct RatingValue(i8);

#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "extra-derive", derive(Debug, Clone, Copy))]
pub struct AvgRatingValue(f64);

impl From<f64> for AvgRatingValue {
    fn from(v: f64) -> Self {
        Self(v)
    }
}

impl From<AvgRatingValue> for f64 {
    fn from(from: AvgRatingValue) -> Self {
        from.0
    }
}

impl From<i8> for RatingValue {
    fn from(from: i8) -> Self {
        Self(from)
    }
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(
    feature = "extra-derive",
    derive(Debug, Clone, Copy, PartialEq, Eq, Hash)
)]
#[serde(rename_all = "snake_case")]
pub enum RatingContext {
    Diversity,
    Renewable,
    Fairness,
    Humanity,
    Transparency,
    Solidarity,
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "extra-derive", derive(Debug, Clone))]
pub struct EntrySearchRatings {
    pub total: AvgRatingValue,
    pub diversity: AvgRatingValue,
    pub fairness: AvgRatingValue,
    pub humanity: AvgRatingValue,
    pub renewable: AvgRatingValue,
    pub solidarity: AvgRatingValue,
    pub transparency: AvgRatingValue,
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "extra-derive", derive(Debug, Clone))]
pub struct Comment {
    pub id: String,
    pub created: i64,
    pub text: String,
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "extra-derive", derive(Debug, Clone))]
pub struct Category {
    pub id: String,
    pub name: String,
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "extra-derive", derive(Debug, Clone))]
pub struct PlaceSearchResult {
    pub id: String,
    pub status: Option<ReviewStatus>,
    pub lat: f64,
    pub lng: f64,
    pub title: String,
    pub description: String,
    pub categories: Vec<String>,
    pub tags: Vec<String>,
    pub ratings: EntrySearchRatings,
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(
    feature = "extra-derive",
    derive(Debug, Clone, Copy, PartialEq, Eq, Hash)
)]
#[serde(rename_all = "lowercase")]
pub enum ReviewStatus {
    Archived,
    Confirmed,
    Created,
    Rejected,
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "extra-derive", derive(Debug, Clone))]
pub struct SearchResponse {
    pub visible: Vec<PlaceSearchResult>,
    pub invisible: Vec<PlaceSearchResult>,
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(
    feature = "extra-derive",
    derive(Debug, Clone, Copy, PartialEq, Eq, Hash)
)]
#[serde(rename_all = "lowercase")]
pub enum UserRole {
    Guest,
    User,
    Scout,
    Admin,
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "extra-derive", derive(Debug, Clone))]
pub struct BboxSubscription {
    pub id: String,
    pub south_west_lat: f64,
    pub south_west_lng: f64,
    pub north_east_lat: f64,
    pub north_east_lng: f64,
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "extra-derive", derive(Debug, Clone))]
pub struct MapBbox {
    pub sw: MapPoint,
    pub ne: MapPoint,
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "extra-derive", derive(Debug, Clone))]
pub struct MapPoint {
    pub lat: f64,
    pub lng: f64,
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "extra-derive", derive(Debug, Clone))]
pub struct RequestPasswordReset {
    pub email: String,
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "extra-derive", derive(Debug, Clone))]
pub struct ResetPassword {
    pub token: String,
    pub new_password: String,
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "extra-derive", derive(Debug, Clone))]
pub struct Credentials {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "extra-derive", derive(Debug, Clone))]
pub struct TagFrequency(pub String, pub u64);

#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "extra-derive", derive(Debug, Clone))]
pub struct Rating {
    pub id: String,
    pub title: String,
    pub created: i64,
    pub value: RatingValue,
    pub context: RatingContext,
    pub comments: Vec<Comment>,
    pub source: String,
}

impl From<e::category::Category> for Category {
    fn from(from: e::category::Category) -> Self {
        let name = from.name();
        Self {
            id: from.id.into(),
            name,
        }
    }
}

impl From<e::review::ReviewStatus> for ReviewStatus {
    fn from(from: e::review::ReviewStatus) -> Self {
        use e::review::ReviewStatus::*;
        match from {
            Archived => ReviewStatus::Archived,
            Confirmed => ReviewStatus::Confirmed,
            Created => ReviewStatus::Created,
            Rejected => ReviewStatus::Rejected,
        }
    }
}

impl From<ReviewStatus> for e::review::ReviewStatus {
    fn from(from: ReviewStatus) -> Self {
        use e::review::ReviewStatus::*;
        match from {
            ReviewStatus::Archived => Archived,
            ReviewStatus::Confirmed => Confirmed,
            ReviewStatus::Created => Created,
            ReviewStatus::Rejected => Rejected,
        }
    }
}

impl From<e::user::User> for User {
    fn from(from: e::user::User) -> Self {
        let e::user::User {
            email,
            email_confirmed,
            role,
            password: _password,
        } = from;
        Self {
            email,
            email_confirmed,
            role: role.into(),
        }
    }
}

impl From<e::user::Role> for UserRole {
    fn from(from: e::user::Role) -> Self {
        use e::user::Role::*;
        match from {
            Guest => UserRole::Guest,
            User => UserRole::User,
            Scout => UserRole::Scout,
            Admin => UserRole::Admin,
        }
    }
}

impl From<UserRole> for e::user::Role {
    fn from(from: UserRole) -> Self {
        use e::user::Role::*;
        match from {
            UserRole::Guest => Guest,
            UserRole::User => User,
            UserRole::Scout => Scout,
            UserRole::Admin => Admin,
        }
    }
}

impl From<Coordinate> for e::geo::MapPoint {
    fn from(c: Coordinate) -> Self {
        e::geo::MapPoint::try_from_lat_lng_deg(c.lat, c.lng).unwrap_or_default()
    }
}

impl From<e::tag::TagFrequency> for TagFrequency {
    fn from(from: e::tag::TagFrequency) -> Self {
        Self(from.0, from.1)
    }
}

impl From<e::rating::RatingContext> for RatingContext {
    fn from(from: e::rating::RatingContext) -> Self {
        use e::rating::RatingContext as E;
        use RatingContext as C;
        match from {
            E::Diversity => C::Diversity,
            E::Renewable => C::Renewable,
            E::Fairness => C::Fairness,
            E::Humanity => C::Humanity,
            E::Transparency => C::Transparency,
            E::Solidarity => C::Solidarity,
        }
    }
}

impl From<RatingContext> for e::rating::RatingContext {
    fn from(from: RatingContext) -> Self {
        use e::rating::RatingContext as E;
        use RatingContext as C;
        match from {
            C::Diversity => E::Diversity,
            C::Renewable => E::Renewable,
            C::Fairness => E::Fairness,
            C::Humanity => E::Humanity,
            C::Transparency => E::Transparency,
            C::Solidarity => E::Solidarity,
        }
    }
}

impl From<e::rating::AvgRatingValue> for AvgRatingValue {
    fn from(v: e::rating::AvgRatingValue) -> Self {
        let v: f64 = v.into();
        AvgRatingValue::from(v)
    }
}

impl From<e::rating::RatingValue> for RatingValue {
    fn from(v: e::rating::RatingValue) -> Self {
        let v: i8 = v.into();
        RatingValue::from(v)
    }
}

impl From<RatingValue> for e::rating::RatingValue {
    fn from(v: RatingValue) -> Self {
        e::rating::RatingValue::from(v.0)
    }
}

impl From<e::event::Event> for Event {
    fn from(e: e::event::Event) -> Self {
        let e::event::Event {
            id,
            title,
            description,
            start,
            end,
            location,
            contact,
            tags,
            homepage,
            registration,
            image_url,
            image_link_url,
            ..
        } = e;

        let (lat, lng, address) = if let Some(location) = location {
            if location.pos.is_valid() {
                let lat = location.pos.lat().to_deg();
                let lng = location.pos.lng().to_deg();
                (Some(lat), Some(lng), location.address)
            } else {
                (None, None, location.address)
            }
        } else {
            (None, None, None)
        };

        let e::address::Address {
            street,
            zip,
            city,
            country,
            state,
        } = address.unwrap_or_default();

        let e::contact::Contact {
            name: organizer,
            email,
            phone: telephone,
        } = contact.unwrap_or_default();

        let registration = registration.map(|r| {
            match r {
                e::event::RegistrationType::Email => "email",
                e::event::RegistrationType::Phone => "telephone",
                e::event::RegistrationType::Homepage => "homepage",
            }
            .to_string()
        });

        let start = start.timestamp();
        let end = end.map(|end| end.timestamp());

        Event {
            id: id.into(),
            title,
            description,
            start,
            end,
            lat,
            lng,
            street,
            zip,
            city,
            country,
            state,
            email: email.map(Into::into),
            telephone,
            homepage: homepage.map(Url::into_string),
            tags,
            registration,
            organizer,
            image_url: image_url.map(Url::into_string),
            image_link_url: image_link_url.map(Url::into_string),
        }
    }
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "extra-derive", derive(Debug, Clone, PartialEq, Eq))]
pub struct PendingClearanceForPlace {
    pub place_id: String,
    pub created_at: i64,
    pub last_cleared_revision: Option<e::revision::RevisionValue>,
}

impl From<e::clearance::PendingClearanceForPlace> for PendingClearanceForPlace {
    fn from(from: e::clearance::PendingClearanceForPlace) -> Self {
        let e::clearance::PendingClearanceForPlace {
            place_id,
            created_at,
            last_cleared_revision,
        } = from;
        Self {
            place_id: place_id.into(),
            created_at: created_at.into_inner(),
            last_cleared_revision: last_cleared_revision.map(Into::into),
        }
    }
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "extra-derive", derive(Debug, Clone, PartialEq, Eq))]
pub struct ClearanceForPlace {
    pub place_id: String,
    pub cleared_revision: Option<e::revision::RevisionValue>,
}

impl From<ClearanceForPlace> for e::clearance::ClearanceForPlace {
    fn from(from: ClearanceForPlace) -> Self {
        let ClearanceForPlace {
            place_id,
            cleared_revision,
        } = from;
        Self {
            place_id: place_id.into(),
            cleared_revision: cleared_revision.map(Into::into),
        }
    }
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "extra-derive", derive(Debug, Clone, PartialEq, Eq))]
pub struct ResultCount {
    pub count: u64,
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "extra-derive", derive(Debug, PartialEq))]
pub struct LatLonDegrees(f64, f64);

impl From<e::geo::MapPoint> for LatLonDegrees {
    fn from(from: e::geo::MapPoint) -> Self {
        Self(from.lat().to_deg(), from.lng().to_deg())
    }
}

impl TryFrom<LatLonDegrees> for e::geo::MapPoint {
    type Error = e::geo::CoordRangeError;

    fn try_from(from: LatLonDegrees) -> Result<Self, Self::Error> {
        e::geo::MapPoint::try_from_lat_lng_deg(from.0, from.1)
    }
}

#[derive(Serialize, Deserialize, Default)]
#[cfg_attr(feature = "extra-derive", derive(Debug, PartialEq, Eq))]
pub struct Address {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub street: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub zip: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

impl Address {
    pub fn is_empty(&self) -> bool {
        self.street.is_none()
            && self.zip.is_none()
            && self.city.is_none()
            && self.country.is_none()
            && self.state.is_none()
    }
}

impl From<e::address::Address> for Address {
    fn from(from: e::address::Address) -> Self {
        let e::address::Address {
            street,
            zip,
            city,
            country,
            state,
        } = from;
        Self {
            street,
            zip,
            city,
            country,
            state,
        }
    }
}

impl From<Address> for e::address::Address {
    fn from(from: Address) -> Self {
        let Address {
            street,
            zip,
            city,
            country,
            state,
        } = from;
        Self {
            street,
            zip,
            city,
            country,
            state,
        }
    }
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "extra-derive", derive(Debug, PartialEq))]
pub struct Location {
    #[serde(rename = "deg")]
    pub latlon: LatLonDegrees,

    #[serde(
        rename = "adr",
        skip_serializing_if = "Address::is_empty",
        default = "Default::default"
    )]
    pub address: Address,
}

impl From<e::location::Location> for Location {
    fn from(from: e::location::Location) -> Self {
        let e::location::Location { pos, address } = from;
        Self {
            latlon: pos.into(),
            address: address.map(Into::into).unwrap_or_default(),
        }
    }
}

// TODO: use TryFrom here
impl From<Location> for e::location::Location {
    fn from(from: Location) -> Self {
        let Location { latlon, address } = from;
        Self {
            pos: latlon.try_into().unwrap(),
            address: Some(address.into()),
        }
    }
}

#[derive(Serialize, Deserialize, Default)]
#[cfg_attr(feature = "extra-derive", derive(Debug, PartialEq, Eq))]
pub struct Contact {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
}

impl Contact {
    pub fn is_empty(&self) -> bool {
        self.email.is_none() && self.phone.is_none()
    }
}

impl From<e::contact::Contact> for Contact {
    fn from(from: e::contact::Contact) -> Self {
        let e::contact::Contact { name, email, phone } = from;
        Self {
            name,
            email: email.map(Into::into),
            phone,
        }
    }
}

impl From<Contact> for e::contact::Contact {
    fn from(from: Contact) -> Self {
        let Contact { name, email, phone } = from;
        Self {
            name,
            email: email.map(Into::into),
            phone,
        }
    }
}

#[derive(Serialize, Deserialize, Default)]
#[cfg_attr(feature = "extra-derive", derive(Debug, PartialEq, Eq))]
pub struct Links {
    #[serde(rename = "www", skip_serializing_if = "Option::is_none")]
    pub homepage: Option<Url>,

    #[serde(rename = "img", skip_serializing_if = "Option::is_none")]
    pub image: Option<Url>,

    #[serde(rename = "img_href", skip_serializing_if = "Option::is_none")]
    pub image_href: Option<Url>,

    #[serde(
        rename = "custom",
        skip_serializing_if = "Vec::is_empty",
        default = "Default::default"
    )]
    pub custom: Vec<CustomLink>,
}

impl Links {
    pub fn is_empty(&self) -> bool {
        let Self {
            homepage,
            image,
            image_href,
            custom,
        } = self;
        homepage.is_none() && image.is_none() && image_href.is_none() && custom.is_empty()
    }
}

impl From<e::links::Links> for Links {
    fn from(from: e::links::Links) -> Self {
        let e::links::Links {
            homepage,
            image,
            image_href,
            custom,
        } = from;
        Self {
            homepage,
            image,
            image_href,
            custom: custom.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<Links> for e::links::Links {
    fn from(from: Links) -> Self {
        let Links {
            homepage,
            image,
            image_href,
            custom,
        } = from;
        Self {
            homepage,
            image,
            image_href,
            custom: custom.into_iter().map(Into::into).collect(),
        }
    }
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "extra-derive", derive(Debug, PartialEq, Eq))]
pub struct Activity {
    pub at: i64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub by: Option<String>,
}

impl From<e::activity::Activity> for Activity {
    fn from(from: e::activity::Activity) -> Self {
        let e::activity::Activity { at, by } = from;
        Self {
            at: at.into_inner(),
            by: by.map(Into::into),
        }
    }
}

impl From<Activity> for e::activity::Activity {
    fn from(from: Activity) -> Self {
        let Activity { at, by } = from;
        Self {
            at: e::time::TimestampMs::from_inner(at),
            by: by.map(Into::into),
        }
    }
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "extra-derive", derive(Debug))]
pub struct PlaceRoot {
    pub id: String,

    #[serde(rename = "lic")]
    pub license: String,
}

impl From<e::place::PlaceRoot> for PlaceRoot {
    fn from(from: e::place::PlaceRoot) -> Self {
        let e::place::PlaceRoot { id, license } = from;
        Self {
            id: id.into(),
            license,
        }
    }
}

impl From<PlaceRoot> for e::place::PlaceRoot {
    fn from(from: PlaceRoot) -> Self {
        let PlaceRoot { id, license } = from;
        Self {
            id: id.into(),
            license,
        }
    }
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "extra-derive", derive(Debug))]
pub struct PlaceRevision {
    #[serde(rename = "rev")]
    pub revision: u64,

    pub created: Activity,

    #[serde(rename = "tit")]
    pub title: String,

    #[serde(rename = "dsc")]
    pub description: String,

    #[serde(rename = "loc")]
    pub location: Location,

    #[serde(
        rename = "cnt",
        skip_serializing_if = "Contact::is_empty",
        default = "Default::default"
    )]
    pub contact: Contact,

    #[serde(rename = "hrs", skip_serializing_if = "Option::is_none")]
    pub opening_hours: Option<String>,

    #[serde(rename = "fnd", skip_serializing_if = "Option::is_none")]
    pub founded_on: Option<NaiveDate>,

    #[serde(
        rename = "lnk",
        skip_serializing_if = "Links::is_empty",
        default = "Default::default"
    )]
    pub links: Links,

    #[serde(
        rename = "tag",
        skip_serializing_if = "Vec::is_empty",
        default = "Default::default"
    )]
    pub tags: Vec<String>,
}

impl From<e::place::PlaceRevision> for PlaceRevision {
    fn from(from: e::place::PlaceRevision) -> Self {
        let e::place::PlaceRevision {
            revision,
            created,
            title,
            description,
            location,
            contact,
            opening_hours,
            founded_on,
            links,
            tags,
        } = from;
        Self {
            revision: revision.into(),
            created: created.into(),
            title,
            description,
            location: location.into(),
            contact: contact.map(Into::into).unwrap_or_default(),
            opening_hours: opening_hours.map(Into::into),
            founded_on: founded_on.map(Into::into),
            links: links.map(Into::into).unwrap_or_default(),
            tags,
        }
    }
}

impl From<PlaceRevision> for e::place::PlaceRevision {
    fn from(from: PlaceRevision) -> Self {
        let PlaceRevision {
            revision,
            created,
            title,
            description,
            location,
            contact,
            opening_hours,
            founded_on,
            links,
            tags,
        } = from;
        Self {
            revision: revision.into(),
            created: created.into(),
            title,
            description,
            location: location.into(),
            contact: Some(contact.into()),
            opening_hours: opening_hours.map(Into::into),
            founded_on: founded_on.map(Into::into),
            links: Some(links.into()),
            tags,
        }
    }
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "extra-derive", derive(Debug))]
pub struct PlaceHistory {
    pub place: PlaceRoot,
    pub revisions: Vec<(PlaceRevision, Vec<ReviewStatusLog>)>,
}

impl From<e::place::PlaceHistory> for PlaceHistory {
    fn from(from: e::place::PlaceHistory) -> Self {
        let e::place::PlaceHistory { place, revisions } = from;
        Self {
            place: place.into(),
            revisions: revisions
                .into_iter()
                .map(|(place_revision, reviews)| {
                    (
                        place_revision.into(),
                        reviews.into_iter().map(Into::into).collect(),
                    )
                })
                .collect(),
        }
    }
}

impl From<PlaceHistory> for e::place::PlaceHistory {
    fn from(from: PlaceHistory) -> Self {
        let PlaceHistory { place, revisions } = from;
        Self {
            place: place.into(),
            revisions: revisions
                .into_iter()
                .map(|(place_revision, reviews)| {
                    (
                        place_revision.into(),
                        reviews.into_iter().map(Into::into).collect(),
                    )
                })
                .collect(),
        }
    }
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "extra-derive", derive(Debug))]
pub struct ActivityLog {
    pub at: i64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub by: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ctx: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}

impl From<e::activity::ActivityLog> for ActivityLog {
    fn from(from: e::activity::ActivityLog) -> Self {
        let e::activity::ActivityLog {
            activity: e::activity::Activity { at, by },
            context: ctx,
            comment,
        } = from;
        Self {
            at: at.into_inner(),
            by: by.map(Into::into),
            ctx,
            comment,
        }
    }
}

impl From<ActivityLog> for e::activity::ActivityLog {
    fn from(from: ActivityLog) -> Self {
        let ActivityLog {
            at,
            by,
            ctx: context,
            comment,
        } = from;
        let at = e::time::TimestampMs::from_inner(at);
        let activity = e::activity::Activity {
            at,
            by: by.map(Into::into),
        };
        Self {
            activity,
            context,
            comment,
        }
    }
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "extra-derive", derive(Debug))]
pub struct ReviewStatusLog {
    pub rev: u64,
    pub act: ActivityLog,
    pub status: ReviewStatus,
}

impl From<e::review::ReviewStatusLog> for ReviewStatusLog {
    fn from(from: e::review::ReviewStatusLog) -> Self {
        let e::review::ReviewStatusLog {
            revision,
            activity,
            status,
        } = from;
        Self {
            rev: revision.into(),
            act: activity.into(),
            status: status.into(),
        }
    }
}

impl From<ReviewStatusLog> for e::review::ReviewStatusLog {
    fn from(from: ReviewStatusLog) -> Self {
        let ReviewStatusLog { rev, act, status } = from;
        Self {
            revision: rev.into(),
            activity: act.into(),
            status: status.into(),
        }
    }
}
