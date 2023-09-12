mod event {
    impl crate::renderer::node_behaviors::Event for web_sys::Event {
        fn type_(&self) -> String {
            web_sys::Event::type_(&self)
        }

        fn event_phase(&self) -> u16 {
            web_sys::Event::event_phase(&self)
        }

        fn bubbles(&self) -> bool {
            web_sys::Event::bubbles(&self)
        }

        fn cancelable(&self) -> bool {
            web_sys::Event::cancelable(&self)
        }

        fn default_prevented(&self) -> bool {
            web_sys::Event::default_prevented(&self)
        }

        fn composed(&self) -> bool {
            web_sys::Event::composed(&self)
        }

        fn is_trusted(&self) -> bool {
            web_sys::Event::is_trusted(&self)
        }

        fn time_stamp(&self) -> f64 {
            web_sys::Event::time_stamp(&self)
        }

        fn cancel_bubble(&self) -> bool {
            web_sys::Event::cancel_bubble(&self)
        }

        fn set_cancel_bubble(&mut self, value: bool) {
            web_sys::Event::set_cancel_bubble(self, value)
        }

        fn prevent_default(&mut self) {
            web_sys::Event::prevent_default(self)
        }

        fn stop_immediate_propagation(&mut self) {
            web_sys::Event::stop_immediate_propagation(self)
        }

        fn stop_propagation(&mut self) {
            web_sys::Event::stop_propagation(self)
        }
    }
}
