// TODO: extract common super traits

macro_rules! for_each_mod {
    ({$(mod $sub_mod:ident $sub_mod_content:tt)*} $commands:tt) => {
        $(crate::expand! {
            {mod $sub_mod $sub_mod_content}
            do $commands
        })*
    };
}

macro_rules! define_node_type_traits {
    (
        $(#[super_traits($($super_trait:ident :: $super_trait_element:ident),+ $(,)?)])?
        mod $node_trait:ident {
            impl $node_type_trait:ident $(for ($($tags:ident),* $(,)?))? {}
            $($sub_mods:tt)*
        }
    ) => {
        pub trait $node_type_trait
            : HasIntrinsicComponentTag + CreateElement
            $($(+ $super_trait)+)? {
            type $node_trait<R: ?Sized + crate::RenderHtml>: crate::renderer::node_behaviors::$node_trait<R>
                $($(+ crate::Identity<This = <Self as $super_trait>::$super_trait_element<R>>)+)?
            ;

            fn from_identity_mut_element<R: ?Sized + crate::RenderHtml>(element: &mut Self::Element<R>) -> &mut Self::$node_trait<R>;
        }

        for_each_mod! {{$($sub_mods)*}{
            prepend( #[super_traits($($($super_trait::$super_trait_element,)+)? $node_type_trait::$node_trait)] )
            wrap {}
            prepend( define_node_type_traits! )
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
            fn $tags(&mut self) -> Self::$tags;
        )*)?

        for_each_mod! {{$($sub_mods)*}{
            wrap {}
            prepend( define_nodes_types! )
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

        impl crate::renderer::CreateElement for $tag {
            fn create_element<R: ?Sized + crate::RenderHtml>(renderer: &mut R) -> crate::ElementOfType<Self, R> {
                renderer.$tag()
            }
        }

        $(
            impl super::$super_trait for $tag {
                type $super_trait_element<R: ?Sized + crate::RenderHtml> = R::$tag;

                #[inline(always)]
                fn from_identity_mut_element<R: ?Sized + crate::RenderHtml>(element: &mut Self::Element<R>) -> &mut Self::$super_trait_element<R> {
                    element
                }
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

        for_each_mod! {{$($sub_mods)*}{
            prepend( #[super_traits($($($super_trait::$super_trait_element,)+)? $node_type_trait::$node_trait)] )
            wrap {}
            prepend( mod_element_types! )
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

        for_each_mod! {{$($sub_mods)*}{
            prepend( #[super_traits($($($super_trait::$super_trait_element,)+)? $node_type_trait::$node_trait)] )
            wrap {}
            prepend( element_type_traits! )
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
        #[alias($element_type_traits:ident)]
        $vis_element_types:vis mod $element_types:ident {
            $($element_types_defs:tt)*
        }
        $vis_render_html:vis trait $render_html:ident
            : Bounds([$($bounds:tt)+] $(,)?) {
                $($render_html_items:tt)*
        }
    ) => {
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
    #[alias(element_type_traits)]
    pub mod element_types {
        mod Element {
            impl ElementType {}
            mod HtmlElement {
                impl HtmlElementType
                    for (
                        abbr,
                        address,
                        article,
                        aside,
                        b,
                        bdi,
                        bdo,
                        cite,
                        code,
                        datalist,
                        dd,
                        dfn,
                        div,
                        dl,
                        dt,
                        em,
                        figcaption,
                        figure,
                        footer,
                        h1,
                        h2,
                        h3,
                        h4,
                        h5,
                        h6,
                        head,
                        header,
                        hgroup,
                        hr,
                        i,
                        kbd,
                        legend,
                        main,
                        mark,
                        menu,
                        nav,
                        noscript,
                        p,
                        picture,
                        pre,
                        rp,
                        rt,
                        ruby,
                        s,
                        samp,
                        section,
                        small,
                        span,
                        strong,
                        sub,
                        summary,
                        sup,
                        template,
                        title,
                        u,
                        var,
                        wbr,
                    )
                {
                }

                mod HtmlElementWithHref {
                    // TODO: make this virtual
                    impl HtmlElementWithHrefType for () {}
                    mod HtmlAnchorElement {
                        impl HtmlAnchorElementType for (a,) {}
                    }
                    mod HtmlAreaElement {
                        impl HtmlAreaElementType for (area,) {}
                    }
                }

                mod HtmlMediaElement {
                    // TODO: make this virtual
                    impl HtmlMediaElementType for () {}

                    mod HtmlAudioElement {
                        impl HtmlAudioElementType for (audio,) {}
                    }
                    mod HtmlVideoElement {
                        impl HtmlVideoElementType for (video,) {}
                    }
                }

                mod HtmlBaseElement {
                    impl HtmlBaseElementType for (base,) {}
                }
                mod HtmlQuoteElement {
                    impl HtmlQuoteElementType for (blockquote, q) {}
                }
                mod HtmlBodyElement {
                    impl HtmlBodyElementType for (body,) {}
                }
                mod HtmlBrElement {
                    impl HtmlBrElementType for (br,) {}
                }
                mod HtmlButtonElement {
                    impl HtmlButtonElementType for (button,) {}
                }
                mod HtmlCanvasElement {
                    impl HtmlCanvasElementType for (canvas,) {}
                }
                mod HtmlTableCaptionElement {
                    impl HtmlTableCaptionElementType for (caption,) {}
                }
                mod HtmlDataElement {
                    impl HtmlDataElementType for (data,) {}
                }
                mod HtmlModElement {
                    impl HtmlModElementType for (del, ins) {}
                }
                mod HtmlDetailsElement {
                    impl HtmlDetailsElementType for (details,) {}
                }
                mod HtmlDialogElement {
                    impl HtmlDialogElementType for (dialog,) {}
                }
                mod HtmlEmbedElement {
                    impl HtmlEmbedElementType for (embed,) {}
                }
                mod HtmlFieldSetElement {
                    impl HtmlFieldSetElementType for (fieldset,) {}
                }
                mod HtmlFormElement {
                    impl HtmlFormElementType for (form,) {}
                }
                mod HtmlHtmlElement {
                    impl HtmlHtmlElementType for (html,) {}
                }
                mod HtmlIFrameElement {
                    impl HtmlIFrameElementType for (iframe,) {}
                }
                mod HtmlImageElement {
                    impl HtmlImageElementType for (img,) {}
                }
                mod HtmlInputElement {
                    impl HtmlInputElementType for (input,) {}
                }
                mod HtmlLabelElement {
                    impl HtmlLabelElementType for (label,) {}
                }
                mod HtmlLiElement {
                    impl HtmlLiElementType for (li,) {}
                }
                mod HtmlLinkElement {
                    impl HtmlLinkElementType for (link,) {}
                }
                mod HtmlMapElement {
                    impl HtmlMapElementType for (map,) {}
                }
                mod HtmlMetaElement {
                    impl HtmlMetaElementType for (meta,) {}
                }
                mod HtmlMeterElement {
                    impl HtmlMeterElementType for (meter,) {}
                }
                mod HtmlObjectElement {
                    impl HtmlObjectElementType for (object,) {}
                }
                mod HtmlOListElement {
                    impl HtmlOListElementType for (ol,) {}
                }
                mod HtmlOptGroupElement {
                    impl HtmlOptGroupElementType for (optgroup,) {}
                }
                mod HtmlOptionElement {
                    impl HtmlOptionElementType for (option,) {}
                }
                mod HtmlOutputElement {
                    impl HtmlOutputElementType for (output,) {}
                }
                mod HtmlProgressElement {
                    impl HtmlProgressElementType for (progress,) {}
                }
                mod HtmlScriptElement {
                    impl HtmlScriptElementType for (script,) {}
                }
                mod HtmlSelectElement {
                    impl HtmlSelectElementType for (select,) {}
                }
                mod HtmlSlotElement {
                    impl HtmlSlotElementType for (slot,) {}
                }
                mod HtmlSourceElement {
                    impl HtmlSourceElementType for (source,) {}
                }
                mod HtmlStyleElement {
                    impl HtmlStyleElementType for (style,) {}
                }
                mod HtmlTableElement {
                    impl HtmlTableElementType for (table,) {}
                }
                mod HtmlTableSectionElement {
                    impl HtmlTableSectionElementType for (tbody, tfoot, thead) {}
                }
                mod HtmlTableRowElement {
                    impl HtmlTableRowElementType for (tr,) {}
                }
                mod HtmlTableColElement {
                    impl HtmlTableColElementType for (col, colgroup) {}
                }
                mod HtmlTableCellElement {
                    impl HtmlTableCellElementType for (td, th) {}
                }
                mod HtmlTextAreaElement {
                    impl HtmlTextAreaElementType for (textarea,) {}
                }
                mod HtmlTimeElement {
                    impl HtmlTimeElementType for (time,) {}
                }
                mod HtmlTrackElement {
                    impl HtmlTrackElementType for (track,) {}
                }
                mod HtmlUListElement {
                    impl HtmlUListElementType for (ul,) {}
                }
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
             + RenderTextFrom<Self::Text, char>],
    )
    {
        #![allow(non_camel_case_types)]

        type Text: node_behaviors::Node<Self>;

        fn into_render_element<S: crate::RenderState<Self> + Default>(self) -> crate::RenderElement<Self, S>
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

pub trait CreateElement {
    fn create_element<R: RenderHtml>(renderer: &mut R) -> Self::Element<R>
    where
        Self: ElementType;
}

pub trait RenderTextFrom<Text, V: ?Sized> {
    /// should not move cursor
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
    use frender_html_common::dom_token::DomTokenList;

    pub use crate::event_types::{ElementWithEvents, HtmlElementWithEvents};

    use crate::{
        event_types::{HtmlFormElementWithEvents, HtmlMediaElementWithEvents},
        RenderHtml,
    };

    pub trait Node<R: ?Sized> {
        fn cursor_is_at_self(&self, renderer: &R) -> bool;

        fn move_cursor_after_self(&mut self, renderer: &mut R);

        /// should move cursor
        fn readd_self(&mut self, renderer: &mut R, force_reposition: bool);

        fn remove_self(&mut self, renderer: &mut R);
    }

    pub trait Element<R: ?Sized>: Node<R> + ElementWithEvents<R> {
        fn move_cursor_at_the_first_child_of_self(&mut self, renderer: &mut R);

        fn set_attribute(&mut self, renderer: &mut R, name: &str, value: &str);
        fn remove_attribute(&mut self, renderer: &mut R, name: &str);

        type ClassList<'a>: DomTokenList
        where
            Self: 'a,
            R: 'a;
        fn class_list<'a>(&'a mut self, renderer: &'a mut R) -> Self::ClassList<'a>;

        fn set_id(&mut self, renderer: &mut R, id: &str);
    }

    #[cfg(aaa)]
    pub trait ElementWithEvents<R: ?Sized> {
        type OnCancelEvent: crate::event::Event;
        type OnCancelEventListener;

        fn on_cancel(&mut self, renderer: &mut R, listener: impl FnMut(&Self::OnCancelEvent) + 'static) -> Self::OnCancelEventListener;
    }

    pub trait HtmlElement<R: ?Sized>: Element<R> + HtmlElementWithEvents<R> {
        fn set_access_key(&mut self, renderer: &mut R, value: &str);
        fn set_content_editable(&mut self, renderer: &mut R, value: &str);
        fn set_dir(&mut self, renderer: &mut R, value: &str);
        fn set_draggable(&mut self, renderer: &mut R, value: bool);
        fn set_hidden(&mut self, renderer: &mut R, value: bool);
        fn set_lang(&mut self, renderer: &mut R, value: &str);
        fn set_spellcheck(&mut self, renderer: &mut R, value: bool);
        fn set_tab_index(&mut self, renderer: &mut R, value: i32);
        fn set_title(&mut self, renderer: &mut R, value: &str);
    }

    pub trait HtmlElementWithHref<R: ?Sized>: HtmlElement<R> {
        fn set_download(&mut self, renderer: &mut R, value: &str);
        fn set_href(&mut self, renderer: &mut R, value: &str);
        fn set_ping(&mut self, renderer: &mut R, value: &str);
        fn set_referrer_policy(&mut self, renderer: &mut R, value: &str);

        type RelList<'a>: DomTokenList
        where
            Self: 'a,
            R: 'a;
        fn rel_list<'a>(&'a mut self, renderer: &'a mut R) -> Self::RelList<'a>;

        fn set_target(&mut self, renderer: &mut R, value: &str);
    }

    pub trait HtmlAnchorElement<R: ?Sized>: HtmlElementWithHref<R> {
        fn set_href_lang(&mut self, renderer: &mut R, value: &str);
        fn set_type(&mut self, renderer: &mut R, value: &str);
    }

    pub trait HtmlAreaElement<R: ?Sized>: HtmlElementWithHref<R> {
        fn set_alt(&mut self, renderer: &mut R, value: &str);
        fn set_coords(&mut self, renderer: &mut R, value: &str);
        fn set_shape(&mut self, renderer: &mut R, value: &str);
    }

    pub trait HtmlMediaElement<R: ?Sized>: HtmlElement<R> + HtmlMediaElementWithEvents<R> {
        fn set_auto_play(&mut self, renderer: &mut R, value: bool);
        fn set_controls(&mut self, renderer: &mut R, value: bool);
        fn set_cross_origin(&mut self, renderer: &mut R, value: Option<&str>);
        fn set_loop(&mut self, renderer: &mut R, value: bool);
        fn set_muted(&mut self, renderer: &mut R, value: bool);
        fn set_preload(&mut self, renderer: &mut R, value: &str);
        fn set_src(&mut self, renderer: &mut R, value: &str);
    }

    pub trait HtmlAudioElement<R: ?Sized>: HtmlMediaElement<R> {}
    pub trait HtmlVideoElement<R: ?Sized>: HtmlMediaElement<R> {
        fn set_height(&mut self, renderer: &mut R, value: u32);
        fn set_poster(&mut self, renderer: &mut R, value: &str);
        fn set_width(&mut self, renderer: &mut R, value: u32);
    }

    pub trait HtmlBaseElement<R: ?Sized>: HtmlElement<R> {
        fn set_href(&mut self, renderer: &mut R, value: &str);
        fn set_target(&mut self, renderer: &mut R, value: &str);
    }
    pub trait HtmlQuoteElement<R: ?Sized>: HtmlElement<R> {
        fn set_cite(&mut self, renderer: &mut R, value: &str);
    }
    pub trait HtmlBodyElement<R: ?Sized>: HtmlElement<R> {}
    pub trait HtmlBrElement<R: ?Sized>: HtmlElement<R> {
        #[deprecated]
        fn set_clear(&mut self, renderer: &mut R, value: &str);
    }
    pub trait HtmlButtonElement<R: ?Sized>: HtmlElement<R> {
        fn set_disabled(&mut self, renderer: &mut R, value: bool);
        fn set_form_action(&mut self, renderer: &mut R, value: &str);
        fn set_form_enctype(&mut self, renderer: &mut R, value: &str);
        fn set_form_method(&mut self, renderer: &mut R, value: &str);
        fn set_form_no_validate(&mut self, renderer: &mut R, value: bool);
        fn set_form_target(&mut self, renderer: &mut R, value: &str);
        fn set_name(&mut self, renderer: &mut R, value: &str);
        fn set_type(&mut self, renderer: &mut R, value: &str);
        fn set_value(&mut self, renderer: &mut R, value: &str);
    }

    pub trait HtmlCanvasElement<R: ?Sized>: HtmlElement<R> {
        fn set_height(&mut self, renderer: &mut R, value: u32);
        fn set_width(&mut self, renderer: &mut R, value: u32);
    }
    pub trait HtmlTableCaptionElement<R: ?Sized>: HtmlElement<R> {
        #[deprecated]
        fn set_align(&mut self, renderer: &mut R, value: &str);
    }
    pub trait HtmlDataElement<R: ?Sized>: HtmlElement<R> {
        fn set_value(&mut self, renderer: &mut R, value: &str);
    }
    pub trait HtmlModElement<R: ?Sized>: HtmlElement<R> {
        fn set_cite(&mut self, renderer: &mut R, value: &str);
        fn set_date_time(&mut self, renderer: &mut R, value: &str);
    }

    pub trait HtmlElementWithOpen<R: ?Sized>: HtmlElement<R> {
        fn set_open(&mut self, renderer: &mut R, value: bool);
    }

    pub trait HtmlDetailsElement<R: ?Sized>: HtmlElement<R> + HtmlElementWithOpen<R> {}
    pub trait HtmlDialogElement<R: ?Sized>: HtmlElement<R> + HtmlElementWithOpen<R> {}
    pub trait HtmlEmbedElement<R: ?Sized>: HtmlElement<R> {
        fn set_height(&mut self, renderer: &mut R, value: &str);
        fn set_src(&mut self, renderer: &mut R, value: &str);
        fn set_type(&mut self, renderer: &mut R, value: &str);
        fn set_width(&mut self, renderer: &mut R, value: &str);
    }
    pub trait HtmlFieldSetElement<R: ?Sized>: HtmlElement<R> {
        fn set_disabled(&mut self, renderer: &mut R, value: bool);
        fn set_name(&mut self, renderer: &mut R, value: &str);
    }
    pub trait HtmlFormElement<R: ?Sized>: HtmlElement<R> + HtmlFormElementWithEvents<R> {
        fn set_accept_charset(&mut self, renderer: &mut R, value: &str);
        fn set_auto_complete(&mut self, renderer: &mut R, value: &str);
        fn set_name(&mut self, renderer: &mut R, value: &str);
        fn set_action(&mut self, renderer: &mut R, value: &str);
        fn set_enctype(&mut self, renderer: &mut R, value: &str);
        fn set_method(&mut self, renderer: &mut R, value: &str);
        fn set_no_validate(&mut self, renderer: &mut R, value: bool);
        fn set_target(&mut self, renderer: &mut R, value: &str);
    }
    pub trait HtmlHtmlElement<R: ?Sized>: HtmlElement<R> {}
    pub trait HtmlIFrameElement<R: ?Sized>: HtmlElement<R> {
        fn set_allow_fullscreen(&mut self, renderer: &mut R, value: bool);
        fn set_allow_payment_request(&mut self, renderer: &mut R, value: bool);
        fn set_height(&mut self, renderer: &mut R, value: &str);
        fn set_name(&mut self, renderer: &mut R, value: &str);
        fn set_referrer_policy(&mut self, renderer: &mut R, value: &str);
        fn set_src(&mut self, renderer: &mut R, value: &str);
        fn set_srcdoc(&mut self, renderer: &mut R, value: &str);
        fn set_width(&mut self, renderer: &mut R, value: &str);
    }
    pub trait HtmlImageElement<R: ?Sized>: HtmlElement<R> {
        fn set_alt(&mut self, renderer: &mut R, value: &str);
        fn set_cross_origin(&mut self, renderer: &mut R, value: Option<&str>);
        fn set_decoding(&mut self, renderer: &mut R, value: &str);
        fn set_height(&mut self, renderer: &mut R, value: u32);
        fn set_is_map(&mut self, renderer: &mut R, value: bool);
        fn set_referrer_policy(&mut self, renderer: &mut R, value: &str);
        fn set_sizes(&mut self, renderer: &mut R, value: &str);
        fn set_src(&mut self, renderer: &mut R, value: &str);
        fn set_srcset(&mut self, renderer: &mut R, value: &str);
        fn set_width(&mut self, renderer: &mut R, value: u32);
        fn set_use_map(&mut self, renderer: &mut R, value: &str);
    }
    pub trait HtmlInputElement<R: ?Sized>: HtmlElement<R> {
        fn set_accept(&mut self, renderer: &mut R, value: &str);
        fn set_alt(&mut self, renderer: &mut R, value: &str);
        fn set_auto_complete(&mut self, renderer: &mut R, value: &str);
        fn set_checked(&mut self, renderer: &mut R, value: bool);
        fn set_disabled(&mut self, renderer: &mut R, value: bool);
        fn set_form_action(&mut self, renderer: &mut R, value: &str);
        fn set_form_enctype(&mut self, renderer: &mut R, value: &str);
        fn set_form_method(&mut self, renderer: &mut R, value: &str);
        fn set_form_no_validate(&mut self, renderer: &mut R, value: bool);
        fn set_form_target(&mut self, renderer: &mut R, value: &str);
        fn set_height(&mut self, renderer: &mut R, value: u32);
        fn set_max(&mut self, renderer: &mut R, value: &str);
        fn set_max_length(&mut self, renderer: &mut R, value: i32);
        fn set_min(&mut self, renderer: &mut R, value: &str);
        fn set_min_length(&mut self, renderer: &mut R, value: i32);
        fn set_multiple(&mut self, renderer: &mut R, value: bool);
        fn set_name(&mut self, renderer: &mut R, value: &str);
        fn set_pattern(&mut self, renderer: &mut R, value: &str);
        fn set_placeholder(&mut self, renderer: &mut R, value: &str);
        fn set_read_only(&mut self, renderer: &mut R, value: bool);
        fn set_required(&mut self, renderer: &mut R, value: bool);
        fn set_size(&mut self, renderer: &mut R, value: u32);
        fn set_src(&mut self, renderer: &mut R, value: &str);
        fn set_step(&mut self, renderer: &mut R, value: &str);
        fn set_type(&mut self, renderer: &mut R, value: &str);
        fn set_value(&mut self, renderer: &mut R, value: &str);
        fn set_width(&mut self, renderer: &mut R, value: u32);
    }
    pub trait HtmlLabelElement<R: ?Sized>: HtmlElement<R> {
        fn set_html_for(&mut self, renderer: &mut R, value: &str);
    }
    pub trait HtmlLiElement<R: ?Sized>: HtmlElement<R> {
        fn set_html_for(&mut self, renderer: &mut R, value: i32);
    }
    pub trait HtmlLinkElement<R: ?Sized>: HtmlElement<R> {
        fn set_as(&mut self, renderer: &mut R, value: &str);
        fn set_href(&mut self, renderer: &mut R, value: &str);
        fn set_hreflang(&mut self, renderer: &mut R, value: &str);
        fn set_integrity(&mut self, renderer: &mut R, value: &str);
        fn set_media(&mut self, renderer: &mut R, value: &str);
        fn set_referrer_policy(&mut self, renderer: &mut R, value: &str);
        fn set_type(&mut self, renderer: &mut R, value: &str);
    }
    pub trait HtmlMapElement<R: ?Sized>: HtmlElement<R> {
        fn set_name(&mut self, renderer: &mut R, value: &str);
    }
    pub trait HtmlMetaElement<R: ?Sized>: HtmlElement<R> {
        fn set_content(&mut self, renderer: &mut R, value: &str);
        fn set_http_equiv(&mut self, renderer: &mut R, value: &str);
        fn set_name(&mut self, renderer: &mut R, value: &str);
    }
    pub trait HtmlMeterElement<R: ?Sized>: HtmlElement<R> {
        fn set_value(&mut self, renderer: &mut R, value: f64);
        fn set_min(&mut self, renderer: &mut R, value: f64);
        fn set_max(&mut self, renderer: &mut R, value: f64);
        fn set_low(&mut self, renderer: &mut R, value: f64);
        fn set_high(&mut self, renderer: &mut R, value: f64);
        fn set_optimum(&mut self, renderer: &mut R, value: f64);
    }
    pub trait HtmlObjectElement<R: ?Sized>: HtmlElement<R> {
        fn set_data(&mut self, renderer: &mut R, value: &str);
        fn set_height(&mut self, renderer: &mut R, value: &str);
        fn set_name(&mut self, renderer: &mut R, value: &str);
        fn set_type(&mut self, renderer: &mut R, value: &str);
        fn set_use_map(&mut self, renderer: &mut R, value: &str);
        fn set_width(&mut self, renderer: &mut R, value: &str);
    }
    pub trait HtmlOListElement<R: ?Sized>: HtmlElement<R> {
        fn set_reversed(&mut self, renderer: &mut R, value: bool);
        fn set_start(&mut self, renderer: &mut R, value: i32);
        fn set_type(&mut self, renderer: &mut R, value: &str);
    }
    pub trait HtmlOptGroupElement<R: ?Sized>: HtmlElement<R> {
        fn set_disabled(&mut self, renderer: &mut R, value: bool);
        fn set_label(&mut self, renderer: &mut R, value: &str);
    }
    pub trait HtmlOptionElement<R: ?Sized>: HtmlElement<R> {
        fn set_disabled(&mut self, renderer: &mut R, value: bool);
        fn set_label(&mut self, renderer: &mut R, value: &str);
        fn set_selected(&mut self, renderer: &mut R, value: bool);
        fn set_value(&mut self, renderer: &mut R, value: &str);
    }
    pub trait HtmlOutputElement<R: ?Sized>: HtmlElement<R> {
        fn set_name(&mut self, renderer: &mut R, value: &str);
    }
    pub trait HtmlProgressElement<R: ?Sized>: HtmlElement<R> {
        fn set_max(&mut self, renderer: &mut R, value: f64);
        fn set_value(&mut self, renderer: &mut R, value: f64);
    }
    pub trait HtmlScriptElement<R: ?Sized>: HtmlElement<R> {
        fn set_async(&mut self, renderer: &mut R, value: bool);
        fn set_defer(&mut self, renderer: &mut R, value: bool);
        fn set_integrity(&mut self, renderer: &mut R, value: &str);
        fn set_no_module(&mut self, renderer: &mut R, value: bool);
        fn set_src(&mut self, renderer: &mut R, value: &str);
        fn set_type(&mut self, renderer: &mut R, value: &str);
    }
    pub trait HtmlSelectElement<R: ?Sized>: HtmlElement<R> {
        fn set_auto_complete(&mut self, renderer: &mut R, value: &str);
        fn set_disabled(&mut self, renderer: &mut R, value: bool);
        fn set_multiple(&mut self, renderer: &mut R, value: bool);
        fn set_name(&mut self, renderer: &mut R, value: &str);
        fn set_required(&mut self, renderer: &mut R, value: bool);
        fn set_size(&mut self, renderer: &mut R, value: u32);
    }
    pub trait HtmlSlotElement<R: ?Sized>: HtmlElement<R> {
        fn set_name(&mut self, renderer: &mut R, value: &str);
    }
    pub trait HtmlSourceElement<R: ?Sized>: HtmlElement<R> {
        fn set_type(&mut self, renderer: &mut R, value: &str);
        fn set_src(&mut self, renderer: &mut R, value: &str);
        fn set_srcset(&mut self, renderer: &mut R, value: &str);
        fn set_sizes(&mut self, renderer: &mut R, value: &str);
        fn set_media(&mut self, renderer: &mut R, value: &str);
    }
    pub trait HtmlStyleElement<R: ?Sized>: HtmlElement<R> {
        fn set_media(&mut self, renderer: &mut R, value: &str);
        fn set_type(&mut self, renderer: &mut R, value: &str);
    }
    pub trait HtmlTableElement<R: ?Sized>: HtmlElement<R> {
        #[deprecated]
        fn set_align(&mut self, renderer: &mut R, value: &str);
        #[deprecated]
        fn set_bg_color(&mut self, renderer: &mut R, value: &str);
        #[deprecated]
        fn set_border(&mut self, renderer: &mut R, value: &str);
        #[deprecated]
        fn set_cell_padding(&mut self, renderer: &mut R, value: &str);
        #[deprecated]
        fn set_cell_spacing(&mut self, renderer: &mut R, value: &str);
        #[deprecated]
        fn set_frame(&mut self, renderer: &mut R, value: &str);
        #[deprecated]
        fn set_rules(&mut self, renderer: &mut R, value: &str);
        #[deprecated]
        fn set_summary(&mut self, renderer: &mut R, value: &str);
        #[deprecated]
        fn set_width(&mut self, renderer: &mut R, value: &str);
    }
    pub trait HtmlTableChildElement<R: ?Sized>: HtmlElement<R> {
        #[deprecated]
        fn set_align(&mut self, renderer: &mut R, value: &str);
        #[deprecated]
        fn set_ch(&mut self, renderer: &mut R, value: &str);
        #[deprecated]
        fn set_ch_off(&mut self, renderer: &mut R, value: &str);
        #[deprecated]
        fn set_v_align(&mut self, renderer: &mut R, value: &str);
    }
    pub trait HtmlTableSectionElement<R: ?Sized>: HtmlElement<R> + HtmlTableChildElement<R> {}
    pub trait HtmlTableRowElement<R: ?Sized>: HtmlElement<R> + HtmlTableChildElement<R> {}
    pub trait HtmlTableColElement<R: ?Sized>: HtmlElement<R> + HtmlTableChildElement<R> {
        fn set_span(&mut self, renderer: &mut R, value: u32);
        #[deprecated]
        fn set_width(&mut self, renderer: &mut R, value: &str);
    }
    pub trait HtmlTableCellElement<R: ?Sized>: HtmlElement<R> {
        fn set_col_span(&mut self, renderer: &mut R, value: u32);
        fn set_headers(&mut self, renderer: &mut R, value: &str);
        fn set_row_span(&mut self, renderer: &mut R, value: u32);
        #[deprecated]
        fn set_axis(&mut self, renderer: &mut R, value: &str);
        #[deprecated]
        fn set_height(&mut self, renderer: &mut R, value: &str);
        #[deprecated]
        fn set_width(&mut self, renderer: &mut R, value: &str);
    }
    pub trait HtmlTextAreaElement<R: ?Sized>: HtmlElement<R> {
        fn set_auto_complete(&mut self, renderer: &mut R, value: &str);
        fn set_cols(&mut self, renderer: &mut R, value: u32);
        fn set_disabled(&mut self, renderer: &mut R, value: bool);
        fn set_max_length(&mut self, renderer: &mut R, value: i32);
        fn set_min_length(&mut self, renderer: &mut R, value: i32);
        fn set_name(&mut self, renderer: &mut R, value: &str);
        fn set_placeholder(&mut self, renderer: &mut R, value: &str);
        fn set_read_only(&mut self, renderer: &mut R, value: bool);
        fn set_required(&mut self, renderer: &mut R, value: bool);
        fn set_rows(&mut self, renderer: &mut R, value: u32);
        fn set_wrap(&mut self, renderer: &mut R, value: &str);
    }
    pub trait HtmlTimeElement<R: ?Sized>: HtmlElement<R> {
        fn set_date_time(&mut self, renderer: &mut R, value: &str);
    }
    pub trait HtmlTrackElement<R: ?Sized>: HtmlElement<R> {
        fn set_default(&mut self, renderer: &mut R, value: bool);
        fn set_kind(&mut self, renderer: &mut R, value: &str);
        fn set_label(&mut self, renderer: &mut R, value: &str);
        fn set_src(&mut self, renderer: &mut R, value: &str);
        fn set_src_lang(&mut self, renderer: &mut R, value: &str);
    }
    pub trait HtmlUListElement<R: ?Sized>: HtmlElement<R> {
        fn set_compact(&mut self, renderer: &mut R, value: bool);
        fn set_type(&mut self, renderer: &mut R, value: &str);
    }
}
