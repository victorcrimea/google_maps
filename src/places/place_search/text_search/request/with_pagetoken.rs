use crate::places::place_search::text_search::request::Request;

// -----------------------------------------------------------------------------

impl<'a> Request<'a> {

    /// Adds the page token parameter to the Places API _Text Search_ query.
    ///
    /// ## Arguments:
    ///
    /// * `pagetoken` ‧ Returns up to 20 results from a previously run search.
    /// Setting a `pagetoken` parameter will execute a search with the same
    /// parameters used previously — all parameters other than pagetoken will be
    /// ignored.

    pub fn with_pagetoken(&'a mut self, pagetoken: String) -> &'a mut Request {
        // Set page token in Request struct.
        self.pagetoken = Some(pagetoken);
        // Return modified Request struct to caller.
        self
    } // fn

} // impl