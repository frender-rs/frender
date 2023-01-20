pub trait UpdateHtmlElementEventListeners {
    type State;
    fn update_dom_event_listeners(self, element: &web_sys::HtmlElement, state: &mut Self::State);
}
