//! Provides some `Waypoint` conversion methods for the
//! [geo](https://crates.io/crates/geo) crate.

use crate::directions::Waypoint;
use crate::error::Error as GoogleMapsError;
use crate::types::error::Error as TypeError;
use crate::LatLng;
use geo_types::geometry::{Coord, Point};
use rust_decimal::{Decimal, prelude::FromPrimitive};

// -----------------------------------------------------------------------------

impl TryFrom<&Coord> for Waypoint {

    // Error definitions are contained in the `google_maps\src\error.rs` module.
    type Error = GoogleMapsError;

    /// Attempts to convert a `geo_types::geometry::Coord` struct to a
    /// `google_maps::directions::Waypoint` struct.
    fn try_from(coordinate: &Coord) -> Result<Self, Self::Error> {

        let lat: Decimal = Decimal::from_f64(coordinate.y)
            .ok_or_else(|| TypeError::FloatToDecimalConversionError(coordinate.y.to_string()))?;

        let lng: Decimal = Decimal::from_f64(coordinate.x)
            .ok_or_else(|| TypeError::FloatToDecimalConversionError(coordinate.x.to_string()))?;

        let lat_lng: LatLng = LatLng::try_from_dec(lat, lng)?;

        Ok(Waypoint::LatLng(lat_lng))

    } // fn

} // impl

// -----------------------------------------------------------------------------

impl TryFrom<&Point> for Waypoint {

    // Error definitions are contained in the `google_maps\src\error.rs` module.
    type Error = GoogleMapsError;

    /// Attempts to convert a `geo_types::geometry::Point` struct to a
    /// `google_maps::directions::Waypoint` struct.
    fn try_from(point: &Point) -> Result<Self, Self::Error> {

        let lat: Decimal = Decimal::from_f64(point.y())
            .ok_or_else(|| TypeError::FloatToDecimalConversionError(point.y().to_string()))?;

        let lng: Decimal = Decimal::from_f64(point.x())
            .ok_or_else(|| TypeError::FloatToDecimalConversionError(point.x().to_string()))?;

        let lat_lng: LatLng = LatLng::try_from_dec(lat, lng)?;

        Ok(Waypoint::LatLng(lat_lng))

    } // fn

} // impl