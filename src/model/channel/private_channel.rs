use chrono::{DateTime, FixedOffset};
use crate::model::prelude::*;
use std::fmt::{
    Display,
    Formatter,
    Result as FmtResult
};

#[cfg(feature = "model")]
use crate::builder::{
    CreateMessage,
    EditMessage,
    GetMessages
};
#[cfg(feature = "model")]
use crate::http::AttachmentType;
#[cfg(feature = "http")]
use crate::http::Http;

/// A Direct Message text channel with another user.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PrivateChannel {
    /// The unique Id of the private channel.
    ///
    /// Can be used to calculate the first message's creation date.
    pub id: ChannelId,
    /// The Id of the last message sent.
    pub last_message_id: Option<MessageId>,
    /// Timestamp of the last time a [`Message`] was pinned.
    ///
    /// [`Message`]: struct.Message.html
    pub last_pin_timestamp: Option<DateTime<FixedOffset>>,
    /// Indicator of the type of channel this is.
    ///
    /// This should always be [`ChannelType::Private`].
    ///
    /// [`ChannelType::Private`]: enum.ChannelType.html#variant.Private
    #[serde(rename = "type")]
    pub kind: ChannelType,
    /// The recipient to the private channel.
    #[serde(deserialize_with = "deserialize_single_recipient",
            serialize_with = "serialize_single_recipient",
            rename = "recipients")]
    pub recipient: User,
    #[serde(skip)]
    pub(crate) _nonexhaustive: (),
}

#[cfg(feature = "model")]
impl PrivateChannel {
    /// Broadcasts that the current user is typing to the recipient.
    #[inline]
    pub async fn broadcast_typing(&self, http: impl AsRef<Http>) -> Result<()> {
        self.id.broadcast_typing(&http).await
    }

    /// React to a [`Message`] with a custom [`Emoji`] or unicode character.
    ///
    /// [`Message::react`] may be a more suited method of reacting in most
    /// cases.
    ///
    /// Requires the [Add Reactions] permission, _if_ the current user is the
    /// first user to perform a react with a certain emoji.
    ///
    /// [`Emoji`]: ../guild/struct.Emoji.html
    /// [`Message`]: struct.Message.html
    /// [`Message::react`]: struct.Message.html#method.react
    /// [Add Reactions]: ../permissions/struct.Permissions.html#associatedconstant.ADD_REACTIONS
    #[inline]
    pub async fn create_reaction(
        &self,
        http: impl AsRef<Http>,
        message_id: impl Into<MessageId>,
        reaction_type: impl Into<ReactionType>
    ) -> Result<()>
    {
        self.id.create_reaction(&http, message_id, reaction_type).await
    }

    /// Deletes the channel. This does not delete the contents of the channel,
    /// and is equivalent to closing a private channel on the client, which can
    /// be re-opened.
    #[inline]
    pub async fn delete(&self, http: impl AsRef<Http>) -> Result<Channel> {
        self.id.delete(&http).await
    }

    /// Deletes all messages by Ids from the given vector in the channel.
    ///
    /// Refer to [`Channel::delete_messages`] for more information.
    ///
    /// Requires the [Manage Messages] permission.
    ///
    /// **Note**: Messages that are older than 2 weeks can't be deleted using
    /// this method.
    ///
    /// # Errors
    ///
    /// Returns [`ModelError::BulkDeleteAmount`] if an attempt was made to
    /// delete either 0 or more than 100 messages.
    ///
    /// [`Channel::delete_messages`]: enum.Channel.html#method.delete_messages
    /// [`ModelError::BulkDeleteAmount`]: ../error/enum.Error.html#variant.BulkDeleteAmount
    /// [Manage Messages]: ../permissions/struct.Permissions.html#associatedconstant.MANAGE_MESSAGES
    #[inline]
    pub async fn delete_messages<T: AsRef<MessageId>, It: IntoIterator<Item=T>>(
        &self,
        http: impl AsRef<Http>,
        message_ids: It
    ) -> Result<()>
    where T: AsRef<MessageId>, It: IntoIterator<Item=T>,
    {
        self.id.delete_messages(&http, message_ids).await
    }

