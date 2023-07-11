macro_rules! wrap_if_csr {
    (
        $(#$attr:tt)*
        $vis:vis struct $name:ident ($inner:ty);
    ) => {
        $(#$attr)*
        #[repr(transparent)]
        #[non_exhaustive]
        $vis struct $name {
            #[cfg(feature = "csr")]
            inner: $inner
        }

        #[cfg(feature = "csr")]
        impl std::ops::Deref for $name {
            type Target = $inner;

            fn deref(&self) -> &Self::Target {
                &self.inner
            }
        }

        #[cfg(feature = "csr")]
        impl std::ops::DerefMut for $name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.inner
            }
        }

        #[cfg(feature = "csr")]
        impl From<$inner> for $name {
            fn from(inner: $inner) -> Self {
                Self { inner }
            }
        }

        #[cfg(feature = "csr")]
        impl Into<$inner> for $name {
            fn into(self) -> $inner {
                self.inner
            }
        }
    };
}

pub(super) use wrap_if_csr;

macro_rules! wrap_events {
    ($($vis:vis struct $name:ident ($inner:ty);)*) => {$(
        crate::event::wrap_if_csr! { $vis struct $name ($inner); }

        impl ::callable::StatedEvent for $name {
            type State = crate::event::EventListener;
        }

        #[cfg(feature = "csr")]
        impl crate::NewFromRef for $name {
            fn new_from_ref(inner: &$inner) -> &Self {
                // SAFETY: Self is just a wrapper around the inner type,
                // therefore converting &Inner to &Self is safe.
                unsafe { &*(inner as *const $inner as *const Self) }
            }
        }
    )*};
}

pub(super) use wrap_events;

wrap_if_csr!(
    #[derive(Debug)]
    pub struct EventListener(gloo::events::EventListener);
);

#[cfg(feature = "csr")]
impl EventListener {
    pub fn new<
        E: ?Sized + NewFromRef,
        C: for<'e> callable::Callable<(&'e E,), Output = ()> + 'static,
    >(
        target: &web_sys::EventTarget,
        event_type: &'static str,
        callable: C,
    ) -> Self
    where
        E::Target: wasm_bindgen::JsCast,
    {
        Self::from(gloo::events::EventListener::new(
            target,
            event_type,
            move |event| {
                let event: &E::Target =
                    wasm_bindgen::JsCast::dyn_ref(event).expect("matched event type");
                let event: &E = E::new_from_ref(event);
                let _: () = callable::Callable::call_fn(&callable, (event,));
            },
        ))
    }
}

pub trait NewFromRef: std::ops::Deref {
    fn new_from_ref(inner: &Self::Target) -> &Self;
}

pub trait MaybeHandleEvent<E: ?Sized + callable::StatedEvent>:
    callable::MaybeHandleEvent<E, Callable = Self::StaticCloneCallable>
{
    type StaticCloneCallable: 'static + Clone + for<'e> callable::Callable<(&'e E,), Output = ()>;
}

impl<C, E: ?Sized + callable::StatedEvent> MaybeHandleEvent<E> for C
where
    C: callable::MaybeHandleEvent<E>,
    C::Callable: 'static + Clone + for<'e> callable::Callable<(&'e E,), Output = ()>,
{
    type StaticCloneCallable = C::Callable;
}
