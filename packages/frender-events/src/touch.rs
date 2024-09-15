use std::borrow::Cow;

pub struct TouchListWeb {
    #[cfg(not(feature = "web"))]
    inner: core::convert::Infallible,
    #[cfg(feature = "web")]
    inner: web_sys::TouchList,
}

#[cfg(feature = "web")]
impl From<web_sys::TouchList> for TouchListWeb {
    fn from(value: web_sys::TouchList) -> Self {
        Self { inner: value }
    }
}

#[cfg(feature = "web")]
impl Into<web_sys::TouchList> for TouchListWeb {
    fn into(self) -> web_sys::TouchList {
        self.inner
    }
}

#[cfg(feature = "web")]
impl std::ops::Deref for TouchListWeb {
    type Target = web_sys::TouchList;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

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
