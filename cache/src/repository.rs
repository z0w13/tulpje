use std::{collections::HashSet, hash::Hash, marker::PhantomData};

use redis::{aio::ConnectionManager as RedisConnectionManager, AsyncCommands as _};
use serde::{de::DeserializeOwned, Serialize};

use crate::hash;

pub struct Repository<K: Hash, V: Serialize + DeserializeOwned> {
    name: String,
    wanted: bool,

    redis: RedisConnectionManager,

    key: PhantomData<K>,
    value: PhantomData<V>,
}

impl<K: Hash, V: Serialize + DeserializeOwned> Repository<K, V> {
    pub(crate) fn new(name: &str, wanted: bool, redis: RedisConnectionManager) -> Self {
        Self {
            name: format!("cache:{}", name),
            wanted,

            redis,

            key: PhantomData,
            value: PhantomData,
        }
    }

    pub async fn get(&self, key: &K) -> Result<Option<V>, crate::Error> {
        Ok(self
            .redis
            .clone()
            .hget::<_, _, Option<String>>(&self.name, hash(key))
            .await?
            .map(|json| serde_json::from_str(&json))
            .transpose()?)
    }

    pub(crate) async fn insert(&self, key: &K, value: &V) -> Result<bool, crate::Error> {
        if !self.wanted {
            return Ok(false);
        }

        Ok(self
            .redis
            .clone()
            .hset::<_, _, _, usize>(&self.name, hash(key), serde_json::to_string(value)?)
            .await?
            > 0)
    }

    pub(crate) async fn remove(&self, key: &K) -> Result<bool, crate::Error> {
        if !self.wanted {
            return Ok(false);
        }

        Ok(self
            .redis
            .clone()
            .hdel::<_, _, usize>(&self.name, hash(key))
            .await?
            > 0)
    }

    pub(crate) async fn remove_multi<'a>(
        &self,
        keys: impl IntoIterator<Item = &'a K>,
    ) -> Result<usize, crate::Error>
    where
        K: 'a,
    {
        if !self.wanted {
            return Ok(0);
        }

        let keys: Vec<u64> = keys.into_iter().map(hash).collect();
        if keys.is_empty() {
            return Ok(0);
        }

        Ok(self.redis.clone().hdel(&self.name, &keys).await?)
    }
}

pub struct SetRepository<T: Serialize + DeserializeOwned + Eq + Hash> {
    name: String,
    wanted: bool,

    redis: RedisConnectionManager,

    value: PhantomData<T>,
}

impl<T: Serialize + DeserializeOwned + Eq + Hash> SetRepository<T> {
    pub(crate) fn new(name: &str, wanted: bool, redis: RedisConnectionManager) -> Self {
        Self {
            name: format!("cache:{}", name),
            wanted,

            redis,

            value: PhantomData,
        }
    }

    pub async fn members(&self) -> Result<HashSet<T>, crate::Error> {
        let json_set: HashSet<String> = self.redis.clone().smembers(&self.name).await?;

        let mut result_set = HashSet::new();
        if json_set.is_empty() {
            return Ok(result_set);
        }

        for json_item in json_set {
            result_set.insert(serde_json::from_str(&json_item)?);
        }

        Ok(result_set)
    }

    pub async fn contains(&self, value: &T) -> Result<bool, crate::Error> {
        Ok(self
            .redis
            .clone()
            .sismember::<_, _, usize>(&self.name, serde_json::to_string(value)?)
            .await?
            > 0)
    }

    pub async fn is_empty(&self) -> Result<bool, crate::Error> {
        if !self.wanted {
            return Ok(false);
        }

        Ok(self.redis.clone().exists(&self.name).await?)
    }

    pub(crate) async fn insert(&self, value: &T) -> Result<bool, crate::Error> {
        if !self.wanted {
            return Ok(false);
        }

        Ok(self
            .redis
            .clone()
            .sadd::<_, _, usize>(&self.name, serde_json::to_string(value)?)
            .await?
            > 0)
    }

