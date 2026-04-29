use serde::{Deserialize, Serialize};
use std::fmt;

/// Macro to define type-safe ID wrappers for different Tidal resources
macro_rules! define_id {
    ($name:ident) => {
        #[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
        #[serde(transparent)]
        pub struct $name(String);

        impl $name {
            pub fn new(id: impl Into<String>) -> Self {
                Self(id.into())
            }

            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl From<String> for $name {
            fn from(s: String) -> Self {
                Self(s)
            }
        }

        impl From<&str> for $name {
            fn from(s: &str) -> Self {
                Self(s.to_string())
            }
        }

        impl From<$name> for String {
            fn from(id: $name) -> String {
                id.0
            }
        }

        impl AsRef<str> for $name {
            fn as_ref(&self) -> &str {
                &self.0
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.0)
            }
        }
    };
}

define_id!(TrackId);
define_id!(AlbumId);
define_id!(ArtistId);
define_id!(PlaylistId);
define_id!(VideoId);
define_id!(MixId);
define_id!(UserId);

#[cfg(test)]
mod tests {
    use super::TrackId;

    #[test]
    fn id_wrapper_conversions_are_lossless() {
        let id = TrackId::new("12345");
        assert_eq!(id.as_str(), "12345");
        assert_eq!(id.to_string(), "12345");

        let from_str: TrackId = "abc".into();
        let from_string: TrackId = String::from("xyz").into();

        assert_eq!(from_str.as_ref(), "abc");
        assert_eq!(from_string.as_ref(), "xyz");

        let back_to_string: String = from_string.into();
        assert_eq!(back_to_string, "xyz");
    }
}
