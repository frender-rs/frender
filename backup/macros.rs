#[macro_export]
macro_rules! component {
    ( $ty_comp:ident { $($prop_name:ident $(= $prop_value:expr)?)* } $([ $($child:expr)* ])?  ) => {{
        type FrenderTempComponentTypeAlias = $crate::component_type!($ty_comp);
        <FrenderTempComponentTypeAlias as $crate::Component>::new_with_props(
            $crate::props! (
                for $ty_comp {
                    $($prop_name $(: $prop_value)?),*
                    $(, children: $crate::children!($($child)*) )?
                }
            )
        )
    }};
    ($(@)? $ty_comp:ty { $($prop_name:ident $(= $prop_value:expr)?)* } $([ $($child:expr)* ])?) => {
        <$ty_comp as $crate::Component>::new_with_props(
            $crate::props! (
                for $ty_comp {
                    $($prop_name $(: $prop_value)?),*
                    $(, children: $crate::children!($($child)*) )?
                }
            )
        )
    };
}

#[macro_export]
macro_rules! __impl_component_excluding_key_prop {
    ( $ty_comp:ident { $($prop_name:ident $(= $prop_value:expr)?)* } $([ $($child:expr),* $(,)? ])?  ) => {{
        type FrenderTempComponentTypeAlias = $crate::component_type!($ty_comp);
        <FrenderTempComponentTypeAlias as $crate::Component>::new_with_props(
            $crate::props! (
                for $ty_comp {
                    $($prop_name $(: $prop_value)?),*
                    $(, children: $crate::children!($($child),*) )?
                }@[-key]
            )
        )
    }};
    ($(@)? $ty_comp:ty { $($prop_name:ident $(= $prop_value:expr)?)* } $([ $($child:expr),* $(,)? ])?) => {
        <$ty_comp as $crate::Component>::new_with_props(
            $crate::props! (
                for $ty_comp {
                    $($prop_name $(: $prop_value)?),*
                    $(, children: $crate::children!($($child)*) )?
                }@[-key]
            )
        )
    };
}

#[macro_export]
macro_rules! __impl_node_prop_filter_key {
    (@[$v:ident] key) => {
        compiler_error!{"value of prop \"key\" must be specified"}
    };
    (@[$v:ident] key $(= $prop_value:expr)? ) => {
        let $v = Some($crate::AsKey::as_key($prop_value))
    };
    (@[$v:ident] $prop_name:ident $(= $prop_value:expr)? ) => {};
    ($($prop_name:ident $(= $prop_value:expr)?)* ) => {{
        let __frender_temp_value_key = None;
        $($crate::__impl_node_prop_filter_key!(
            @[__frender_temp_value_key]
            $prop_name $(= $prop_value)?
        );)*

        __frender_temp_value_key
    }};
}

#[macro_export]
macro_rules! node {
    ( $ty_comp:ident { $($prop_name:ident $(= $prop_value:expr)?)* } $([ $($child:expr),* $(,)? ])?  ) => {{
        let __frender_temp_component_instance =
            $crate::__impl_component_excluding_key_prop! (
                $ty_comp
                {$($prop_name $(= $prop_value)?)*}
                $([ $($child),* ])?
            );
        __frender_temp_component_instance.call_create_element(
            $crate::__impl_node_prop_filter_key! (
                $($prop_name $(= $prop_value)?)*
            )
        )
    }};
    ($(@)? $ty_comp:ty { $($prop_name:ident $(= $prop_value:expr)?)* } $([ $($child:expr),* $(,)? ])?) => {{
        let __frender_temp_component_instance =
            $crate::__impl_component_excluding_key_prop! (
                @ $ty_comp
                {($prop_name $(= $prop_value)?)*}
                $([ $($child),* ])?
            );

        __frender_temp_component_instance.call_create_element(
            $crate::__impl_node_prop_filter_key! (
                $($prop_name $(= $prop_value)?)*
            )
        )
    }};
}

