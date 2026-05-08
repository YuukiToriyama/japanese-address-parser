pub mod parse;
#[deprecated(
    since = "0.3.2",
    note = "Use japanese_address_parser::parser::parse::ParseResult instead. This will be deleted in v0.4"
)]
pub use self::parse::ParseResult;

use std::sync::Arc;

use crate::http::reqwest_client::ReqwestApiClient;
use crate::interactor::geolonia::GeoloniaInteractorImpl;

/// An asynchronous `Parser` to process addresses.
///
/// # Example
/// ```
/// use japanese_address_parser::parser::Parser;
///
/// async fn example() {
///     let parser : Parser = Default::default();
///     let result = parser.parse("東京都新宿区西新宿2-8-1").await;
///     println!("{:?}", result);
/// }
/// ```
pub struct Parser {
    interactor: Arc<GeoloniaInteractorImpl<ReqwestApiClient>>,
}

impl Default for Parser {
    /// Constructs a new `Parser`.
    fn default() -> Self {
        Self {
            interactor: Arc::new(Default::default()),
        }
    }
}
