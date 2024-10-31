use serde::Serialize;

#[derive(Debug, Serialize)]
struct ReplyData {
    in_reply_to_tweet_id: String,
}

#[derive(Debug, Serialize)]
struct MediaData {
    media_ids: Vec<String>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Default)]
pub struct Tweet {
    content: String,
    quote_tweet_id: Option<String>,
    reply_data: Option<ReplyData>,
    media_data: Option<MediaData>,
}

impl Tweet {
    pub fn new(content: String) -> Self {
        Self {
            content,
            quote_tweet_id: None,
            reply_data: None,
            media_data: None,
        }
    }

    pub fn validate_content(&self) -> eyre::Result<()> {
        if self.content.is_empty() {
            eyre::bail!("Tweet content cannot be empty");
        }
        if self.quote_tweet_id.is_some() && self.reply_data.is_some() {
            eyre::bail!("Tweet cannot be both a quote and a reply");
        }
        if let Some(media_data) = &self.media_data {
            if media_data.media_ids.is_empty() {
                eyre::bail!("Media IDs cannot be empty");
            }
        }
        Ok(())
    }

    pub fn assign_quote_tweet_id(&mut self, quote_tweet_id: String) {
        self.quote_tweet_id = Some(quote_tweet_id);
    }

    pub fn assign_reply_tweet_id(&mut self, reply_tweet_id: String) {
        self.reply_data = Some(ReplyData {
            in_reply_to_tweet_id: reply_tweet_id,
        });
    }

    pub fn assign_media_ids(&mut self, media_ids: Vec<String>) {
        self.media_data = Some(MediaData { media_ids });
    }
}
