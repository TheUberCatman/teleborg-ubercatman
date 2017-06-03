pub use self::inline_query_result_article::InlineQueryResultArticle;
pub use self::marker::InlineQueryResult;
pub use self::inline_query_result_type::InlineQueryResultType;

mod inline_query_result_article;
mod marker;
mod inline_query_result_type;

use serde::{Serialize, Serializer};

impl Serialize for Box<InlineQueryResult> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        match self.get_type() {
            InlineQueryResultType::Article => {
                serializer.serialize_some((&**self)
                                              .as_any()
                                              .downcast_ref::<InlineQueryResultArticle>()
                                              .unwrap())
            }
        }
    }
}