#[macro_export]
macro_rules! __impl_rsx_rest {
    ($v:ident []) => {};
    ($v:ident [] $($rest:tt)+) => {
        compile_error! {
            concat! {
                "Unexpected tokens: ",
                stringify! ($($rest)+),
            }
        }
    };
    ($v:ident [ $($comp_stack:tt)+ ]) => {
        compile_error! { "rsx root element not enclosed" }
    };
    ($v:ident [ $($comp_stack:tt)+ ] $($lit:literal)+ $($rest:tt)*) => {
        $(
            let $v = $crate::__private::rsx::ReceiveNode::receive_node($v, $lit);
        )+
        $crate::__impl_rsx_rest!{ $v [$($comp_stack)+] $($rest:tt)* }
    };
    ($v:ident [ $($comp_stack:tt)+ ] $({$e:expr})+ $($rest:tt)*) => {
        $(
            let $v = $crate::__private::rsx::ReceiveNode::receive_node($e, $lit);
        )+
        $crate::__impl_rsx_rest!{ $v [$($comp_stack)+] $($rest:tt)* }
    };
    ($v:ident [ {$($comp:tt)*} $($comp_stack:tt)* ] </ $($close:ident)? > $($rest:tt)*) => {
        $crate::__private::frender_macros::assert_rsx_element_start_end_ident_match! {
            {$($comp)*}
            {$($close)?}
        }
        let $v = $crate::__private::rsx::EndElement::end_element($v, |b, c| b.children(c));
        $crate::__impl_rsx_rest!{ $v [$($comp_stack)*] $($rest:tt)* }
    };
    ($v:ident [ $($comp_stack:tt)+ ] <$comp:ident $($prop_name:ident $(=$prop_value:tt)? )* />) => {
        $(
            let $v = $crate::__private::rsx::ReceiveNode::receive_node(
                $v,
                $crate::node! {
                    $comp [$($prop_name $(=$prop_value)?)*]
                },
            );
        )+
        $crate::__impl_rsx_rest!{ $v [$($comp_stack)+] $($rest:tt)* }
    };
    ($v:ident [ $($comp_stack:tt)+ ] $(<$comp:ident $($prop_name:ident $(=$prop_value:tt)? )* ></ $($close:ident)?>)+ $($rest:tt)*) => {
        $(
            $crate::__private::frender_macros::assert_rsx_element_start_end_ident_match! {
                {$comp}
                {$($close)?}
            }
            let $v = $crate::__private::rsx::ReceiveNode::receive_node(
                $v,
                $crate::node! {
                    $comp [$($prop_name $(=$prop_value)?)*]
                },
            );
        )+
        $crate::__impl_rsx_rest!{ $v [$($comp_stack)+] $($rest:tt)* }
    };
    ($v:ident [ $($comp_stack:tt)+ ] <$comp:ident $($prop_name:ident $(=$prop_value:tt)? )*> $($rest:tt)*) => {
        let $v = {
            type FrenderTempComponentTypeAlias = $crate::component_type!($comp);
            $crate::__private::rsx::StartElement::start_element(
                $v,
                $crate::__private::rsx::ElementBuildingData::<FrenderTempComponentTypeAlias, _>::new(
                    $crate::props! { for FrenderTempComponentTypeAlias {
                        $($prop_name:ident $(: $prop_value:tt)? )*
                    }@[-key -build]},
                    $crate::__impl_node_prop_filter_key! (
                        $($prop_name $(= $prop_value)?)*
                    )
                ),
            )
        };
        $crate::__impl_rsx_rest!{$v [{ $comp } $($comp_stack)+] $($rest)*}
    };
}

#[macro_export]
macro_rules! rsx {
    ($lit:literal) => {
        $lit
    };
    ({ $e:expr }) => {
        $e
    };
    (<$comp:ident $($prop_name:ident $(=$prop_value:tt)? )* />) => {
        $crate::node! {
            $comp {$($prop_name $(=$prop_value)?)*}
        }
    };
    (<$comp:ident $($prop_name:ident $(=$prop_value:tt)? )* ></ $($close:ident)? >) => {{
        $crate::__private::frender_macros::assert_rsx_element_start_end_ident_match! {
            {$comp}
            {$($close)?}
        };
        $crate::node! {
            $comp {$($prop_name $(=$prop_value)?)*}
        }
    }};
    (<$comp:ident $($prop_name:ident $(=$prop_value:tt)? )*> $($rest:tt)*) => {{
        let __frender_rsx = $crate::__private::rsx::ExpectSingleNode;

        type FrenderTempComponentTypeAlias = $crate::component_type!($comp);
        let __frender_rsx = $crate::__private::rsx::StartElement::start_element(
            __frender_rsx,
            $crate::__private::rsx::ElementBuildingData::<FrenderTempComponentTypeAlias, _>::new(
                $crate::props! { for FrenderTempComponentTypeAlias {
                    $($prop_name $(: $prop_value)? ),*
                }@[-key -build]},
                $crate::__impl_node_prop_filter_key! (
                    $($prop_name $(= $prop_value)?)*
                )
            )
        );
        $crate::__impl_rsx_rest!{__frender_rsx [{ $comp }] $($rest)*}
        __frender_rsx
    }};
    ($($t:tt)*) => {
        compile_error!("rsx macro expects exactly one node")
    };
}