    pub(crate) async fn remove(&self, value: &T) -> Result<bool, crate::Error> {
        if !self.wanted {
            return Ok(false);
        }

        Ok(self
            .redis
            .clone()
            .srem::<_, _, usize>(&self.name, serde_json::to_string(value)?)
            .await?
            > 0)
    }

    pub(crate) async fn remove_multi<'a>(
        &self,
        values: impl IntoIterator<Item = &'a T>,
    ) -> Result<usize, crate::Error>
    where
        T: 'a,
    {
        if !self.wanted {
            return Ok(0);
        }

        let mut json_values = Vec::new();
        for value in values {
            json_values.push(serde_json::to_string(&value)?);
        }

        if json_values.is_empty() {
            return Ok(0);
        }

        Ok(self.redis.clone().srem(&self.name, json_values).await?)
    }

    pub(crate) async fn clear(&self) -> Result<bool, crate::Error> {
        if !self.wanted {
            return Ok(false);
        }

        Ok(self.redis.clone().del::<_, usize>(&self.name).await? > 0)
    }
}

pub struct MappedSetRepository<K: Hash, V: Serialize + DeserializeOwned + Eq + Hash> {
    name: String,
    wanted: bool,

    redis: RedisConnectionManager,

    key: PhantomData<K>,
    value: PhantomData<V>,
}

impl<K: Hash, V: Serialize + DeserializeOwned + Eq + Hash> MappedSetRepository<K, V> {
    pub(crate) fn new(name: &str, wanted: bool, redis: RedisConnectionManager) -> Self {
        Self {
            name: String::from(name),
            wanted,

            redis,

            key: PhantomData,
            value: PhantomData,
        }
    }

    fn key(&self, key: &K) -> String {
        format!("{}:{}", self.name, hash(key))
    }

    fn set_repository(&self, key: &K) -> SetRepository<V> {
        SetRepository::new(&self.key(key), self.wanted, self.redis.clone())
    }

    pub async fn members(&self, key: &K) -> Result<HashSet<V>, crate::Error> {
        self.set_repository(key).members().await
    }

    pub async fn is_empty(&self, key: &K) -> Result<bool, crate::Error> {
        self.set_repository(key).is_empty().await
    }

    pub async fn contains(&self, key: &K, value: &V) -> Result<bool, crate::Error> {
        self.set_repository(key).contains(value).await
    }

    pub(crate) async fn insert(&self, key: &K, value: &V) -> Result<bool, crate::Error> {
        self.set_repository(key).insert(value).await
    }

    pub(crate) async fn remove(&self, key: &K, value: &V) -> Result<bool, crate::Error> {
        self.set_repository(key).remove(value).await
    }

    pub(crate) async fn clear(&self, key: &K) -> Result<bool, crate::Error> {
        self.set_repository(key).clear().await
    }

    pub(crate) async fn remove_multi<'a>(
        &self,
        key: &K,
        values: impl IntoIterator<Item = &'a V>,
    ) -> Result<usize, crate::Error>
    where
        V: 'a,
    {
        self.set_repository(key).remove_multi(values).await
    }
}

pub struct SingleRepository<T: Serialize + DeserializeOwned> {
    name: String,
    wanted: bool,

    redis: RedisConnectionManager,

    value: PhantomData<T>,
}

impl<T: Serialize + DeserializeOwned> SingleRepository<T> {
    pub(crate) fn new(name: &str, wanted: bool, redis: RedisConnectionManager) -> Self {
        Self {
            name: format!("cache:{}", name),
            wanted,

            redis,

            value: PhantomData,
        }
    }

    pub(crate) async fn set(&self, value: T) -> Result<(), crate::Error> {
        if !self.wanted {
            return Ok(());
        }

        Ok(self
            .redis
            .clone()
            .set(&self.name, serde_json::to_string(&value)?)
            .await?)
    }

    pub async fn get(&self) -> Result<Option<T>, crate::Error> {
        if !self.wanted {
            return Ok(None);
        }

        Ok(self
            .redis
            .clone()
            .get::<_, Option<String>>(&self.name)
            .await?
            .map(|json| serde_json::from_str(&json))
            .transpose()?)
    }
}
