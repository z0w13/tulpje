use twilight_cache_inmemory::ResourceType;

pub struct Config {
    pub resource_types: ResourceType,
    pub message_cache_size: usize,
}

impl Config {
    pub fn new() -> Self {
        Self {
            resource_types: ResourceType::empty(),
            message_cache_size: 100,
        }
    }

    pub fn resource_types(mut self, resource_types: ResourceType) -> Self {
        self.resource_types = resource_types;
        self
    }

    pub fn message_cache_size(mut self, message_cache_size: usize) -> Self {
        self.message_cache_size = message_cache_size;
        self
    }

    pub(crate) fn wants(&self, resource_type: ResourceType) -> bool {
        self.resource_types.contains(resource_type)
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}
