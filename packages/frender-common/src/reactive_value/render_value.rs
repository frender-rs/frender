use super::ReactiveValueKind;

/// This trait is sealed to make sure [`RenderValueMut`] has the same behavior as [`RenderValueOnce`].
pub trait RenderValueOnce<VK: ?Sized + ReactiveValueKind>: sealed::RenderValueOnce<VK> {
    type RenderOutput;
    fn render_value_once_update(self, value: VK::Value<'_>) -> Self::RenderOutput;
    fn render_value_once_remove(self) -> Self::RenderOutput;
}

pub trait RenderValueMut<VK: ?Sized + ReactiveValueKind>: RenderValueOnce<VK> {
    fn render_value_mut_update(&mut self, value: VK::Value<'_>) -> Self::RenderOutput;
    fn render_value_mut_remove(&mut self) -> Self::RenderOutput;
}

#[derive(Debug)]
pub struct RenderValueWithFn<Update, Remove> {
    pub update: Update,
    pub remove: Remove,
}

#[derive(Debug)]
pub struct RenderValueWithFnOnceAndData<Update, Remove, Data> {
    pub update: Update,
    pub remove: Remove,
    pub data: Data,
}

#[derive(Debug)]
pub struct RenderValueWithFnMutAndData<Update, Remove, Data: ?Sized> {
    pub update: Update,
    pub remove: Remove,
    pub data: Data,
}

// region: &mut _
impl<T: ?Sized + RenderValueMut<VK>, VK: ?Sized + ReactiveValueKind> sealed::RenderValueOnce<VK>
    for &mut T
{
}

impl<T: ?Sized + RenderValueMut<VK>, VK: ?Sized + ReactiveValueKind> RenderValueOnce<VK>
    for &mut T
{
    type RenderOutput = T::RenderOutput;

    fn render_value_once_update(
        self,
        value: <VK as ReactiveValueKind>::Value<'_>,
    ) -> Self::RenderOutput {
        T::render_value_mut_update(self, value)
    }

    fn render_value_once_remove(self) -> Self::RenderOutput {
        T::render_value_mut_remove(self)
    }
}

impl<T: ?Sized + RenderValueMut<VK>, VK: ?Sized + ReactiveValueKind> RenderValueMut<VK> for &mut T {
    fn render_value_mut_update(
        &mut self,
        value: <VK as ReactiveValueKind>::Value<'_>,
    ) -> Self::RenderOutput {
        T::render_value_mut_update(self, value)
    }

    fn render_value_mut_remove(&mut self) -> Self::RenderOutput {
        T::render_value_mut_remove(self)
    }
}
// endregion
// region: RenderValueWithFn
impl<
        Update: FnOnce(VK::Value<'_>) -> Out,
        Remove: FnOnce() -> Out,
        Out,
        VK: ?Sized + ReactiveValueKind,
    > sealed::RenderValueOnce<VK> for RenderValueWithFn<Update, Remove>
{
}

impl<
        Update: FnOnce(VK::Value<'_>) -> Out,
        Remove: FnOnce() -> Out,
        Out,
        VK: ?Sized + ReactiveValueKind,
    > RenderValueOnce<VK> for RenderValueWithFn<Update, Remove>
{
    type RenderOutput = Out;

    fn render_value_once_update(
        self,
        value: <VK as ReactiveValueKind>::Value<'_>,
    ) -> Self::RenderOutput {
        (self.update)(value)
    }

    fn render_value_once_remove(self) -> Self::RenderOutput {
        (self.remove)()
    }
}

impl<
        Update: FnMut(VK::Value<'_>) -> Out,
        Remove: FnMut() -> Out,
        Out,
        VK: ?Sized + ReactiveValueKind,
    > RenderValueMut<VK> for RenderValueWithFn<Update, Remove>
{
    fn render_value_mut_update(
        &mut self,
        value: <VK as ReactiveValueKind>::Value<'_>,
    ) -> Self::RenderOutput {
        (self.update)(value)
    }

    fn render_value_mut_remove(&mut self) -> Self::RenderOutput {
        (self.remove)()
    }
}
// endregion
// region: RenderValueWithFnOnceAndData
impl<
        Update: FnOnce(Data, VK::Value<'_>) -> Out,
        Remove: FnOnce(Data) -> Out,
        Data,
        Out,
        VK: ?Sized + ReactiveValueKind,
    > sealed::RenderValueOnce<VK> for RenderValueWithFnOnceAndData<Update, Remove, Data>
{
}

impl<
        Data,
        Update: FnOnce(Data, VK::Value<'_>) -> Out,
        Remove: FnOnce(Data) -> Out,
        Out,
        VK: ?Sized + ReactiveValueKind,
    > RenderValueOnce<VK> for RenderValueWithFnOnceAndData<Update, Remove, Data>
{
    type RenderOutput = Out;

    fn render_value_once_update(
        self,
        value: <VK as ReactiveValueKind>::Value<'_>,
    ) -> Self::RenderOutput {
        (self.update)(self.data, value)
    }

    fn render_value_once_remove(self) -> Self::RenderOutput {
        (self.remove)(self.data)
    }
}
// endregion
// region: RenderValueWithFnMutAndData
impl<
        Update: FnOnce(&mut Data, VK::Value<'_>) -> Out,
        Remove: FnOnce(&mut Data) -> Out,
        Data: ?Sized,
        Out,
        VK: ?Sized + ReactiveValueKind,
    > sealed::RenderValueOnce<VK> for RenderValueWithFnMutAndData<Update, Remove, Data>
{
}

impl<
        Update: FnOnce(&mut Data, VK::Value<'_>) -> Out,
        Remove: FnOnce(&mut Data) -> Out,
        Data,
        Out,
        VK: ?Sized + ReactiveValueKind,
    > RenderValueOnce<VK> for RenderValueWithFnMutAndData<Update, Remove, Data>
{
    type RenderOutput = Out;

    fn render_value_once_update(
        mut self,
        value: <VK as ReactiveValueKind>::Value<'_>,
    ) -> Self::RenderOutput {
        (self.update)(&mut self.data, value)
    }

    fn render_value_once_remove(mut self) -> Self::RenderOutput {
        (self.remove)(&mut self.data)
    }
}

impl<
        Update: FnMut(&mut Data, VK::Value<'_>) -> Out,
        Remove: FnMut(&mut Data) -> Out,
        Data,
        Out,
        VK: ?Sized + ReactiveValueKind,
    > RenderValueMut<VK> for RenderValueWithFnMutAndData<Update, Remove, Data>
{
    fn render_value_mut_update(
        &mut self,
        value: <VK as ReactiveValueKind>::Value<'_>,
    ) -> Self::RenderOutput {
        (self.update)(&mut self.data, value)
    }

    fn render_value_mut_remove(&mut self) -> Self::RenderOutput {
        (self.remove)(&mut self.data)
    }
}
// endregion
mod sealed {
    use super::ReactiveValueKind;

    pub trait RenderValueOnce<VK: ?Sized + ReactiveValueKind> {}
}
