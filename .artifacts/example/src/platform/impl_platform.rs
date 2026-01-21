/*
    Appellation: impl_platform <module>
    Contrib: @FL03
*/
use crate::platform::*;
use std::sync::Arc;

impl Platform {
    pub fn new() -> Self {
        let inner = PlatformInner::default();
        Self {
            inner: Arc::new(inner),
        }
    }

    pub fn arc_clone(&self) -> Arc<PlatformInner> {
        Arc::clone(&self.inner)
    }

    pub fn get(self) -> Arc<PlatformInner> {
        self.inner
    }

    pub fn get_mut(&mut self) -> &mut PlatformInner {
        Arc::make_mut(&mut self.inner)
    }

    pub fn get_ref(&self) -> &PlatformInner {
        &self.inner
    }

    pub fn set(&mut self, inner: PlatformInner) {
        self.inner = Arc::new(inner);
    }

    pub fn with(self, inner: PlatformInner) -> Self {
        Self {
            inner: Arc::new(inner),
        }
    }

    pub async fn run(&mut self) -> anyhow::Result<()> {
        // parse command line arguments
        let cli = crate::Cli::parse();
        // handle command line arguments
        cli.handle(self.as_mut()).await?;

        Ok(())
    }
}

impl Clone for Platform {
    fn clone(&self) -> Self {
        Self {
            inner: Arc::clone(&self.inner),
        }
    }
}

impl AsRef<PlatformInner> for Platform {
    fn as_ref(&self) -> &PlatformInner {
        &self.inner
    }
}

impl AsMut<PlatformInner> for Platform {
    fn as_mut(&mut self) -> &mut PlatformInner {
        Arc::make_mut(&mut self.inner)
    }
}

impl core::ops::Deref for Platform {
    type Target = PlatformInner;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for Platform {
    fn deref_mut(&mut self) -> &mut Self::Target {
        Arc::make_mut(&mut self.inner)
    }
}
