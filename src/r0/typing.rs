//! Endpoints for typing notifications.

/// [PUT /_matrix/client/r0/rooms/{roomId}/typing/{userId}](https://matrix.org/docs/spec/client_server/r0.2.0.html#put-matrix-client-r0-rooms-roomid-typing-userid)
pub mod create_typing_event {
    use ruma_identifiers::{UserId, RoomId};

    /// Details about this API endpoint.
    #[derive(Clone, Copy, Debug)]
    pub struct Endpoint;

    /// This API endpoint's path parameters.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct PathParams {
        pub room_id: RoomId,
        pub user_id: UserId
    }

    /// This API endpoint's body parameters.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct BodyParams {
        pub typing: bool,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub timeout: Option<u64>
    }

    impl<'de> ::Endpoint<'de> for Endpoint {
        type BodyParams = BodyParams;
        type PathParams = PathParams;
        type QueryParams = ();
        type Response = ();

        fn method() -> ::Method {
            ::Method::Put
        }

        fn request_path(params: Self::PathParams) -> String {
            format!(
                "/_matrix/client/r0/rooms/{}/typing/{}",
                params.room_id,
                params.user_id
            )
        }

        fn router_path() -> &'static str {
            "/_matrix/client/r0/rooms/:room_id/typing/:user_id"
        }

        fn name() -> &'static str {
            "create_typing_event"
        }

        fn description() -> &'static str {
            "Send a typing event to a room."
        }

        fn requires_authentication() -> bool {
            true
        }

        fn rate_limited() -> bool {
            true
        }
    }
}
