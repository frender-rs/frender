use frender_common::{Elements, Keyed};

use crate::{AsyncStrIterator, Element, IntoAsyncStrIterator, RenderState};

pin_project_lite::pin_project!(
    pub struct State<S> {
        #[pin]
        states: Vec<S>,
        current: Option<usize>,
    }
);

impl<S: RenderState> RenderState for State<S> {
    fn poll_render<W: futures_io::AsyncWrite + ?Sized>(
        self: std::pin::Pin<&mut Self>,
        mut writer: std::pin::Pin<&mut W>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        let mut this = self.project();

        while let Some(current) = this.current {
            let state =
                frender_common::utils::pin_project_index_vec(this.states.as_mut(), *current);
            crate::ready_ok!(state.poll_render(writer.as_mut(), cx));

            if *current == this.states.len() {
                *this.current = None;
            } else {
                *current += 1;
            }
        }

        std::task::Poll::Ready(Ok(()))
    }
}

impl<K, E> Element for Vec<Keyed<K, E>>
where
    K: std::hash::Hash + Eq, // TODO: ToString ?
    E: Element,
{
    type SsrState = State<E::SsrState>;

    fn into_ssr_state(self) -> Self::SsrState {
        Elements(self).into_ssr_state()
    }

    type IntoAsyncHtmlChunks = crate::str_iter::Flat<std::vec::IntoIter<Keyed<K, E>>>;

    fn into_async_html_chunks(self) -> Self::IntoAsyncHtmlChunks {
        crate::str_iter::Flat::new(self.into_iter())
    }
}

impl<K, E> IntoAsyncStrIterator for Keyed<K, E>
where
    K: std::hash::Hash + Eq, // TODO: ToString ?
    E: Element,
{
    type IntoAsyncStrIterator = E::IntoAsyncHtmlChunks;

    fn into_async_str_iterator(self) -> Self::IntoAsyncStrIterator {
        self.1.into_async_html_chunks()
    }
}

impl<K, E, A, I: IntoIterator<Item = Keyed<K, E>>> Element for Elements<I, A>
where
    K: std::hash::Hash + Eq, // TODO: ToString ?
    E: Element,
{
    type SsrState = State<E::SsrState>;

    fn into_ssr_state(self) -> Self::SsrState {
        let states: Vec<_> = self
            .iter
            .into_iter()
            .map(|Keyed(_, e)| e.into_ssr_state())
            .collect();
        State {
            current: if states.is_empty() {
                None
            } else {
                Some(states.len())
            },
            states,
        }
    }

    type IntoAsyncHtmlChunks = crate::str_iter::Flat<I::IntoIter>;

    fn into_async_html_chunks(self) -> Self::IntoAsyncHtmlChunks {
        crate::str_iter::Flat::new(self.iter.into_iter())
    }
}
