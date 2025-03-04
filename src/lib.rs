//! # google_maps
//!
//! 🗺 An unofficial Google Maps Platform client library for the Rust
//! programming language. This client currently implements the Directions API,
//! Distance Matrix API, Elevation API, Geocoding API, Time Zone API, parts of
//! the Places API, and parts of the Roads API.
//!
//! ![alt text](https://www.arkiteq.ca/crates/google_maps/banner.jpg "Unofficial Google Maps Platform Client for Rust")
//!
//! # Welcome
//!
//! This crate is expected to work well and have the more important Google Maps
//! features implemented. It should work well because
//! [serde](https://crates.io/crates/serde) and, by default,
//! [reqwest](https://crates.io/crates/reqwest) do most of the heavy lifting!
//!
//! I created this client library because I needed several Google Maps Platform
//! features for a project that I'm working on. So, I've decided to spin my
//! library off into a public crate. This is a very small token of gratitude and
//! an attempt to give back to the Rust community. I hope it saves someone out
//! there some work.
//!
//! # Before You Begin
//!
//! * In your project's `Cargo.toml` file, under the `[dependencies]` section:
//!
//!     * Add `google_maps = "3.3"`. Check
//!         [crates.io](https://crates.io/crates/google_maps) for the latest
//!         version number.
//!
//!     * Optionally, add `rust_decimal = "1"` and `rust_decimal_macros = "1"`
//!         for access to the `dec!` macro. This macro can be used to define
//!         decimal numbers in your program. This is useful for efficiently
//!         hard-coding latitudes and longitudes into your code for development
//!         and testing.
//!
//! * The full documentation is available at [docs.rs](https://docs.rs/google_maps/)
//!
//! # What's new?
//!
//! * The full [change
//! log](https://github.com/leontoeides/google_maps/blob/master/CHANGELOG.md) is
//! available on GitHub.
//!
//! ## Example Directions API Request
//!
//! The Directions API is a service that calculates directions between
//! locations. You can search for directions for several modes of
//! transportation, including transit, driving, walking, or cycling.
//!
//! ```rust
//! use google_maps::prelude::*;
//!
//! let google_maps_client = GoogleMapsClient::new("YOUR_GOOGLE_API_KEY_HERE");
//!
//! // Example request:
//!
//! let directions = google_maps_client.directions(
//!     // Origin: Canadian Museum of Nature
//!     Location::Address(String::from("240 McLeod St, Ottawa, ON K2P 2R1")),
//!     // Destination: Canada Science and Technology Museum
//!     Location::LatLng(LatLng::try_from_f64(45.403_509, -75.618_904)?),
//! )
//! .with_travel_mode(TravelMode::Driving)
//! .execute()
//! .await?;
//!
//! // Dump entire response:
//!
//! println!("{:#?}", directions);
//! ```
//!
//! ## Example Distance Matrix API Request
//!
//! The Distance Matrix API is a service that provides travel distance and time
//! for a matrix of origins and destinations, based on the recommended route
//! between start and end points.
//!
//! ```rust
//! use google_maps::prelude::*;
//!
//! let google_maps_client = GoogleMapsClient::new("YOUR_GOOGLE_API_KEY_HERE");
//!
//! // Example request:
//!
//! let distance_matrix = google_maps_client.distance_matrix(
//!     // Origins
//!     vec![
//!         // Microsoft
//!         Waypoint::Address(String::from("One Microsoft Way, Redmond, WA 98052, United States")),
//!         // Cloudflare
//!         Waypoint::Address(String::from("101 Townsend St, San Francisco, CA 94107, United States")),
//!     ],
//!     // Destinations
//!     vec![
//!         // Google
//!         Waypoint::PlaceId(String::from("ChIJj61dQgK6j4AR4GeTYWZsKWw")),
//!         // Mozilla
//!         Waypoint::LatLng(LatLng::try_from_dec(dec!(37.387_316), dec!(-122.060_008))?),
//!     ],
//! ).execute().await?;
//!
//! // Dump entire response:
//!
//! println!("{:#?}", distance_matrix);
//! ```
//!
//! ## Example Elevation API Positional Request
//!
//! The Elevation API provides elevation data for all locations on the surface
//! of the earth, including depth locations on the ocean floor (which return
//! negative values).
//!
//! ```rust
//! use google_maps::prelude::*;
//!
//! let google_maps_client = GoogleMapsClient::new("YOUR_GOOGLE_API_KEY_HERE");
//!
//! // Example request:
//!
//! let elevation = google_maps_client.elevation()
//!     // Denver, Colorado, the "Mile High City"
//!     .for_positional_request(LatLng::try_from_dec(dec!(39.739_154), dec!(-104.984_703))?)
//!     .execute()
//!     .await?;
//!
//! // Dump entire response:
//!
//! println!("{:#?}", elevation);
//!
//! // Display all results:
//!
//! if let Some(results) = &elevation.results {
//!     for result in results {
//!         println!("Elevation: {} meters", result.elevation)
//!     }
//! }
//! ```
//!
//! ## Example Geocoding API Request
//!
//! The Geocoding API is a service that provides geocoding and reverse geocoding
//! of addresses. Geocoding is the process of converting addresses (like a
//! street address) into geographic coordinates (like latitude and longitude),
//! which you can use to place markers on a map, or position the map.
//!
//! ```rust
//! use google_maps::prelude::*;
//!
//! let google_maps_client = GoogleMapsClient::new("YOUR_GOOGLE_API_KEY_HERE");
//!
//! // Example request:
//!
//! let location = google_maps_client.geocoding()
//!     .with_address("10 Downing Street London")
//!     .execute()
//!     .await?;
//!
//! // Dump entire response:
//!
//! println!("{:#?}", location);
//!
//! // Print latitude & longitude coordinates:
//!
//! for result in location.results {
//!     println!("{}", result.geometry.location)
//! }
//! ```
//!
//! ## Example Reverse Geocoding API Request
//!
//! The Geocoding API is a service that provides geocoding and reverse geocoding
//! of addresses. Reverse geocoding is the process of converting geographic
//! coordinates into a human-readable address.
//!
//! ```rust
//! use google_maps::prelude::*;
//!
//! let google_maps_client = GoogleMapsClient::new("YOUR_GOOGLE_API_KEY_HERE");
//!
//! // Example request:
//!
//! let location = google_maps_client.reverse_geocoding(
//!     // 10 Downing St, Westminster, London
//!     LatLng::try_from_dec(dec!(51.503_364), dec!(-0.127_625))?,
//! )
//! .with_result_type(PlaceType::StreetAddress)
//! .execute()
//! .await?;
//!
//! // Dump entire response:
//!
//! println!("{:#?}", location);
//!
//! // Display all results:
//!
//! for result in location.results {
//!     println!(
//!         "{}",
//!         result.address_components.iter()
//!             .map(|address_component| address_component.short_name.to_string())
//!             .collect::<Vec<String>>()
//!             .join(", ")
//!     );
//! }
//! ```
//!
//! ## Example Time Zone API Request
//!
//! The Time Zone API provides time offset data for locations on the surface of
//! the earth. You request the time zone information for a specific
//! latitude/longitude pair and date. The API returns the name of that time
//! zone, the time offset from UTC, and the daylight savings offset.
//!
//! ```rust
//! use google_maps::prelude::*;
//!
//! let google_maps_client = GoogleMapsClient::new("YOUR_GOOGLE_API_KEY_HERE");
//!
//! // Example request:
//!
//! let time_zone = google_maps_client.time_zone(
//!      // St. Vitus Cathedral in Prague, Czechia
//!      LatLng::try_from_dec(dec!(50.090_903), dec!(14.400_512))?,
//!      // The time right now in UTC (Coordinated Universal Time)
//!      Utc::now()
//! ).execute().await?;
//!
//! // Dump entire response:
//!
//! println!("{:#?}", time_zone);
//!
//! // Usage example:
//!
//! println!("Time at your computer: {}", Utc::now().to_rfc2822());
//!
//! if let Some(time_zone_id) = time_zone.time_zone_id {
//!     println!(
//!         "Time in {}: {}",
//!         time_zone_id.name(),
//!         Local::now().with_timezone(&time_zone_id).to_rfc2822()
//!     );
//! }
//! ```
//!
//! ## [Geolocation API](https://developers.google.com/maps/documentation/geolocation/intro)
//!
//! Google's Geolocation API seems to be offline. While the online documentation
//! is still available and the API appears configurable through the Google Cloud
//! Platform console, the Geolocation API responds Status code `404 Not Found`
//! with an empty body to all requests. This API cannot be implemented until the
//! server responds as expected.
//!
//! ## Example Client Settings
//!
//! The Google Maps client settings can be used to change the request rate and
//! automatic retry parameters.
//!
//! ```rust
//! use google_maps::prelude::*;
//!
//! let google_maps_client = GoogleMapsClient::new("YOUR_GOOGLE_API_KEY_HERE")
//!     // For all Google Maps Platform APIs, the client will limit 2 sucessful
//!     // requests for every 10 seconds:
//!     .with_rate(Api::All, 2, std::time::Duration::from_secs(10))
//!     // Returns the `GoogleMapsClient` struct to the caller. This struct is
//!     // used to make Google Maps Platform requests.
//!     .build();
//! ```
//!
//! ## Feature Flags
//!
//! It is possible to change the Reqwest features that are in turn used by the
//! Google Maps API client through feature flags. It is also possible to only
//! include desired Google Maps APIs by using Cargo.toml feature flags.
//!
//! #### Google Maps API Client feature flags:
//!
//! * autocomplete
//! * directions
//! * distance_matrix
//! * elevation
//! * geocoding
//! * places
//! * roads
//! * time_zone
//! * enable-reqwest (uses [reqwest](https://crates.io/crates/reqwest) for
//! querying Google Maps API).
//! * geo (support for [geo](https://crates.io/crates/geo-types) crate types)
//!
//! Note: The Places autocomplete APIs have been put in the `autocomplete`
//! feature flag. The rest of the Places APIs will be put under the `places`
//! feature flag.
//!
//! #### Reqwest feature flags:
//!
//! * native-tls
//! * rustls
//! * gzip
//! * brotli
//!
//! **Feature flag usage example**: This example will only include the Google
//! Maps Directions API. Reqwest will secure the connection using the Rustls
//! library, and has brotli compression enabled.
//!
//! ```toml
//! google_maps = {
//!     version = "3.0",
//!     default-features = false,
//!     features = [
//!         "directions",
//!         "enable-reqwest",
//!         "rustls",
//!         "brotli"
//!     ]
//! }
//! ```
//!
//! **Default feature flag configuration**: By default, the Google Maps client
//! includes all implemented Google Maps APIs. Reqwest will secure the
//! connection using the system-native TLS (`native-tls`), and has gzip
//! compression enabled (`gzip`).
//!
//! ```toml
//! default = [
//!     "autocomplete",
//!     "directions",
//!     "distance_matrix",
//!     "elevation",
//!     "geocoding",
//!     "places",
//!     "roads",
//!     "time_zone",
//!     "enable-reqwest",
//!     "reqwest/default-tls",
//!     "reqwest/gzip",
//! ]
//! ```
//!
//! # Feedback
//!
//! I would like for you to be successful with your project! If this crate is
//! not working for you, doesn't work how you think it should, or if you have
//! requests, or suggestions - please [report them to
//! me](https://github.com/leontoeides/google_maps/issues)! I'm not always fast
//! at responding but I will respond. Thanks!
//!
//! # To do
//!
//! 1. Track both _requests_ and request _elements_ for rate limiting.
//! 2. Make a generic get() function for that can be used by all APIs.
//! 3. Convert explicit query validation to session types wherever reasonable.
//! 4. [Places API](https://developers.google.com/places/web-service/intro).
//! Only partly implemented. If you would like to have any missing pieces
//! implemented, please contact me.
//! 5. [Roads API](https://developers.google.com/maps/documentation/roads/intro).
//! Only partly implemented. If you would like to have any missing pieces
//! implemented, please contact me.

