use std::convert::Into;
use std::fmt::Error as FormatterError;
use std::fmt::{Debug, Formatter};
use std::ops::Deref;

use rand::{thread_rng, Rng};
use sha2::{Digest, Sha256};
use url::Url;

use crate::helpers;

macro_rules! new_type {
    // Convenience pattern without an impl.
    (
        $(#[$attr:meta])*
        $name:ident(
            $(#[$type_attr:meta])*
            $type:ty
        )
    ) => {
        new_type![
            @new_type $(#[$attr])*,
            $name(
                $(#[$type_attr])*
                $type
            ),
            concat!(
                "Create a new `",
                stringify!($name),
                "` to wrap the given `",
                stringify!($type),
                "`."
            ),
            impl {}
        ];
    };
    // Main entry point with an impl.
    (
        $(#[$attr:meta])*
        $name:ident(
            $(#[$type_attr:meta])*
            $type:ty
        )
        impl {
            $($item:tt)*
        }
    ) => {
        new_type![
            @new_type $(#[$attr])*,
            $name(
                $(#[$type_attr])*
                $type
            ),
            concat!(
                "Create a new `",
                stringify!($name),
                "` to wrap the given `",
                stringify!($type),
                "`."
            ),
            impl {
                $($item)*
            }
        ];
    };
    // Actual implementation, after stringifying the #[doc] attr.
    (
        @new_type $(#[$attr:meta])*,
        $name:ident(
            $(#[$type_attr:meta])*
            $type:ty
        ),
        $new_doc:expr,
        impl {
            $($item:tt)*
        }
    ) => {
        $(#[$attr])*
        #[derive(Clone, Debug, PartialEq)]
        pub struct $name(
            $(#[$type_attr])*
            $type
        );
        impl $name {
            $($item)*

            #[doc = $new_doc]
            pub fn new(s: $type) -> Self {
                $name(s)
            }
        }
        impl Deref for $name {
            type Target = $type;
            fn deref(&self) -> &$type {
                &self.0
            }
        }
        impl Into<$type> for $name {
            fn into(self) -> $type {
                self.0
            }
        }
    }
}

macro_rules! new_secret_type {
    (
        $(#[$attr:meta])*
        $name:ident($type:ty)
    ) => {
        new_secret_type![
            $(#[$attr])*
            $name($type)
            impl {}
        ];
    };
    (
        $(#[$attr:meta])*
        $name:ident($type:ty)
        impl {
            $($item:tt)*
        }
    ) => {
        new_secret_type![
            $(#[$attr])*,
            $name($type),
            concat!(
                "Create a new `",
                stringify!($name),
                "` to wrap the given `",
                stringify!($type),
                "`."
            ),
            concat!("Get the secret contained within this `", stringify!($name), "`."),
            impl {
                $($item)*
            }
        ];
    };
    (
        $(#[$attr:meta])*,
        $name:ident($type:ty),
        $new_doc:expr,
        $secret_doc:expr,
        impl {
            $($item:tt)*
        }
    ) => {
        $(
            #[$attr]
        )*
        #[derive(Clone, PartialEq)]
        pub struct $name($type);
        impl $name {
            $($item)*

            #[doc = $new_doc]
            pub fn new(s: $type) -> Self {
                $name(s)
            }
            ///
            #[doc = $secret_doc]
            ///
            /// # Security Warning
            ///
            /// Leaking this value may compromise the security of the OAuth2 flow.
            ///
            pub fn secret(&self) -> &$type { &self.0 }
        }
        impl Debug for $name {
            fn fmt(&self, f: &mut Formatter) -> Result<(), FormatterError> {
                write!(f, concat!(stringify!($name), "([redacted])"))
            }
        }
    };
}

new_type![
    ///
    /// Client identifier issued to the client during the registration process described by
    /// [Section 2.2](https://tools.ietf.org/html/rfc6749#section-2.2).
    ///
    #[derive(Deserialize, Serialize)]
    ClientId(String)
];

new_type![
    #[derive(Deserialize, Serialize)]
    ///
    /// URL of the authorization server's authorization endpoint.
    ///
    AuthUrl(
        #[serde(
            deserialize_with = "helpers::deserialize_url",
            serialize_with = "helpers::serialize_url"
        )]
        Url
    )
];
new_type![
    #[derive(Deserialize, Serialize)]
    ///
    /// URL of the authorization server's token endpoint.
    ///
    TokenUrl(
        #[serde(
            deserialize_with = "helpers::deserialize_url",
            serialize_with = "helpers::serialize_url"
        )]
        Url
    )
];
new_type![
    #[derive(Deserialize, Serialize)]
    ///
    /// URL of the client's redirection endpoint.
    ///
    RedirectUrl(
        #[serde(
            deserialize_with = "helpers::deserialize_url",
            serialize_with = "helpers::serialize_url"
        )]
        Url
    )
];
new_type![
    ///
    /// Authorization endpoint response (grant) type defined in
    /// [Section 3.1.1](https://tools.ietf.org/html/rfc6749#section-3.1.1).
    ///
    #[derive(Deserialize, Serialize)]
    ResponseType(String)
];
new_type![
    ///
    /// Resource owner's username used directly as an authorization grant to obtain an access
    /// token.
    ///
    ResourceOwnerUsername(String)
];

new_type![
    ///
    /// Access token scope, as defined by the authorization server.
    ///
    #[derive(Deserialize, Serialize)]
    Scope(String)
];
impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        self
    }
}
new_type![
    ///
    /// Code Challenge used for [PKCE]((https://tools.ietf.org/html/rfc7636)) protection via the
    /// `code_challenge` parameter.
    ///
    #[derive(Deserialize, Serialize)]
    PkceCodeChallengeS256(String)
];
new_type![
    ///
    /// Code Challenge Method used for [PKCE]((https://tools.ietf.org/html/rfc7636)) protection
    /// via the `code_challenge_method` parameter.
    ///
    #[derive(Deserialize, Serialize)]
    PkceCodeChallengeMethod(String)
];

new_secret_type![
    ///
    /// Client password issued to the client during the registration process described by
    /// [Section 2.2](https://tools.ietf.org/html/rfc6749#section-2.2).
    ///
    #[derive(Deserialize, Serialize)]
    ClientSecret(String)
];
new_secret_type![
    ///
    /// Value used for [CSRF]((https://tools.ietf.org/html/rfc6749#section-10.12)) protection
    /// via the `state` parameter.
    ///
    #[must_use]
    #[derive(Deserialize, Serialize)]
    CsrfToken(String)
    impl {
        ///
        /// Generate a new random, base64-encoded 128-bit CSRF token.
        ///
        pub fn new_random() -> Self {
            CsrfToken::new_random_len(16)
        }
        ///
        /// Generate a new random, base64-encoded CSRF token of the specified length.
        ///
        /// # Arguments
        ///
        /// * `num_bytes` - Number of random bytes to generate, prior to base64-encoding.
        ///
        pub fn new_random_len(num_bytes: u32) -> Self {
            let random_bytes: Vec<u8> = (0..num_bytes).map(|_| thread_rng().gen::<u8>()).collect();
            CsrfToken::new(base64::encode_config(&random_bytes, base64::URL_SAFE_NO_PAD))
        }
    }
];
new_secret_type![
    ///
    /// Code Verifier used for [PKCE]((https://tools.ietf.org/html/rfc7636)) protection via the
    /// `code_verifier` parameter. The value must have a minimum length of 43 characters and a
    /// maximum length of 128 characters.  Each character must be ASCII alphanumeric or one of
    /// the characters "-" / "." / "_" / "~".
    ///
    #[derive(Deserialize, Serialize)]
    PkceCodeVerifierS256(String)
    impl {
        ///
        /// Generate a new random, base64-encoded code verifier.
        ///
        pub fn new_random() -> Self {
            PkceCodeVerifierS256::new_random_len(32)
        }
        ///
        /// Generate a new random, base64-encoded code verifier.
        ///
        /// # Arguments
        ///
        /// * `num_bytes` - Number of random bytes to generate, prior to base64-encoding.
        ///   The value must be in the range 32 to 96 inclusive in order to generate a verifier
        ///   with a suitable length.
        ///
        pub fn new_random_len(num_bytes: u32) -> Self {
            // The RFC specifies that the code verifier must have "a minimum length of 43
            // characters and a maximum length of 128 characters".
            // This implies 32-96 octets of random data to be base64 encoded.
            assert!(num_bytes >= 32 && num_bytes <= 96);
            let random_bytes: Vec<u8> = (0..num_bytes).map(|_| thread_rng().gen::<u8>()).collect();
            let code = base64::encode_config(&random_bytes, base64::URL_SAFE_NO_PAD);
            assert!(code.len() >=43 && code.len() <= 128);
            PkceCodeVerifierS256::new(code)
        }
        ///
        /// Return the code challenge for the code verifier.
        ///
        pub fn code_challenge(&self) -> PkceCodeChallengeS256 {
            let digest = Sha256::digest(self.secret().as_bytes());
            PkceCodeChallengeS256::new(base64::encode_config(&digest, base64::URL_SAFE_NO_PAD))
        }

        ///
        /// Return the code challenge method for this code verifier.
        ///
        pub fn code_challenge_method() -> PkceCodeChallengeMethod {
            PkceCodeChallengeMethod::new("S256".to_string())
        }

        ///
        /// Return the extension params used for authorize_url.
        ///
        pub fn authorize_url_params(&self) -> Vec<(&'static str, String)> {
            vec![
                (
                    "code_challenge_method",
                    PkceCodeVerifierS256::code_challenge_method().into(),
                ),
                ("code_challenge", self.code_challenge().into()),
            ]
        }
    }
];
new_secret_type![
    ///
    /// Authorization code returned from the authorization endpoint.
    ///
    #[derive(Deserialize, Serialize)]
    AuthorizationCode(String)
];
new_secret_type![
    ///
    /// Refresh token used to obtain a new access token (if supported by the authorization server).
    ///
    #[derive(Deserialize, Serialize)]
    RefreshToken(String)
];
new_secret_type![
    ///
    /// Access token returned by the token endpoint and used to access protected resources.
    ///
    #[derive(Deserialize, Serialize)]
    AccessToken(String)
];
new_secret_type![
    ///
    /// Resource owner's password used directly as an authorization grant to obtain an access
    /// token.
    ///
    ResourceOwnerPassword(String)
];
