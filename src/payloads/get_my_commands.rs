// This file is auto generated by `cg` <https://github.com/teloxide/cg> (e634f65).
// **DO NOT EDIT THIS FILE**,
// edit `cg` instead.
use serde::Serialize;

impl_payload! {
    /// Use this method to get the current list of the bot's commands. Requires no parameters. Returns Array of [`BotCommand`] on success.
    ///
    /// [`BotCommand`]: crate::types::BotCommand
    #[derive(Debug, PartialEq, Eq, Hash, Default, Clone, Serialize)]
    pub GetMyCommands (GetMyCommandsSetters) => u32 {

    }
}
