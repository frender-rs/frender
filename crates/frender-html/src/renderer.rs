macro_rules! expand {
    (if (                 ) { $($expand:tt)* }) => {};
    (if ($($predicate:tt)+) { $($expand:tt)* }) => { $($expand)* };
}

macro_rules! define_node_type_traits {
    (
        $(#[super_trait($super_trait:ident :: $super_trait_element:ident)])?
        mod $node_trait:ident {
            impl $node_type_trait:ident $(for ($($tags:ident),* $(,)?))? {}
            $($sub_mods:tt)*
        }
    ) => {
        pub trait $node_type_trait
            : HasIntrinsicComponentTag
            $(+ $super_trait)? {
            type $node_trait<R: ?Sized + crate::RenderHtml>: crate::renderer::node_behaviors::$node_trait<R>
                $(+ crate::Identity<This = <Self as $super_trait>::$super_trait_element<R>>)?
            ;
        }

        expand! { if ($($sub_mods)*) {
            define_node_type_traits! {
                #[super_trait($node_type_trait::$node_trait)]
                $($sub_mods)*
            }
        }}
    };
    (
        $(#[super_traits($($super_trait:ident :: $super_trait_element:ident),+ $(,)?)])?
        mod $node_trait:ident {
            impl $node_type_trait:ident $(for ($($tags:ident),* $(,)?))? {}
            $($sub_mods:tt)*
        }
    ) => {
        pub trait $node_type_trait
        $(: $($super_trait)++)? {
            type $node_trait<R: crate::RenderHtml>: crate::renderer::node_behaviors::$node_trait<R>
                $($(+ crate::Identity<This = <Self as $super_trait>::$super_trait_element<R>>)+)?
            ;
        }

        expand! { if ($($sub_mods)*) {
            define_node_type_traits! {
                #[super_traits($($($super_trait::$super_trait_element,)+)? $node_type_trait::$node_trait)]
                $($sub_mods)*
            }
        }}
    };
}

macro_rules! define_nodes_types {
    (
        mod $node_trait:ident {
            impl $node_type_trait:ident $(for ($($tags:ident),* $(,)?))? {}
            $($sub_mods:tt)*
        }
    ) => {
        $($(
            type $tags: crate::renderer::node_behaviors::$node_trait<Self>;
        )*)?

        expand! { if ($($sub_mods)*) {
            define_nodes_types! {
                $($sub_mods)*
            }
        }}
    };
}

macro_rules! define_element_type {
    (($($tags:ident)*) $super_traits:tt ) => {
        $(define_element_type! { $tags $super_traits })*
    };
    ($tag:ident [$($super_trait:ident :: $super_trait_element:ident),+ $(,)?]) => {
        pub enum $tag {}

        impl crate::renderer::HasIntrinsicComponentTag for $tag {
            const INTRINSIC_COMPONENT_TAG: &'static str = stringify!($tag);
        }

        $(
            impl super::$super_trait for $tag {
                type $super_trait_element<R: ?Sized + crate::RenderHtml> = R::$tag;
            }
        )+

    };
}

macro_rules! mod_element_types {
    (
        $(#[super_traits($($super_trait:ident :: $super_trait_element:ident),+ $(,)?)])?
        mod $node_trait:ident {
            impl $node_type_trait:ident $(for ($($tags:ident),* $(,)?))? {}
            $($sub_mods:tt)*
        }
    ) => {
        define_element_type! {
            ($($($tags)*)?)
            [$($($super_trait::$super_trait_element,)+)? $node_type_trait::$node_trait]
        }

        expand! { if ($($sub_mods)*) {
            mod_element_types! {
                #[super_traits($($($super_trait::$super_trait_element,)+)? $node_type_trait::$node_trait)]
                $($sub_mods)*
            }
        }}
    };
}

macro_rules! element_type_traits {
    (
        $(#[super_traits($($super_trait:ident :: $super_trait_element:ident),+ $(,)?)])?
        mod $node_trait:ident {
            impl $node_type_trait:ident $(for ($($tags:ident),* $(,)?))? {}
            $($sub_mods:tt)*
        }
    ) => {
        pub use super::$node_type_trait as $node_trait;

        expand! { if ($($sub_mods)*) {
            element_type_traits! {
                #[super_traits($($($super_trait::$super_trait_element,)+)? $node_type_trait::$node_trait)]
                $($sub_mods)*
            }
        }}
    };
}

macro_rules! trait_create_element {
    (
        [$vis:vis trait $trait_name:ident]
        {}
        [ $($components:ident)* ]
    ) => {
        $vis trait $trait_name: $(CreateElementOfType<element_types::$components> +)* {}
        impl<T: ?Sized $(+ CreateElementOfType<element_types::$components>)*> $trait_name for T {}
    };
    (
        $sig:tt {
            $(#[super_traits($($super_trait:ident :: $super_trait_element:ident),+ $(,)?)])?
            mod $node_trait:ident {
                impl $node_type_trait:ident $(for ($($tags:ident),* $(,)?))? {}
                $($sub_mods:tt)*
            }
        } [ $($components:ident)* ]
    ) => {
        trait_create_element! {
            $sig
            {
                $($sub_mods)*
            }
            [ $($components)* $($($tags)*)? ]
        }
    };
}

macro_rules! define {
    (
        $vis_create_element:vis trait $trait_create_element:ident {
        }

        #[alias($element_type_traits:ident)]
        $vis_element_types:vis mod $element_types:ident {
            $($element_types_defs:tt)*
        }
        $vis_render_html:vis trait $render_html:ident
            : Bounds([$($bounds:tt)+] $(,)?) {
                $($render_html_items:tt)*
        }
    ) => {
        trait_create_element! {
            [$vis_create_element trait $trait_create_element]
            {$($element_types_defs)*}
            []
        }
        define_node_type_traits! { $($element_types_defs)* }
        $vis_element_types mod $element_types {
            mod_element_types! { $($element_types_defs)* }
        }

        $vis_element_types mod $element_type_traits {
            element_type_traits! { $($element_types_defs)* }
        }

        $vis_render_html trait $render_html
            : $($bounds)+ {
                $($render_html_items)*

            define_nodes_types! { $($element_types_defs)* }
        }
    };
}

#[cfg(remove)]
pub mod element_types {
    use crate::{ElementType, HtmlElementType};

    pub enum div {}

    impl ElementType for div {
        type Element<Renderer: crate::RenderHtml> = Renderer::div;
    }

    impl HtmlElementType for div {
        type HtmlElement<Renderer: crate::RenderHtml> = Renderer::div;
    }
}

define!(
    pub trait CreateElement {}

    #[alias(element_type_traits)]
    pub mod element_types {
        mod Element {
            impl ElementType {}
            mod HtmlElement {
                impl HtmlElementType for (div,) {}
            }
        }
    }

    pub trait RenderHtml:
        Bounds(
        [RenderTextFrom<Self::Text, str>
             + RenderTextFrom<Self::Text, i8>
             + RenderTextFrom<Self::Text, u8>
             + RenderTextFrom<Self::Text, i16>
             + RenderTextFrom<Self::Text, u16>
             + RenderTextFrom<Self::Text, i32>
             + RenderTextFrom<Self::Text, u32>
             + RenderTextFrom<Self::Text, i64>
             + RenderTextFrom<Self::Text, u64>
             + RenderTextFrom<Self::Text, i128>
             + RenderTextFrom<Self::Text, u128>
             + RenderTextFrom<Self::Text, isize>
             + RenderTextFrom<Self::Text, usize>
             + RenderTextFrom<Self::Text, f32>
             + RenderTextFrom<Self::Text, f64>
             + RenderTextFrom<Self::Text, bool>
             + RenderTextFrom<Self::Text, char>
             + CreateElement],
    )
    {
        #![allow(non_camel_case_types)]

        type EventListenerId;
        type Event: node_behaviors::Event;

        type Text: node_behaviors::Node<Self>;

        fn into_render_element<S: crate::RenderState<Self> + Default>(
            self,
        ) -> crate::RenderElement<Self, S>
        where
            Self: Sized,
        {
            crate::RenderElement::new(self)
        }
    }
);

pub trait HasIntrinsicComponentTag {
    const INTRINSIC_COMPONENT_TAG: &'static str;
}

pub trait CreateElementOfType<ET: ElementType> {
    fn create_element_of_type(&mut self) -> ET::Element<Self>
    where
        Self: RenderHtml;
}

pub trait RenderTextFrom<Text, V: ?Sized> {
    // let text = dom_ctx.document.create_text_node(&data);
    // dom_ctx
    //     .next_node_position
    //     .add_node(Cow::Owned(text.clone().into()));
    fn render_text_from(&mut self, v: &V) -> Text;
    fn update_text_from(&mut self, text: &mut Text, v: &V);
}

#[cfg(remove)]
pub trait RenderHtml:
    RenderTextFrom<Self::Text, str>
    + RenderTextFrom<Self::Text, i8>
    + RenderTextFrom<Self::Text, u8>
    + RenderTextFrom<Self::Text, i16>
    + RenderTextFrom<Self::Text, u16>
    + RenderTextFrom<Self::Text, i32>
    + RenderTextFrom<Self::Text, u32>
    + RenderTextFrom<Self::Text, i64>
    + RenderTextFrom<Self::Text, u64>
    + RenderTextFrom<Self::Text, i128>
    + RenderTextFrom<Self::Text, u128>
    + RenderTextFrom<Self::Text, isize>
    + RenderTextFrom<Self::Text, usize>
    + RenderTextFrom<Self::Text, f32>
    + RenderTextFrom<Self::Text, f64>
    + RenderTextFrom<Self::Text, bool>
    + RenderTextFrom<Self::Text, char>
{
    #![allow(non_camel_case_types)]

    type EventListenerId;
    type Event: node_behaviors::Event;

    type Text: node_behaviors::Node<Self>;

    type div: node_behaviors::HtmlElement<Self>;
    // type button;
}

pub mod node_behaviors {
    use crate::RenderHtml;

    /// See [DOMTokenList](https://developer.mozilla.org/en-US/docs/Web/API/DOMTokenList).
    pub trait DomTokenList<R: ?Sized> {
        fn set_value(&mut self, renderer: &mut R, value: &str);
        fn add_1(&mut self, renderer: &mut R, token: &str);
        fn remove_1(&mut self, renderer: &mut R, token: &str);
        fn replace(&mut self, renderer: &mut R, old_token: &str, new_token: &str);
    }

    pub trait UpdateDomTokenList {
        fn update_dom_token_list<R: ?Sized>(
            self,
            renderer: &mut R,
            dom_token_list: &mut impl DomTokenList<R>,
        );
    }

    pub trait Node<R: ?Sized> {
        fn cursor_is_at_self(&self, renderer: &R) -> bool;

        fn move_cursor_after_self(&mut self, renderer: &mut R);

        /// should move cursor
        fn readd_self(&mut self, renderer: &mut R, force_reposition: bool);

        fn remove_self(&mut self, renderer: &mut R);
    }

    pub trait Element<R: ?Sized>: Node<R> {
        fn move_cursor_at_the_first_child_of_self(&mut self, renderer: &mut R);

        fn set_attribute(&mut self, renderer: &mut R, name: &str, value: &str);
        fn remove_attribute(&mut self, renderer: &mut R, name: &str);

        fn update_class_list(&mut self, renderer: &mut R, updater: impl UpdateDomTokenList);
        fn set_id(&mut self, renderer: &mut R, id: &str);
    }

    pub trait Event {
        fn type_(&self) -> String;
        fn event_phase(&self) -> u16;
        fn bubbles(&self) -> bool;
        fn cancelable(&self) -> bool;
        fn default_prevented(&self) -> bool;
        fn composed(&self) -> bool;
        fn is_trusted(&self) -> bool;
        fn time_stamp(&self) -> f64;
        fn cancel_bubble(&self) -> bool;

        fn set_cancel_bubble(&mut self, value: bool);
        fn prevent_default(&mut self);
        fn stop_immediate_propagation(&mut self);
        fn stop_propagation(&mut self);
    }

    pub trait ElementWithEvents<R: ?Sized + RenderHtml> {
        fn on_cancel(
            &mut self,
            renderer: &mut R,
            listener: impl Fn(&R::Event),
        ) -> R::EventListenerId;
    }

    pub trait HtmlElement<R: ?Sized>: Element<R> {
        fn set_access_key(&mut self, renderer: &mut R, value: &str);
        fn set_content_editable(&mut self, renderer: &mut R, value: &str);
    }
    pub trait HtmlElementWithEvents<R: ?Sized + RenderHtml> {
        fn on_invalid(
            &mut self,
            renderer: &mut R,
            listener: impl Fn(&R::Event),
        ) -> R::EventListenerId;
    }
}