    /// Deletes all permission overrides in the channel from a member
    /// or role.
    ///
    /// **Note**: Requires the [Manage Channel] permission.
    ///
    /// [Manage Channel]: ../permissions/struct.Permissions.html#associatedconstant.MANAGE_CHANNELS
    #[inline]
    pub async fn delete_permission(&self, http: impl AsRef<Http>, permission_type: PermissionOverwriteType) -> Result<()> {
        self.id.delete_permission(&http, permission_type).await
    }

    /// Deletes the given [`Reaction`] from the channel.
    ///
    /// **Note**: Requires the [Manage Messages] permission, _if_ the current
    /// user did not perform the reaction.
    ///
    /// [`Reaction`]: struct.Reaction.html
    /// [Manage Messages]: ../permissions/struct.Permissions.html#associatedconstant.MANAGE_MESSAGES
    #[inline]
    pub async fn delete_reaction(
        &self,
        http: impl AsRef<Http>,
        message_id: impl Into<MessageId>,
        user_id: Option<UserId>,
        reaction_type: impl Into<ReactionType>,
    ) -> Result<()>
    {
        self.id.delete_reaction(&http, message_id, user_id, reaction_type).await
    }

    /// Edits a [`Message`] in the channel given its Id.
    ///
    /// Message editing preserves all unchanged message data.
    ///
    /// Refer to the documentation for [`EditMessage`] for more information
    /// regarding message restrictions and requirements.
    ///
    /// **Note**: Requires that the current user be the author of the message.
    ///
    /// # Errors
    ///
    /// Returns a [`ModelError::MessageTooLong`] if the content of the message
    /// is over the [`the limit`], containing the number of unicode code points
    /// over the limit.
    ///
    /// [`ModelError::MessageTooLong`]: ../error/enum.Error.html#variant.MessageTooLong
    /// [`EditMessage`]: ../../builder/struct.EditMessage.html
    /// [`Message`]: struct.Message.html
    /// [`the limit`]: ../../builder/struct.EditMessage.html#method.content
    #[inline]
    pub async fn edit_message<F>(
        &self,
        http: impl AsRef<Http>,
        message_id: impl Into<MessageId>,
        f: F
    ) -> Result<Message>
    where F: FnOnce(&mut EditMessage) -> &mut EditMessage
    {
        self.id.edit_message(&http, message_id, f).await
    }

    /// Determines if the channel is NSFW.
    ///
    /// **Note**: This method is for consistency. This will always return
    /// `false`, due to DMs not being considered NSFW.
    #[inline]
    pub fn is_nsfw(&self) -> bool { false }

    /// Gets a message from the channel.
    ///
    /// Requires the [Read Message History] permission.
    ///
    /// [Read Message History]: ../permissions/struct.Permissions.html#associatedconstant.READ_MESSAGE_HISTORY
    #[inline]
    pub async fn message(&self, http: impl AsRef<Http>, message_id: impl Into<MessageId>) -> Result<Message> {
        self.id.message(&http, message_id).await
    }

    /// Gets messages from the channel.
    ///
    /// Refer to [`GetMessages`] for more information on how to use `builder`.
    ///
    /// Requires the [Read Message History] permission.
    ///
    /// [`GetMessages`]: ../../builder/struct.GetMessages.html
    /// [Read Message History]: ../permissions/struct.Permissions.html#associatedconstant.READ_MESSAGE_HISTORY
    #[inline]
    pub async fn messages<F>(&self, http: impl AsRef<Http>, builder: F) -> Result<Vec<Message>>
    where F: FnOnce(&mut GetMessages) -> &mut GetMessages
    {
        self.id.messages(&http, builder).await
    }

    /// Returns "DM with $username#discriminator".
    pub fn name(&self) -> String { format!("DM with {}", self.recipient.tag()) }

    /// Gets the list of [`User`]s who have reacted to a [`Message`] with a
    /// certain [`Emoji`].
    ///
    /// Refer to [`Channel::reaction_users`] for more information.
    ///
    /// **Note**: Requires the [Read Message History] permission.
    ///
    /// [`Channel::reaction_users`]: enum.Channel.html#method.reaction_users
    /// [`Emoji`]: ../guild/struct.Emoji.html
    /// [`Message`]: struct.Message.html
    /// [`User`]: ../user/struct.User.html
    /// [Read Message History]: ../permissions/struct.Permissions.html#associatedconstant.READ_MESSAGE_HISTORY
    #[inline]
    pub async fn reaction_users<M, R, U>(&self,
        http: impl AsRef<Http>,
        message_id: impl Into<MessageId>,
        reaction_type: impl Into<ReactionType>,
        limit: Option<u8>,
        after: impl Into<Option<UserId>>,
    ) -> Result<Vec<User>>
    {
        self.id.reaction_users(&http, message_id, reaction_type, limit, after).await
    }

