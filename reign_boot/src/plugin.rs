use crate::boot::Reign;

use reign_plugin::Plugin;

impl Reign {
    pub fn add_plugin<P>(mut self, plugin: P) -> Self
    where
        P: Plugin + 'static,
    {
        // Initialize the plugin
        plugin.init();

        self.plugins.push(Box::new(plugin));
        self
    }
}
