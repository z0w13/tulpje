use twilight_model::{
    channel::message::{EmojiReactionType, Reaction, ReactionCountDetails},
    gateway::payload::incoming::{
        ReactionAdd, ReactionRemove, ReactionRemoveAll, ReactionRemoveEmoji,
    },
};

use crate::{Cache, Error, UpdateCache};

impl UpdateCache for ReactionAdd {
    async fn update(&self, cache: &Cache) -> Result<(), Error> {
        let key = self.0.message_id;

        let Some(mut message) = cache.messages.get(&key).await? else {
            return Ok(());
        };

        if let Some(reaction) = message
            .reactions
            .iter_mut()
            .find(|r| reactions_eq(&r.emoji, &self.0.emoji))
        {
            if !reaction.me
                && let Some(current_user) = cache.current_user.get().await?
                && current_user.id == self.0.user_id
            {
                reaction.me = true;
            }

            reaction.count += 1;
        } else {
            let me = cache
                .current_user
                .get()
                .await?
                .is_some_and(|user| user.id == self.0.user_id);

            message.reactions.push(Reaction {
                burst_colors: Vec::new(),
                count: 1,
                count_details: ReactionCountDetails {
                    burst: 0,
                    normal: 1,
                },
                emoji: self.0.emoji.clone(),
                me,
                me_burst: false,
            });
        }

        cache.messages.insert(&message.id, &message).await?;

        Ok(())
    }
}

impl UpdateCache for ReactionRemove {
    async fn update(&self, cache: &Cache) -> Result<(), Error> {
        let Some(mut message) = cache.messages.get(&self.0.message_id).await? else {
            return Ok(());
        };

        if let Some(reaction) = message
            .reactions
            .iter_mut()
            .find(|r| reactions_eq(&r.emoji, &self.0.emoji))
        {
            if reaction.me
                && let Some(current_user) = cache.current_user.get().await?
                && current_user.id == self.0.user_id
            {
                reaction.me = false;
            }

            if reaction.count > 1 {
                reaction.count -= 1;
            } else {
                message
                    .reactions
                    .retain(|e| !(reactions_eq(&e.emoji, &self.0.emoji)));
            }
        }

        cache.messages.insert(&message.id, &message).await?;

        Ok(())
    }
}

impl UpdateCache for ReactionRemoveAll {
    async fn update(&self, cache: &Cache) -> Result<(), Error> {
        let Some(mut message) = cache.messages.get(&self.message_id).await? else {
            return Ok(());
        };

        message.reactions.clear();
        cache.messages.insert(&message.id, &message).await?;

        Ok(())
    }
}

impl UpdateCache for ReactionRemoveEmoji {
    async fn update(&self, cache: &Cache) -> Result<(), Error> {
        let Some(mut message) = cache.messages.get(&self.message_id).await? else {
            return Ok(());
        };

        let maybe_index = message
            .reactions
            .iter()
            .position(|r| reactions_eq(&r.emoji, &self.emoji));

        if let Some(index) = maybe_index {
            message.reactions.remove(index);
        }

        cache.messages.insert(&message.id, &message).await?;

        Ok(())
    }
}

fn reactions_eq(a: &EmojiReactionType, b: &EmojiReactionType) -> bool {
    match (a, b) {
        (
            EmojiReactionType::Custom { id: id_a, .. },
            EmojiReactionType::Custom { id: id_b, .. },
        ) => id_a == id_b,
        (
            EmojiReactionType::Unicode { name: name_a },
            EmojiReactionType::Unicode { name: name_b },
        ) => name_a == name_b,
        _ => false,
    }
}
