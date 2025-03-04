use crate::types::PlaceType;
use crate::places::place_search::text_search::request::Request;

// -----------------------------------------------------------------------------

impl<'a> Request<'a> {

    /// Adds the types parameter to the Places API _Text Search_ query.
    ///
    /// ## Arguments:
    ///
    /// * `type` ‧ Restricts the results to places matching the specified type.
    /// Only one type may be specified. If more than one type is provided, all
    /// types following the first entry are ignored.
    ///
    /// `type=hospital|pharmacy|doctor` becomes `type=hospital`
    /// `type=hospital,pharmacy,doctor` is ignored entirely
    ///
    /// See the list of [supported types](https://developers.google.com/maps/documentation/places/web-service/supported_types).
    ///
    /// Note: Adding both `keyword` and `type` with the same value
    /// (`keyword=cafe&type=cafe` or `keyword=parking&type=parking`) can yield
    /// `ZERO_RESULTS`.

    pub fn with_type(&'a mut self, place_type: PlaceType) -> &'a mut Request {
        // Set location in Request struct.
        self.place_type = Some(place_type);
        // Return modified Request struct to caller.
        self
    } // fn

} // impl