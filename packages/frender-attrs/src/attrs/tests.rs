use std::task::Poll;

use crate::{
    csr::{CsrAttributes, RenderAttributes},
    ssr::SsrAttributes,
    values::{
        r#const::{ConstAttributes, HasConstAttributes},
        EitherAttributes, Empty,
    },
};

use crate::attrs::one;

fn csr_ssr_const<T: ?Sized + HasConstAttributes>(
    v: ConstAttributes<T>,
) -> (Vec<(String, String)>, String) {
    use async_str_iter::AsyncStrIterator;

    let ssr = v.into_ssr_attributes();
    let mut ssr = std::pin::pin!(ssr);
    let cx = &mut std::task::Context::from_waker(std::task::Waker::noop());
    let ssr_str = match ssr.as_mut().poll_next_str(cx) {
        Poll::Ready(v) => v.unwrap_or(""),
        Poll::Pending => panic!(),
    };

    let ssr_str = ssr_str.to_string();

    match ssr.poll_next_str(cx) {
        Poll::Ready(None) => {}
        _ => panic!(),
    }

    let csr = {
        struct RenderInit(Vec<(String, String)>);

        impl RenderAttributes for RenderInit {
            fn set_attribute(&mut self, name: &str, value: &str) {
                self.0.push((name.into(), value.into()));
            }

            fn remove_attribute(&mut self, _: &str) {
                panic!()
            }
        }

        let mut renderer = RenderInit(Vec::new());
        let mut state = v.render_init(&mut renderer);

        struct RenderUpdate;

        impl RenderAttributes for RenderUpdate {
            fn set_attribute(&mut self, _: &str, _: &str) {
                panic!()
            }

            fn remove_attribute(&mut self, _: &str) {
                panic!()
            }
        }
        v.render_update(&mut RenderUpdate, &mut state);

        renderer.0
    };

    (csr, ssr_str)
}

#[test]
fn match_clause() {
    let f = || one!(match (panic!()) {});

    let _ = f as fn() -> crate::values::Never;

    let Empty = one!(match (()) {
        _ => (),
    });

    {
        let (csr, ssr) = csr_ssr_const(one!(match (()) {
            _ => "",
        }));
        assert_eq!(csr, []);
        assert_eq!(ssr, "");
    }

    match one!(match (true) {
        a if a => "",
        _ => attrs!(),
    }) {
        EitherAttributes::A(ConstAttributes { .. }) => {}
        EitherAttributes::B(Empty) => unreachable!(),
    }

    match one!(match (1) {
        // empty (think unit tuple `()` as an empty list)
        a if a > 0 => (),
        b if b < 0 => {
            // empty wrapped in a block
            ()
        }
        // empty wrapped in parenthesis
        _ => (()),
    }) {
        EitherAttributes::A(Empty) => {}
        EitherAttributes::B(other) => match other {
            EitherAttributes::A(Empty) => panic!(),
            EitherAttributes::B(Empty) => panic!(),
        },
    }
}
