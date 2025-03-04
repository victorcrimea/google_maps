//! Places API error types and error messages.

// -----------------------------------------------------------------------------

use crate::places::status::Status;
use miette::Diagnostic;
use thiserror::Error;

// -----------------------------------------------------------------------------
//
/// Errors that may be produced by the Google Maps Places API client.

#[derive(Debug, Diagnostic, Error)]
#[diagnostic(code(google_maps::places::error), url(docsrs))]
pub enum Error {
    /// Google Maps Places API server generated an error. See the `Status`
    /// enum for more information.
    GoogleMapsService(Status, Option<String>),
    /// The HTTP request was unsuccessful.
    HttpUnsuccessful(String),
    /// API client library attempted to parse a string that contained an invalid
    /// status code. See
    /// `google_maps\src\places\place_autocomplete\response\status.rs` for more
    /// information.
    InvalidStatusCode(String),
    /// API client library attempted to parse a string that contained an invalid
    /// business status code. See
    /// `google_maps\src\places\business_status.rs` for more information.
    InvalidBusinessStatusCode(String),
    /// API client library attempted to parse a string that contained an invalid
    /// secondary hours type code. See
    /// `google_maps\src\places\secondary_hours_type.rs` for more information.
    InvalidSecondaryHoursType(String),
    /// API client library attempted to parse a string that contained an invalid
    /// field type code. See
    /// `google_maps\src\places\place_details\field.rs` for more information.
    InvalidFieldCode(String),
    /// API client library attempted to parse a string that contained an invalid
    /// rank by order type code. See
    /// `google_maps\src\places\place_details\rank_by.rs` for more information.
    InvalidRankByCode(String),
    /// API client library attempted to parse a string that contained an invalid
    /// sort order type code. See
    /// `google_maps\src\places\place_details\sort_order.rs` for more information.
    InvalidSortOrderCode(String),
    /// The query string must be built before the request may be sent to the
    /// Google Maps Places API server.
    QueryNotBuilt,
    /// The dependency library Reqwest generated an error.
    #[cfg(feature = "enable-reqwest")]
    Reqwest(reqwest::Error),
    /// The dependency library Reqwest generated an error. The error could
    /// not be passed normally so a `String` representation is passed instead.
    #[cfg(feature = "enable-reqwest")]
    ReqwestMessage(String),
    /// The dependency library Serde JSON generated an error.
    SerdeJson(serde_json::error::Error),
} // enum

// -----------------------------------------------------------------------------

impl std::fmt::Display for Error {
    /// This trait converts the error code into a format that may be presented
    /// to the user.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::GoogleMapsService(status, error_message) => match error_message {
                // If the Google Maps Places API server generated an error
                // message, return that:
                Some(error_message) => write!(f, "Google Maps Places API service: {error_message}"),
                // If the Google Maps Places API server did not generate an
                // error message, return a generic message derived from the
                // response status:
                None => match status {
                    Status::InvalidRequest => write!(f, "Google Maps Places API service: \
                        Invalid request. \
                        The request was malformed."),
                    Status::Ok => write!(f, "Google Maps Time Zone service: \
                        Ok. \
                        The request was successful."),
                    Status::OverQueryLimit => write!(f, "Google Maps Places API service: \
                        Over query limit. \
                        Requestor has exceeded quota."),
                    Status::RequestDenied => write!(f, "Google Maps Places API service: \
                        Request denied. \
                        Service did not complete the request."),
                    Status::UnknownError => write!(f, "Google Maps Places API service: \
                        Unknown error."),
                    Status::ZeroResults => write!(f, "Google Maps Places API service: \
                        Zero results. \
                        This may occur if the geocoder was passed a non-existent address."),
                    Status::NotFound => write!(f, "Google Maps Places API service: \
                        Not found. \
                        Referenced location, `place_id`, was not found \
                        in the Places database."),
                } // match
            }, // match
            Error::HttpUnsuccessful(status) => write!(f,
                "Google Maps Places API client: \
                Could not successfully query the Google Cloud Platform service. \
                The service last responded with a `{status}` status."),
            Error::InvalidBusinessStatusCode(status_code) => write!(f, "Google Maps Places API client: \
                `{status_code}` is not a valid business status code. \
                Valid codes are `OPERATIONAL`, `CLOSED_TEMPORARILY`, and \
                `CLOSED_PERMANENTLY`."),
            Error::InvalidRankByCode(type_code) => write!(f, "Google Maps Places API client: \
                `{type_code}` is not a valid rankby order type. \
                Valid codes are `prominence`, and `distance`."),
            Error::InvalidSecondaryHoursType(type_code) => write!(f, "Google Maps Places API client: \
                `{type_code}` is not a valid secondary hours type. \
                Valid codes are `DRIVE_THROUGH`, `HAPPY_HOUR`, `DELIVERY`, \
                `TAKEOUT`, `KITCHEN`, `BREAKFAST`, `LUNCH`, `DINNER`, \
                `BRUNCH`, `PICKUP`, and `SENIOR_HOURS`."),
            Error::InvalidStatusCode(status_code) => write!(f, "Google Maps Places API client: \
                `{status_code}` is not a valid status code. \
                Valid codes are `INVALID_REQUEST`, `OK`, `OVER_DAILY_LIMIT`, \
                `OVER_QUERY_LIMIT`, `REQUEST_DENIED`, `UNKNOWN_ERROR`, \
                `ZERO_RESULTS`, and `NOT_FOUND`."),
            Error::InvalidFieldCode(type_code) => write!(f, "Google Maps Places API client: \
                `{type_code}` is not a valid field type. \
                See `https://developers.google.com/maps/documentation/places/web-service/details#fields` \
                for a list of valid fields."),
            Error::InvalidSortOrderCode(sort_order_code) => write!(f, "Google Maps Places API client: \
                `{sort_order_code}` is not a valid sort order code. \
                Valid codes are `most_relevant` and `newest`."),
            Error::QueryNotBuilt => write!(f, "Google Maps Places API client library: \
                The query string must be built before the request may be sent to the Google Cloud Maps Platform. \
                Ensure the build() method is called before run()."),
            #[cfg(feature = "enable-reqwest")]
            Error::Reqwest(error) => write!(f, "Google Maps Places API client in the Reqwest library: {error}"),
            #[cfg(feature = "enable-reqwest")]
            Error::ReqwestMessage(error) => write!(f, "Google Maps Geocoding API client in the Reqwest library: {error}"),
            Error::SerdeJson(error) => write!(f, "Google Maps Places API client in the Serde JSON library: {error}"),
        } // match
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl From<serde_json::error::Error> for Error {
    /// This trait converts from an Serde JSON (`serde_json::error::Error`)
    /// error type into a Google Maps Places API error type
    /// (`google_maps::place::error::Error`) by wrapping it inside. This
    /// function is required to use the `?` operator.
    fn from(error: serde_json::error::Error) -> Error {
        Error::SerdeJson(error)
    } // fn
} // impl