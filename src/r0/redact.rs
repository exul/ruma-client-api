//! Endpoints for event redaction.

/// [PUT /_matrix/client/r0/rooms/{roomId}/redact/{eventId}/{txnId}](https://matrix.org/docs/spec/client_server/r0.2.0.html#put-matrix-client-r0-rooms-roomid-redact-eventid-txnid)
pub mod redact_event {
    use ruma_identifiers::{RoomId, EventId};

    /// Details about this API endpoint.
    #[derive(Clone, Copy, Debug)]
    pub struct Endpoint;

    /// This API endpoint's path parameters.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct PathParams {
        pub room_id: RoomId,
        pub event_id: EventId,
        pub txn_id: String
    }

    /// This API endpoint's path parameters.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct BodyParams {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub reason: Option<String>
    }

    /// This API endpoint's response.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Response {
        pub event_id: EventId,
    }


    impl<'de> ::Endpoint<'de> for Endpoint {
        type BodyParams = BodyParams;
        type PathParams = PathParams;
        type QueryParams = ();
        type Response = Response;

        fn method() -> ::Method {
            ::Method::Put
        }

        fn request_path(params: Self::PathParams) -> String {
            format!(
                "/_matrix/client/r0/rooms/{}/redact/{}/{}",
                params.room_id,
                params.event_id,
                params.txn_id
            )
        }

        fn router_path() -> &'static str {
            "/_matrix/client/r0/rooms/:room_id/redact/:event_id/:txn_id"
        }

        fn name() -> &'static str {
            "redact_event"
        }

        fn description() -> &'static str {
            "Redact an event, stripping all information not critical to the event graph integrity."
        }

        fn requires_authentication() -> bool {
            true
        }

        fn rate_limited() -> bool {
            false
        }
    }
}
