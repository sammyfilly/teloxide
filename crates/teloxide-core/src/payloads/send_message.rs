//! Generated by `codegen_payloads`, do not edit by hand.

use serde::Serialize;

use crate::types::{Message, MessageEntity, MessageId, ParseMode, Recipient, ReplyMarkup};

impl_payload! {
    /// Use this method to send text messages. On success, the sent [`Message`] is returned.
    ///
    /// [`Message`]: crate::types::Message
    #[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize)]
    pub SendMessage (SendMessageSetters) => Message {
        required {
            /// Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
            pub chat_id: Recipient [into],
            /// Text of the message to be sent, 1-4096 characters after entities parsing
            pub text: String [into],
        }
        optional {
            /// Unique identifier for the target message thread (topic) of the forum; for forum supergroups only
            pub message_thread_id: i32,
            /// Mode for parsing entities in the message text. See [formatting options] for more details.
            ///
            /// [formatting options]: https://core.telegram.org/bots/api#formatting-options
            pub parse_mode: ParseMode,
            /// List of special entities that appear in the message text, which can be specified instead of _parse\_mode_
            pub entities: Vec<MessageEntity> [collect],
            /// Disables link previews for links in this message
            pub disable_web_page_preview: bool,
            /// Sends the message [silently]. Users will receive a notification with no sound.
            ///
            /// [silently]: https://telegram.org/blog/channels-2-0#silent-messages
            pub disable_notification: bool,
            /// Protects the contents of sent messages from forwarding and saving
            pub protect_content: bool,
            /// If the message is a reply, ID of the original message
            #[serde(serialize_with = "crate::types::serialize_reply_to_message_id")]
            pub reply_to_message_id: MessageId,
            /// Pass _True_, if the message should be sent even if the specified replied-to message is not found
            pub allow_sending_without_reply: bool,
            /// Additional interface options. A JSON-serialized object for an [inline keyboard], [custom reply keyboard], instructions to remove reply keyboard or to force a reply from the user.
            ///
            /// [inline keyboard]: https://core.telegram.org/bots#inline-keyboards-and-on-the-fly-updating
            /// [custom reply keyboard]: https://core.telegram.org/bots#keyboards
            pub reply_markup: ReplyMarkup [into],
        }
    }
}