#![forbid(unsafe_code)]

#![doc(html_favicon_url = "https://www.arkiteq.ca/crates/google_maps/icon.png")]
#![doc(html_logo_url = "https://www.arkiteq.ca/crates/google_maps/logo.png")]

// Common / global modules:

mod client;
mod serde;
pub mod error;
pub mod prelude;
pub mod types;

// Optional Google Maps API modules. Their inclusion can be changed with
// feature flags:

#[cfg(any(feature = "directions", feature = "distance_matrix"))]
pub mod directions;
#[cfg(feature = "distance_matrix")]
pub mod distance_matrix;
#[cfg(feature = "elevation")]
pub mod elevation;
#[cfg(feature = "geocoding")]
pub mod geocoding;
#[cfg(feature = "time_zone")]
pub mod time_zone;
pub mod places;
#[cfg(feature = "roads")]
pub mod roads;

// Re-exports. Not great for organization but needed for backward compatibility.

pub use crate::{
    client::GoogleMapsClient as ClientSettings,
    client::GoogleMapsClient,
    error::Error as GoogleMapsError,
    error::Error,
    types::error::Error as TypeError,
}; // crate

#[cfg(any(feature = "geocoding", feature = "places", feature = "geocoding"))]
pub use crate::types::address_component::AddressComponent;
#[cfg(any(feature = "directions", feature = "distance_matrix", feature = "geocoding", feature = "places"))]
pub use crate::types::bounds::Bounds;
#[cfg(any(feature = "autocomplete", feature = "directions", feature = "geocoding"))]
pub use crate::types::country::Country;
#[cfg(any(feature = "geocoding", feature = "places", feature = "geocoding"))]
pub use crate::types::geometry::Geometry;
#[cfg(any(feature = "autocomplete", feature = "directions", feature = "distance_matrix", feature = "geocoding", feature = "places", feature = "time_zone"))]
pub use crate::types::language::Language;
#[cfg(any(feature = "autocomplete", feature = "directions", feature = "distance_matrix", feature = "elevation", feature = "geocoding", feature = "places", feature = "roads", feature = "time_zone"))]
pub use crate::types::latlng::LatLng;
#[cfg(any(feature = "geocoding", feature = "places", feature = "geocoding"))]
pub use crate::types::location_type::LocationType;
#[cfg(any(feature = "autocomplete", feature = "directions", feature = "distance_matrix", feature = "geocoding", feature = "places"))]
pub use crate::types::place_type::PlaceType;
#[cfg(any(feature = "autocomplete", feature = "directions", feature = "distance_matrix", feature = "geocoding", feature = "places"))]
pub use crate::types::region::Region;

// Optional dependencies:

#[cfg(feature = "enable-reqwest")]
mod request_rate;

#[cfg(feature = "enable-reqwest")]
pub use crate::request_rate::api::Api;