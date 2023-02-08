#[allow(non_snake_case)]
pub fn HtmlInputElementProps() -> Building<TypesInitial> {
    #[allow(unused_imports)]
    use super::*;
    self::Building(self::Data {
        HtmlElementProps: HtmlElementProps::build(HtmlElementProps()),
        accept: (),
        alt: (),
        auto_complete: (),
        capture: (),
        checked: (),
        dirname: (),
        disabled: (),
        form: (),
        form_action: (),
        form_enc_type: (),
        form_method: (),
        form_no_validate: (),
        form_target: (),
        height: (),
        list: (),
        max: (),
        max_length: (),
        min: (),
        min_length: (),
        multiple: (),
        name: (),
        pattern: (),
        placeholder: (),
        read_only: (),
        required: (),
        size: (),
        src: (),
        step: (),
        type_: (),
        value: (),
        width: (),
    })
}
pub mod prelude {}
pub mod overwrite {
    #![allow(non_camel_case_types)]
    pub type HtmlElementProps<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = Value,
        accept = <TypeDefs as super::Types>::accept,
        alt = <TypeDefs as super::Types>::alt,
        auto_complete = <TypeDefs as super::Types>::auto_complete,
        capture = <TypeDefs as super::Types>::capture,
        checked = <TypeDefs as super::Types>::checked,
        dirname = <TypeDefs as super::Types>::dirname,
        disabled = <TypeDefs as super::Types>::disabled,
        form = <TypeDefs as super::Types>::form,
        form_action = <TypeDefs as super::Types>::form_action,
        form_enc_type = <TypeDefs as super::Types>::form_enc_type,
        form_method = <TypeDefs as super::Types>::form_method,
        form_no_validate = <TypeDefs as super::Types>::form_no_validate,
        form_target = <TypeDefs as super::Types>::form_target,
        height = <TypeDefs as super::Types>::height,
        list = <TypeDefs as super::Types>::list,
        max = <TypeDefs as super::Types>::max,
        max_length = <TypeDefs as super::Types>::max_length,
        min = <TypeDefs as super::Types>::min,
        min_length = <TypeDefs as super::Types>::min_length,
        multiple = <TypeDefs as super::Types>::multiple,
        name = <TypeDefs as super::Types>::name,
        pattern = <TypeDefs as super::Types>::pattern,
        placeholder = <TypeDefs as super::Types>::placeholder,
        read_only = <TypeDefs as super::Types>::read_only,
        required = <TypeDefs as super::Types>::required,
        size = <TypeDefs as super::Types>::size,
        src = <TypeDefs as super::Types>::src,
        step = <TypeDefs as super::Types>::step,
        type_ = <TypeDefs as super::Types>::type_,
        value = <TypeDefs as super::Types>::value,
        width = <TypeDefs as super::Types>::width,
    >;
    pub type ElementProps<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::ElementProps<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type children<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::children<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type class<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::class<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type id<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::id<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type part<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::part<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type access_key<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::access_key<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type auto_capitalize<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::auto_capitalize<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type auto_focus<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::auto_focus<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type content_editable<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::content_editable<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type context_menu<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::context_menu<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type dir<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::dir<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type draggable<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::draggable<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type enter_key_hint<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::enter_key_hint<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type hidden<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::hidden<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type inert<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::inert<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type input_mode<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::input_mode<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type is<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::is<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type item_id<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::item_id<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type item_prop<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::item_prop<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type item_ref<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::item_ref<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type item_scope<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::item_scope<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type item_type<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::item_type<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type lang<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::lang<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type nonce<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::nonce<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type role<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::role<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type slot<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::slot<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type spellcheck<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::spellcheck<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type style<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::style<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type tab_index<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::tab_index<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type title<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::title<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type translate<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::translate<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type virtual_keyboard_policy<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::virtual_keyboard_policy<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type on_click<TypeDefs, Value> = self::HtmlElementProps<
        TypeDefs,
        super::super::HtmlElementProps::overwrite::on_click<
            <TypeDefs as super::Types>::HtmlElementProps,
            Value,
        >,
    >;
    pub type accept<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        accept = Value,
        alt = <TypeDefs as super::Types>::alt,
        auto_complete = <TypeDefs as super::Types>::auto_complete,
        capture = <TypeDefs as super::Types>::capture,
        checked = <TypeDefs as super::Types>::checked,
        dirname = <TypeDefs as super::Types>::dirname,
        disabled = <TypeDefs as super::Types>::disabled,
        form = <TypeDefs as super::Types>::form,
        form_action = <TypeDefs as super::Types>::form_action,
        form_enc_type = <TypeDefs as super::Types>::form_enc_type,
        form_method = <TypeDefs as super::Types>::form_method,
        form_no_validate = <TypeDefs as super::Types>::form_no_validate,
        form_target = <TypeDefs as super::Types>::form_target,
        height = <TypeDefs as super::Types>::height,
        list = <TypeDefs as super::Types>::list,
        max = <TypeDefs as super::Types>::max,
        max_length = <TypeDefs as super::Types>::max_length,
        min = <TypeDefs as super::Types>::min,
        min_length = <TypeDefs as super::Types>::min_length,
        multiple = <TypeDefs as super::Types>::multiple,
        name = <TypeDefs as super::Types>::name,
        pattern = <TypeDefs as super::Types>::pattern,
        placeholder = <TypeDefs as super::Types>::placeholder,
        read_only = <TypeDefs as super::Types>::read_only,
        required = <TypeDefs as super::Types>::required,
        size = <TypeDefs as super::Types>::size,
        src = <TypeDefs as super::Types>::src,
        step = <TypeDefs as super::Types>::step,
        type_ = <TypeDefs as super::Types>::type_,
        value = <TypeDefs as super::Types>::value,
        width = <TypeDefs as super::Types>::width,
    >;
    pub type alt<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        accept = <TypeDefs as super::Types>::accept,
        alt = Value,
        auto_complete = <TypeDefs as super::Types>::auto_complete,
        capture = <TypeDefs as super::Types>::capture,
        checked = <TypeDefs as super::Types>::checked,
        dirname = <TypeDefs as super::Types>::dirname,
        disabled = <TypeDefs as super::Types>::disabled,
        form = <TypeDefs as super::Types>::form,
        form_action = <TypeDefs as super::Types>::form_action,
        form_enc_type = <TypeDefs as super::Types>::form_enc_type,
        form_method = <TypeDefs as super::Types>::form_method,
        form_no_validate = <TypeDefs as super::Types>::form_no_validate,
        form_target = <TypeDefs as super::Types>::form_target,
        height = <TypeDefs as super::Types>::height,
        list = <TypeDefs as super::Types>::list,
        max = <TypeDefs as super::Types>::max,
        max_length = <TypeDefs as super::Types>::max_length,
        min = <TypeDefs as super::Types>::min,
        min_length = <TypeDefs as super::Types>::min_length,
        multiple = <TypeDefs as super::Types>::multiple,
        name = <TypeDefs as super::Types>::name,
        pattern = <TypeDefs as super::Types>::pattern,
        placeholder = <TypeDefs as super::Types>::placeholder,
        read_only = <TypeDefs as super::Types>::read_only,
        required = <TypeDefs as super::Types>::required,
        size = <TypeDefs as super::Types>::size,
        src = <TypeDefs as super::Types>::src,
        step = <TypeDefs as super::Types>::step,
        type_ = <TypeDefs as super::Types>::type_,
        value = <TypeDefs as super::Types>::value,
        width = <TypeDefs as super::Types>::width,
    >;
    pub type auto_complete<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        accept = <TypeDefs as super::Types>::accept,
        alt = <TypeDefs as super::Types>::alt,
        auto_complete = Value,
        capture = <TypeDefs as super::Types>::capture,
        checked = <TypeDefs as super::Types>::checked,
        dirname = <TypeDefs as super::Types>::dirname,
        disabled = <TypeDefs as super::Types>::disabled,
        form = <TypeDefs as super::Types>::form,
        form_action = <TypeDefs as super::Types>::form_action,
        form_enc_type = <TypeDefs as super::Types>::form_enc_type,
        form_method = <TypeDefs as super::Types>::form_method,
        form_no_validate = <TypeDefs as super::Types>::form_no_validate,
        form_target = <TypeDefs as super::Types>::form_target,
        height = <TypeDefs as super::Types>::height,
        list = <TypeDefs as super::Types>::list,
        max = <TypeDefs as super::Types>::max,
        max_length = <TypeDefs as super::Types>::max_length,
        min = <TypeDefs as super::Types>::min,
        min_length = <TypeDefs as super::Types>::min_length,
        multiple = <TypeDefs as super::Types>::multiple,
        name = <TypeDefs as super::Types>::name,
        pattern = <TypeDefs as super::Types>::pattern,
        placeholder = <TypeDefs as super::Types>::placeholder,
        read_only = <TypeDefs as super::Types>::read_only,
        required = <TypeDefs as super::Types>::required,
        size = <TypeDefs as super::Types>::size,
        src = <TypeDefs as super::Types>::src,
        step = <TypeDefs as super::Types>::step,
        type_ = <TypeDefs as super::Types>::type_,
        value = <TypeDefs as super::Types>::value,
        width = <TypeDefs as super::Types>::width,
    >;
    pub type capture<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        accept = <TypeDefs as super::Types>::accept,
        alt = <TypeDefs as super::Types>::alt,
        auto_complete = <TypeDefs as super::Types>::auto_complete,
        capture = Value,
        checked = <TypeDefs as super::Types>::checked,
        dirname = <TypeDefs as super::Types>::dirname,
        disabled = <TypeDefs as super::Types>::disabled,
        form = <TypeDefs as super::Types>::form,
        form_action = <TypeDefs as super::Types>::form_action,
        form_enc_type = <TypeDefs as super::Types>::form_enc_type,
        form_method = <TypeDefs as super::Types>::form_method,
        form_no_validate = <TypeDefs as super::Types>::form_no_validate,
        form_target = <TypeDefs as super::Types>::form_target,
        height = <TypeDefs as super::Types>::height,
        list = <TypeDefs as super::Types>::list,
        max = <TypeDefs as super::Types>::max,
        max_length = <TypeDefs as super::Types>::max_length,
        min = <TypeDefs as super::Types>::min,
        min_length = <TypeDefs as super::Types>::min_length,
        multiple = <TypeDefs as super::Types>::multiple,
        name = <TypeDefs as super::Types>::name,
        pattern = <TypeDefs as super::Types>::pattern,
        placeholder = <TypeDefs as super::Types>::placeholder,
        read_only = <TypeDefs as super::Types>::read_only,
        required = <TypeDefs as super::Types>::required,
        size = <TypeDefs as super::Types>::size,
        src = <TypeDefs as super::Types>::src,
        step = <TypeDefs as super::Types>::step,
        type_ = <TypeDefs as super::Types>::type_,
        value = <TypeDefs as super::Types>::value,
        width = <TypeDefs as super::Types>::width,
    >;
    pub type checked<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        accept = <TypeDefs as super::Types>::accept,
        alt = <TypeDefs as super::Types>::alt,
        auto_complete = <TypeDefs as super::Types>::auto_complete,
        capture = <TypeDefs as super::Types>::capture,
        checked = Value,
        dirname = <TypeDefs as super::Types>::dirname,
        disabled = <TypeDefs as super::Types>::disabled,
        form = <TypeDefs as super::Types>::form,
        form_action = <TypeDefs as super::Types>::form_action,
        form_enc_type = <TypeDefs as super::Types>::form_enc_type,
        form_method = <TypeDefs as super::Types>::form_method,
        form_no_validate = <TypeDefs as super::Types>::form_no_validate,
        form_target = <TypeDefs as super::Types>::form_target,
        height = <TypeDefs as super::Types>::height,
        list = <TypeDefs as super::Types>::list,
        max = <TypeDefs as super::Types>::max,
        max_length = <TypeDefs as super::Types>::max_length,
        min = <TypeDefs as super::Types>::min,
        min_length = <TypeDefs as super::Types>::min_length,
        multiple = <TypeDefs as super::Types>::multiple,
        name = <TypeDefs as super::Types>::name,
        pattern = <TypeDefs as super::Types>::pattern,
        placeholder = <TypeDefs as super::Types>::placeholder,
        read_only = <TypeDefs as super::Types>::read_only,
        required = <TypeDefs as super::Types>::required,
        size = <TypeDefs as super::Types>::size,
        src = <TypeDefs as super::Types>::src,
        step = <TypeDefs as super::Types>::step,
        type_ = <TypeDefs as super::Types>::type_,
        value = <TypeDefs as super::Types>::value,
        width = <TypeDefs as super::Types>::width,
    >;
    pub type dirname<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        accept = <TypeDefs as super::Types>::accept,
        alt = <TypeDefs as super::Types>::alt,
        auto_complete = <TypeDefs as super::Types>::auto_complete,
        capture = <TypeDefs as super::Types>::capture,
        checked = <TypeDefs as super::Types>::checked,
        dirname = Value,
        disabled = <TypeDefs as super::Types>::disabled,
        form = <TypeDefs as super::Types>::form,
        form_action = <TypeDefs as super::Types>::form_action,
        form_enc_type = <TypeDefs as super::Types>::form_enc_type,
        form_method = <TypeDefs as super::Types>::form_method,
        form_no_validate = <TypeDefs as super::Types>::form_no_validate,
        form_target = <TypeDefs as super::Types>::form_target,
        height = <TypeDefs as super::Types>::height,
        list = <TypeDefs as super::Types>::list,
        max = <TypeDefs as super::Types>::max,
        max_length = <TypeDefs as super::Types>::max_length,
        min = <TypeDefs as super::Types>::min,
        min_length = <TypeDefs as super::Types>::min_length,
        multiple = <TypeDefs as super::Types>::multiple,
        name = <TypeDefs as super::Types>::name,
        pattern = <TypeDefs as super::Types>::pattern,
        placeholder = <TypeDefs as super::Types>::placeholder,
        read_only = <TypeDefs as super::Types>::read_only,
        required = <TypeDefs as super::Types>::required,
        size = <TypeDefs as super::Types>::size,
        src = <TypeDefs as super::Types>::src,
        step = <TypeDefs as super::Types>::step,
        type_ = <TypeDefs as super::Types>::type_,
        value = <TypeDefs as super::Types>::value,
        width = <TypeDefs as super::Types>::width,
    >;
    pub type disabled<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        accept = <TypeDefs as super::Types>::accept,
        alt = <TypeDefs as super::Types>::alt,
        auto_complete = <TypeDefs as super::Types>::auto_complete,
        capture = <TypeDefs as super::Types>::capture,
        checked = <TypeDefs as super::Types>::checked,
        dirname = <TypeDefs as super::Types>::dirname,
        disabled = Value,
        form = <TypeDefs as super::Types>::form,
        form_action = <TypeDefs as super::Types>::form_action,
        form_enc_type = <TypeDefs as super::Types>::form_enc_type,
        form_method = <TypeDefs as super::Types>::form_method,
        form_no_validate = <TypeDefs as super::Types>::form_no_validate,
        form_target = <TypeDefs as super::Types>::form_target,
        height = <TypeDefs as super::Types>::height,
        list = <TypeDefs as super::Types>::list,
        max = <TypeDefs as super::Types>::max,
        max_length = <TypeDefs as super::Types>::max_length,
        min = <TypeDefs as super::Types>::min,
        min_length = <TypeDefs as super::Types>::min_length,
        multiple = <TypeDefs as super::Types>::multiple,
        name = <TypeDefs as super::Types>::name,
        pattern = <TypeDefs as super::Types>::pattern,
        placeholder = <TypeDefs as super::Types>::placeholder,
        read_only = <TypeDefs as super::Types>::read_only,
        required = <TypeDefs as super::Types>::required,
        size = <TypeDefs as super::Types>::size,
        src = <TypeDefs as super::Types>::src,
        step = <TypeDefs as super::Types>::step,
        type_ = <TypeDefs as super::Types>::type_,
        value = <TypeDefs as super::Types>::value,
        width = <TypeDefs as super::Types>::width,
    >;
    pub type form<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        accept = <TypeDefs as super::Types>::accept,
        alt = <TypeDefs as super::Types>::alt,
        auto_complete = <TypeDefs as super::Types>::auto_complete,
        capture = <TypeDefs as super::Types>::capture,
        checked = <TypeDefs as super::Types>::checked,
        dirname = <TypeDefs as super::Types>::dirname,
        disabled = <TypeDefs as super::Types>::disabled,
        form = Value,
        form_action = <TypeDefs as super::Types>::form_action,
        form_enc_type = <TypeDefs as super::Types>::form_enc_type,
        form_method = <TypeDefs as super::Types>::form_method,
        form_no_validate = <TypeDefs as super::Types>::form_no_validate,
        form_target = <TypeDefs as super::Types>::form_target,
        height = <TypeDefs as super::Types>::height,
        list = <TypeDefs as super::Types>::list,
        max = <TypeDefs as super::Types>::max,
        max_length = <TypeDefs as super::Types>::max_length,
        min = <TypeDefs as super::Types>::min,
        min_length = <TypeDefs as super::Types>::min_length,
        multiple = <TypeDefs as super::Types>::multiple,
        name = <TypeDefs as super::Types>::name,
        pattern = <TypeDefs as super::Types>::pattern,
        placeholder = <TypeDefs as super::Types>::placeholder,
        read_only = <TypeDefs as super::Types>::read_only,
        required = <TypeDefs as super::Types>::required,
        size = <TypeDefs as super::Types>::size,
        src = <TypeDefs as super::Types>::src,
        step = <TypeDefs as super::Types>::step,
        type_ = <TypeDefs as super::Types>::type_,
        value = <TypeDefs as super::Types>::value,
        width = <TypeDefs as super::Types>::width,
    >;
    pub type form_action<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        accept = <TypeDefs as super::Types>::accept,
        alt = <TypeDefs as super::Types>::alt,
        auto_complete = <TypeDefs as super::Types>::auto_complete,
        capture = <TypeDefs as super::Types>::capture,
        checked = <TypeDefs as super::Types>::checked,
        dirname = <TypeDefs as super::Types>::dirname,
        disabled = <TypeDefs as super::Types>::disabled,
        form = <TypeDefs as super::Types>::form,
        form_action = Value,
        form_enc_type = <TypeDefs as super::Types>::form_enc_type,
        form_method = <TypeDefs as super::Types>::form_method,
        form_no_validate = <TypeDefs as super::Types>::form_no_validate,
        form_target = <TypeDefs as super::Types>::form_target,
        height = <TypeDefs as super::Types>::height,
        list = <TypeDefs as super::Types>::list,
        max = <TypeDefs as super::Types>::max,
        max_length = <TypeDefs as super::Types>::max_length,
        min = <TypeDefs as super::Types>::min,
        min_length = <TypeDefs as super::Types>::min_length,
        multiple = <TypeDefs as super::Types>::multiple,
        name = <TypeDefs as super::Types>::name,
        pattern = <TypeDefs as super::Types>::pattern,
        placeholder = <TypeDefs as super::Types>::placeholder,
        read_only = <TypeDefs as super::Types>::read_only,
        required = <TypeDefs as super::Types>::required,
        size = <TypeDefs as super::Types>::size,
        src = <TypeDefs as super::Types>::src,
        step = <TypeDefs as super::Types>::step,
        type_ = <TypeDefs as super::Types>::type_,
        value = <TypeDefs as super::Types>::value,
        width = <TypeDefs as super::Types>::width,
    >;
    pub type form_enc_type<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        accept = <TypeDefs as super::Types>::accept,
        alt = <TypeDefs as super::Types>::alt,
        auto_complete = <TypeDefs as super::Types>::auto_complete,
        capture = <TypeDefs as super::Types>::capture,
        checked = <TypeDefs as super::Types>::checked,
        dirname = <TypeDefs as super::Types>::dirname,
        disabled = <TypeDefs as super::Types>::disabled,
        form = <TypeDefs as super::Types>::form,
        form_action = <TypeDefs as super::Types>::form_action,
        form_enc_type = Value,
        form_method = <TypeDefs as super::Types>::form_method,
        form_no_validate = <TypeDefs as super::Types>::form_no_validate,
        form_target = <TypeDefs as super::Types>::form_target,
        height = <TypeDefs as super::Types>::height,
        list = <TypeDefs as super::Types>::list,
        max = <TypeDefs as super::Types>::max,
        max_length = <TypeDefs as super::Types>::max_length,
        min = <TypeDefs as super::Types>::min,
        min_length = <TypeDefs as super::Types>::min_length,
        multiple = <TypeDefs as super::Types>::multiple,
        name = <TypeDefs as super::Types>::name,
        pattern = <TypeDefs as super::Types>::pattern,
        placeholder = <TypeDefs as super::Types>::placeholder,
        read_only = <TypeDefs as super::Types>::read_only,
        required = <TypeDefs as super::Types>::required,
        size = <TypeDefs as super::Types>::size,
        src = <TypeDefs as super::Types>::src,
        step = <TypeDefs as super::Types>::step,
        type_ = <TypeDefs as super::Types>::type_,
        value = <TypeDefs as super::Types>::value,
        width = <TypeDefs as super::Types>::width,
    >;
    pub type form_method<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        accept = <TypeDefs as super::Types>::accept,
        alt = <TypeDefs as super::Types>::alt,
        auto_complete = <TypeDefs as super::Types>::auto_complete,
        capture = <TypeDefs as super::Types>::capture,
        checked = <TypeDefs as super::Types>::checked,
        dirname = <TypeDefs as super::Types>::dirname,
        disabled = <TypeDefs as super::Types>::disabled,
        form = <TypeDefs as super::Types>::form,
        form_action = <TypeDefs as super::Types>::form_action,
        form_enc_type = <TypeDefs as super::Types>::form_enc_type,
        form_method = Value,
        form_no_validate = <TypeDefs as super::Types>::form_no_validate,
        form_target = <TypeDefs as super::Types>::form_target,
        height = <TypeDefs as super::Types>::height,
        list = <TypeDefs as super::Types>::list,
        max = <TypeDefs as super::Types>::max,
        max_length = <TypeDefs as super::Types>::max_length,
        min = <TypeDefs as super::Types>::min,
        min_length = <TypeDefs as super::Types>::min_length,
        multiple = <TypeDefs as super::Types>::multiple,
        name = <TypeDefs as super::Types>::name,
        pattern = <TypeDefs as super::Types>::pattern,
        placeholder = <TypeDefs as super::Types>::placeholder,
        read_only = <TypeDefs as super::Types>::read_only,
        required = <TypeDefs as super::Types>::required,
        size = <TypeDefs as super::Types>::size,
        src = <TypeDefs as super::Types>::src,
        step = <TypeDefs as super::Types>::step,
        type_ = <TypeDefs as super::Types>::type_,
        value = <TypeDefs as super::Types>::value,
        width = <TypeDefs as super::Types>::width,
    >;
    pub type form_no_validate<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        accept = <TypeDefs as super::Types>::accept,
        alt = <TypeDefs as super::Types>::alt,
        auto_complete = <TypeDefs as super::Types>::auto_complete,
        capture = <TypeDefs as super::Types>::capture,
        checked = <TypeDefs as super::Types>::checked,
        dirname = <TypeDefs as super::Types>::dirname,
        disabled = <TypeDefs as super::Types>::disabled,
        form = <TypeDefs as super::Types>::form,
        form_action = <TypeDefs as super::Types>::form_action,
        form_enc_type = <TypeDefs as super::Types>::form_enc_type,
        form_method = <TypeDefs as super::Types>::form_method,
        form_no_validate = Value,
        form_target = <TypeDefs as super::Types>::form_target,
        height = <TypeDefs as super::Types>::height,
        list = <TypeDefs as super::Types>::list,
        max = <TypeDefs as super::Types>::max,
        max_length = <TypeDefs as super::Types>::max_length,
        min = <TypeDefs as super::Types>::min,
        min_length = <TypeDefs as super::Types>::min_length,
        multiple = <TypeDefs as super::Types>::multiple,
        name = <TypeDefs as super::Types>::name,
        pattern = <TypeDefs as super::Types>::pattern,
        placeholder = <TypeDefs as super::Types>::placeholder,
        read_only = <TypeDefs as super::Types>::read_only,
        required = <TypeDefs as super::Types>::required,
        size = <TypeDefs as super::Types>::size,
        src = <TypeDefs as super::Types>::src,
        step = <TypeDefs as super::Types>::step,
        type_ = <TypeDefs as super::Types>::type_,
        value = <TypeDefs as super::Types>::value,
        width = <TypeDefs as super::Types>::width,
    >;
    pub type form_target<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        accept = <TypeDefs as super::Types>::accept,
        alt = <TypeDefs as super::Types>::alt,
        auto_complete = <TypeDefs as super::Types>::auto_complete,
        capture = <TypeDefs as super::Types>::capture,
        checked = <TypeDefs as super::Types>::checked,
        dirname = <TypeDefs as super::Types>::dirname,
        disabled = <TypeDefs as super::Types>::disabled,
        form = <TypeDefs as super::Types>::form,
        form_action = <TypeDefs as super::Types>::form_action,
        form_enc_type = <TypeDefs as super::Types>::form_enc_type,
        form_method = <TypeDefs as super::Types>::form_method,
        form_no_validate = <TypeDefs as super::Types>::form_no_validate,
        form_target = Value,
        height = <TypeDefs as super::Types>::height,
        list = <TypeDefs as super::Types>::list,
        max = <TypeDefs as super::Types>::max,
        max_length = <TypeDefs as super::Types>::max_length,
        min = <TypeDefs as super::Types>::min,
        min_length = <TypeDefs as super::Types>::min_length,
        multiple = <TypeDefs as super::Types>::multiple,
        name = <TypeDefs as super::Types>::name,
        pattern = <TypeDefs as super::Types>::pattern,
        placeholder = <TypeDefs as super::Types>::placeholder,
        read_only = <TypeDefs as super::Types>::read_only,
        required = <TypeDefs as super::Types>::required,
        size = <TypeDefs as super::Types>::size,
        src = <TypeDefs as super::Types>::src,
        step = <TypeDefs as super::Types>::step,
        type_ = <TypeDefs as super::Types>::type_,
        value = <TypeDefs as super::Types>::value,
        width = <TypeDefs as super::Types>::width,
    >;
    pub type height<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        accept = <TypeDefs as super::Types>::accept,
        alt = <TypeDefs as super::Types>::alt,
        auto_complete = <TypeDefs as super::Types>::auto_complete,
        capture = <TypeDefs as super::Types>::capture,
        checked = <TypeDefs as super::Types>::checked,
        dirname = <TypeDefs as super::Types>::dirname,
        disabled = <TypeDefs as super::Types>::disabled,
        form = <TypeDefs as super::Types>::form,
        form_action = <TypeDefs as super::Types>::form_action,
        form_enc_type = <TypeDefs as super::Types>::form_enc_type,
        form_method = <TypeDefs as super::Types>::form_method,
        form_no_validate = <TypeDefs as super::Types>::form_no_validate,
        form_target = <TypeDefs as super::Types>::form_target,
        height = Value,
        list = <TypeDefs as super::Types>::list,
        max = <TypeDefs as super::Types>::max,
        max_length = <TypeDefs as super::Types>::max_length,
        min = <TypeDefs as super::Types>::min,
        min_length = <TypeDefs as super::Types>::min_length,
        multiple = <TypeDefs as super::Types>::multiple,
        name = <TypeDefs as super::Types>::name,
        pattern = <TypeDefs as super::Types>::pattern,
        placeholder = <TypeDefs as super::Types>::placeholder,
        read_only = <TypeDefs as super::Types>::read_only,
        required = <TypeDefs as super::Types>::required,
        size = <TypeDefs as super::Types>::size,
        src = <TypeDefs as super::Types>::src,
        step = <TypeDefs as super::Types>::step,
        type_ = <TypeDefs as super::Types>::type_,
        value = <TypeDefs as super::Types>::value,
        width = <TypeDefs as super::Types>::width,
    >;
    pub type list<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        accept = <TypeDefs as super::Types>::accept,
        alt = <TypeDefs as super::Types>::alt,
        auto_complete = <TypeDefs as super::Types>::auto_complete,
        capture = <TypeDefs as super::Types>::capture,
        checked = <TypeDefs as super::Types>::checked,
        dirname = <TypeDefs as super::Types>::dirname,
        disabled = <TypeDefs as super::Types>::disabled,
        form = <TypeDefs as super::Types>::form,
        form_action = <TypeDefs as super::Types>::form_action,
        form_enc_type = <TypeDefs as super::Types>::form_enc_type,
        form_method = <TypeDefs as super::Types>::form_method,
        form_no_validate = <TypeDefs as super::Types>::form_no_validate,
        form_target = <TypeDefs as super::Types>::form_target,
        height = <TypeDefs as super::Types>::height,
        list = Value,
        max = <TypeDefs as super::Types>::max,
        max_length = <TypeDefs as super::Types>::max_length,
        min = <TypeDefs as super::Types>::min,
        min_length = <TypeDefs as super::Types>::min_length,
        multiple = <TypeDefs as super::Types>::multiple,
        name = <TypeDefs as super::Types>::name,
        pattern = <TypeDefs as super::Types>::pattern,
        placeholder = <TypeDefs as super::Types>::placeholder,
        read_only = <TypeDefs as super::Types>::read_only,
        required = <TypeDefs as super::Types>::required,
        size = <TypeDefs as super::Types>::size,
        src = <TypeDefs as super::Types>::src,
        step = <TypeDefs as super::Types>::step,
        type_ = <TypeDefs as super::Types>::type_,
        value = <TypeDefs as super::Types>::value,
        width = <TypeDefs as super::Types>::width,
    >;
    pub type max<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        accept = <TypeDefs as super::Types>::accept,
        alt = <TypeDefs as super::Types>::alt,
        auto_complete = <TypeDefs as super::Types>::auto_complete,
        capture = <TypeDefs as super::Types>::capture,
        checked = <TypeDefs as super::Types>::checked,
        dirname = <TypeDefs as super::Types>::dirname,
        disabled = <TypeDefs as super::Types>::disabled,
        form = <TypeDefs as super::Types>::form,
        form_action = <TypeDefs as super::Types>::form_action,
        form_enc_type = <TypeDefs as super::Types>::form_enc_type,
        form_method = <TypeDefs as super::Types>::form_method,
        form_no_validate = <TypeDefs as super::Types>::form_no_validate,
        form_target = <TypeDefs as super::Types>::form_target,
        height = <TypeDefs as super::Types>::height,
        list = <TypeDefs as super::Types>::list,
        max = Value,
        max_length = <TypeDefs as super::Types>::max_length,
        min = <TypeDefs as super::Types>::min,
        min_length = <TypeDefs as super::Types>::min_length,
        multiple = <TypeDefs as super::Types>::multiple,
        name = <TypeDefs as super::Types>::name,
        pattern = <TypeDefs as super::Types>::pattern,
        placeholder = <TypeDefs as super::Types>::placeholder,
        read_only = <TypeDefs as super::Types>::read_only,
        required = <TypeDefs as super::Types>::required,
        size = <TypeDefs as super::Types>::size,
        src = <TypeDefs as super::Types>::src,
        step = <TypeDefs as super::Types>::step,
        type_ = <TypeDefs as super::Types>::type_,
        value = <TypeDefs as super::Types>::value,
        width = <TypeDefs as super::Types>::width,
    >;
    pub type max_length<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        accept = <TypeDefs as super::Types>::accept,
        alt = <TypeDefs as super::Types>::alt,
        auto_complete = <TypeDefs as super::Types>::auto_complete,
        capture = <TypeDefs as super::Types>::capture,
        checked = <TypeDefs as super::Types>::checked,
        dirname = <TypeDefs as super::Types>::dirname,
        disabled = <TypeDefs as super::Types>::disabled,
        form = <TypeDefs as super::Types>::form,
        form_action = <TypeDefs as super::Types>::form_action,
        form_enc_type = <TypeDefs as super::Types>::form_enc_type,
        form_method = <TypeDefs as super::Types>::form_method,
        form_no_validate = <TypeDefs as super::Types>::form_no_validate,
        form_target = <TypeDefs as super::Types>::form_target,
        height = <TypeDefs as super::Types>::height,
        list = <TypeDefs as super::Types>::list,
        max = <TypeDefs as super::Types>::max,
        max_length = Value,
        min = <TypeDefs as super::Types>::min,
        min_length = <TypeDefs as super::Types>::min_length,
        multiple = <TypeDefs as super::Types>::multiple,
        name = <TypeDefs as super::Types>::name,
        pattern = <TypeDefs as super::Types>::pattern,
        placeholder = <TypeDefs as super::Types>::placeholder,
        read_only = <TypeDefs as super::Types>::read_only,
        required = <TypeDefs as super::Types>::required,
        size = <TypeDefs as super::Types>::size,
        src = <TypeDefs as super::Types>::src,
        step = <TypeDefs as super::Types>::step,
        type_ = <TypeDefs as super::Types>::type_,
        value = <TypeDefs as super::Types>::value,
        width = <TypeDefs as super::Types>::width,
    >;
    pub type min<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        accept = <TypeDefs as super::Types>::accept,
        alt = <TypeDefs as super::Types>::alt,
        auto_complete = <TypeDefs as super::Types>::auto_complete,
        capture = <TypeDefs as super::Types>::capture,
        checked = <TypeDefs as super::Types>::checked,
        dirname = <TypeDefs as super::Types>::dirname,
        disabled = <TypeDefs as super::Types>::disabled,
        form = <TypeDefs as super::Types>::form,
        form_action = <TypeDefs as super::Types>::form_action,
        form_enc_type = <TypeDefs as super::Types>::form_enc_type,
        form_method = <TypeDefs as super::Types>::form_method,
        form_no_validate = <TypeDefs as super::Types>::form_no_validate,
        form_target = <TypeDefs as super::Types>::form_target,
        height = <TypeDefs as super::Types>::height,
        list = <TypeDefs as super::Types>::list,
        max = <TypeDefs as super::Types>::max,
        max_length = <TypeDefs as super::Types>::max_length,
        min = Value,
        min_length = <TypeDefs as super::Types>::min_length,
        multiple = <TypeDefs as super::Types>::multiple,
        name = <TypeDefs as super::Types>::name,
        pattern = <TypeDefs as super::Types>::pattern,
        placeholder = <TypeDefs as super::Types>::placeholder,
        read_only = <TypeDefs as super::Types>::read_only,
        required = <TypeDefs as super::Types>::required,
        size = <TypeDefs as super::Types>::size,
        src = <TypeDefs as super::Types>::src,
        step = <TypeDefs as super::Types>::step,
        type_ = <TypeDefs as super::Types>::type_,
        value = <TypeDefs as super::Types>::value,
        width = <TypeDefs as super::Types>::width,
    >;
    pub type min_length<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        accept = <TypeDefs as super::Types>::accept,
        alt = <TypeDefs as super::Types>::alt,
        auto_complete = <TypeDefs as super::Types>::auto_complete,
        capture = <TypeDefs as super::Types>::capture,
        checked = <TypeDefs as super::Types>::checked,
        dirname = <TypeDefs as super::Types>::dirname,
        disabled = <TypeDefs as super::Types>::disabled,
        form = <TypeDefs as super::Types>::form,
        form_action = <TypeDefs as super::Types>::form_action,
        form_enc_type = <TypeDefs as super::Types>::form_enc_type,
        form_method = <TypeDefs as super::Types>::form_method,
        form_no_validate = <TypeDefs as super::Types>::form_no_validate,
        form_target = <TypeDefs as super::Types>::form_target,
        height = <TypeDefs as super::Types>::height,
        list = <TypeDefs as super::Types>::list,
        max = <TypeDefs as super::Types>::max,
        max_length = <TypeDefs as super::Types>::max_length,
        min = <TypeDefs as super::Types>::min,
        min_length = Value,
        multiple = <TypeDefs as super::Types>::multiple,
        name = <TypeDefs as super::Types>::name,
        pattern = <TypeDefs as super::Types>::pattern,
        placeholder = <TypeDefs as super::Types>::placeholder,
        read_only = <TypeDefs as super::Types>::read_only,
        required = <TypeDefs as super::Types>::required,
        size = <TypeDefs as super::Types>::size,
        src = <TypeDefs as super::Types>::src,
        step = <TypeDefs as super::Types>::step,
        type_ = <TypeDefs as super::Types>::type_,
        value = <TypeDefs as super::Types>::value,
        width = <TypeDefs as super::Types>::width,
    >;
    pub type multiple<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        accept = <TypeDefs as super::Types>::accept,
        alt = <TypeDefs as super::Types>::alt,
        auto_complete = <TypeDefs as super::Types>::auto_complete,
        capture = <TypeDefs as super::Types>::capture,
        checked = <TypeDefs as super::Types>::checked,
        dirname = <TypeDefs as super::Types>::dirname,
        disabled = <TypeDefs as super::Types>::disabled,
        form = <TypeDefs as super::Types>::form,
        form_action = <TypeDefs as super::Types>::form_action,
        form_enc_type = <TypeDefs as super::Types>::form_enc_type,
        form_method = <TypeDefs as super::Types>::form_method,
        form_no_validate = <TypeDefs as super::Types>::form_no_validate,
        form_target = <TypeDefs as super::Types>::form_target,
        height = <TypeDefs as super::Types>::height,
        list = <TypeDefs as super::Types>::list,
        max = <TypeDefs as super::Types>::max,
        max_length = <TypeDefs as super::Types>::max_length,
        min = <TypeDefs as super::Types>::min,
        min_length = <TypeDefs as super::Types>::min_length,
        multiple = Value,
        name = <TypeDefs as super::Types>::name,
        pattern = <TypeDefs as super::Types>::pattern,
        placeholder = <TypeDefs as super::Types>::placeholder,
        read_only = <TypeDefs as super::Types>::read_only,
        required = <TypeDefs as super::Types>::required,
        size = <TypeDefs as super::Types>::size,
        src = <TypeDefs as super::Types>::src,
        step = <TypeDefs as super::Types>::step,
        type_ = <TypeDefs as super::Types>::type_,
        value = <TypeDefs as super::Types>::value,
        width = <TypeDefs as super::Types>::width,
    >;
    pub type name<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        accept = <TypeDefs as super::Types>::accept,
        alt = <TypeDefs as super::Types>::alt,
        auto_complete = <TypeDefs as super::Types>::auto_complete,
        capture = <TypeDefs as super::Types>::capture,
        checked = <TypeDefs as super::Types>::checked,
        dirname = <TypeDefs as super::Types>::dirname,
        disabled = <TypeDefs as super::Types>::disabled,
        form = <TypeDefs as super::Types>::form,
        form_action = <TypeDefs as super::Types>::form_action,
        form_enc_type = <TypeDefs as super::Types>::form_enc_type,
        form_method = <TypeDefs as super::Types>::form_method,
        form_no_validate = <TypeDefs as super::Types>::form_no_validate,
        form_target = <TypeDefs as super::Types>::form_target,
        height = <TypeDefs as super::Types>::height,
        list = <TypeDefs as super::Types>::list,
        max = <TypeDefs as super::Types>::max,
        max_length = <TypeDefs as super::Types>::max_length,
        min = <TypeDefs as super::Types>::min,
        min_length = <TypeDefs as super::Types>::min_length,
        multiple = <TypeDefs as super::Types>::multiple,
        name = Value,
        pattern = <TypeDefs as super::Types>::pattern,
        placeholder = <TypeDefs as super::Types>::placeholder,
        read_only = <TypeDefs as super::Types>::read_only,
        required = <TypeDefs as super::Types>::required,
        size = <TypeDefs as super::Types>::size,
        src = <TypeDefs as super::Types>::src,
        step = <TypeDefs as super::Types>::step,
        type_ = <TypeDefs as super::Types>::type_,
        value = <TypeDefs as super::Types>::value,
        width = <TypeDefs as super::Types>::width,
    >;
    pub type pattern<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        accept = <TypeDefs as super::Types>::accept,
        alt = <TypeDefs as super::Types>::alt,
        auto_complete = <TypeDefs as super::Types>::auto_complete,
        capture = <TypeDefs as super::Types>::capture,
        checked = <TypeDefs as super::Types>::checked,
        dirname = <TypeDefs as super::Types>::dirname,
        disabled = <TypeDefs as super::Types>::disabled,
        form = <TypeDefs as super::Types>::form,
        form_action = <TypeDefs as super::Types>::form_action,
        form_enc_type = <TypeDefs as super::Types>::form_enc_type,
        form_method = <TypeDefs as super::Types>::form_method,
        form_no_validate = <TypeDefs as super::Types>::form_no_validate,
        form_target = <TypeDefs as super::Types>::form_target,
        height = <TypeDefs as super::Types>::height,
        list = <TypeDefs as super::Types>::list,
        max = <TypeDefs as super::Types>::max,
        max_length = <TypeDefs as super::Types>::max_length,
        min = <TypeDefs as super::Types>::min,
        min_length = <TypeDefs as super::Types>::min_length,
        multiple = <TypeDefs as super::Types>::multiple,
        name = <TypeDefs as super::Types>::name,
        pattern = Value,
        placeholder = <TypeDefs as super::Types>::placeholder,
        read_only = <TypeDefs as super::Types>::read_only,
        required = <TypeDefs as super::Types>::required,
        size = <TypeDefs as super::Types>::size,
        src = <TypeDefs as super::Types>::src,
        step = <TypeDefs as super::Types>::step,
        type_ = <TypeDefs as super::Types>::type_,
        value = <TypeDefs as super::Types>::value,
        width = <TypeDefs as super::Types>::width,
    >;
    pub type placeholder<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        accept = <TypeDefs as super::Types>::accept,
        alt = <TypeDefs as super::Types>::alt,
        auto_complete = <TypeDefs as super::Types>::auto_complete,
        capture = <TypeDefs as super::Types>::capture,
        checked = <TypeDefs as super::Types>::checked,
        dirname = <TypeDefs as super::Types>::dirname,
        disabled = <TypeDefs as super::Types>::disabled,
        form = <TypeDefs as super::Types>::form,
        form_action = <TypeDefs as super::Types>::form_action,
        form_enc_type = <TypeDefs as super::Types>::form_enc_type,
        form_method = <TypeDefs as super::Types>::form_method,
        form_no_validate = <TypeDefs as super::Types>::form_no_validate,
        form_target = <TypeDefs as super::Types>::form_target,
        height = <TypeDefs as super::Types>::height,
        list = <TypeDefs as super::Types>::list,
        max = <TypeDefs as super::Types>::max,
        max_length = <TypeDefs as super::Types>::max_length,
        min = <TypeDefs as super::Types>::min,
        min_length = <TypeDefs as super::Types>::min_length,
        multiple = <TypeDefs as super::Types>::multiple,
        name = <TypeDefs as super::Types>::name,
        pattern = <TypeDefs as super::Types>::pattern,
        placeholder = Value,
        read_only = <TypeDefs as super::Types>::read_only,
        required = <TypeDefs as super::Types>::required,
        size = <TypeDefs as super::Types>::size,
        src = <TypeDefs as super::Types>::src,
        step = <TypeDefs as super::Types>::step,
        type_ = <TypeDefs as super::Types>::type_,
        value = <TypeDefs as super::Types>::value,
        width = <TypeDefs as super::Types>::width,
    >;
    pub type read_only<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        accept = <TypeDefs as super::Types>::accept,
        alt = <TypeDefs as super::Types>::alt,
        auto_complete = <TypeDefs as super::Types>::auto_complete,
        capture = <TypeDefs as super::Types>::capture,
        checked = <TypeDefs as super::Types>::checked,
        dirname = <TypeDefs as super::Types>::dirname,
        disabled = <TypeDefs as super::Types>::disabled,
        form = <TypeDefs as super::Types>::form,
        form_action = <TypeDefs as super::Types>::form_action,
        form_enc_type = <TypeDefs as super::Types>::form_enc_type,
        form_method = <TypeDefs as super::Types>::form_method,
        form_no_validate = <TypeDefs as super::Types>::form_no_validate,
        form_target = <TypeDefs as super::Types>::form_target,
        height = <TypeDefs as super::Types>::height,
        list = <TypeDefs as super::Types>::list,
        max = <TypeDefs as super::Types>::max,
        max_length = <TypeDefs as super::Types>::max_length,
        min = <TypeDefs as super::Types>::min,
        min_length = <TypeDefs as super::Types>::min_length,
        multiple = <TypeDefs as super::Types>::multiple,
        name = <TypeDefs as super::Types>::name,
        pattern = <TypeDefs as super::Types>::pattern,
        placeholder = <TypeDefs as super::Types>::placeholder,
        read_only = Value,
        required = <TypeDefs as super::Types>::required,
        size = <TypeDefs as super::Types>::size,
        src = <TypeDefs as super::Types>::src,
        step = <TypeDefs as super::Types>::step,
        type_ = <TypeDefs as super::Types>::type_,
        value = <TypeDefs as super::Types>::value,
        width = <TypeDefs as super::Types>::width,
    >;
    pub type required<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        accept = <TypeDefs as super::Types>::accept,
        alt = <TypeDefs as super::Types>::alt,
        auto_complete = <TypeDefs as super::Types>::auto_complete,
        capture = <TypeDefs as super::Types>::capture,
        checked = <TypeDefs as super::Types>::checked,
        dirname = <TypeDefs as super::Types>::dirname,
        disabled = <TypeDefs as super::Types>::disabled,
        form = <TypeDefs as super::Types>::form,
        form_action = <TypeDefs as super::Types>::form_action,
        form_enc_type = <TypeDefs as super::Types>::form_enc_type,
        form_method = <TypeDefs as super::Types>::form_method,
        form_no_validate = <TypeDefs as super::Types>::form_no_validate,
        form_target = <TypeDefs as super::Types>::form_target,
        height = <TypeDefs as super::Types>::height,
        list = <TypeDefs as super::Types>::list,
        max = <TypeDefs as super::Types>::max,
        max_length = <TypeDefs as super::Types>::max_length,
        min = <TypeDefs as super::Types>::min,
        min_length = <TypeDefs as super::Types>::min_length,
        multiple = <TypeDefs as super::Types>::multiple,
        name = <TypeDefs as super::Types>::name,
        pattern = <TypeDefs as super::Types>::pattern,
        placeholder = <TypeDefs as super::Types>::placeholder,
        read_only = <TypeDefs as super::Types>::read_only,
        required = Value,
        size = <TypeDefs as super::Types>::size,
        src = <TypeDefs as super::Types>::src,
        step = <TypeDefs as super::Types>::step,
        type_ = <TypeDefs as super::Types>::type_,
        value = <TypeDefs as super::Types>::value,
        width = <TypeDefs as super::Types>::width,
    >;
    pub type size<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        accept = <TypeDefs as super::Types>::accept,
        alt = <TypeDefs as super::Types>::alt,
        auto_complete = <TypeDefs as super::Types>::auto_complete,
        capture = <TypeDefs as super::Types>::capture,
        checked = <TypeDefs as super::Types>::checked,
        dirname = <TypeDefs as super::Types>::dirname,
        disabled = <TypeDefs as super::Types>::disabled,
        form = <TypeDefs as super::Types>::form,
        form_action = <TypeDefs as super::Types>::form_action,
        form_enc_type = <TypeDefs as super::Types>::form_enc_type,
        form_method = <TypeDefs as super::Types>::form_method,
        form_no_validate = <TypeDefs as super::Types>::form_no_validate,
        form_target = <TypeDefs as super::Types>::form_target,
        height = <TypeDefs as super::Types>::height,
        list = <TypeDefs as super::Types>::list,
        max = <TypeDefs as super::Types>::max,
        max_length = <TypeDefs as super::Types>::max_length,
        min = <TypeDefs as super::Types>::min,
        min_length = <TypeDefs as super::Types>::min_length,
        multiple = <TypeDefs as super::Types>::multiple,
        name = <TypeDefs as super::Types>::name,
        pattern = <TypeDefs as super::Types>::pattern,
        placeholder = <TypeDefs as super::Types>::placeholder,
        read_only = <TypeDefs as super::Types>::read_only,
        required = <TypeDefs as super::Types>::required,
        size = Value,
        src = <TypeDefs as super::Types>::src,
        step = <TypeDefs as super::Types>::step,
        type_ = <TypeDefs as super::Types>::type_,
        value = <TypeDefs as super::Types>::value,
        width = <TypeDefs as super::Types>::width,
    >;
    pub type src<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        accept = <TypeDefs as super::Types>::accept,
        alt = <TypeDefs as super::Types>::alt,
        auto_complete = <TypeDefs as super::Types>::auto_complete,
        capture = <TypeDefs as super::Types>::capture,
        checked = <TypeDefs as super::Types>::checked,
        dirname = <TypeDefs as super::Types>::dirname,
        disabled = <TypeDefs as super::Types>::disabled,
        form = <TypeDefs as super::Types>::form,
        form_action = <TypeDefs as super::Types>::form_action,
        form_enc_type = <TypeDefs as super::Types>::form_enc_type,
        form_method = <TypeDefs as super::Types>::form_method,
        form_no_validate = <TypeDefs as super::Types>::form_no_validate,
        form_target = <TypeDefs as super::Types>::form_target,
        height = <TypeDefs as super::Types>::height,
        list = <TypeDefs as super::Types>::list,
        max = <TypeDefs as super::Types>::max,
        max_length = <TypeDefs as super::Types>::max_length,
        min = <TypeDefs as super::Types>::min,
        min_length = <TypeDefs as super::Types>::min_length,
        multiple = <TypeDefs as super::Types>::multiple,
        name = <TypeDefs as super::Types>::name,
        pattern = <TypeDefs as super::Types>::pattern,
        placeholder = <TypeDefs as super::Types>::placeholder,
        read_only = <TypeDefs as super::Types>::read_only,
        required = <TypeDefs as super::Types>::required,
        size = <TypeDefs as super::Types>::size,
        src = Value,
        step = <TypeDefs as super::Types>::step,
        type_ = <TypeDefs as super::Types>::type_,
        value = <TypeDefs as super::Types>::value,
        width = <TypeDefs as super::Types>::width,
    >;
    pub type step<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        accept = <TypeDefs as super::Types>::accept,
        alt = <TypeDefs as super::Types>::alt,
        auto_complete = <TypeDefs as super::Types>::auto_complete,
        capture = <TypeDefs as super::Types>::capture,
        checked = <TypeDefs as super::Types>::checked,
        dirname = <TypeDefs as super::Types>::dirname,
        disabled = <TypeDefs as super::Types>::disabled,
        form = <TypeDefs as super::Types>::form,
        form_action = <TypeDefs as super::Types>::form_action,
        form_enc_type = <TypeDefs as super::Types>::form_enc_type,
        form_method = <TypeDefs as super::Types>::form_method,
        form_no_validate = <TypeDefs as super::Types>::form_no_validate,
        form_target = <TypeDefs as super::Types>::form_target,
        height = <TypeDefs as super::Types>::height,
        list = <TypeDefs as super::Types>::list,
        max = <TypeDefs as super::Types>::max,
        max_length = <TypeDefs as super::Types>::max_length,
        min = <TypeDefs as super::Types>::min,
        min_length = <TypeDefs as super::Types>::min_length,
        multiple = <TypeDefs as super::Types>::multiple,
        name = <TypeDefs as super::Types>::name,
        pattern = <TypeDefs as super::Types>::pattern,
        placeholder = <TypeDefs as super::Types>::placeholder,
        read_only = <TypeDefs as super::Types>::read_only,
        required = <TypeDefs as super::Types>::required,
        size = <TypeDefs as super::Types>::size,
        src = <TypeDefs as super::Types>::src,
        step = Value,
        type_ = <TypeDefs as super::Types>::type_,
        value = <TypeDefs as super::Types>::value,
        width = <TypeDefs as super::Types>::width,
    >;
    pub type type_<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        accept = <TypeDefs as super::Types>::accept,
        alt = <TypeDefs as super::Types>::alt,
        auto_complete = <TypeDefs as super::Types>::auto_complete,
        capture = <TypeDefs as super::Types>::capture,
        checked = <TypeDefs as super::Types>::checked,
        dirname = <TypeDefs as super::Types>::dirname,
        disabled = <TypeDefs as super::Types>::disabled,
        form = <TypeDefs as super::Types>::form,
        form_action = <TypeDefs as super::Types>::form_action,
        form_enc_type = <TypeDefs as super::Types>::form_enc_type,
        form_method = <TypeDefs as super::Types>::form_method,
        form_no_validate = <TypeDefs as super::Types>::form_no_validate,
        form_target = <TypeDefs as super::Types>::form_target,
        height = <TypeDefs as super::Types>::height,
        list = <TypeDefs as super::Types>::list,
        max = <TypeDefs as super::Types>::max,
        max_length = <TypeDefs as super::Types>::max_length,
        min = <TypeDefs as super::Types>::min,
        min_length = <TypeDefs as super::Types>::min_length,
        multiple = <TypeDefs as super::Types>::multiple,
        name = <TypeDefs as super::Types>::name,
        pattern = <TypeDefs as super::Types>::pattern,
        placeholder = <TypeDefs as super::Types>::placeholder,
        read_only = <TypeDefs as super::Types>::read_only,
        required = <TypeDefs as super::Types>::required,
        size = <TypeDefs as super::Types>::size,
        src = <TypeDefs as super::Types>::src,
        step = <TypeDefs as super::Types>::step,
        type_ = Value,
        value = <TypeDefs as super::Types>::value,
        width = <TypeDefs as super::Types>::width,
    >;
    pub type value<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        accept = <TypeDefs as super::Types>::accept,
        alt = <TypeDefs as super::Types>::alt,
        auto_complete = <TypeDefs as super::Types>::auto_complete,
        capture = <TypeDefs as super::Types>::capture,
        checked = <TypeDefs as super::Types>::checked,
        dirname = <TypeDefs as super::Types>::dirname,
        disabled = <TypeDefs as super::Types>::disabled,
        form = <TypeDefs as super::Types>::form,
        form_action = <TypeDefs as super::Types>::form_action,
        form_enc_type = <TypeDefs as super::Types>::form_enc_type,
        form_method = <TypeDefs as super::Types>::form_method,
        form_no_validate = <TypeDefs as super::Types>::form_no_validate,
        form_target = <TypeDefs as super::Types>::form_target,
        height = <TypeDefs as super::Types>::height,
        list = <TypeDefs as super::Types>::list,
        max = <TypeDefs as super::Types>::max,
        max_length = <TypeDefs as super::Types>::max_length,
        min = <TypeDefs as super::Types>::min,
        min_length = <TypeDefs as super::Types>::min_length,
        multiple = <TypeDefs as super::Types>::multiple,
        name = <TypeDefs as super::Types>::name,
        pattern = <TypeDefs as super::Types>::pattern,
        placeholder = <TypeDefs as super::Types>::placeholder,
        read_only = <TypeDefs as super::Types>::read_only,
        required = <TypeDefs as super::Types>::required,
        size = <TypeDefs as super::Types>::size,
        src = <TypeDefs as super::Types>::src,
        step = <TypeDefs as super::Types>::step,
        type_ = <TypeDefs as super::Types>::type_,
        value = Value,
        width = <TypeDefs as super::Types>::width,
    >;
    pub type width<TypeDefs, Value> = dyn super::Types<
        HtmlElementProps = <TypeDefs as super::Types>::HtmlElementProps,
        accept = <TypeDefs as super::Types>::accept,
        alt = <TypeDefs as super::Types>::alt,
        auto_complete = <TypeDefs as super::Types>::auto_complete,
        capture = <TypeDefs as super::Types>::capture,
        checked = <TypeDefs as super::Types>::checked,
        dirname = <TypeDefs as super::Types>::dirname,
        disabled = <TypeDefs as super::Types>::disabled,
        form = <TypeDefs as super::Types>::form,
        form_action = <TypeDefs as super::Types>::form_action,
        form_enc_type = <TypeDefs as super::Types>::form_enc_type,
        form_method = <TypeDefs as super::Types>::form_method,
        form_no_validate = <TypeDefs as super::Types>::form_no_validate,
        form_target = <TypeDefs as super::Types>::form_target,
        height = <TypeDefs as super::Types>::height,
        list = <TypeDefs as super::Types>::list,
        max = <TypeDefs as super::Types>::max,
        max_length = <TypeDefs as super::Types>::max_length,
        min = <TypeDefs as super::Types>::min,
        min_length = <TypeDefs as super::Types>::min_length,
        multiple = <TypeDefs as super::Types>::multiple,
        name = <TypeDefs as super::Types>::name,
        pattern = <TypeDefs as super::Types>::pattern,
        placeholder = <TypeDefs as super::Types>::placeholder,
        read_only = <TypeDefs as super::Types>::read_only,
        required = <TypeDefs as super::Types>::required,
        size = <TypeDefs as super::Types>::size,
        src = <TypeDefs as super::Types>::src,
        step = <TypeDefs as super::Types>::step,
        type_ = <TypeDefs as super::Types>::type_,
        value = <TypeDefs as super::Types>::value,
        width = Value,
    >;
}
mod trait_types {
    #[allow(unused_imports)]
    use super::super::*;
    #[allow(non_camel_case_types)]
    pub trait Types {
        type HtmlElementProps: ?::core::marker::Sized + HtmlElementProps::Types;
        type accept: crate::MaybeUpdateValueByRef<str>;
        type alt: crate::MaybeUpdateValueByRef<str>;
        type auto_complete: crate::MaybeUpdateValueByRef<str>;
        type capture: crate::MaybeUpdateValueByRef<str>;
        type checked: crate::MaybeUpdateValue<bool>;
        type dirname: crate::MaybeUpdateValueByRef<str>;
        type disabled: crate::MaybeUpdateValue<bool>;
        type form: crate::MaybeUpdateValueByRef<str>;
        type form_action: crate::MaybeUpdateValueByRef<str>;
        type form_enc_type: crate::MaybeUpdateValueByRef<str>;
        type form_method: crate::MaybeUpdateValueByRef<str>;
        type form_no_validate: crate::MaybeUpdateValue<bool>;
        type form_target: crate::MaybeUpdateValueByRef<str>;
        type height: crate::MaybeUpdateValue<u32>;
        type list: crate::MaybeUpdateValueByRef<str>;
        type max: crate::MaybeUpdateValueByRef<str>;
        type max_length: crate::MaybeUpdateValue<i32>;
        type min: crate::MaybeUpdateValueByRef<str>;
        type min_length: crate::MaybeUpdateValue<i32>;
        type multiple: crate::MaybeUpdateValue<bool>;
        type name: crate::MaybeUpdateValueByRef<str>;
        type pattern: crate::MaybeUpdateValueByRef<str>;
        type placeholder: crate::MaybeUpdateValueByRef<str>;
        type read_only: crate::MaybeUpdateValue<bool>;
        type required: crate::MaybeUpdateValue<bool>;
        type size: crate::MaybeUpdateValue<u32>;
        type src: crate::MaybeUpdateValueByRef<str>;
        type step: crate::MaybeUpdateValueByRef<str>;
        type type_: crate::MaybeUpdateValueByRef<str>;
        type value: crate::MaybeUpdateValueByRef<str>;
        type width: crate::MaybeUpdateValue<u32>;
    }
}
pub use trait_types::Types;
pub use trait_types::Types as ValidTypes;
pub mod data_struct {
    #[non_exhaustive]
    pub struct HtmlInputElementProps<TypeDefs: super::Types + ?::core::marker::Sized> {
        pub HtmlElementProps: super::super::HtmlElementProps::Data<TypeDefs::HtmlElementProps>,
        pub accept: TypeDefs::accept,
        pub alt: TypeDefs::alt,
        pub auto_complete: TypeDefs::auto_complete,
        pub capture: TypeDefs::capture,
        pub checked: TypeDefs::checked,
        pub dirname: TypeDefs::dirname,
        pub disabled: TypeDefs::disabled,
        pub form: TypeDefs::form,
        pub form_action: TypeDefs::form_action,
        pub form_enc_type: TypeDefs::form_enc_type,
        pub form_method: TypeDefs::form_method,
        pub form_no_validate: TypeDefs::form_no_validate,
        pub form_target: TypeDefs::form_target,
        pub height: TypeDefs::height,
        pub list: TypeDefs::list,
        pub max: TypeDefs::max,
        pub max_length: TypeDefs::max_length,
        pub min: TypeDefs::min,
        pub min_length: TypeDefs::min_length,
        pub multiple: TypeDefs::multiple,
        pub name: TypeDefs::name,
        pub pattern: TypeDefs::pattern,
        pub placeholder: TypeDefs::placeholder,
        pub read_only: TypeDefs::read_only,
        pub required: TypeDefs::required,
        pub size: TypeDefs::size,
        pub src: TypeDefs::src,
        pub step: TypeDefs::step,
        pub type_: TypeDefs::type_,
        pub value: TypeDefs::value,
        pub width: TypeDefs::width,
    }
}
pub use data_struct::HtmlInputElementProps as Data;
pub struct Building<TypeDefs: ?::core::marker::Sized + Types>(pub Data<TypeDefs>);
pub struct Replacing<TypeDefs: ?::core::marker::Sized + Types>(pub Data<TypeDefs>);
mod types_initial {
    #[allow(unused_imports)]
    use super::super::*;
    pub type TypesInitial = dyn super::Types<
        HtmlElementProps = HtmlElementProps::TypesInitial,
        accept = (),
        alt = (),
        auto_complete = (),
        capture = (),
        checked = (),
        dirname = (),
        disabled = (),
        form = (),
        form_action = (),
        form_enc_type = (),
        form_method = (),
        form_no_validate = (),
        form_target = (),
        height = (),
        list = (),
        max = (),
        max_length = (),
        min = (),
        min_length = (),
        multiple = (),
        name = (),
        pattern = (),
        placeholder = (),
        read_only = (),
        required = (),
        size = (),
        src = (),
        step = (),
        type_ = (),
        value = (),
        width = (),
    >;
}
pub use types_initial::TypesInitial;
pub type DataInitial = Data<TypesInitial>;
#[cfg(feature = "dom")]
pub use super::HtmlElementProps::render_state;
#[inline]
pub fn build<TypeDefs: ?::core::marker::Sized + Types>(
    building: Building<TypeDefs>,
) -> Data<TypeDefs> {
    building.0
}
mod builder_and_replacer {
    #[allow(unused_imports)]
    use super::super::*;
    impl<TypeDefs: super::Types + ?::core::marker::Sized> super::Building<TypeDefs> {
        #[doc = "See [`HtmlElementProps::children`]"]
        #[inline]
        pub fn children<V>(
            self,
            children: V,
        ) -> super::Building<super::overwrite::children<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps).children(children),
                ),
                accept: self.0.accept,
                alt: self.0.alt,
                auto_complete: self.0.auto_complete,
                capture: self.0.capture,
                checked: self.0.checked,
                dirname: self.0.dirname,
                disabled: self.0.disabled,
                form: self.0.form,
                form_action: self.0.form_action,
                form_enc_type: self.0.form_enc_type,
                form_method: self.0.form_method,
                form_no_validate: self.0.form_no_validate,
                form_target: self.0.form_target,
                height: self.0.height,
                list: self.0.list,
                max: self.0.max,
                max_length: self.0.max_length,
                min: self.0.min,
                min_length: self.0.min_length,
                multiple: self.0.multiple,
                name: self.0.name,
                pattern: self.0.pattern,
                placeholder: self.0.placeholder,
                read_only: self.0.read_only,
                required: self.0.required,
                size: self.0.size,
                src: self.0.src,
                step: self.0.step,
                type_: self.0.type_,
                value: self.0.value,
                width: self.0.width,
            })
        }
        #[doc = "See [`HtmlElementProps::class`]"]
        #[inline]
        pub fn class<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            class: V,
        ) -> super::Building<super::overwrite::class<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps).class(class),
                ),
                accept: self.0.accept,
                alt: self.0.alt,
                auto_complete: self.0.auto_complete,
                capture: self.0.capture,
                checked: self.0.checked,
                dirname: self.0.dirname,
                disabled: self.0.disabled,
                form: self.0.form,
                form_action: self.0.form_action,
                form_enc_type: self.0.form_enc_type,
                form_method: self.0.form_method,
                form_no_validate: self.0.form_no_validate,
                form_target: self.0.form_target,
                height: self.0.height,
                list: self.0.list,
                max: self.0.max,
                max_length: self.0.max_length,
                min: self.0.min,
                min_length: self.0.min_length,
                multiple: self.0.multiple,
                name: self.0.name,
                pattern: self.0.pattern,
                placeholder: self.0.placeholder,
                read_only: self.0.read_only,
                required: self.0.required,
                size: self.0.size,
                src: self.0.src,
                step: self.0.step,
                type_: self.0.type_,
                value: self.0.value,
                width: self.0.width,
            })
        }
        #[doc = "See [`HtmlElementProps::id`]"]
        #[inline]
        pub fn id<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            id: V,
        ) -> super::Building<super::overwrite::id<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps).id(id),
                ),
                accept: self.0.accept,
                alt: self.0.alt,
                auto_complete: self.0.auto_complete,
                capture: self.0.capture,
                checked: self.0.checked,
                dirname: self.0.dirname,
                disabled: self.0.disabled,
                form: self.0.form,
                form_action: self.0.form_action,
                form_enc_type: self.0.form_enc_type,
                form_method: self.0.form_method,
                form_no_validate: self.0.form_no_validate,
                form_target: self.0.form_target,
                height: self.0.height,
                list: self.0.list,
                max: self.0.max,
                max_length: self.0.max_length,
                min: self.0.min,
                min_length: self.0.min_length,
                multiple: self.0.multiple,
                name: self.0.name,
                pattern: self.0.pattern,
                placeholder: self.0.placeholder,
                read_only: self.0.read_only,
                required: self.0.required,
                size: self.0.size,
                src: self.0.src,
                step: self.0.step,
                type_: self.0.type_,
                value: self.0.value,
                width: self.0.width,
            })
        }
        #[doc = "See [`HtmlElementProps::part`]"]
        #[inline]
        pub fn part<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            part: V,
        ) -> super::Building<super::overwrite::part<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps).part(part),
                ),
                accept: self.0.accept,
                alt: self.0.alt,
                auto_complete: self.0.auto_complete,
                capture: self.0.capture,
                checked: self.0.checked,
                dirname: self.0.dirname,
                disabled: self.0.disabled,
                form: self.0.form,
                form_action: self.0.form_action,
                form_enc_type: self.0.form_enc_type,
                form_method: self.0.form_method,
                form_no_validate: self.0.form_no_validate,
                form_target: self.0.form_target,
                height: self.0.height,
                list: self.0.list,
                max: self.0.max,
                max_length: self.0.max_length,
                min: self.0.min,
                min_length: self.0.min_length,
                multiple: self.0.multiple,
                name: self.0.name,
                pattern: self.0.pattern,
                placeholder: self.0.placeholder,
                read_only: self.0.read_only,
                required: self.0.required,
                size: self.0.size,
                src: self.0.src,
                step: self.0.step,
                type_: self.0.type_,
                value: self.0.value,
                width: self.0.width,
            })
        }
        #[doc = "See [`HtmlElementProps::access_key`]"]
        #[inline]
        pub fn access_key<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            access_key: V,
        ) -> super::Building<super::overwrite::access_key<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps).access_key(access_key),
                ),
                accept: self.0.accept,
                alt: self.0.alt,
                auto_complete: self.0.auto_complete,
                capture: self.0.capture,
                checked: self.0.checked,
                dirname: self.0.dirname,
                disabled: self.0.disabled,
                form: self.0.form,
                form_action: self.0.form_action,
                form_enc_type: self.0.form_enc_type,
                form_method: self.0.form_method,
                form_no_validate: self.0.form_no_validate,
                form_target: self.0.form_target,
                height: self.0.height,
                list: self.0.list,
                max: self.0.max,
                max_length: self.0.max_length,
                min: self.0.min,
                min_length: self.0.min_length,
                multiple: self.0.multiple,
                name: self.0.name,
                pattern: self.0.pattern,
                placeholder: self.0.placeholder,
                read_only: self.0.read_only,
                required: self.0.required,
                size: self.0.size,
                src: self.0.src,
                step: self.0.step,
                type_: self.0.type_,
                value: self.0.value,
                width: self.0.width,
            })
        }
        #[doc = "See [`HtmlElementProps::auto_capitalize`]"]
        #[inline]
        pub fn auto_capitalize<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            auto_capitalize: V,
        ) -> super::Building<super::overwrite::auto_capitalize<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps)
                        .auto_capitalize(auto_capitalize),
                ),
                accept: self.0.accept,
                alt: self.0.alt,
                auto_complete: self.0.auto_complete,
                capture: self.0.capture,
                checked: self.0.checked,
                dirname: self.0.dirname,
                disabled: self.0.disabled,
                form: self.0.form,
                form_action: self.0.form_action,
                form_enc_type: self.0.form_enc_type,
                form_method: self.0.form_method,
                form_no_validate: self.0.form_no_validate,
                form_target: self.0.form_target,
                height: self.0.height,
                list: self.0.list,
                max: self.0.max,
                max_length: self.0.max_length,
                min: self.0.min,
                min_length: self.0.min_length,
                multiple: self.0.multiple,
                name: self.0.name,
                pattern: self.0.pattern,
                placeholder: self.0.placeholder,
                read_only: self.0.read_only,
                required: self.0.required,
                size: self.0.size,
                src: self.0.src,
                step: self.0.step,
                type_: self.0.type_,
                value: self.0.value,
                width: self.0.width,
            })
        }
        #[doc = "See [`HtmlElementProps::auto_focus`]"]
        #[inline]
        pub fn auto_focus<V: crate::MaybeUpdateValue<bool>>(
            self,
            auto_focus: V,
        ) -> super::Building<super::overwrite::auto_focus<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps).auto_focus(auto_focus),
                ),
                accept: self.0.accept,
                alt: self.0.alt,
                auto_complete: self.0.auto_complete,
                capture: self.0.capture,
                checked: self.0.checked,
                dirname: self.0.dirname,
                disabled: self.0.disabled,
                form: self.0.form,
                form_action: self.0.form_action,
                form_enc_type: self.0.form_enc_type,
                form_method: self.0.form_method,
                form_no_validate: self.0.form_no_validate,
                form_target: self.0.form_target,
                height: self.0.height,
                list: self.0.list,
                max: self.0.max,
                max_length: self.0.max_length,
                min: self.0.min,
                min_length: self.0.min_length,
                multiple: self.0.multiple,
                name: self.0.name,
                pattern: self.0.pattern,
                placeholder: self.0.placeholder,
                read_only: self.0.read_only,
                required: self.0.required,
                size: self.0.size,
                src: self.0.src,
                step: self.0.step,
                type_: self.0.type_,
                value: self.0.value,
                width: self.0.width,
            })
        }
        #[doc = "See [`HtmlElementProps::content_editable`]"]
        #[inline]
        pub fn content_editable<V: crate::props::MaybeInherit<bool>>(
            self,
            content_editable: V,
        ) -> super::Building<super::overwrite::content_editable<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps)
                        .content_editable(content_editable),
                ),
                accept: self.0.accept,
                alt: self.0.alt,
                auto_complete: self.0.auto_complete,
                capture: self.0.capture,
                checked: self.0.checked,
                dirname: self.0.dirname,
                disabled: self.0.disabled,
                form: self.0.form,
                form_action: self.0.form_action,
                form_enc_type: self.0.form_enc_type,
                form_method: self.0.form_method,
                form_no_validate: self.0.form_no_validate,
                form_target: self.0.form_target,
                height: self.0.height,
                list: self.0.list,
                max: self.0.max,
                max_length: self.0.max_length,
                min: self.0.min,
                min_length: self.0.min_length,
                multiple: self.0.multiple,
                name: self.0.name,
                pattern: self.0.pattern,
                placeholder: self.0.placeholder,
                read_only: self.0.read_only,
                required: self.0.required,
                size: self.0.size,
                src: self.0.src,
                step: self.0.step,
                type_: self.0.type_,
                value: self.0.value,
                width: self.0.width,
            })
        }
        #[doc = "See [`HtmlElementProps::context_menu`]"]
        #[inline]
        pub fn context_menu<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            context_menu: V,
        ) -> super::Building<super::overwrite::context_menu<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps).context_menu(context_menu),
                ),
                accept: self.0.accept,
                alt: self.0.alt,
                auto_complete: self.0.auto_complete,
                capture: self.0.capture,
                checked: self.0.checked,
                dirname: self.0.dirname,
                disabled: self.0.disabled,
                form: self.0.form,
                form_action: self.0.form_action,
                form_enc_type: self.0.form_enc_type,
                form_method: self.0.form_method,
                form_no_validate: self.0.form_no_validate,
                form_target: self.0.form_target,
                height: self.0.height,
                list: self.0.list,
                max: self.0.max,
                max_length: self.0.max_length,
                min: self.0.min,
                min_length: self.0.min_length,
                multiple: self.0.multiple,
                name: self.0.name,
                pattern: self.0.pattern,
                placeholder: self.0.placeholder,
                read_only: self.0.read_only,
                required: self.0.required,
                size: self.0.size,
                src: self.0.src,
                step: self.0.step,
                type_: self.0.type_,
                value: self.0.value,
                width: self.0.width,
            })
        }
        #[doc = "See [`HtmlElementProps::dir`]"]
        #[inline]
        pub fn dir<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            dir: V,
        ) -> super::Building<super::overwrite::dir<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps).dir(dir),
                ),
                accept: self.0.accept,
                alt: self.0.alt,
                auto_complete: self.0.auto_complete,
                capture: self.0.capture,
                checked: self.0.checked,
                dirname: self.0.dirname,
                disabled: self.0.disabled,
                form: self.0.form,
                form_action: self.0.form_action,
                form_enc_type: self.0.form_enc_type,
                form_method: self.0.form_method,
                form_no_validate: self.0.form_no_validate,
                form_target: self.0.form_target,
                height: self.0.height,
                list: self.0.list,
                max: self.0.max,
                max_length: self.0.max_length,
                min: self.0.min,
                min_length: self.0.min_length,
                multiple: self.0.multiple,
                name: self.0.name,
                pattern: self.0.pattern,
                placeholder: self.0.placeholder,
                read_only: self.0.read_only,
                required: self.0.required,
                size: self.0.size,
                src: self.0.src,
                step: self.0.step,
                type_: self.0.type_,
                value: self.0.value,
                width: self.0.width,
            })
        }
        #[doc = "See [`HtmlElementProps::draggable`]"]
        #[inline]
        pub fn draggable<V: crate::MaybeUpdateValue<bool>>(
            self,
            draggable: V,
        ) -> super::Building<super::overwrite::draggable<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps).draggable(draggable),
                ),
                accept: self.0.accept,
                alt: self.0.alt,
                auto_complete: self.0.auto_complete,
                capture: self.0.capture,
                checked: self.0.checked,
                dirname: self.0.dirname,
                disabled: self.0.disabled,
                form: self.0.form,
                form_action: self.0.form_action,
                form_enc_type: self.0.form_enc_type,
                form_method: self.0.form_method,
                form_no_validate: self.0.form_no_validate,
                form_target: self.0.form_target,
                height: self.0.height,
                list: self.0.list,
                max: self.0.max,
                max_length: self.0.max_length,
                min: self.0.min,
                min_length: self.0.min_length,
                multiple: self.0.multiple,
                name: self.0.name,
                pattern: self.0.pattern,
                placeholder: self.0.placeholder,
                read_only: self.0.read_only,
                required: self.0.required,
                size: self.0.size,
                src: self.0.src,
                step: self.0.step,
                type_: self.0.type_,
                value: self.0.value,
                width: self.0.width,
            })
        }
        #[doc = "See [`HtmlElementProps::enter_key_hint`]"]
        #[inline]
        pub fn enter_key_hint<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            enter_key_hint: V,
        ) -> super::Building<super::overwrite::enter_key_hint<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps)
                        .enter_key_hint(enter_key_hint),
                ),
                accept: self.0.accept,
                alt: self.0.alt,
                auto_complete: self.0.auto_complete,
                capture: self.0.capture,
                checked: self.0.checked,
                dirname: self.0.dirname,
                disabled: self.0.disabled,
                form: self.0.form,
                form_action: self.0.form_action,
                form_enc_type: self.0.form_enc_type,
                form_method: self.0.form_method,
                form_no_validate: self.0.form_no_validate,
                form_target: self.0.form_target,
                height: self.0.height,
                list: self.0.list,
                max: self.0.max,
                max_length: self.0.max_length,
                min: self.0.min,
                min_length: self.0.min_length,
                multiple: self.0.multiple,
                name: self.0.name,
                pattern: self.0.pattern,
                placeholder: self.0.placeholder,
                read_only: self.0.read_only,
                required: self.0.required,
                size: self.0.size,
                src: self.0.src,
                step: self.0.step,
                type_: self.0.type_,
                value: self.0.value,
                width: self.0.width,
            })
        }
        #[doc = "See [`HtmlElementProps::hidden`]"]
        #[inline]
        pub fn hidden<V: crate::MaybeUpdateValue<bool>>(
            self,
            hidden: V,
        ) -> super::Building<super::overwrite::hidden<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps).hidden(hidden),
                ),
                accept: self.0.accept,
                alt: self.0.alt,
                auto_complete: self.0.auto_complete,
                capture: self.0.capture,
                checked: self.0.checked,
                dirname: self.0.dirname,
                disabled: self.0.disabled,
                form: self.0.form,
                form_action: self.0.form_action,
                form_enc_type: self.0.form_enc_type,
                form_method: self.0.form_method,
                form_no_validate: self.0.form_no_validate,
                form_target: self.0.form_target,
                height: self.0.height,
                list: self.0.list,
                max: self.0.max,
                max_length: self.0.max_length,
                min: self.0.min,
                min_length: self.0.min_length,
                multiple: self.0.multiple,
                name: self.0.name,
                pattern: self.0.pattern,
                placeholder: self.0.placeholder,
                read_only: self.0.read_only,
                required: self.0.required,
                size: self.0.size,
                src: self.0.src,
                step: self.0.step,
                type_: self.0.type_,
                value: self.0.value,
                width: self.0.width,
            })
        }
        #[doc = "See [`HtmlElementProps::inert`]"]
        #[inline]
        pub fn inert<V: crate::MaybeUpdateValue<bool>>(
            self,
            inert: V,
        ) -> super::Building<super::overwrite::inert<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps).inert(inert),
                ),
                accept: self.0.accept,
                alt: self.0.alt,
                auto_complete: self.0.auto_complete,
                capture: self.0.capture,
                checked: self.0.checked,
                dirname: self.0.dirname,
                disabled: self.0.disabled,
                form: self.0.form,
                form_action: self.0.form_action,
                form_enc_type: self.0.form_enc_type,
                form_method: self.0.form_method,
                form_no_validate: self.0.form_no_validate,
                form_target: self.0.form_target,
                height: self.0.height,
                list: self.0.list,
                max: self.0.max,
                max_length: self.0.max_length,
                min: self.0.min,
                min_length: self.0.min_length,
                multiple: self.0.multiple,
                name: self.0.name,
                pattern: self.0.pattern,
                placeholder: self.0.placeholder,
                read_only: self.0.read_only,
                required: self.0.required,
                size: self.0.size,
                src: self.0.src,
                step: self.0.step,
                type_: self.0.type_,
                value: self.0.value,
                width: self.0.width,
            })
        }
        #[doc = "See [`HtmlElementProps::input_mode`]"]
        #[inline]
        pub fn input_mode<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            input_mode: V,
        ) -> super::Building<super::overwrite::input_mode<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps).input_mode(input_mode),
                ),
                accept: self.0.accept,
                alt: self.0.alt,
                auto_complete: self.0.auto_complete,
                capture: self.0.capture,
                checked: self.0.checked,
                dirname: self.0.dirname,
                disabled: self.0.disabled,
                form: self.0.form,
                form_action: self.0.form_action,
                form_enc_type: self.0.form_enc_type,
                form_method: self.0.form_method,
                form_no_validate: self.0.form_no_validate,
                form_target: self.0.form_target,
                height: self.0.height,
                list: self.0.list,
                max: self.0.max,
                max_length: self.0.max_length,
                min: self.0.min,
                min_length: self.0.min_length,
                multiple: self.0.multiple,
                name: self.0.name,
                pattern: self.0.pattern,
                placeholder: self.0.placeholder,
                read_only: self.0.read_only,
                required: self.0.required,
                size: self.0.size,
                src: self.0.src,
                step: self.0.step,
                type_: self.0.type_,
                value: self.0.value,
                width: self.0.width,
            })
        }
        #[doc = "See [`HtmlElementProps::is`]"]
        #[inline]
        pub fn is<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            is: V,
        ) -> super::Building<super::overwrite::is<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps).is(is),
                ),
                accept: self.0.accept,
                alt: self.0.alt,
                auto_complete: self.0.auto_complete,
                capture: self.0.capture,
                checked: self.0.checked,
                dirname: self.0.dirname,
                disabled: self.0.disabled,
                form: self.0.form,
                form_action: self.0.form_action,
                form_enc_type: self.0.form_enc_type,
                form_method: self.0.form_method,
                form_no_validate: self.0.form_no_validate,
                form_target: self.0.form_target,
                height: self.0.height,
                list: self.0.list,
                max: self.0.max,
                max_length: self.0.max_length,
                min: self.0.min,
                min_length: self.0.min_length,
                multiple: self.0.multiple,
                name: self.0.name,
                pattern: self.0.pattern,
                placeholder: self.0.placeholder,
                read_only: self.0.read_only,
                required: self.0.required,
                size: self.0.size,
                src: self.0.src,
                step: self.0.step,
                type_: self.0.type_,
                value: self.0.value,
                width: self.0.width,
            })
        }
        #[doc = "See [`HtmlElementProps::item_id`]"]
        #[inline]
        pub fn item_id<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            item_id: V,
        ) -> super::Building<super::overwrite::item_id<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps).item_id(item_id),
                ),
                accept: self.0.accept,
                alt: self.0.alt,
                auto_complete: self.0.auto_complete,
                capture: self.0.capture,
                checked: self.0.checked,
                dirname: self.0.dirname,
                disabled: self.0.disabled,
                form: self.0.form,
                form_action: self.0.form_action,
                form_enc_type: self.0.form_enc_type,
                form_method: self.0.form_method,
                form_no_validate: self.0.form_no_validate,
                form_target: self.0.form_target,
                height: self.0.height,
                list: self.0.list,
                max: self.0.max,
                max_length: self.0.max_length,
                min: self.0.min,
                min_length: self.0.min_length,
                multiple: self.0.multiple,
                name: self.0.name,
                pattern: self.0.pattern,
                placeholder: self.0.placeholder,
                read_only: self.0.read_only,
                required: self.0.required,
                size: self.0.size,
                src: self.0.src,
                step: self.0.step,
                type_: self.0.type_,
                value: self.0.value,
                width: self.0.width,
            })
        }
        #[doc = "See [`HtmlElementProps::item_prop`]"]
        #[inline]
        pub fn item_prop<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            item_prop: V,
        ) -> super::Building<super::overwrite::item_prop<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps).item_prop(item_prop),
                ),
                accept: self.0.accept,
                alt: self.0.alt,
                auto_complete: self.0.auto_complete,
                capture: self.0.capture,
                checked: self.0.checked,
                dirname: self.0.dirname,
                disabled: self.0.disabled,
                form: self.0.form,
                form_action: self.0.form_action,
                form_enc_type: self.0.form_enc_type,
                form_method: self.0.form_method,
                form_no_validate: self.0.form_no_validate,
                form_target: self.0.form_target,
                height: self.0.height,
                list: self.0.list,
                max: self.0.max,
                max_length: self.0.max_length,
                min: self.0.min,
                min_length: self.0.min_length,
                multiple: self.0.multiple,
                name: self.0.name,
                pattern: self.0.pattern,
                placeholder: self.0.placeholder,
                read_only: self.0.read_only,
                required: self.0.required,
                size: self.0.size,
                src: self.0.src,
                step: self.0.step,
                type_: self.0.type_,
                value: self.0.value,
                width: self.0.width,
            })
        }
        #[doc = "See [`HtmlElementProps::item_ref`]"]
        #[inline]
        pub fn item_ref<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            item_ref: V,
        ) -> super::Building<super::overwrite::item_ref<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps).item_ref(item_ref),
                ),
                accept: self.0.accept,
                alt: self.0.alt,
                auto_complete: self.0.auto_complete,
                capture: self.0.capture,
                checked: self.0.checked,
                dirname: self.0.dirname,
                disabled: self.0.disabled,
                form: self.0.form,
                form_action: self.0.form_action,
                form_enc_type: self.0.form_enc_type,
                form_method: self.0.form_method,
                form_no_validate: self.0.form_no_validate,
                form_target: self.0.form_target,
                height: self.0.height,
                list: self.0.list,
                max: self.0.max,
                max_length: self.0.max_length,
                min: self.0.min,
                min_length: self.0.min_length,
                multiple: self.0.multiple,
                name: self.0.name,
                pattern: self.0.pattern,
                placeholder: self.0.placeholder,
                read_only: self.0.read_only,
                required: self.0.required,
                size: self.0.size,
                src: self.0.src,
                step: self.0.step,
                type_: self.0.type_,
                value: self.0.value,
                width: self.0.width,
            })
        }
        #[doc = "See [`HtmlElementProps::item_scope`]"]
        #[inline]
        pub fn item_scope<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            item_scope: V,
        ) -> super::Building<super::overwrite::item_scope<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps).item_scope(item_scope),
                ),
                accept: self.0.accept,
                alt: self.0.alt,
                auto_complete: self.0.auto_complete,
                capture: self.0.capture,
                checked: self.0.checked,
                dirname: self.0.dirname,
                disabled: self.0.disabled,
                form: self.0.form,
                form_action: self.0.form_action,
                form_enc_type: self.0.form_enc_type,
                form_method: self.0.form_method,
                form_no_validate: self.0.form_no_validate,
                form_target: self.0.form_target,
                height: self.0.height,
                list: self.0.list,
                max: self.0.max,
                max_length: self.0.max_length,
                min: self.0.min,
                min_length: self.0.min_length,
                multiple: self.0.multiple,
                name: self.0.name,
                pattern: self.0.pattern,
                placeholder: self.0.placeholder,
                read_only: self.0.read_only,
                required: self.0.required,
                size: self.0.size,
                src: self.0.src,
                step: self.0.step,
                type_: self.0.type_,
                value: self.0.value,
                width: self.0.width,
            })
        }
        #[doc = "See [`HtmlElementProps::item_type`]"]
        #[inline]
        pub fn item_type<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            item_type: V,
        ) -> super::Building<super::overwrite::item_type<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps).item_type(item_type),
                ),
                accept: self.0.accept,
                alt: self.0.alt,
                auto_complete: self.0.auto_complete,
                capture: self.0.capture,
                checked: self.0.checked,
                dirname: self.0.dirname,
                disabled: self.0.disabled,
                form: self.0.form,
                form_action: self.0.form_action,
                form_enc_type: self.0.form_enc_type,
                form_method: self.0.form_method,
                form_no_validate: self.0.form_no_validate,
                form_target: self.0.form_target,
                height: self.0.height,
                list: self.0.list,
                max: self.0.max,
                max_length: self.0.max_length,
                min: self.0.min,
                min_length: self.0.min_length,
                multiple: self.0.multiple,
                name: self.0.name,
                pattern: self.0.pattern,
                placeholder: self.0.placeholder,
                read_only: self.0.read_only,
                required: self.0.required,
                size: self.0.size,
                src: self.0.src,
                step: self.0.step,
                type_: self.0.type_,
                value: self.0.value,
                width: self.0.width,
            })
        }
        #[doc = "See [`HtmlElementProps::lang`]"]
        #[inline]
        pub fn lang<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            lang: V,
        ) -> super::Building<super::overwrite::lang<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps).lang(lang),
                ),
                accept: self.0.accept,
                alt: self.0.alt,
                auto_complete: self.0.auto_complete,
                capture: self.0.capture,
                checked: self.0.checked,
                dirname: self.0.dirname,
                disabled: self.0.disabled,
                form: self.0.form,
                form_action: self.0.form_action,
                form_enc_type: self.0.form_enc_type,
                form_method: self.0.form_method,
                form_no_validate: self.0.form_no_validate,
                form_target: self.0.form_target,
                height: self.0.height,
                list: self.0.list,
                max: self.0.max,
                max_length: self.0.max_length,
                min: self.0.min,
                min_length: self.0.min_length,
                multiple: self.0.multiple,
                name: self.0.name,
                pattern: self.0.pattern,
                placeholder: self.0.placeholder,
                read_only: self.0.read_only,
                required: self.0.required,
                size: self.0.size,
                src: self.0.src,
                step: self.0.step,
                type_: self.0.type_,
                value: self.0.value,
                width: self.0.width,
            })
        }
        #[doc = "See [`HtmlElementProps::nonce`]"]
        #[inline]
        pub fn nonce<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            nonce: V,
        ) -> super::Building<super::overwrite::nonce<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps).nonce(nonce),
                ),
                accept: self.0.accept,
                alt: self.0.alt,
                auto_complete: self.0.auto_complete,
                capture: self.0.capture,
                checked: self.0.checked,
                dirname: self.0.dirname,
                disabled: self.0.disabled,
                form: self.0.form,
                form_action: self.0.form_action,
                form_enc_type: self.0.form_enc_type,
                form_method: self.0.form_method,
                form_no_validate: self.0.form_no_validate,
                form_target: self.0.form_target,
                height: self.0.height,
                list: self.0.list,
                max: self.0.max,
                max_length: self.0.max_length,
                min: self.0.min,
                min_length: self.0.min_length,
                multiple: self.0.multiple,
                name: self.0.name,
                pattern: self.0.pattern,
                placeholder: self.0.placeholder,
                read_only: self.0.read_only,
                required: self.0.required,
                size: self.0.size,
                src: self.0.src,
                step: self.0.step,
                type_: self.0.type_,
                value: self.0.value,
                width: self.0.width,
            })
        }
        #[doc = "See [`HtmlElementProps::role`]"]
        #[inline]
        pub fn role<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            role: V,
        ) -> super::Building<super::overwrite::role<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps).role(role),
                ),
                accept: self.0.accept,
                alt: self.0.alt,
                auto_complete: self.0.auto_complete,
                capture: self.0.capture,
                checked: self.0.checked,
                dirname: self.0.dirname,
                disabled: self.0.disabled,
                form: self.0.form,
                form_action: self.0.form_action,
                form_enc_type: self.0.form_enc_type,
                form_method: self.0.form_method,
                form_no_validate: self.0.form_no_validate,
                form_target: self.0.form_target,
                height: self.0.height,
                list: self.0.list,
                max: self.0.max,
                max_length: self.0.max_length,
                min: self.0.min,
                min_length: self.0.min_length,
                multiple: self.0.multiple,
                name: self.0.name,
                pattern: self.0.pattern,
                placeholder: self.0.placeholder,
                read_only: self.0.read_only,
                required: self.0.required,
                size: self.0.size,
                src: self.0.src,
                step: self.0.step,
                type_: self.0.type_,
                value: self.0.value,
                width: self.0.width,
            })
        }
        #[doc = "See [`HtmlElementProps::slot`]"]
        #[inline]
        pub fn slot<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            slot: V,
        ) -> super::Building<super::overwrite::slot<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps).slot(slot),
                ),
                accept: self.0.accept,
                alt: self.0.alt,
                auto_complete: self.0.auto_complete,
                capture: self.0.capture,
                checked: self.0.checked,
                dirname: self.0.dirname,
                disabled: self.0.disabled,
                form: self.0.form,
                form_action: self.0.form_action,
                form_enc_type: self.0.form_enc_type,
                form_method: self.0.form_method,
                form_no_validate: self.0.form_no_validate,
                form_target: self.0.form_target,
                height: self.0.height,
                list: self.0.list,
                max: self.0.max,
                max_length: self.0.max_length,
                min: self.0.min,
                min_length: self.0.min_length,
                multiple: self.0.multiple,
                name: self.0.name,
                pattern: self.0.pattern,
                placeholder: self.0.placeholder,
                read_only: self.0.read_only,
                required: self.0.required,
                size: self.0.size,
                src: self.0.src,
                step: self.0.step,
                type_: self.0.type_,
                value: self.0.value,
                width: self.0.width,
            })
        }
        #[doc = "See [`HtmlElementProps::spellcheck`]"]
        #[inline]
        pub fn spellcheck<V: crate::MaybeUpdateValue<bool>>(
            self,
            spellcheck: V,
        ) -> super::Building<super::overwrite::spellcheck<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps).spellcheck(spellcheck),
                ),
                accept: self.0.accept,
                alt: self.0.alt,
                auto_complete: self.0.auto_complete,
                capture: self.0.capture,
                checked: self.0.checked,
                dirname: self.0.dirname,
                disabled: self.0.disabled,
                form: self.0.form,
                form_action: self.0.form_action,
                form_enc_type: self.0.form_enc_type,
                form_method: self.0.form_method,
                form_no_validate: self.0.form_no_validate,
                form_target: self.0.form_target,
                height: self.0.height,
                list: self.0.list,
                max: self.0.max,
                max_length: self.0.max_length,
                min: self.0.min,
                min_length: self.0.min_length,
                multiple: self.0.multiple,
                name: self.0.name,
                pattern: self.0.pattern,
                placeholder: self.0.placeholder,
                read_only: self.0.read_only,
                required: self.0.required,
                size: self.0.size,
                src: self.0.src,
                step: self.0.step,
                type_: self.0.type_,
                value: self.0.value,
                width: self.0.width,
            })
        }
        #[doc = "See [`HtmlElementProps::style`]"]
        #[inline]
        pub fn style<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            style: V,
        ) -> super::Building<super::overwrite::style<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps).style(style),
                ),
                accept: self.0.accept,
                alt: self.0.alt,
                auto_complete: self.0.auto_complete,
                capture: self.0.capture,
                checked: self.0.checked,
                dirname: self.0.dirname,
                disabled: self.0.disabled,
                form: self.0.form,
                form_action: self.0.form_action,
                form_enc_type: self.0.form_enc_type,
                form_method: self.0.form_method,
                form_no_validate: self.0.form_no_validate,
                form_target: self.0.form_target,
                height: self.0.height,
                list: self.0.list,
                max: self.0.max,
                max_length: self.0.max_length,
                min: self.0.min,
                min_length: self.0.min_length,
                multiple: self.0.multiple,
                name: self.0.name,
                pattern: self.0.pattern,
                placeholder: self.0.placeholder,
                read_only: self.0.read_only,
                required: self.0.required,
                size: self.0.size,
                src: self.0.src,
                step: self.0.step,
                type_: self.0.type_,
                value: self.0.value,
                width: self.0.width,
            })
        }
        #[doc = "See [`HtmlElementProps::tab_index`]"]
        #[inline]
        pub fn tab_index<V: crate::MaybeUpdateValue<i32>>(
            self,
            tab_index: V,
        ) -> super::Building<super::overwrite::tab_index<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps).tab_index(tab_index),
                ),
                accept: self.0.accept,
                alt: self.0.alt,
                auto_complete: self.0.auto_complete,
                capture: self.0.capture,
                checked: self.0.checked,
                dirname: self.0.dirname,
                disabled: self.0.disabled,
                form: self.0.form,
                form_action: self.0.form_action,
                form_enc_type: self.0.form_enc_type,
                form_method: self.0.form_method,
                form_no_validate: self.0.form_no_validate,
                form_target: self.0.form_target,
                height: self.0.height,
                list: self.0.list,
                max: self.0.max,
                max_length: self.0.max_length,
                min: self.0.min,
                min_length: self.0.min_length,
                multiple: self.0.multiple,
                name: self.0.name,
                pattern: self.0.pattern,
                placeholder: self.0.placeholder,
                read_only: self.0.read_only,
                required: self.0.required,
                size: self.0.size,
                src: self.0.src,
                step: self.0.step,
                type_: self.0.type_,
                value: self.0.value,
                width: self.0.width,
            })
        }
        #[doc = "See [`HtmlElementProps::title`]"]
        #[inline]
        pub fn title<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            title: V,
        ) -> super::Building<super::overwrite::title<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps).title(title),
                ),
                accept: self.0.accept,
                alt: self.0.alt,
                auto_complete: self.0.auto_complete,
                capture: self.0.capture,
                checked: self.0.checked,
                dirname: self.0.dirname,
                disabled: self.0.disabled,
                form: self.0.form,
                form_action: self.0.form_action,
                form_enc_type: self.0.form_enc_type,
                form_method: self.0.form_method,
                form_no_validate: self.0.form_no_validate,
                form_target: self.0.form_target,
                height: self.0.height,
                list: self.0.list,
                max: self.0.max,
                max_length: self.0.max_length,
                min: self.0.min,
                min_length: self.0.min_length,
                multiple: self.0.multiple,
                name: self.0.name,
                pattern: self.0.pattern,
                placeholder: self.0.placeholder,
                read_only: self.0.read_only,
                required: self.0.required,
                size: self.0.size,
                src: self.0.src,
                step: self.0.step,
                type_: self.0.type_,
                value: self.0.value,
                width: self.0.width,
            })
        }
        #[doc = "See [`HtmlElementProps::translate`]"]
        #[inline]
        pub fn translate<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            translate: V,
        ) -> super::Building<super::overwrite::translate<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps).translate(translate),
                ),
                accept: self.0.accept,
                alt: self.0.alt,
                auto_complete: self.0.auto_complete,
                capture: self.0.capture,
                checked: self.0.checked,
                dirname: self.0.dirname,
                disabled: self.0.disabled,
                form: self.0.form,
                form_action: self.0.form_action,
                form_enc_type: self.0.form_enc_type,
                form_method: self.0.form_method,
                form_no_validate: self.0.form_no_validate,
                form_target: self.0.form_target,
                height: self.0.height,
                list: self.0.list,
                max: self.0.max,
                max_length: self.0.max_length,
                min: self.0.min,
                min_length: self.0.min_length,
                multiple: self.0.multiple,
                name: self.0.name,
                pattern: self.0.pattern,
                placeholder: self.0.placeholder,
                read_only: self.0.read_only,
                required: self.0.required,
                size: self.0.size,
                src: self.0.src,
                step: self.0.step,
                type_: self.0.type_,
                value: self.0.value,
                width: self.0.width,
            })
        }
        #[doc = "See [`HtmlElementProps::virtual_keyboard_policy`]"]
        #[inline]
        pub fn virtual_keyboard_policy<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            virtual_keyboard_policy: V,
        ) -> super::Building<super::overwrite::virtual_keyboard_policy<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps)
                        .virtual_keyboard_policy(virtual_keyboard_policy),
                ),
                accept: self.0.accept,
                alt: self.0.alt,
                auto_complete: self.0.auto_complete,
                capture: self.0.capture,
                checked: self.0.checked,
                dirname: self.0.dirname,
                disabled: self.0.disabled,
                form: self.0.form,
                form_action: self.0.form_action,
                form_enc_type: self.0.form_enc_type,
                form_method: self.0.form_method,
                form_no_validate: self.0.form_no_validate,
                form_target: self.0.form_target,
                height: self.0.height,
                list: self.0.list,
                max: self.0.max,
                max_length: self.0.max_length,
                min: self.0.min,
                min_length: self.0.min_length,
                multiple: self.0.multiple,
                name: self.0.name,
                pattern: self.0.pattern,
                placeholder: self.0.placeholder,
                read_only: self.0.read_only,
                required: self.0.required,
                size: self.0.size,
                src: self.0.src,
                step: self.0.step,
                type_: self.0.type_,
                value: self.0.value,
                width: self.0.width,
            })
        }
        #[doc = "See [`HtmlElementProps::on_click`]"]
        #[inline]
        pub fn on_click<V>(
            self,
            on_click: V,
        ) -> super::Building<super::overwrite::on_click<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: HtmlElementProps::build(
                    HtmlElementProps::Building(self.0.HtmlElementProps).on_click(on_click),
                ),
                accept: self.0.accept,
                alt: self.0.alt,
                auto_complete: self.0.auto_complete,
                capture: self.0.capture,
                checked: self.0.checked,
                dirname: self.0.dirname,
                disabled: self.0.disabled,
                form: self.0.form,
                form_action: self.0.form_action,
                form_enc_type: self.0.form_enc_type,
                form_method: self.0.form_method,
                form_no_validate: self.0.form_no_validate,
                form_target: self.0.form_target,
                height: self.0.height,
                list: self.0.list,
                max: self.0.max,
                max_length: self.0.max_length,
                min: self.0.min,
                min_length: self.0.min_length,
                multiple: self.0.multiple,
                name: self.0.name,
                pattern: self.0.pattern,
                placeholder: self.0.placeholder,
                read_only: self.0.read_only,
                required: self.0.required,
                size: self.0.size,
                src: self.0.src,
                step: self.0.step,
                type_: self.0.type_,
                value: self.0.value,
                width: self.0.width,
            })
        }
        #[inline]
        pub fn accept<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            accept: V,
        ) -> super::Building<super::overwrite::accept<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                accept,
                alt: self.0.alt,
                auto_complete: self.0.auto_complete,
                capture: self.0.capture,
                checked: self.0.checked,
                dirname: self.0.dirname,
                disabled: self.0.disabled,
                form: self.0.form,
                form_action: self.0.form_action,
                form_enc_type: self.0.form_enc_type,
                form_method: self.0.form_method,
                form_no_validate: self.0.form_no_validate,
                form_target: self.0.form_target,
                height: self.0.height,
                list: self.0.list,
                max: self.0.max,
                max_length: self.0.max_length,
                min: self.0.min,
                min_length: self.0.min_length,
                multiple: self.0.multiple,
                name: self.0.name,
                pattern: self.0.pattern,
                placeholder: self.0.placeholder,
                read_only: self.0.read_only,
                required: self.0.required,
                size: self.0.size,
                src: self.0.src,
                step: self.0.step,
                type_: self.0.type_,
                value: self.0.value,
                width: self.0.width,
            })
        }
        #[inline]
        pub fn alt<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            alt: V,
        ) -> super::Building<super::overwrite::alt<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                accept: self.0.accept,
                alt,
                auto_complete: self.0.auto_complete,
                capture: self.0.capture,
                checked: self.0.checked,
                dirname: self.0.dirname,
                disabled: self.0.disabled,
                form: self.0.form,
                form_action: self.0.form_action,
                form_enc_type: self.0.form_enc_type,
                form_method: self.0.form_method,
                form_no_validate: self.0.form_no_validate,
                form_target: self.0.form_target,
                height: self.0.height,
                list: self.0.list,
                max: self.0.max,
                max_length: self.0.max_length,
                min: self.0.min,
                min_length: self.0.min_length,
                multiple: self.0.multiple,
                name: self.0.name,
                pattern: self.0.pattern,
                placeholder: self.0.placeholder,
                read_only: self.0.read_only,
                required: self.0.required,
                size: self.0.size,
                src: self.0.src,
                step: self.0.step,
                type_: self.0.type_,
                value: self.0.value,
                width: self.0.width,
            })
        }
        #[inline]
        pub fn auto_complete<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            auto_complete: V,
        ) -> super::Building<super::overwrite::auto_complete<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                accept: self.0.accept,
                alt: self.0.alt,
                auto_complete,
                capture: self.0.capture,
                checked: self.0.checked,
                dirname: self.0.dirname,
                disabled: self.0.disabled,
                form: self.0.form,
                form_action: self.0.form_action,
                form_enc_type: self.0.form_enc_type,
                form_method: self.0.form_method,
                form_no_validate: self.0.form_no_validate,
                form_target: self.0.form_target,
                height: self.0.height,
                list: self.0.list,
                max: self.0.max,
                max_length: self.0.max_length,
                min: self.0.min,
                min_length: self.0.min_length,
                multiple: self.0.multiple,
                name: self.0.name,
                pattern: self.0.pattern,
                placeholder: self.0.placeholder,
                read_only: self.0.read_only,
                required: self.0.required,
                size: self.0.size,
                src: self.0.src,
                step: self.0.step,
                type_: self.0.type_,
                value: self.0.value,
                width: self.0.width,
            })
        }
        #[inline]
        pub fn capture<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            capture: V,
        ) -> super::Building<super::overwrite::capture<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                accept: self.0.accept,
                alt: self.0.alt,
                auto_complete: self.0.auto_complete,
                capture,
                checked: self.0.checked,
                dirname: self.0.dirname,
                disabled: self.0.disabled,
                form: self.0.form,
                form_action: self.0.form_action,
                form_enc_type: self.0.form_enc_type,
                form_method: self.0.form_method,
                form_no_validate: self.0.form_no_validate,
                form_target: self.0.form_target,
                height: self.0.height,
                list: self.0.list,
                max: self.0.max,
                max_length: self.0.max_length,
                min: self.0.min,
                min_length: self.0.min_length,
                multiple: self.0.multiple,
                name: self.0.name,
                pattern: self.0.pattern,
                placeholder: self.0.placeholder,
                read_only: self.0.read_only,
                required: self.0.required,
                size: self.0.size,
                src: self.0.src,
                step: self.0.step,
                type_: self.0.type_,
                value: self.0.value,
                width: self.0.width,
            })
        }
        #[inline]
        pub fn checked<V: crate::MaybeUpdateValue<bool>>(
            self,
            checked: V,
        ) -> super::Building<super::overwrite::checked<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                accept: self.0.accept,
                alt: self.0.alt,
                auto_complete: self.0.auto_complete,
                capture: self.0.capture,
                checked,
                dirname: self.0.dirname,
                disabled: self.0.disabled,
                form: self.0.form,
                form_action: self.0.form_action,
                form_enc_type: self.0.form_enc_type,
                form_method: self.0.form_method,
                form_no_validate: self.0.form_no_validate,
                form_target: self.0.form_target,
                height: self.0.height,
                list: self.0.list,
                max: self.0.max,
                max_length: self.0.max_length,
                min: self.0.min,
                min_length: self.0.min_length,
                multiple: self.0.multiple,
                name: self.0.name,
                pattern: self.0.pattern,
                placeholder: self.0.placeholder,
                read_only: self.0.read_only,
                required: self.0.required,
                size: self.0.size,
                src: self.0.src,
                step: self.0.step,
                type_: self.0.type_,
                value: self.0.value,
                width: self.0.width,
            })
        }
        #[inline]
        pub fn dirname<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            dirname: V,
        ) -> super::Building<super::overwrite::dirname<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                accept: self.0.accept,
                alt: self.0.alt,
                auto_complete: self.0.auto_complete,
                capture: self.0.capture,
                checked: self.0.checked,
                dirname,
                disabled: self.0.disabled,
                form: self.0.form,
                form_action: self.0.form_action,
                form_enc_type: self.0.form_enc_type,
                form_method: self.0.form_method,
                form_no_validate: self.0.form_no_validate,
                form_target: self.0.form_target,
                height: self.0.height,
                list: self.0.list,
                max: self.0.max,
                max_length: self.0.max_length,
                min: self.0.min,
                min_length: self.0.min_length,
                multiple: self.0.multiple,
                name: self.0.name,
                pattern: self.0.pattern,
                placeholder: self.0.placeholder,
                read_only: self.0.read_only,
                required: self.0.required,
                size: self.0.size,
                src: self.0.src,
                step: self.0.step,
                type_: self.0.type_,
                value: self.0.value,
                width: self.0.width,
            })
        }
        #[inline]
        pub fn disabled<V: crate::MaybeUpdateValue<bool>>(
            self,
            disabled: V,
        ) -> super::Building<super::overwrite::disabled<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                accept: self.0.accept,
                alt: self.0.alt,
                auto_complete: self.0.auto_complete,
                capture: self.0.capture,
                checked: self.0.checked,
                dirname: self.0.dirname,
                disabled,
                form: self.0.form,
                form_action: self.0.form_action,
                form_enc_type: self.0.form_enc_type,
                form_method: self.0.form_method,
                form_no_validate: self.0.form_no_validate,
                form_target: self.0.form_target,
                height: self.0.height,
                list: self.0.list,
                max: self.0.max,
                max_length: self.0.max_length,
                min: self.0.min,
                min_length: self.0.min_length,
                multiple: self.0.multiple,
                name: self.0.name,
                pattern: self.0.pattern,
                placeholder: self.0.placeholder,
                read_only: self.0.read_only,
                required: self.0.required,
                size: self.0.size,
                src: self.0.src,
                step: self.0.step,
                type_: self.0.type_,
                value: self.0.value,
                width: self.0.width,
            })
        }
        #[inline]
        pub fn form<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            form: V,
        ) -> super::Building<super::overwrite::form<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                accept: self.0.accept,
                alt: self.0.alt,
                auto_complete: self.0.auto_complete,
                capture: self.0.capture,
                checked: self.0.checked,
                dirname: self.0.dirname,
                disabled: self.0.disabled,
                form,
                form_action: self.0.form_action,
                form_enc_type: self.0.form_enc_type,
                form_method: self.0.form_method,
                form_no_validate: self.0.form_no_validate,
                form_target: self.0.form_target,
                height: self.0.height,
                list: self.0.list,
                max: self.0.max,
                max_length: self.0.max_length,
                min: self.0.min,
                min_length: self.0.min_length,
                multiple: self.0.multiple,
                name: self.0.name,
                pattern: self.0.pattern,
                placeholder: self.0.placeholder,
                read_only: self.0.read_only,
                required: self.0.required,
                size: self.0.size,
                src: self.0.src,
                step: self.0.step,
                type_: self.0.type_,
                value: self.0.value,
                width: self.0.width,
            })
        }
        #[inline]
        pub fn form_action<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            form_action: V,
        ) -> super::Building<super::overwrite::form_action<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                accept: self.0.accept,
                alt: self.0.alt,
                auto_complete: self.0.auto_complete,
                capture: self.0.capture,
                checked: self.0.checked,
                dirname: self.0.dirname,
                disabled: self.0.disabled,
                form: self.0.form,
                form_action,
                form_enc_type: self.0.form_enc_type,
                form_method: self.0.form_method,
                form_no_validate: self.0.form_no_validate,
                form_target: self.0.form_target,
                height: self.0.height,
                list: self.0.list,
                max: self.0.max,
                max_length: self.0.max_length,
                min: self.0.min,
                min_length: self.0.min_length,
                multiple: self.0.multiple,
                name: self.0.name,
                pattern: self.0.pattern,
                placeholder: self.0.placeholder,
                read_only: self.0.read_only,
                required: self.0.required,
                size: self.0.size,
                src: self.0.src,
                step: self.0.step,
                type_: self.0.type_,
                value: self.0.value,
                width: self.0.width,
            })
        }
        #[inline]
        pub fn form_enc_type<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            form_enc_type: V,
        ) -> super::Building<super::overwrite::form_enc_type<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                accept: self.0.accept,
                alt: self.0.alt,
                auto_complete: self.0.auto_complete,
                capture: self.0.capture,
                checked: self.0.checked,
                dirname: self.0.dirname,
                disabled: self.0.disabled,
                form: self.0.form,
                form_action: self.0.form_action,
                form_enc_type,
                form_method: self.0.form_method,
                form_no_validate: self.0.form_no_validate,
                form_target: self.0.form_target,
                height: self.0.height,
                list: self.0.list,
                max: self.0.max,
                max_length: self.0.max_length,
                min: self.0.min,
                min_length: self.0.min_length,
                multiple: self.0.multiple,
                name: self.0.name,
                pattern: self.0.pattern,
                placeholder: self.0.placeholder,
                read_only: self.0.read_only,
                required: self.0.required,
                size: self.0.size,
                src: self.0.src,
                step: self.0.step,
                type_: self.0.type_,
                value: self.0.value,
                width: self.0.width,
            })
        }
        #[inline]
        pub fn form_method<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            form_method: V,
        ) -> super::Building<super::overwrite::form_method<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                accept: self.0.accept,
                alt: self.0.alt,
                auto_complete: self.0.auto_complete,
                capture: self.0.capture,
                checked: self.0.checked,
                dirname: self.0.dirname,
                disabled: self.0.disabled,
                form: self.0.form,
                form_action: self.0.form_action,
                form_enc_type: self.0.form_enc_type,
                form_method,
                form_no_validate: self.0.form_no_validate,
                form_target: self.0.form_target,
                height: self.0.height,
                list: self.0.list,
                max: self.0.max,
                max_length: self.0.max_length,
                min: self.0.min,
                min_length: self.0.min_length,
                multiple: self.0.multiple,
                name: self.0.name,
                pattern: self.0.pattern,
                placeholder: self.0.placeholder,
                read_only: self.0.read_only,
                required: self.0.required,
                size: self.0.size,
                src: self.0.src,
                step: self.0.step,
                type_: self.0.type_,
                value: self.0.value,
                width: self.0.width,
            })
        }
        #[inline]
        pub fn form_no_validate<V: crate::MaybeUpdateValue<bool>>(
            self,
            form_no_validate: V,
        ) -> super::Building<super::overwrite::form_no_validate<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                accept: self.0.accept,
                alt: self.0.alt,
                auto_complete: self.0.auto_complete,
                capture: self.0.capture,
                checked: self.0.checked,
                dirname: self.0.dirname,
                disabled: self.0.disabled,
                form: self.0.form,
                form_action: self.0.form_action,
                form_enc_type: self.0.form_enc_type,
                form_method: self.0.form_method,
                form_no_validate,
                form_target: self.0.form_target,
                height: self.0.height,
                list: self.0.list,
                max: self.0.max,
                max_length: self.0.max_length,
                min: self.0.min,
                min_length: self.0.min_length,
                multiple: self.0.multiple,
                name: self.0.name,
                pattern: self.0.pattern,
                placeholder: self.0.placeholder,
                read_only: self.0.read_only,
                required: self.0.required,
                size: self.0.size,
                src: self.0.src,
                step: self.0.step,
                type_: self.0.type_,
                value: self.0.value,
                width: self.0.width,
            })
        }
        #[inline]
        pub fn form_target<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            form_target: V,
        ) -> super::Building<super::overwrite::form_target<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                accept: self.0.accept,
                alt: self.0.alt,
                auto_complete: self.0.auto_complete,
                capture: self.0.capture,
                checked: self.0.checked,
                dirname: self.0.dirname,
                disabled: self.0.disabled,
                form: self.0.form,
                form_action: self.0.form_action,
                form_enc_type: self.0.form_enc_type,
                form_method: self.0.form_method,
                form_no_validate: self.0.form_no_validate,
                form_target,
                height: self.0.height,
                list: self.0.list,
                max: self.0.max,
                max_length: self.0.max_length,
                min: self.0.min,
                min_length: self.0.min_length,
                multiple: self.0.multiple,
                name: self.0.name,
                pattern: self.0.pattern,
                placeholder: self.0.placeholder,
                read_only: self.0.read_only,
                required: self.0.required,
                size: self.0.size,
                src: self.0.src,
                step: self.0.step,
                type_: self.0.type_,
                value: self.0.value,
                width: self.0.width,
            })
        }
        #[inline]
        pub fn height<V: crate::MaybeUpdateValue<u32>>(
            self,
            height: V,
        ) -> super::Building<super::overwrite::height<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                accept: self.0.accept,
                alt: self.0.alt,
                auto_complete: self.0.auto_complete,
                capture: self.0.capture,
                checked: self.0.checked,
                dirname: self.0.dirname,
                disabled: self.0.disabled,
                form: self.0.form,
                form_action: self.0.form_action,
                form_enc_type: self.0.form_enc_type,
                form_method: self.0.form_method,
                form_no_validate: self.0.form_no_validate,
                form_target: self.0.form_target,
                height,
                list: self.0.list,
                max: self.0.max,
                max_length: self.0.max_length,
                min: self.0.min,
                min_length: self.0.min_length,
                multiple: self.0.multiple,
                name: self.0.name,
                pattern: self.0.pattern,
                placeholder: self.0.placeholder,
                read_only: self.0.read_only,
                required: self.0.required,
                size: self.0.size,
                src: self.0.src,
                step: self.0.step,
                type_: self.0.type_,
                value: self.0.value,
                width: self.0.width,
            })
        }
        #[inline]
        pub fn list<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            list: V,
        ) -> super::Building<super::overwrite::list<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                accept: self.0.accept,
                alt: self.0.alt,
                auto_complete: self.0.auto_complete,
                capture: self.0.capture,
                checked: self.0.checked,
                dirname: self.0.dirname,
                disabled: self.0.disabled,
                form: self.0.form,
                form_action: self.0.form_action,
                form_enc_type: self.0.form_enc_type,
                form_method: self.0.form_method,
                form_no_validate: self.0.form_no_validate,
                form_target: self.0.form_target,
                height: self.0.height,
                list,
                max: self.0.max,
                max_length: self.0.max_length,
                min: self.0.min,
                min_length: self.0.min_length,
                multiple: self.0.multiple,
                name: self.0.name,
                pattern: self.0.pattern,
                placeholder: self.0.placeholder,
                read_only: self.0.read_only,
                required: self.0.required,
                size: self.0.size,
                src: self.0.src,
                step: self.0.step,
                type_: self.0.type_,
                value: self.0.value,
                width: self.0.width,
            })
        }
        #[inline]
        pub fn max<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            max: V,
        ) -> super::Building<super::overwrite::max<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                accept: self.0.accept,
                alt: self.0.alt,
                auto_complete: self.0.auto_complete,
                capture: self.0.capture,
                checked: self.0.checked,
                dirname: self.0.dirname,
                disabled: self.0.disabled,
                form: self.0.form,
                form_action: self.0.form_action,
                form_enc_type: self.0.form_enc_type,
                form_method: self.0.form_method,
                form_no_validate: self.0.form_no_validate,
                form_target: self.0.form_target,
                height: self.0.height,
                list: self.0.list,
                max,
                max_length: self.0.max_length,
                min: self.0.min,
                min_length: self.0.min_length,
                multiple: self.0.multiple,
                name: self.0.name,
                pattern: self.0.pattern,
                placeholder: self.0.placeholder,
                read_only: self.0.read_only,
                required: self.0.required,
                size: self.0.size,
                src: self.0.src,
                step: self.0.step,
                type_: self.0.type_,
                value: self.0.value,
                width: self.0.width,
            })
        }
        #[inline]
        pub fn max_length<V: crate::MaybeUpdateValue<i32>>(
            self,
            max_length: V,
        ) -> super::Building<super::overwrite::max_length<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                accept: self.0.accept,
                alt: self.0.alt,
                auto_complete: self.0.auto_complete,
                capture: self.0.capture,
                checked: self.0.checked,
                dirname: self.0.dirname,
                disabled: self.0.disabled,
                form: self.0.form,
                form_action: self.0.form_action,
                form_enc_type: self.0.form_enc_type,
                form_method: self.0.form_method,
                form_no_validate: self.0.form_no_validate,
                form_target: self.0.form_target,
                height: self.0.height,
                list: self.0.list,
                max: self.0.max,
                max_length,
                min: self.0.min,
                min_length: self.0.min_length,
                multiple: self.0.multiple,
                name: self.0.name,
                pattern: self.0.pattern,
                placeholder: self.0.placeholder,
                read_only: self.0.read_only,
                required: self.0.required,
                size: self.0.size,
                src: self.0.src,
                step: self.0.step,
                type_: self.0.type_,
                value: self.0.value,
                width: self.0.width,
            })
        }
        #[inline]
        pub fn min<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            min: V,
        ) -> super::Building<super::overwrite::min<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                accept: self.0.accept,
                alt: self.0.alt,
                auto_complete: self.0.auto_complete,
                capture: self.0.capture,
                checked: self.0.checked,
                dirname: self.0.dirname,
                disabled: self.0.disabled,
                form: self.0.form,
                form_action: self.0.form_action,
                form_enc_type: self.0.form_enc_type,
                form_method: self.0.form_method,
                form_no_validate: self.0.form_no_validate,
                form_target: self.0.form_target,
                height: self.0.height,
                list: self.0.list,
                max: self.0.max,
                max_length: self.0.max_length,
                min,
                min_length: self.0.min_length,
                multiple: self.0.multiple,
                name: self.0.name,
                pattern: self.0.pattern,
                placeholder: self.0.placeholder,
                read_only: self.0.read_only,
                required: self.0.required,
                size: self.0.size,
                src: self.0.src,
                step: self.0.step,
                type_: self.0.type_,
                value: self.0.value,
                width: self.0.width,
            })
        }
        #[inline]
        pub fn min_length<V: crate::MaybeUpdateValue<i32>>(
            self,
            min_length: V,
        ) -> super::Building<super::overwrite::min_length<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                accept: self.0.accept,
                alt: self.0.alt,
                auto_complete: self.0.auto_complete,
                capture: self.0.capture,
                checked: self.0.checked,
                dirname: self.0.dirname,
                disabled: self.0.disabled,
                form: self.0.form,
                form_action: self.0.form_action,
                form_enc_type: self.0.form_enc_type,
                form_method: self.0.form_method,
                form_no_validate: self.0.form_no_validate,
                form_target: self.0.form_target,
                height: self.0.height,
                list: self.0.list,
                max: self.0.max,
                max_length: self.0.max_length,
                min: self.0.min,
                min_length,
                multiple: self.0.multiple,
                name: self.0.name,
                pattern: self.0.pattern,
                placeholder: self.0.placeholder,
                read_only: self.0.read_only,
                required: self.0.required,
                size: self.0.size,
                src: self.0.src,
                step: self.0.step,
                type_: self.0.type_,
                value: self.0.value,
                width: self.0.width,
            })
        }
        #[inline]
        pub fn multiple<V: crate::MaybeUpdateValue<bool>>(
            self,
            multiple: V,
        ) -> super::Building<super::overwrite::multiple<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                accept: self.0.accept,
                alt: self.0.alt,
                auto_complete: self.0.auto_complete,
                capture: self.0.capture,
                checked: self.0.checked,
                dirname: self.0.dirname,
                disabled: self.0.disabled,
                form: self.0.form,
                form_action: self.0.form_action,
                form_enc_type: self.0.form_enc_type,
                form_method: self.0.form_method,
                form_no_validate: self.0.form_no_validate,
                form_target: self.0.form_target,
                height: self.0.height,
                list: self.0.list,
                max: self.0.max,
                max_length: self.0.max_length,
                min: self.0.min,
                min_length: self.0.min_length,
                multiple,
                name: self.0.name,
                pattern: self.0.pattern,
                placeholder: self.0.placeholder,
                read_only: self.0.read_only,
                required: self.0.required,
                size: self.0.size,
                src: self.0.src,
                step: self.0.step,
                type_: self.0.type_,
                value: self.0.value,
                width: self.0.width,
            })
        }
        #[inline]
        pub fn name<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            name: V,
        ) -> super::Building<super::overwrite::name<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                accept: self.0.accept,
                alt: self.0.alt,
                auto_complete: self.0.auto_complete,
                capture: self.0.capture,
                checked: self.0.checked,
                dirname: self.0.dirname,
                disabled: self.0.disabled,
                form: self.0.form,
                form_action: self.0.form_action,
                form_enc_type: self.0.form_enc_type,
                form_method: self.0.form_method,
                form_no_validate: self.0.form_no_validate,
                form_target: self.0.form_target,
                height: self.0.height,
                list: self.0.list,
                max: self.0.max,
                max_length: self.0.max_length,
                min: self.0.min,
                min_length: self.0.min_length,
                multiple: self.0.multiple,
                name,
                pattern: self.0.pattern,
                placeholder: self.0.placeholder,
                read_only: self.0.read_only,
                required: self.0.required,
                size: self.0.size,
                src: self.0.src,
                step: self.0.step,
                type_: self.0.type_,
                value: self.0.value,
                width: self.0.width,
            })
        }
        #[inline]
        pub fn pattern<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            pattern: V,
        ) -> super::Building<super::overwrite::pattern<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                accept: self.0.accept,
                alt: self.0.alt,
                auto_complete: self.0.auto_complete,
                capture: self.0.capture,
                checked: self.0.checked,
                dirname: self.0.dirname,
                disabled: self.0.disabled,
                form: self.0.form,
                form_action: self.0.form_action,
                form_enc_type: self.0.form_enc_type,
                form_method: self.0.form_method,
                form_no_validate: self.0.form_no_validate,
                form_target: self.0.form_target,
                height: self.0.height,
                list: self.0.list,
                max: self.0.max,
                max_length: self.0.max_length,
                min: self.0.min,
                min_length: self.0.min_length,
                multiple: self.0.multiple,
                name: self.0.name,
                pattern,
                placeholder: self.0.placeholder,
                read_only: self.0.read_only,
                required: self.0.required,
                size: self.0.size,
                src: self.0.src,
                step: self.0.step,
                type_: self.0.type_,
                value: self.0.value,
                width: self.0.width,
            })
        }
        #[inline]
        pub fn placeholder<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            placeholder: V,
        ) -> super::Building<super::overwrite::placeholder<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                accept: self.0.accept,
                alt: self.0.alt,
                auto_complete: self.0.auto_complete,
                capture: self.0.capture,
                checked: self.0.checked,
                dirname: self.0.dirname,
                disabled: self.0.disabled,
                form: self.0.form,
                form_action: self.0.form_action,
                form_enc_type: self.0.form_enc_type,
                form_method: self.0.form_method,
                form_no_validate: self.0.form_no_validate,
                form_target: self.0.form_target,
                height: self.0.height,
                list: self.0.list,
                max: self.0.max,
                max_length: self.0.max_length,
                min: self.0.min,
                min_length: self.0.min_length,
                multiple: self.0.multiple,
                name: self.0.name,
                pattern: self.0.pattern,
                placeholder,
                read_only: self.0.read_only,
                required: self.0.required,
                size: self.0.size,
                src: self.0.src,
                step: self.0.step,
                type_: self.0.type_,
                value: self.0.value,
                width: self.0.width,
            })
        }
        #[inline]
        pub fn read_only<V: crate::MaybeUpdateValue<bool>>(
            self,
            read_only: V,
        ) -> super::Building<super::overwrite::read_only<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                accept: self.0.accept,
                alt: self.0.alt,
                auto_complete: self.0.auto_complete,
                capture: self.0.capture,
                checked: self.0.checked,
                dirname: self.0.dirname,
                disabled: self.0.disabled,
                form: self.0.form,
                form_action: self.0.form_action,
                form_enc_type: self.0.form_enc_type,
                form_method: self.0.form_method,
                form_no_validate: self.0.form_no_validate,
                form_target: self.0.form_target,
                height: self.0.height,
                list: self.0.list,
                max: self.0.max,
                max_length: self.0.max_length,
                min: self.0.min,
                min_length: self.0.min_length,
                multiple: self.0.multiple,
                name: self.0.name,
                pattern: self.0.pattern,
                placeholder: self.0.placeholder,
                read_only,
                required: self.0.required,
                size: self.0.size,
                src: self.0.src,
                step: self.0.step,
                type_: self.0.type_,
                value: self.0.value,
                width: self.0.width,
            })
        }
        #[inline]
        pub fn required<V: crate::MaybeUpdateValue<bool>>(
            self,
            required: V,
        ) -> super::Building<super::overwrite::required<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                accept: self.0.accept,
                alt: self.0.alt,
                auto_complete: self.0.auto_complete,
                capture: self.0.capture,
                checked: self.0.checked,
                dirname: self.0.dirname,
                disabled: self.0.disabled,
                form: self.0.form,
                form_action: self.0.form_action,
                form_enc_type: self.0.form_enc_type,
                form_method: self.0.form_method,
                form_no_validate: self.0.form_no_validate,
                form_target: self.0.form_target,
                height: self.0.height,
                list: self.0.list,
                max: self.0.max,
                max_length: self.0.max_length,
                min: self.0.min,
                min_length: self.0.min_length,
                multiple: self.0.multiple,
                name: self.0.name,
                pattern: self.0.pattern,
                placeholder: self.0.placeholder,
                read_only: self.0.read_only,
                required,
                size: self.0.size,
                src: self.0.src,
                step: self.0.step,
                type_: self.0.type_,
                value: self.0.value,
                width: self.0.width,
            })
        }
        #[inline]
        pub fn size<V: crate::MaybeUpdateValue<u32>>(
            self,
            size: V,
        ) -> super::Building<super::overwrite::size<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                accept: self.0.accept,
                alt: self.0.alt,
                auto_complete: self.0.auto_complete,
                capture: self.0.capture,
                checked: self.0.checked,
                dirname: self.0.dirname,
                disabled: self.0.disabled,
                form: self.0.form,
                form_action: self.0.form_action,
                form_enc_type: self.0.form_enc_type,
                form_method: self.0.form_method,
                form_no_validate: self.0.form_no_validate,
                form_target: self.0.form_target,
                height: self.0.height,
                list: self.0.list,
                max: self.0.max,
                max_length: self.0.max_length,
                min: self.0.min,
                min_length: self.0.min_length,
                multiple: self.0.multiple,
                name: self.0.name,
                pattern: self.0.pattern,
                placeholder: self.0.placeholder,
                read_only: self.0.read_only,
                required: self.0.required,
                size,
                src: self.0.src,
                step: self.0.step,
                type_: self.0.type_,
                value: self.0.value,
                width: self.0.width,
            })
        }
        #[inline]
        pub fn src<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            src: V,
        ) -> super::Building<super::overwrite::src<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                accept: self.0.accept,
                alt: self.0.alt,
                auto_complete: self.0.auto_complete,
                capture: self.0.capture,
                checked: self.0.checked,
                dirname: self.0.dirname,
                disabled: self.0.disabled,
                form: self.0.form,
                form_action: self.0.form_action,
                form_enc_type: self.0.form_enc_type,
                form_method: self.0.form_method,
                form_no_validate: self.0.form_no_validate,
                form_target: self.0.form_target,
                height: self.0.height,
                list: self.0.list,
                max: self.0.max,
                max_length: self.0.max_length,
                min: self.0.min,
                min_length: self.0.min_length,
                multiple: self.0.multiple,
                name: self.0.name,
                pattern: self.0.pattern,
                placeholder: self.0.placeholder,
                read_only: self.0.read_only,
                required: self.0.required,
                size: self.0.size,
                src,
                step: self.0.step,
                type_: self.0.type_,
                value: self.0.value,
                width: self.0.width,
            })
        }
        #[inline]
        pub fn step<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            step: V,
        ) -> super::Building<super::overwrite::step<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                accept: self.0.accept,
                alt: self.0.alt,
                auto_complete: self.0.auto_complete,
                capture: self.0.capture,
                checked: self.0.checked,
                dirname: self.0.dirname,
                disabled: self.0.disabled,
                form: self.0.form,
                form_action: self.0.form_action,
                form_enc_type: self.0.form_enc_type,
                form_method: self.0.form_method,
                form_no_validate: self.0.form_no_validate,
                form_target: self.0.form_target,
                height: self.0.height,
                list: self.0.list,
                max: self.0.max,
                max_length: self.0.max_length,
                min: self.0.min,
                min_length: self.0.min_length,
                multiple: self.0.multiple,
                name: self.0.name,
                pattern: self.0.pattern,
                placeholder: self.0.placeholder,
                read_only: self.0.read_only,
                required: self.0.required,
                size: self.0.size,
                src: self.0.src,
                step,
                type_: self.0.type_,
                value: self.0.value,
                width: self.0.width,
            })
        }
        #[inline]
        pub fn type_<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            type_: V,
        ) -> super::Building<super::overwrite::type_<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                accept: self.0.accept,
                alt: self.0.alt,
                auto_complete: self.0.auto_complete,
                capture: self.0.capture,
                checked: self.0.checked,
                dirname: self.0.dirname,
                disabled: self.0.disabled,
                form: self.0.form,
                form_action: self.0.form_action,
                form_enc_type: self.0.form_enc_type,
                form_method: self.0.form_method,
                form_no_validate: self.0.form_no_validate,
                form_target: self.0.form_target,
                height: self.0.height,
                list: self.0.list,
                max: self.0.max,
                max_length: self.0.max_length,
                min: self.0.min,
                min_length: self.0.min_length,
                multiple: self.0.multiple,
                name: self.0.name,
                pattern: self.0.pattern,
                placeholder: self.0.placeholder,
                read_only: self.0.read_only,
                required: self.0.required,
                size: self.0.size,
                src: self.0.src,
                step: self.0.step,
                type_,
                value: self.0.value,
                width: self.0.width,
            })
        }
        #[inline]
        pub fn value<V: crate::MaybeUpdateValueByRef<str>>(
            self,
            value: V,
        ) -> super::Building<super::overwrite::value<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                accept: self.0.accept,
                alt: self.0.alt,
                auto_complete: self.0.auto_complete,
                capture: self.0.capture,
                checked: self.0.checked,
                dirname: self.0.dirname,
                disabled: self.0.disabled,
                form: self.0.form,
                form_action: self.0.form_action,
                form_enc_type: self.0.form_enc_type,
                form_method: self.0.form_method,
                form_no_validate: self.0.form_no_validate,
                form_target: self.0.form_target,
                height: self.0.height,
                list: self.0.list,
                max: self.0.max,
                max_length: self.0.max_length,
                min: self.0.min,
                min_length: self.0.min_length,
                multiple: self.0.multiple,
                name: self.0.name,
                pattern: self.0.pattern,
                placeholder: self.0.placeholder,
                read_only: self.0.read_only,
                required: self.0.required,
                size: self.0.size,
                src: self.0.src,
                step: self.0.step,
                type_: self.0.type_,
                value,
                width: self.0.width,
            })
        }
        #[inline]
        pub fn width<V: crate::MaybeUpdateValue<u32>>(
            self,
            width: V,
        ) -> super::Building<super::overwrite::width<TypeDefs, V>> {
            super::Building(super::Data {
                HtmlElementProps: self.0.HtmlElementProps,
                accept: self.0.accept,
                alt: self.0.alt,
                auto_complete: self.0.auto_complete,
                capture: self.0.capture,
                checked: self.0.checked,
                dirname: self.0.dirname,
                disabled: self.0.disabled,
                form: self.0.form,
                form_action: self.0.form_action,
                form_enc_type: self.0.form_enc_type,
                form_method: self.0.form_method,
                form_no_validate: self.0.form_no_validate,
                form_target: self.0.form_target,
                height: self.0.height,
                list: self.0.list,
                max: self.0.max,
                max_length: self.0.max_length,
                min: self.0.min,
                min_length: self.0.min_length,
                multiple: self.0.multiple,
                name: self.0.name,
                pattern: self.0.pattern,
                placeholder: self.0.placeholder,
                read_only: self.0.read_only,
                required: self.0.required,
                size: self.0.size,
                src: self.0.src,
                step: self.0.step,
                type_: self.0.type_,
                value: self.0.value,
                width,
            })
        }
    }
}
#[cfg(feature = "dom")]
mod impl_update_element {
    #[allow(unused_imports)]
    use super::super::*;
    impl<TypeDefs: ?::core::marker::Sized + super::Types>
        crate::props::UpdateElement<web_sys::HtmlInputElement> for super::Data<TypeDefs>
    where
        HtmlElementProps::Data<TypeDefs::HtmlElementProps>:
            crate::props::UpdateElement<web_sys::HtmlElement>,
    {
        type State =
            <HtmlElementProps::Data<TypeDefs::HtmlElementProps> as crate::props::UpdateElement<
                web_sys::HtmlElement,
            >>::State;
        fn update_element(
            this: Self,
            element: &web_sys::HtmlInputElement,
            children_ctx: &mut ::frender_dom::Dom,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let dom_element: &::web_sys::Element = element.as_ref();
            crate::props::UpdateElement::update_element(
                this.HtmlElementProps,
                element.as_ref(),
                children_ctx,
                state,
            );
            {
                #[allow(unused)]
                const ATTR_NAME: &::core::primitive::str = "accept";
                < TypeDefs :: accept as :: frender_dom :: props :: MaybeUpdateValueByRef < str , > > :: maybe_update_value_by_ref (& this . accept , | v | element . set_accept (v) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
            }
            {
                #[allow(unused)]
                const ATTR_NAME: &::core::primitive::str = "alt";
                < TypeDefs :: alt as :: frender_dom :: props :: MaybeUpdateValueByRef < str , > > :: maybe_update_value_by_ref (& this . alt , | v | element . set_alt (v) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
            }
            {
                #[allow(unused)]
                const ATTR_NAME: &::core::primitive::str = "autocomplete";
                < TypeDefs :: auto_complete as :: frender_dom :: props :: MaybeUpdateValueByRef < str , > > :: maybe_update_value_by_ref (& this . auto_complete , | v | element . set_autocomplete (v) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
            }
            {
                #[allow(unused)]
                const ATTR_NAME: &::core::primitive::str = "capture";
                < TypeDefs :: capture as :: frender_dom :: props :: MaybeUpdateValueByRef < str , > > :: maybe_update_value_by_ref (& this . capture , | v | crate :: props :: UpdateElementAttribute :: update_element_attribute (v , dom_element , ATTR_NAME ,) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
            }
            {
                #[allow(unused)]
                const ATTR_NAME: &::core::primitive::str = "checked";
                < TypeDefs :: checked as :: frender_dom :: props :: MaybeUpdateValue < bool , > > :: maybe_update_value (this . checked , | v | element . set_checked (v) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
            }
            {
                #[allow(unused)]
                const ATTR_NAME: &::core::primitive::str = "dirname";
                < TypeDefs :: dirname as :: frender_dom :: props :: MaybeUpdateValueByRef < str , > > :: maybe_update_value_by_ref (& this . dirname , | v | crate :: props :: UpdateElementAttribute :: update_element_attribute (v , dom_element , ATTR_NAME ,) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
            }
            {
                #[allow(unused)]
                const ATTR_NAME: &::core::primitive::str = "disabled";
                < TypeDefs :: disabled as :: frender_dom :: props :: MaybeUpdateValue < bool , > > :: maybe_update_value (this . disabled , | v | element . set_disabled (v) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
            }
            {
                #[allow(unused)]
                const ATTR_NAME: &::core::primitive::str = "form";
                < TypeDefs :: form as :: frender_dom :: props :: MaybeUpdateValueByRef < str , > > :: maybe_update_value_by_ref (& this . form , | v | crate :: props :: UpdateElementAttribute :: update_element_attribute (v , dom_element , ATTR_NAME ,) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
            }
            {
                #[allow(unused)]
                const ATTR_NAME: &::core::primitive::str = "formaction";
                < TypeDefs :: form_action as :: frender_dom :: props :: MaybeUpdateValueByRef < str , > > :: maybe_update_value_by_ref (& this . form_action , | v | element . set_form_action (v) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
            }
            {
                #[allow(unused)]
                const ATTR_NAME: &::core::primitive::str = "formenctype";
                < TypeDefs :: form_enc_type as :: frender_dom :: props :: MaybeUpdateValueByRef < str , > > :: maybe_update_value_by_ref (& this . form_enc_type , | v | element . set_form_enctype (v) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
            }
            {
                #[allow(unused)]
                const ATTR_NAME: &::core::primitive::str = "formmethod";
                < TypeDefs :: form_method as :: frender_dom :: props :: MaybeUpdateValueByRef < str , > > :: maybe_update_value_by_ref (& this . form_method , | v | element . set_form_method (v) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
            }
            {
                #[allow(unused)]
                const ATTR_NAME: &::core::primitive::str = "formnovalidate";
                < TypeDefs :: form_no_validate as :: frender_dom :: props :: MaybeUpdateValue < bool , > > :: maybe_update_value (this . form_no_validate , | v | element . set_form_no_validate (v) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
            }
            {
                #[allow(unused)]
                const ATTR_NAME: &::core::primitive::str = "formtarget";
                < TypeDefs :: form_target as :: frender_dom :: props :: MaybeUpdateValueByRef < str , > > :: maybe_update_value_by_ref (& this . form_target , | v | element . set_form_target (v) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
            }
            {
                #[allow(unused)]
                const ATTR_NAME: &::core::primitive::str = "height";
                < TypeDefs :: height as :: frender_dom :: props :: MaybeUpdateValue < u32 , > > :: maybe_update_value (this . height , | v | element . set_height (v) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
            }
            {
                #[allow(unused)]
                const ATTR_NAME: &::core::primitive::str = "list";
                < TypeDefs :: list as :: frender_dom :: props :: MaybeUpdateValueByRef < str , > > :: maybe_update_value_by_ref (& this . list , | v | crate :: props :: UpdateElementAttribute :: update_element_attribute (v , dom_element , ATTR_NAME ,) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
            }
            {
                #[allow(unused)]
                const ATTR_NAME: &::core::primitive::str = "max";
                < TypeDefs :: max as :: frender_dom :: props :: MaybeUpdateValueByRef < str , > > :: maybe_update_value_by_ref (& this . max , | v | element . set_max (v) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
            }
            {
                #[allow(unused)]
                const ATTR_NAME: &::core::primitive::str = "maxlength";
                < TypeDefs :: max_length as :: frender_dom :: props :: MaybeUpdateValue < i32 , > > :: maybe_update_value (this . max_length , | v | element . set_max_length (v) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
            }
            {
                #[allow(unused)]
                const ATTR_NAME: &::core::primitive::str = "min";
                < TypeDefs :: min as :: frender_dom :: props :: MaybeUpdateValueByRef < str , > > :: maybe_update_value_by_ref (& this . min , | v | element . set_min (v) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
            }
            {
                #[allow(unused)]
                const ATTR_NAME: &::core::primitive::str = "minlength";
                < TypeDefs :: min_length as :: frender_dom :: props :: MaybeUpdateValue < i32 , > > :: maybe_update_value (this . min_length , | v | element . set_min_length (v) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
            }
            {
                #[allow(unused)]
                const ATTR_NAME: &::core::primitive::str = "multiple";
                < TypeDefs :: multiple as :: frender_dom :: props :: MaybeUpdateValue < bool , > > :: maybe_update_value (this . multiple , | v | element . set_multiple (v) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
            }
            {
                #[allow(unused)]
                const ATTR_NAME: &::core::primitive::str = "name";
                < TypeDefs :: name as :: frender_dom :: props :: MaybeUpdateValueByRef < str , > > :: maybe_update_value_by_ref (& this . name , | v | element . set_name (v) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
            }
            {
                #[allow(unused)]
                const ATTR_NAME: &::core::primitive::str = "pattern";
                < TypeDefs :: pattern as :: frender_dom :: props :: MaybeUpdateValueByRef < str , > > :: maybe_update_value_by_ref (& this . pattern , | v | element . set_pattern (v) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
            }
            {
                #[allow(unused)]
                const ATTR_NAME: &::core::primitive::str = "placeholder";
                < TypeDefs :: placeholder as :: frender_dom :: props :: MaybeUpdateValueByRef < str , > > :: maybe_update_value_by_ref (& this . placeholder , | v | element . set_placeholder (v) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
            }
            {
                #[allow(unused)]
                const ATTR_NAME: &::core::primitive::str = "readonly";
                < TypeDefs :: read_only as :: frender_dom :: props :: MaybeUpdateValue < bool , > > :: maybe_update_value (this . read_only , | v | element . set_read_only (v) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
            }
            {
                #[allow(unused)]
                const ATTR_NAME: &::core::primitive::str = "required";
                < TypeDefs :: required as :: frender_dom :: props :: MaybeUpdateValue < bool , > > :: maybe_update_value (this . required , | v | element . set_required (v) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
            }
            {
                #[allow(unused)]
                const ATTR_NAME: &::core::primitive::str = "size";
                <TypeDefs::size as ::frender_dom::props::MaybeUpdateValue<u32>>::maybe_update_value(
                    this.size,
                    |v| element.set_size(v),
                    || dom_element.remove_attribute(ATTR_NAME).unwrap(),
                )
            }
            {
                #[allow(unused)]
                const ATTR_NAME: &::core::primitive::str = "src";
                < TypeDefs :: src as :: frender_dom :: props :: MaybeUpdateValueByRef < str , > > :: maybe_update_value_by_ref (& this . src , | v | element . set_src (v) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
            }
            {
                #[allow(unused)]
                const ATTR_NAME: &::core::primitive::str = "step";
                < TypeDefs :: step as :: frender_dom :: props :: MaybeUpdateValueByRef < str , > > :: maybe_update_value_by_ref (& this . step , | v | element . set_step (v) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
            }
            {
                #[allow(unused)]
                const ATTR_NAME: &::core::primitive::str = "type";
                < TypeDefs :: type_ as :: frender_dom :: props :: MaybeUpdateValueByRef < str , > > :: maybe_update_value_by_ref (& this . type_ , | v | element . set_type (v) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
            }
            {
                #[allow(unused)]
                const ATTR_NAME: &::core::primitive::str = "value";
                < TypeDefs :: value as :: frender_dom :: props :: MaybeUpdateValueByRef < str , > > :: maybe_update_value_by_ref (& this . value , | v | element . set_value (v) , | | dom_element . remove_attribute (ATTR_NAME) . unwrap () ,)
            }
            {
                #[allow(unused)]
                const ATTR_NAME: &::core::primitive::str = "width";
                <TypeDefs::width as ::frender_dom::props::MaybeUpdateValue<u32>>::maybe_update_value(
                    this.width,
                    |v| element.set_width(v),
                    || dom_element.remove_attribute(ATTR_NAME).unwrap(),
                )
            }
        }
    }
}
