use std::borrow::Cow;

/// [`web_sys::Touch`]
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct Touch {
    pub identifier: i32,
    pub screen_x: i32,
    pub screen_y: i32,
    pub client_x: i32,
    pub client_y: i32,
    pub page_x: i32,
    pub page_y: i32,
    pub radius_x: i32,
    pub radius_y: i32,
    pub rotation_angle: f32,
    pub force: f32,
}

impl Touch {
    #[cfg(feature = "web")]
    pub fn from_web(touch: web_sys::Touch) -> Self {
        Self {
            identifier: touch.identifier(),
            screen_x: touch.screen_x(),
            screen_y: touch.screen_y(),
            client_x: touch.client_x(),
            client_y: touch.client_y(),
            page_x: touch.page_x(),
            page_y: touch.page_y(),
            radius_x: touch.radius_x(),
            radius_y: touch.radius_y(),
            rotation_angle: touch.rotation_angle(),
            force: touch.force(),
        }
    }
}

pub trait TouchList {
    fn touches_count(&self) -> usize;
    fn get_touch(&self, index: usize) -> Option<Cow<'_, Touch>>;

    fn into_vec_of_touches(self) -> Vec<Touch>;
}

impl TouchList for Vec<Touch> {
    fn touches_count(&self) -> usize {
        self.len()
    }

    fn get_touch(&self, index: usize) -> Option<Cow<'_, Touch>> {
        <[_]>::get(self, index).map(Cow::Borrowed)
    }

    fn into_vec_of_touches(self) -> Vec<Touch> {
        self
    }
}

#[cfg(feature = "web")]
impl TouchList for web_sys::TouchList {
    fn touches_count(&self) -> usize {
        self.length() as usize
    }

    fn get_touch(&self, index: usize) -> Option<Cow<'_, Touch>> {
        self.item(index as _)
            .map(|touch| Cow::Owned(Touch::from_web(touch)))
    }

    fn into_vec_of_touches(self) -> Vec<Touch> {
        (0..self.length())
            .map(|index| Touch::from_web(self.item(index).unwrap()))
            .collect()
    }
}

// TODO: this is now not used
#[cfg(feature = "stack_dst")]
pub mod generic {
    /// [`web_sys::Touch`]
    pub trait Touch {
        fn identifier(&self) -> i32;
        //  fn target(&self) -> Option<EventTarget>; // TODO:
        fn screen_x(&self) -> i32;
        fn screen_y(&self) -> i32;
        fn client_x(&self) -> i32;
        fn client_y(&self) -> i32;
        fn page_x(&self) -> i32;
        fn page_y(&self) -> i32;
        fn radius_x(&self) -> i32;
        fn radius_y(&self) -> i32;
        fn rotation_angle(&self) -> f32;
        fn force(&self) -> f32;
    }

    /// [`web_sys::TouchList`]
    pub trait TouchList {
        type Touch<'a>: Touch
        where
            Self: 'a;

        fn length(&self) -> u32;
        fn item(&self, index: u32) -> Option<Self::Touch<'_>>;
    }

    #[cfg(feature = "web")]
    mod web {
        use super::{Touch, TouchList};

        impl Touch for web_sys::Touch {
            fn identifier(&self) -> i32 {
                web_sys::Touch::identifier(self)
            }
            fn screen_x(&self) -> i32 {
                web_sys::Touch::screen_x(self)
            }
            fn screen_y(&self) -> i32 {
                web_sys::Touch::screen_y(self)
            }
            fn client_x(&self) -> i32 {
                web_sys::Touch::client_x(self)
            }
            fn client_y(&self) -> i32 {
                web_sys::Touch::client_y(self)
            }
            fn page_x(&self) -> i32 {
                web_sys::Touch::page_x(self)
            }
            fn page_y(&self) -> i32 {
                web_sys::Touch::page_y(self)
            }
            fn radius_x(&self) -> i32 {
                web_sys::Touch::radius_x(self)
            }
            fn radius_y(&self) -> i32 {
                web_sys::Touch::radius_y(self)
            }
            fn rotation_angle(&self) -> f32 {
                web_sys::Touch::rotation_angle(self)
            }
            fn force(&self) -> f32 {
                web_sys::Touch::force(self)
            }
        }

        impl TouchList for web_sys::TouchList {
            type Touch<'a> = web_sys::Touch;

            fn length(&self) -> u32 {
                self.length()
            }

            fn item(&self, index: u32) -> Option<Self::Touch<'_>> {
                self.item(index)
            }
        }
    }
}

// TODO: this is now not used
#[cfg(feature = "stack_dst")]
pub mod stack_dst {
    pub struct Touch<'a>(stack_dst::Value<dyn 'a + super::Touch, ::stack_dst::buffers::Ptr2>);

    impl<'a> From<web_sys::Touch> for Touch<'a> {
        fn from(val: web_sys::Touch) -> Self {
            Self(stack_dst::Value::new_stable(val, |v| v as _))
        }
    }

    impl<'a, T: ?Sized + super::Touch> From<&'a T> for Touch<'a> {
        fn from(val: web_sys::Touch) -> Self {
            Self(stack_dst::Value::new_stable(val, |v| v as _))
        }
    }
}
