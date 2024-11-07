use crate::conn::Cache;
use crate::session::ty::SessionCache;

impl Cache {
    pub fn session_cache(&self) -> SessionCache {
        SessionCache { cache: &self.cache }
    }
}
