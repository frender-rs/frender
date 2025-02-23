use frender_common::utils::pin_project_iter_mut_array;

use super::RenderInitPinned;

impl<
        T: for<'r> RenderInitPinned<&'r mut R, S, Output = Out>,
        const N: usize,
        R: ?Sized,
        S,
        Out,
    > RenderInitPinned<&mut R, [S; N]> for [T; N]
{
    type Output = [Out; N];

    fn render_init_pinned(
        self,
        renderer: &mut R,
        state: std::pin::Pin<&mut [S; N]>,
    ) -> Self::Output {
        let mut state = pin_project_iter_mut_array(state);
        // This relies on a documented feature of <[_; N]>::map():
        // > ..., with function f applied to each element in order
        self.map(|this| this.render_init_pinned(renderer, state.next().unwrap()))
    }
}
