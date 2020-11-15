// This file is auto generated by `cg` <https://github.com/teloxide/cg> (e634f65).
// **DO NOT EDIT THIS FILE**,
// edit `cg` instead.
use serde::Serialize;

use crate::types::{ChatId, Message, ReplyMarkup};

impl_payload! {
    /// Use this method to edit live location messages. A location can be edited until its live_period expires or editing is explicitly disabled by a call to [`StopMessageLiveLocation`]. On success, the edited Message is returned.
    ///
    /// See also: [`EditMessageLiveLocationInline`](crate::payloads::EditMessageLiveLocationInline)
    ///
    /// [`StopMessageLiveLocation`]: crate::payloads::StopMessageLiveLocation
    #[derive(Debug, PartialEq, Clone, Serialize)]
    pub EditMessageLiveLocation (EditMessageLiveLocationSetters) => Message {
        required {
            /// Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
            pub chat_id: ChatId [into],
            /// Identifier of the message to edit
            pub message_id: i64,
            /// Latitude of new location
            pub latitude: f64,
            /// Longitude of new location
            pub longitude: f64,
        }
        optional {
            /// Additional interface options. A JSON-serialized object for an [inline keyboard], [custom reply keyboard], instructions to remove reply keyboard or to force a reply from the user.
            ///
            /// [custom reply keyboard]: https://core.telegram.org/bots#keyboards
            /// [inline keyboard]: https://core.telegram.org/bots#inline-keyboards-and-on-the-fly-updating
            pub reply_markup: ReplyMarkup [into],
        }
    }
}