    /// Pins a [`Message`] to the channel.
    ///
    /// [`Message`]: struct.Message.html
    #[inline]
    pub async fn pin(&self, http: impl AsRef<Http>, message_id: impl Into<MessageId>) -> Result<()> {
        self.id.pin(&http, message_id).await
    }

    /// Retrieves the list of messages that have been pinned in the private
    /// channel.
    #[inline]
    pub async fn pins(&self, http: impl AsRef<Http>) -> Result<Vec<Message>> {
        self.id.pins(&http).await
    }

    /// Sends a message with just the given message content in the channel.
    ///
    /// # Errors
    ///
    /// Returns a [`ModelError::MessageTooLong`] if the content of the message
    /// is over the above limit, containing the number of unicode code points
    /// over the limit.
    ///
    /// [`ChannelId`]: ../id/struct.ChannelId.html
    /// [`ModelError::MessageTooLong`]: ../error/enum.Error.html#variant.MessageTooLong
    #[inline]
    pub async fn say(&self, http: impl AsRef<Http>, content: impl std::fmt::Display) -> Result<Message> {
        self.id.say(&http, content).await
    }

    /// Sends (a) file(s) along with optional message contents.
    ///
    /// Refer to [`ChannelId::send_files`] for examples and more information.
    ///
    /// The [Attach Files] and [Send Messages] permissions are required.
    ///
    /// **Note**: Message contents must be under 2000 unicode code points.
    ///
    /// # Errors
    ///
    /// If the content of the message is over the above limit, then a
    /// [`ClientError::MessageTooLong`] will be returned, containing the number
    /// of unicode code points over the limit.
    ///
    /// [`ChannelId::send_files`]: ../id/struct.ChannelId.html#method.send_files
    /// [`ClientError::MessageTooLong`]: ../../client/enum.ClientError.html#variant.MessageTooLong
    /// [Attach Files]: ../permissions/struct.Permissions.html#associatedconstant.ATTACH_FILES
    /// [Send Messages]: ../permissions/struct.Permissions.html#associatedconstant.SEND_MESSAGES
    #[inline]
    pub async fn send_files<'a, F, T, It>(&self, http: impl AsRef<Http>, files: It, f: F) -> Result<Message>
    where for <'b> F: FnOnce(&'b mut CreateMessage<'a>) -> &'b mut CreateMessage<'a>,
          T: Into<AttachmentType<'a>>, It: IntoIterator<Item=T>
    {
        self.id.send_files(&http, files, f).await
    }

    /// Sends a message to the channel with the given content.
    ///
    /// Refer to the documentation for [`CreateMessage`] for more information
    /// regarding message restrictions and requirements.
    ///
    /// # Errors
    ///
    /// Returns a [`ModelError::MessageTooLong`] if the content of the message
    /// is over the above limit, containing the number of unicode code points
    /// over the limit.
    ///
    /// [`ModelError::MessageTooLong`]: ../error/enum.Error.html#variant.MessageTooLong
    /// [`CreateMessage`]: ../../builder/struct.CreateMessage.html
    /// [`Message`]: struct.Message.html
    #[inline]
    pub async fn send_message<'a, F>(&self, http: impl AsRef<Http>, f: F) -> Result<Message>
    where for <'b> F: FnOnce(&'b mut CreateMessage<'a>) -> &'b mut CreateMessage<'a>
    {
        self.id.send_message(&http, f).await
    }

    /// Unpins a [`Message`] in the channel given by its Id.
    ///
    /// Requires the [Manage Messages] permission.
    ///
    /// [`Message`]: struct.Message.html
    /// [Manage Messages]: ../permissions/struct.Permissions.html#associatedconstant.MANAGE_MESSAGES
    #[inline]
    pub async fn unpin(&self, http: impl AsRef<Http>, message_id: impl Into<MessageId>) -> Result<()> {
        self.id.unpin(&http, message_id).await
    }
}

impl Display for PrivateChannel {
    /// Formats the private channel, displaying the recipient's username.
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str(&self.recipient.name)
    }
}
