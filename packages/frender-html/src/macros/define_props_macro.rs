macro_rules! run_command_unbraced {
    (
        $Props:ident
        $command:tt
        $commands:tt
    ) => {
        $Props! {
            $command {
                wrap {}
                append {
                    then $commands
                }
                wrap {} prepend { crate::macros::unwrap_brace_concat! }
            }
        }
    };
}

macro_rules! check_is_ancestor_of_any {
    (
        {}
        $check_is_ancestor:ident
        $expand_if_yes:tt
        $expand_if_no:tt
    ) => {
        ::frender_common::expand! $expand_if_no
    };
    (
        {$any:ident $($any_rest:ident)*}
        $check_is_ancestor:ident
        $expand_if_yes:tt
        $expand_if_no:tt
    ) => {
        $any! {
            check_is_ancestor $check_is_ancestor
            $expand_if_yes
            {{
                crate::macros::define_props_macro::check_is_ancestor_of_any! {
                    {$($any_rest)*}
                    $check_is_ancestor
                    $expand_if_yes
                    $expand_if_no
                }
            }}
        }
    };
}

macro_rules! define_unfinished {
    (
        ($dollar:tt)
        main_ancestors($($recorded_main_ancestors:tt)*)
        other_ancestors($($recorded_other_ancestors:tt)*)
        then $commands_after_define_unfinished:tt
    ) => {
        macro_rules! __not_finished_props {
            $(
                (record_main_ancestor $recorded_main_ancestors then $commands:tt else { $dollar($else_expand:tt)* }) => {
                    // does nothing
                    $dollar($else_expand)*
                };
                (record_other_ancestor $recorded_main_ancestors then $commands:tt else { $dollar($else_expand:tt)* }) => {
                    // does nothing
                    $dollar($else_expand)*
                };
            )*
            $(
                (record_main_ancestor $recorded_other_ancestors then $commands:tt else { $dollar($else_expand:tt)* }) => {
                    compile_error!{stringify!(
                        recorded other ancestor $recorded_other_ancestors
                        cannot be then recorded as main ancestor
                    )}
                };
                (record_other_ancestor $recorded_other_ancestors then $commands:tt else { $dollar($else_expand:tt)* }) => {
                    // does nothing
                    $dollar($else_expand)*
                };
            )*
            (record_main_ancestor $unrecorded_main_ancestor:tt then $commands:tt else { $dollar($else_expand:tt)* }) => {
                crate::macros::define_props_macro::define_unfinished! {
                    ($dollar)
                    main_ancestors($($recorded_main_ancestors)* $unrecorded_main_ancestor)
                    other_ancestors($($recorded_other_ancestors)*)
                    then $commands
                }
            };
            (record_other_ancestor $unrecorded_other_ancestor:tt then $commands:tt else { $dollar($else_expand:tt)* }) => {
                crate::macros::define_props_macro::define_unfinished! {
                    ($dollar)
                    main_ancestors($($recorded_main_ancestors)*)
                    other_ancestors($($recorded_other_ancestors)* $unrecorded_other_ancestor)
                    then $commands
                }
            };
            (
                finish_as
                $macro_name:tt
                {$dollar($other_branches:tt)*}
            ) => {
                macro_rules! $macro_name {
                    // check_is_ancestor
                    $(
                        (check_is_ancestor $recorded_main_ancestors $expand_if_is_ancestor:tt $expand_if_is_not_ancestor:tt) => {
                            ::frender_common::expand! $expand_if_is_ancestor
                        };
                    )*
                    $(
                        (check_is_ancestor $recorded_other_ancestors $expand_if_is_ancestor:tt $expand_if_is_not_ancestor:tt) => {
                            ::frender_common::expand! $expand_if_is_ancestor
                        };
                    )*
                    (check_is_ancestor $check_is_ancestor:ident $expand_if_is_ancestor:tt $expand_if_is_not_ancestor:tt) => {
                        ::frender_common::expand! $expand_if_is_not_ancestor
                    };

                    /* This is not used currently
                    (
                        check_is_ancestor_of_main_ancestors
                        $check_is_ancestor_of_main_ancestors:ident
                        $expand_if_yes:tt
                        $expand_if_no:tt
                    ) => {
                        crate::macros::define_props_macro::check_is_ancestor_of_any! {
                            {$($recorded_main_ancestors)*}
                            $check_is_ancestor_of_main_ancestors
                            $expand_if_yes
                            $expand_if_no
                        }
                    };
                    */

                    (for_all_main_ancestors $commands:tt) => {
                        ::frender_common::expand! {
                            {$({$recorded_main_ancestors})*} do $commands
                        }
                    };
                    (for_each_main_ancestor $commands:tt) => {
                        ::frender_common::expand! {
                            while ($({$recorded_main_ancestors})*) $commands
                        }
                    };
                    (for_all_other_ancestors $commands:tt) => {
                        ::frender_common::expand! {
                            {$({$recorded_other_ancestors})*} do $commands
                        }
                    };
                    (for_each_other_ancestor $commands:tt) => {
                        ::frender_common::expand! {
                            while ($({$recorded_other_ancestors})*) $commands
                        }
                    };
                    (for_all_ancestors $commands:tt) => {
                        ::frender_common::expand! {
                            {$({$recorded_main_ancestors})* $({$recorded_other_ancestors})*} do $commands
                        }
                    };
                    (for_each_ancestor $commands:tt) => {
                        ::frender_common::expand! {
                            while ($({$recorded_main_ancestors})* $({$recorded_other_ancestors})*) $commands
                        }
                    };
                    $dollar($other_branches)*
                }
            };
        }

        ::frender_common::expand! {
            {__not_finished_props} do $commands_after_define_unfinished
        }
    };
}

