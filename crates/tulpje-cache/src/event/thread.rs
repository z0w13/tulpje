use twilight_model::gateway::payload::incoming::{
    ThreadCreate, ThreadDelete, ThreadListSync, ThreadUpdate,
};

use crate::{Error, UpdateCache};

impl UpdateCache for ThreadCreate {
    async fn update(&self, cache: &crate::Cache) -> Result<(), Error> {
        cache.cache_channel(self.0.clone()).await?;
        Ok(())
    }
}

impl UpdateCache for ThreadDelete {
    async fn update(&self, cache: &crate::Cache) -> Result<(), Error> {
        cache.delete_channel(self.id).await?;
        Ok(())
    }
}

impl UpdateCache for ThreadListSync {
    async fn update(&self, cache: &crate::Cache) -> Result<(), Error> {
        cache.cache_channels(self.threads.clone()).await?;
        Ok(())
    }
}

impl UpdateCache for ThreadUpdate {
    async fn update(&self, cache: &crate::Cache) -> Result<(), Error> {
        cache.cache_channel(self.0.clone()).await?;
        Ok(())
    }
}
