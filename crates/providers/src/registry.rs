use std::{
    collections::HashMap,
    sync::Arc,
};

use tokio::sync::RwLock;

use crate::Provider;

pub type SharedProvider = Arc<dyn Provider>;

#[derive(Clone, Default)]
pub struct ProviderRegistry {
    providers: Arc<RwLock<HashMap<String, SharedProvider>>>,
}

impl ProviderRegistry {
    /// Create an empty registry.
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a provider.
    pub async fn register<P>(
        &self,
        name: impl Into<String>,
        provider: P,
    )
    where
        P: Provider + 'static,
    {
        let mut providers = self.providers.write().await;

        providers.insert(name.into(), Arc::new(provider));
    }

    /// Retrieve a provider.
    pub async fn get(
        &self,
        name: &str,
    ) -> Option<SharedProvider> {
        let providers = self.providers.read().await;

        providers.get(name).cloned()
    }

    /// Remove a provider.
    pub async fn remove(
        &self,
        name: &str,
    ) -> Option<SharedProvider> {
        let mut providers = self.providers.write().await;

        providers.remove(name)
    }

    /// Returns true if the provider exists.
    pub async fn contains(
        &self,
        name: &str,
    ) -> bool {
        let providers = self.providers.read().await;

        providers.contains_key(name)
    }

    /// Number of providers.
    pub async fn len(&self) -> usize {
        let providers = self.providers.read().await;

        providers.len()
    }

    /// Returns all registered providers.
    pub async fn list(
        &self,
    ) -> Vec<String> {
        let providers = self.providers.read().await;

        providers.keys().cloned().collect()
    }

    /// Returns true when empty.
    pub async fn is_empty(&self) -> bool {
        self.len().await == 0
    }
}