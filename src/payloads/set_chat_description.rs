// This file is auto generated by `cg` <https://github.com/teloxide/cg> (e634f65).
// **DO NOT EDIT THIS FILE**,
// edit `cg` instead.
use serde::Serialize;

use crate::types::ChatId;

impl_payload! {
    /// Use this method to change the description of a group, a supergroup or a channel. The bot must be an administrator in the chat for this to work and must have the appropriate admin rights. Returns _True_ on success.
    #[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize)]
    pub SetChatDescription (SetChatDescriptionSetters) => String {
        required {
            /// Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
            pub chat_id: ChatId [into],
        }
        optional {
            /// New chat description, 0-255 characters
            pub description: String [into],
        }
    }
}
