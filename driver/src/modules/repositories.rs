use shaku::module;
use std::sync::Arc;

module! {
    pub RepositoryModule {
        components = [],
        providers = []
    }
}

pub fn repositories() -> Arc<RepositoryModule> {
    Arc::new(RepositoryModule::builder().build())
}