macro_rules! for_all_main_and_other_ancestors {
    // end
    (
        unrecorded_main()
        unrecorded_other()
        recorded_main $recorded_main:tt
        recorded_other $recorded_other:tt
        do $commands:tt
    ) => {
        ::frender_common::expand! {
            {
                main $recorded_main
                other $recorded_other
            } do $commands
        }
    };
    // main is done
    (
        unrecorded_main()
        unrecorded_other($other:tt $($others:tt)*)
        recorded_main $recorded_main:tt
        recorded_other($($recorded_others:tt)*)
        do $commands:tt
    ) => {
        crate::macros::define_props_macro::run_command_unbraced! {
            $other
            for_all_ancestors {
                append { $($others)* }
                wrap ()
                prepend {
                    unrecorded_main()
                    unrecorded_other
                }
                append {
                    recorded_main $recorded_main
                    recorded_other($($recorded_others)* $other)
                    do $commands
                }
                wrap {} prepend { crate::macros::define_props_macro::for_all_main_and_other_ancestors! }
            }
        }
    };
    // main is not done
    (
        unrecorded_main($main:tt $($mains:tt)*)
        unrecorded_other $unrecorded_other:tt
        recorded_main $recorded_main:tt
        recorded_other $recorded_other:tt
        do $commands:tt
    ) => {
        crate::macros::define_props_macro::run_command_unbraced! {
            $main
            for_all_main_ancestors {
                append { $($mains)* }
                wrap ()
                prepend {
                    recording_main($main)
                    unrecorded_main
                }
                append {
                    unrecorded_other $unrecorded_other
                    recorded_main $recorded_main
                    recorded_other $recorded_other
                    do $commands
                }
                wrap {} prepend { crate::macros::define_props_macro::for_all_main_and_other_ancestors! }
            }
        }
    };
    (
        recording_main($main:tt)
        unrecorded_main $unrecorded_main:tt
        unrecorded_other($($unrecorded_others:tt)*)
        recorded_main($($recorded_mains:tt)*)
        recorded_other $recorded_other:tt
        do $commands:tt
    ) => {
        crate::macros::define_props_macro::run_command_unbraced! {
            $main
            for_all_other_ancestors {
                append { $($unrecorded_others)* }
                wrap ()
                prepend {
                    unrecorded_main $unrecorded_main
                    unrecorded_other
                }
                append {
                    recorded_main($($recorded_mains)* $main)
                    recorded_other $recorded_other
                    do $commands
                }
                wrap {} prepend { crate::macros::define_props_macro::for_all_main_and_other_ancestors! }
            }
        }
    };
}

macro_rules! after_initial_define_unfinished {
    (
        $macro_name:ident
        unrecorded_main $main_ancestors:tt
        unrecorded_other $other_ancestors:tt
        finish_as $finish_as:tt
    ) => {
        crate::macros::define_props_macro::for_all_main_and_other_ancestors! {
            unrecorded_main $main_ancestors
            unrecorded_other $other_ancestors
            recorded_main()
            recorded_other()
            do {
                prepend {
                    $macro_name
                }
                append {
                    finish_as $finish_as
                }
                wrap {} prepend { crate::macros::define_props_macro::record! }
            }
        }
    };
}

macro_rules! record {
    // end
    (
        $macro_name:ident
        main()
        other()
        finish_as($($finish:tt)*)
    ) => {
        $macro_name! { finish_as $($finish)* }
    };
    // main is done
    (
        $macro_name:ident
        main()
        other($other:tt $($others:tt)*)
        finish_as $finish:tt
    ) => {
        $macro_name! {
            record_other_ancestor $other then {
                append {
                    main()
                    other($($others)*)
                    finish_as $finish
                }
                wrap {} prepend { crate::macros::define_props_macro::record! }
            } else {
                crate::macros::define_props_macro::record! {
                    $macro_name
                    main()
                    other($($others)*)
                    finish_as $finish
                }
            }
        }
    };
    // main is not done
    (
        $macro_name:ident
        main($main:tt $($mains:tt)*)
        other $other:tt
        finish_as $finish:tt
    ) => {
        $macro_name! {
            record_main_ancestor $main then {
                append {
                    main($($mains)*)
                    other $other
                    finish_as $finish
                }
                wrap {} prepend { crate::macros::define_props_macro::record! }
            } else {
                crate::macros::define_props_macro::record! {
                    $macro_name
                    main($($mains)*)
                    other $other
                    finish_as $finish
                }
            }
        }
    };
}

macro_rules! define {
    (
        $Props:ident
        main_ancestors $main_ancestors:tt
        other_ancestors $other_ancestors:tt
        $other_branches:tt
    ) => {
        crate::macros::define_props_macro::define_unfinished! {
            ($)
            main_ancestors $main_ancestors
            other_ancestors $other_ancestors
            then {
                append {
                    unrecorded_main $main_ancestors
                    unrecorded_other $other_ancestors
                    finish_as( $Props $other_branches )
                }
                wrap {}
                prepend { crate::macros::define_props_macro::after_initial_define_unfinished! }
            }
        }
    };
}

pub(crate) use {after_initial_define_unfinished, check_is_ancestor_of_any, define, define_unfinished, for_all_main_and_other_ancestors, record, run_command_unbraced};
