use crate::{env::load_env_files, plugin::Plugin};

use env_logger::{from_env, Env};
use reign_router::{serve, Router};

use std::net::ToSocketAddrs;

#[derive(Default)]
pub struct Reign {
    pub(crate) plugins: Vec<Box<dyn Plugin>>,
}

// TODO: cli: tasks with feature
impl Reign {
    pub fn build() -> Self {
        load_env_files();

        // TODO: framework:log: Allow custom loggers by adding an option to exclude this call
        from_env(Env::default().default_filter_or("info"))
            .format_timestamp(None)
            .init();

        Self::default()
    }

    pub async fn serve<A, R>(self, addr: A, f: R)
    where
        A: ToSocketAddrs + Send + 'static,
        R: FnOnce(&mut Router) + 'static,
    {
        let mut router_fn: Box<dyn FnOnce(&mut Router)> = Box::new(f);

        for plugin in self.plugins {
            router_fn = plugin.router(router_fn);
        }

        serve(addr, router_fn).await.unwrap()
    }
}
