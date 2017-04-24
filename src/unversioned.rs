//! Endpoints that cannot change with new versions of the Matrix specification.

/// [GET /_matrix/client/versions](https://matrix.org/docs/spec/client_server/r0.2.0.html#get-matrix-client-versions)
pub mod get_supported_versions {
    /// Details about this API endpoint.
    #[derive(Clone, Copy, Debug)]
    pub struct Endpoint;

    /// This API endpoint's response.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Response {
        /// A list of Matrix client API protocol versions supported by the homeserver.
        pub versions: Vec<String>,
    }

    impl<'de> ::Endpoint<'de> for Endpoint {
        type BodyParams = ();
        type PathParams = ();
        type QueryParams = ();
        type Response = Response;

        fn method() -> ::Method {
            ::Method::Get
        }

        fn request_path(_params: Self::PathParams) -> String {
            Self::router_path().to_string()
        }

        fn router_path() -> &'static str {
            "/_matrix/client/versions"
        }

        fn name() -> &'static str {
            "api_version"
        }

        fn description() -> &'static str {
            "Get the versions of the client-server API supported by this homeserver."
        }

        fn requires_authentication() -> bool {
            false
        }

        fn rate_limited() -> bool {
            false
        }
    }
}
