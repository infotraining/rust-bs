use corelib::Bus;

pub fn register_builtin_plugins(bus: &mut Bus) {
    #[cfg(feature = "plugin_a")]
    {
        let h = plugin_a::make();
        bus.register(h);
    }
    #[cfg(feature = "plugin_b")]
    {
        let h = plugin_b::make();
        bus.register(h);
    }
}
