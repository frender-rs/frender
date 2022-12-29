macro_rules! def_event_types {
    (
        $(
            $event_enum:ident ($event_type_str:literal : $event_type:ty)
        ),* $(,)?
    ) => {
        $(
            pub enum $event_enum {}

            impl super::StaticEventType for $event_enum {
                const EVENT_TYPE: &'static str = $event_type_str;

                type Event = $event_type;
            }
        )*
    };
}

def_event_types! {
    OnClick("click" : web_sys::MouseEvent),
}
