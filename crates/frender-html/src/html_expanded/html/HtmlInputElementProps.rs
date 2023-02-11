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
        type accept: crate::MaybeUpdateValueWithState<str>;
        type alt: crate::MaybeUpdateValueWithState<str>;
        type auto_complete: crate::MaybeUpdateValueWithState<str>;
        type capture: crate::MaybeUpdateValueWithState<str>;
        type checked: crate::MaybeUpdateValueWithState<bool>;
        type dirname: crate::MaybeUpdateValueWithState<str>;
        type disabled: crate::MaybeUpdateValueWithState<bool>;
        type form: crate::MaybeUpdateValueWithState<str>;
        type form_action: crate::MaybeUpdateValueWithState<str>;
        type form_enc_type: crate::MaybeUpdateValueWithState<str>;
        type form_method: crate::MaybeUpdateValueWithState<str>;
        type form_no_validate: crate::MaybeUpdateValueWithState<bool>;
        type form_target: crate::MaybeUpdateValueWithState<str>;
        type height: crate::MaybeUpdateValueWithState<u32>;
        type list: crate::MaybeUpdateValueWithState<str>;
        type max: crate::MaybeUpdateValueWithState<str>;
        type max_length: crate::MaybeUpdateValueWithState<i32>;
        type min: crate::MaybeUpdateValueWithState<str>;
        type min_length: crate::MaybeUpdateValueWithState<i32>;
        type multiple: crate::MaybeUpdateValueWithState<bool>;
        type name: crate::MaybeUpdateValueWithState<str>;
        type pattern: crate::MaybeUpdateValueWithState<str>;
        type placeholder: crate::MaybeUpdateValueWithState<str>;
        type read_only: crate::MaybeUpdateValueWithState<bool>;
        type required: crate::MaybeUpdateValueWithState<bool>;
        type size: crate::MaybeUpdateValueWithState<u32>;
        type src: crate::MaybeUpdateValueWithState<str>;
        type step: crate::MaybeUpdateValueWithState<str>;
        type type_: crate::MaybeUpdateValueWithState<str>;
        type value: crate::MaybeUpdateValueWithState<str>;
        type width: crate::MaybeUpdateValueWithState<u32>;
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
pub mod render_state {
    #[allow(non_camel_case_types)]
    pub trait RenderStateTypes {
        type HtmlElementProps: crate::props::IntrinsicComponentPollReactive;
        type accept;
        type alt;
        type auto_complete;
        type capture;
        type checked;
        type dirname;
        type disabled;
        type form;
        type form_action;
        type form_enc_type;
        type form_method;
        type form_no_validate;
        type form_target;
        type height;
        type list;
        type max;
        type max_length;
        type min;
        type min_length;
        type multiple;
        type name;
        type pattern;
        type placeholder;
        type read_only;
        type required;
        type size;
        type src;
        type step;
        type type_;
        type value;
        type width;
    }
    pub struct RenderState<TypeDefs: RenderStateTypes>
    where
        TypeDefs: ?::core::marker::Sized,
    {
        pub HtmlElementProps: TypeDefs::HtmlElementProps,
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
    #[allow(dead_code)]
    #[allow(single_use_lifetimes)]
    #[allow(clippy::unknown_clippy_lints)]
    #[allow(clippy::mut_mut)]
    #[allow(clippy::redundant_pub_crate)]
    #[allow(clippy::ref_option_ref)]
    #[allow(clippy::type_repetition_in_bounds)]
    pub(crate) struct RenderStateProj<'__pin, TypeDefs: RenderStateTypes>
    where
        RenderState<TypeDefs>: '__pin,
        TypeDefs: ?::core::marker::Sized,
    {
        pub HtmlElementProps:
            ::pin_project_lite::__private::Pin<&'__pin mut (TypeDefs::HtmlElementProps)>,
        pub accept: &'__pin mut (TypeDefs::accept),
        pub alt: &'__pin mut (TypeDefs::alt),
        pub auto_complete: &'__pin mut (TypeDefs::auto_complete),
        pub capture: &'__pin mut (TypeDefs::capture),
        pub checked: &'__pin mut (TypeDefs::checked),
        pub dirname: &'__pin mut (TypeDefs::dirname),
        pub disabled: &'__pin mut (TypeDefs::disabled),
        pub form: &'__pin mut (TypeDefs::form),
        pub form_action: &'__pin mut (TypeDefs::form_action),
        pub form_enc_type: &'__pin mut (TypeDefs::form_enc_type),
        pub form_method: &'__pin mut (TypeDefs::form_method),
        pub form_no_validate: &'__pin mut (TypeDefs::form_no_validate),
        pub form_target: &'__pin mut (TypeDefs::form_target),
        pub height: &'__pin mut (TypeDefs::height),
        pub list: &'__pin mut (TypeDefs::list),
        pub max: &'__pin mut (TypeDefs::max),
        pub max_length: &'__pin mut (TypeDefs::max_length),
        pub min: &'__pin mut (TypeDefs::min),
        pub min_length: &'__pin mut (TypeDefs::min_length),
        pub multiple: &'__pin mut (TypeDefs::multiple),
        pub name: &'__pin mut (TypeDefs::name),
        pub pattern: &'__pin mut (TypeDefs::pattern),
        pub placeholder: &'__pin mut (TypeDefs::placeholder),
        pub read_only: &'__pin mut (TypeDefs::read_only),
        pub required: &'__pin mut (TypeDefs::required),
        pub size: &'__pin mut (TypeDefs::size),
        pub src: &'__pin mut (TypeDefs::src),
        pub step: &'__pin mut (TypeDefs::step),
        pub type_: &'__pin mut (TypeDefs::type_),
        pub value: &'__pin mut (TypeDefs::value),
        pub width: &'__pin mut (TypeDefs::width),
    }
    #[allow(explicit_outlives_requirements)]
    #[allow(single_use_lifetimes)]
    #[allow(clippy::unknown_clippy_lints)]
    #[allow(clippy::redundant_pub_crate)]
    #[allow(clippy::used_underscore_binding)]
    const _: () = {
        #[allow(dead_code)]
        #[allow(single_use_lifetimes)]
        #[allow(clippy::unknown_clippy_lints)]
        #[allow(clippy::mut_mut)]
        #[allow(clippy::redundant_pub_crate)]
        #[allow(clippy::ref_option_ref)]
        #[allow(clippy::type_repetition_in_bounds)]
        pub(crate) struct ProjectionRef<'__pin, TypeDefs: RenderStateTypes>
        where
            RenderState<TypeDefs>: '__pin,
            TypeDefs: ?::core::marker::Sized,
        {
            pub HtmlElementProps:
                ::pin_project_lite::__private::Pin<&'__pin (TypeDefs::HtmlElementProps)>,
            pub accept: &'__pin (TypeDefs::accept),
            pub alt: &'__pin (TypeDefs::alt),
            pub auto_complete: &'__pin (TypeDefs::auto_complete),
            pub capture: &'__pin (TypeDefs::capture),
            pub checked: &'__pin (TypeDefs::checked),
            pub dirname: &'__pin (TypeDefs::dirname),
            pub disabled: &'__pin (TypeDefs::disabled),
            pub form: &'__pin (TypeDefs::form),
            pub form_action: &'__pin (TypeDefs::form_action),
            pub form_enc_type: &'__pin (TypeDefs::form_enc_type),
            pub form_method: &'__pin (TypeDefs::form_method),
            pub form_no_validate: &'__pin (TypeDefs::form_no_validate),
            pub form_target: &'__pin (TypeDefs::form_target),
            pub height: &'__pin (TypeDefs::height),
            pub list: &'__pin (TypeDefs::list),
            pub max: &'__pin (TypeDefs::max),
            pub max_length: &'__pin (TypeDefs::max_length),
            pub min: &'__pin (TypeDefs::min),
            pub min_length: &'__pin (TypeDefs::min_length),
            pub multiple: &'__pin (TypeDefs::multiple),
            pub name: &'__pin (TypeDefs::name),
            pub pattern: &'__pin (TypeDefs::pattern),
            pub placeholder: &'__pin (TypeDefs::placeholder),
            pub read_only: &'__pin (TypeDefs::read_only),
            pub required: &'__pin (TypeDefs::required),
            pub size: &'__pin (TypeDefs::size),
            pub src: &'__pin (TypeDefs::src),
            pub step: &'__pin (TypeDefs::step),
            pub type_: &'__pin (TypeDefs::type_),
            pub value: &'__pin (TypeDefs::value),
            pub width: &'__pin (TypeDefs::width),
        }
        impl<TypeDefs: RenderStateTypes> RenderState<TypeDefs>
        where
            TypeDefs: ?::core::marker::Sized,
        {
            pub(crate) fn project<'__pin>(
                self: ::pin_project_lite::__private::Pin<&'__pin mut Self>,
            ) -> RenderStateProj<'__pin, TypeDefs> {
                unsafe {
                    let Self {
                        HtmlElementProps,
                        accept,
                        alt,
                        auto_complete,
                        capture,
                        checked,
                        dirname,
                        disabled,
                        form,
                        form_action,
                        form_enc_type,
                        form_method,
                        form_no_validate,
                        form_target,
                        height,
                        list,
                        max,
                        max_length,
                        min,
                        min_length,
                        multiple,
                        name,
                        pattern,
                        placeholder,
                        read_only,
                        required,
                        size,
                        src,
                        step,
                        type_,
                        value,
                        width,
                    } = self.get_unchecked_mut();
                    RenderStateProj {
                        HtmlElementProps: ::pin_project_lite::__private::Pin::new_unchecked(
                            HtmlElementProps,
                        ),
                        accept: accept,
                        alt: alt,
                        auto_complete: auto_complete,
                        capture: capture,
                        checked: checked,
                        dirname: dirname,
                        disabled: disabled,
                        form: form,
                        form_action: form_action,
                        form_enc_type: form_enc_type,
                        form_method: form_method,
                        form_no_validate: form_no_validate,
                        form_target: form_target,
                        height: height,
                        list: list,
                        max: max,
                        max_length: max_length,
                        min: min,
                        min_length: min_length,
                        multiple: multiple,
                        name: name,
                        pattern: pattern,
                        placeholder: placeholder,
                        read_only: read_only,
                        required: required,
                        size: size,
                        src: src,
                        step: step,
                        type_: type_,
                        value: value,
                        width: width,
                    }
                }
            }
            pub(crate) fn project_ref<'__pin>(
                self: ::pin_project_lite::__private::Pin<&'__pin Self>,
            ) -> ProjectionRef<'__pin, TypeDefs> {
                unsafe {
                    let Self {
                        HtmlElementProps,
                        accept,
                        alt,
                        auto_complete,
                        capture,
                        checked,
                        dirname,
                        disabled,
                        form,
                        form_action,
                        form_enc_type,
                        form_method,
                        form_no_validate,
                        form_target,
                        height,
                        list,
                        max,
                        max_length,
                        min,
                        min_length,
                        multiple,
                        name,
                        pattern,
                        placeholder,
                        read_only,
                        required,
                        size,
                        src,
                        step,
                        type_,
                        value,
                        width,
                    } = self.get_ref();
                    ProjectionRef {
                        HtmlElementProps: ::pin_project_lite::__private::Pin::new_unchecked(
                            HtmlElementProps,
                        ),
                        accept: accept,
                        alt: alt,
                        auto_complete: auto_complete,
                        capture: capture,
                        checked: checked,
                        dirname: dirname,
                        disabled: disabled,
                        form: form,
                        form_action: form_action,
                        form_enc_type: form_enc_type,
                        form_method: form_method,
                        form_no_validate: form_no_validate,
                        form_target: form_target,
                        height: height,
                        list: list,
                        max: max,
                        max_length: max_length,
                        min: min,
                        min_length: min_length,
                        multiple: multiple,
                        name: name,
                        pattern: pattern,
                        placeholder: placeholder,
                        read_only: read_only,
                        required: required,
                        size: size,
                        src: src,
                        step: step,
                        type_: type_,
                        value: value,
                        width: width,
                    }
                }
            }
        }
        #[allow(non_snake_case)]
        pub struct __Origin<'__pin, TypeDefs: RenderStateTypes>
        where
            TypeDefs: ?::core::marker::Sized,
        {
            __dummy_lifetime: ::pin_project_lite::__private::PhantomData<&'__pin ()>,
            HtmlElementProps: TypeDefs::HtmlElementProps,
            accept: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::accept>,
            alt: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::alt>,
            auto_complete: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::auto_complete>,
            capture: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::capture>,
            checked: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::checked>,
            dirname: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::dirname>,
            disabled: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::disabled>,
            form: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::form>,
            form_action: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::form_action>,
            form_enc_type: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::form_enc_type>,
            form_method: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::form_method>,
            form_no_validate:
                ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::form_no_validate>,
            form_target: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::form_target>,
            height: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::height>,
            list: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::list>,
            max: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::max>,
            max_length: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::max_length>,
            min: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::min>,
            min_length: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::min_length>,
            multiple: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::multiple>,
            name: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::name>,
            pattern: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::pattern>,
            placeholder: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::placeholder>,
            read_only: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::read_only>,
            required: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::required>,
            size: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::size>,
            src: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::src>,
            step: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::step>,
            type_: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::type_>,
            value: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::value>,
            width: ::pin_project_lite::__private::AlwaysUnpin<TypeDefs::width>,
        }
        impl<'__pin, TypeDefs: RenderStateTypes> ::pin_project_lite::__private::Unpin
            for RenderState<TypeDefs>
        where
            __Origin<'__pin, TypeDefs>: ::pin_project_lite::__private::Unpin,
            TypeDefs: ?::core::marker::Sized,
        {
        }
        trait MustNotImplDrop {}
        #[allow(clippy::drop_bounds, drop_bounds)]
        impl<T: ::pin_project_lite::__private::Drop> MustNotImplDrop for T {}
        impl<TypeDefs: RenderStateTypes> MustNotImplDrop for RenderState<TypeDefs> where
            TypeDefs: ?::core::marker::Sized
        {
        }
        #[forbid(unaligned_references, safe_packed_borrows)]
        fn __assert_not_repr_packed<TypeDefs: RenderStateTypes>(this: &RenderState<TypeDefs>)
        where
            TypeDefs: ?::core::marker::Sized,
        {
            let _ = &this.HtmlElementProps;
            let _ = &this.accept;
            let _ = &this.alt;
            let _ = &this.auto_complete;
            let _ = &this.capture;
            let _ = &this.checked;
            let _ = &this.dirname;
            let _ = &this.disabled;
            let _ = &this.form;
            let _ = &this.form_action;
            let _ = &this.form_enc_type;
            let _ = &this.form_method;
            let _ = &this.form_no_validate;
            let _ = &this.form_target;
            let _ = &this.height;
            let _ = &this.list;
            let _ = &this.max;
            let _ = &this.max_length;
            let _ = &this.min;
            let _ = &this.min_length;
            let _ = &this.multiple;
            let _ = &this.name;
            let _ = &this.pattern;
            let _ = &this.placeholder;
            let _ = &this.read_only;
            let _ = &this.required;
            let _ = &this.size;
            let _ = &this.src;
            let _ = &this.step;
            let _ = &this.type_;
            let _ = &this.value;
            let _ = &this.width;
        }
    };
    impl<TypeDefs: ?::core::marker::Sized + RenderStateTypes> RenderState<TypeDefs> {
        #[inline]
        pub(crate) fn pin_project(
            self: ::core::pin::Pin<&mut Self>,
        ) -> RenderStateProj<'_, TypeDefs> {
            self.project()
        }
    }
    impl<TypeDefs: ?::core::marker::Sized + RenderStateTypes>
        crate::props::IntrinsicComponentPollReactive for RenderState<TypeDefs>
    {
        #[inline]
        fn intrinsic_component_poll_reactive(
            self: ::core::pin::Pin<&mut Self>,
            cx: &mut ::core::task::Context<'_>,
        ) -> ::core::task::Poll<bool> {
            crate::props::IntrinsicComponentPollReactive::intrinsic_component_poll_reactive(
                self.project().HtmlElementProps,
                cx,
            )
        }
    }
}
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
        ///See [`HtmlElementProps::children`]
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
        ///See [`HtmlElementProps::class`]
        #[inline]
        pub fn class<V: crate::MaybeUpdateValueWithState<str>>(
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
        ///See [`HtmlElementProps::id`]
        #[inline]
        pub fn id<V: crate::MaybeUpdateValueWithState<str>>(
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
        ///See [`HtmlElementProps::part`]
        #[inline]
        pub fn part<V: crate::MaybeUpdateValueWithState<str>>(
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
        ///See [`HtmlElementProps::access_key`]
        #[inline]
        pub fn access_key<V: crate::MaybeUpdateValueWithState<str>>(
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
        ///See [`HtmlElementProps::auto_capitalize`]
        #[inline]
        pub fn auto_capitalize<V: crate::MaybeUpdateValueWithState<str>>(
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
        ///See [`HtmlElementProps::auto_focus`]
        #[inline]
        pub fn auto_focus<V: crate::MaybeUpdateValueWithState<bool>>(
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
        ///See [`HtmlElementProps::content_editable`]
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
        ///See [`HtmlElementProps::context_menu`]
        #[inline]
        pub fn context_menu<V: crate::MaybeUpdateValueWithState<str>>(
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
        ///See [`HtmlElementProps::dir`]
        #[inline]
        pub fn dir<V: crate::MaybeUpdateValueWithState<str>>(
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
        ///See [`HtmlElementProps::draggable`]
        #[inline]
        pub fn draggable<V: crate::MaybeUpdateValueWithState<bool>>(
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
        ///See [`HtmlElementProps::enter_key_hint`]
        #[inline]
        pub fn enter_key_hint<V: crate::MaybeUpdateValueWithState<str>>(
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
        ///See [`HtmlElementProps::hidden`]
        #[inline]
        pub fn hidden<V: crate::MaybeUpdateValueWithState<bool>>(
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
        ///See [`HtmlElementProps::inert`]
        #[inline]
        pub fn inert<V: crate::MaybeUpdateValueWithState<bool>>(
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
        ///See [`HtmlElementProps::input_mode`]
        #[inline]
        pub fn input_mode<V: crate::MaybeUpdateValueWithState<str>>(
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
        ///See [`HtmlElementProps::is`]
        #[inline]
        pub fn is<V: crate::MaybeUpdateValueWithState<str>>(
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
        ///See [`HtmlElementProps::item_id`]
        #[inline]
        pub fn item_id<V: crate::MaybeUpdateValueWithState<str>>(
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
        ///See [`HtmlElementProps::item_prop`]
        #[inline]
        pub fn item_prop<V: crate::MaybeUpdateValueWithState<str>>(
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
        ///See [`HtmlElementProps::item_ref`]
        #[inline]
        pub fn item_ref<V: crate::MaybeUpdateValueWithState<str>>(
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
        ///See [`HtmlElementProps::item_scope`]
        #[inline]
        pub fn item_scope<V: crate::MaybeUpdateValueWithState<str>>(
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
        ///See [`HtmlElementProps::item_type`]
        #[inline]
        pub fn item_type<V: crate::MaybeUpdateValueWithState<str>>(
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
        ///See [`HtmlElementProps::lang`]
        #[inline]
        pub fn lang<V: crate::MaybeUpdateValueWithState<str>>(
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
        ///See [`HtmlElementProps::nonce`]
        #[inline]
        pub fn nonce<V: crate::MaybeUpdateValueWithState<str>>(
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
        ///See [`HtmlElementProps::role`]
        #[inline]
        pub fn role<V: crate::MaybeUpdateValueWithState<str>>(
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
        ///See [`HtmlElementProps::slot`]
        #[inline]
        pub fn slot<V: crate::MaybeUpdateValueWithState<str>>(
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
        ///See [`HtmlElementProps::spellcheck`]
        #[inline]
        pub fn spellcheck<V: crate::MaybeUpdateValueWithState<bool>>(
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
        ///See [`HtmlElementProps::style`]
        #[inline]
        pub fn style<V: crate::MaybeUpdateValueWithState<str>>(
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
        ///See [`HtmlElementProps::tab_index`]
        #[inline]
        pub fn tab_index<V: crate::MaybeUpdateValueWithState<i32>>(
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
        ///See [`HtmlElementProps::title`]
        #[inline]
        pub fn title<V: crate::MaybeUpdateValueWithState<str>>(
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
        ///See [`HtmlElementProps::translate`]
        #[inline]
        pub fn translate<V: crate::MaybeUpdateValueWithState<str>>(
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
        ///See [`HtmlElementProps::virtual_keyboard_policy`]
        #[inline]
        pub fn virtual_keyboard_policy<V: crate::MaybeUpdateValueWithState<str>>(
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
        ///See [`HtmlElementProps::on_click`]
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
        pub fn accept<V: crate::MaybeUpdateValueWithState<str>>(
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
        pub fn alt<V: crate::MaybeUpdateValueWithState<str>>(
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
        pub fn auto_complete<V: crate::MaybeUpdateValueWithState<str>>(
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
        pub fn capture<V: crate::MaybeUpdateValueWithState<str>>(
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
        pub fn checked<V: crate::MaybeUpdateValueWithState<bool>>(
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
        pub fn dirname<V: crate::MaybeUpdateValueWithState<str>>(
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
        pub fn disabled<V: crate::MaybeUpdateValueWithState<bool>>(
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
        pub fn form<V: crate::MaybeUpdateValueWithState<str>>(
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
        pub fn form_action<V: crate::MaybeUpdateValueWithState<str>>(
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
        pub fn form_enc_type<V: crate::MaybeUpdateValueWithState<str>>(
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
        pub fn form_method<V: crate::MaybeUpdateValueWithState<str>>(
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
        pub fn form_no_validate<V: crate::MaybeUpdateValueWithState<bool>>(
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
        pub fn form_target<V: crate::MaybeUpdateValueWithState<str>>(
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
        pub fn height<V: crate::MaybeUpdateValueWithState<u32>>(
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
        pub fn list<V: crate::MaybeUpdateValueWithState<str>>(
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
        pub fn max<V: crate::MaybeUpdateValueWithState<str>>(
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
        pub fn max_length<V: crate::MaybeUpdateValueWithState<i32>>(
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
        pub fn min<V: crate::MaybeUpdateValueWithState<str>>(
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
        pub fn min_length<V: crate::MaybeUpdateValueWithState<i32>>(
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
        pub fn multiple<V: crate::MaybeUpdateValueWithState<bool>>(
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
        pub fn name<V: crate::MaybeUpdateValueWithState<str>>(
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
        pub fn pattern<V: crate::MaybeUpdateValueWithState<str>>(
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
        pub fn placeholder<V: crate::MaybeUpdateValueWithState<str>>(
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
        pub fn read_only<V: crate::MaybeUpdateValueWithState<bool>>(
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
        pub fn required<V: crate::MaybeUpdateValueWithState<bool>>(
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
        pub fn size<V: crate::MaybeUpdateValueWithState<u32>>(
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
        pub fn src<V: crate::MaybeUpdateValueWithState<str>>(
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
        pub fn step<V: crate::MaybeUpdateValueWithState<str>>(
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
        pub fn type_<V: crate::MaybeUpdateValueWithState<str>>(
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
        pub fn value<V: crate::MaybeUpdateValueWithState<str>>(
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
        pub fn width<V: crate::MaybeUpdateValueWithState<u32>>(
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
        type State = super::render_state::RenderState<
            dyn super::render_state::RenderStateTypes<
                HtmlElementProps = <HtmlElementProps::Data<
                    TypeDefs::HtmlElementProps,
                > as crate::props::UpdateElement<web_sys::HtmlElement>>::State,
                accept = <TypeDefs::accept as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::State,
                alt = <TypeDefs::alt as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::State,
                auto_complete = <TypeDefs::auto_complete as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::State,
                capture = <TypeDefs::capture as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::State,
                checked = <TypeDefs::checked as ::frender_dom::props::MaybeUpdateValueWithState<
                    bool,
                >>::State,
                dirname = <TypeDefs::dirname as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::State,
                disabled = <TypeDefs::disabled as ::frender_dom::props::MaybeUpdateValueWithState<
                    bool,
                >>::State,
                form = <TypeDefs::form as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::State,
                form_action = <TypeDefs::form_action as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::State,
                form_enc_type = <TypeDefs::form_enc_type as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::State,
                form_method = <TypeDefs::form_method as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::State,
                form_no_validate = <TypeDefs::form_no_validate as ::frender_dom::props::MaybeUpdateValueWithState<
                    bool,
                >>::State,
                form_target = <TypeDefs::form_target as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::State,
                height = <TypeDefs::height as ::frender_dom::props::MaybeUpdateValueWithState<
                    u32,
                >>::State,
                list = <TypeDefs::list as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::State,
                max = <TypeDefs::max as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::State,
                max_length = <TypeDefs::max_length as ::frender_dom::props::MaybeUpdateValueWithState<
                    i32,
                >>::State,
                min = <TypeDefs::min as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::State,
                min_length = <TypeDefs::min_length as ::frender_dom::props::MaybeUpdateValueWithState<
                    i32,
                >>::State,
                multiple = <TypeDefs::multiple as ::frender_dom::props::MaybeUpdateValueWithState<
                    bool,
                >>::State,
                name = <TypeDefs::name as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::State,
                pattern = <TypeDefs::pattern as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::State,
                placeholder = <TypeDefs::placeholder as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::State,
                read_only = <TypeDefs::read_only as ::frender_dom::props::MaybeUpdateValueWithState<
                    bool,
                >>::State,
                required = <TypeDefs::required as ::frender_dom::props::MaybeUpdateValueWithState<
                    bool,
                >>::State,
                size = <TypeDefs::size as ::frender_dom::props::MaybeUpdateValueWithState<
                    u32,
                >>::State,
                src = <TypeDefs::src as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::State,
                step = <TypeDefs::step as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::State,
                type_ = <TypeDefs::type_ as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::State,
                value = <TypeDefs::value as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::State,
                width = <TypeDefs::width as ::frender_dom::props::MaybeUpdateValueWithState<
                    u32,
                >>::State,
            >,
        >;
        fn initialize_state(
            this: Self,
            element: &web_sys::HtmlInputElement,
            children_ctx: &mut ::frender_dom::Dom,
        ) -> Self::State {
            let dom_element: &::web_sys::Element = element.as_ref();
            super::render_state::RenderState {
                HtmlElementProps: <HtmlElementProps::Data<
                    TypeDefs::HtmlElementProps,
                > as crate::props::UpdateElement<
                    web_sys::HtmlElement,
                >>::initialize_state(this.HtmlElementProps, element, children_ctx),
                accept: <TypeDefs::accept as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.accept,
                    |v| element.set_accept(v),
                    || dom_element.remove_attribute("accept").unwrap(),
                ),
                alt: <TypeDefs::alt as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.alt,
                    |v| element.set_alt(v),
                    || dom_element.remove_attribute("alt").unwrap(),
                ),
                auto_complete: <TypeDefs::auto_complete as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.auto_complete,
                    |v| element.set_autocomplete(v),
                    || dom_element.remove_attribute("autocomplete").unwrap(),
                ),
                capture: <TypeDefs::capture as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.capture,
                    |v| crate::props::UpdateElementAttribute::update_element_attribute(
                        v,
                        dom_element,
                        "capture",
                    ),
                    || dom_element.remove_attribute("capture").unwrap(),
                ),
                checked: <TypeDefs::checked as ::frender_dom::props::MaybeUpdateValueWithState<
                    bool,
                >>::initialize_state_and_update(
                    this.checked,
                    |v| element.set_checked(*v),
                    || dom_element.remove_attribute("checked").unwrap(),
                ),
                dirname: <TypeDefs::dirname as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.dirname,
                    |v| crate::props::UpdateElementAttribute::update_element_attribute(
                        v,
                        dom_element,
                        "dirname",
                    ),
                    || dom_element.remove_attribute("dirname").unwrap(),
                ),
                disabled: <TypeDefs::disabled as ::frender_dom::props::MaybeUpdateValueWithState<
                    bool,
                >>::initialize_state_and_update(
                    this.disabled,
                    |v| element.set_disabled(*v),
                    || dom_element.remove_attribute("disabled").unwrap(),
                ),
                form: <TypeDefs::form as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.form,
                    |v| crate::props::UpdateElementAttribute::update_element_attribute(
                        v,
                        dom_element,
                        "form",
                    ),
                    || dom_element.remove_attribute("form").unwrap(),
                ),
                form_action: <TypeDefs::form_action as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.form_action,
                    |v| element.set_form_action(v),
                    || dom_element.remove_attribute("formaction").unwrap(),
                ),
                form_enc_type: <TypeDefs::form_enc_type as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.form_enc_type,
                    |v| element.set_form_enctype(v),
                    || dom_element.remove_attribute("formenctype").unwrap(),
                ),
                form_method: <TypeDefs::form_method as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.form_method,
                    |v| element.set_form_method(v),
                    || dom_element.remove_attribute("formmethod").unwrap(),
                ),
                form_no_validate: <TypeDefs::form_no_validate as ::frender_dom::props::MaybeUpdateValueWithState<
                    bool,
                >>::initialize_state_and_update(
                    this.form_no_validate,
                    |v| element.set_form_no_validate(*v),
                    || dom_element.remove_attribute("formnovalidate").unwrap(),
                ),
                form_target: <TypeDefs::form_target as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.form_target,
                    |v| element.set_form_target(v),
                    || dom_element.remove_attribute("formtarget").unwrap(),
                ),
                height: <TypeDefs::height as ::frender_dom::props::MaybeUpdateValueWithState<
                    u32,
                >>::initialize_state_and_update(
                    this.height,
                    |v| element.set_height(*v),
                    || dom_element.remove_attribute("height").unwrap(),
                ),
                list: <TypeDefs::list as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.list,
                    |v| crate::props::UpdateElementAttribute::update_element_attribute(
                        v,
                        dom_element,
                        "list",
                    ),
                    || dom_element.remove_attribute("list").unwrap(),
                ),
                max: <TypeDefs::max as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.max,
                    |v| element.set_max(v),
                    || dom_element.remove_attribute("max").unwrap(),
                ),
                max_length: <TypeDefs::max_length as ::frender_dom::props::MaybeUpdateValueWithState<
                    i32,
                >>::initialize_state_and_update(
                    this.max_length,
                    |v| element.set_max_length(*v),
                    || dom_element.remove_attribute("maxlength").unwrap(),
                ),
                min: <TypeDefs::min as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.min,
                    |v| element.set_min(v),
                    || dom_element.remove_attribute("min").unwrap(),
                ),
                min_length: <TypeDefs::min_length as ::frender_dom::props::MaybeUpdateValueWithState<
                    i32,
                >>::initialize_state_and_update(
                    this.min_length,
                    |v| element.set_min_length(*v),
                    || dom_element.remove_attribute("minlength").unwrap(),
                ),
                multiple: <TypeDefs::multiple as ::frender_dom::props::MaybeUpdateValueWithState<
                    bool,
                >>::initialize_state_and_update(
                    this.multiple,
                    |v| element.set_multiple(*v),
                    || dom_element.remove_attribute("multiple").unwrap(),
                ),
                name: <TypeDefs::name as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.name,
                    |v| element.set_name(v),
                    || dom_element.remove_attribute("name").unwrap(),
                ),
                pattern: <TypeDefs::pattern as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.pattern,
                    |v| element.set_pattern(v),
                    || dom_element.remove_attribute("pattern").unwrap(),
                ),
                placeholder: <TypeDefs::placeholder as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.placeholder,
                    |v| element.set_placeholder(v),
                    || dom_element.remove_attribute("placeholder").unwrap(),
                ),
                read_only: <TypeDefs::read_only as ::frender_dom::props::MaybeUpdateValueWithState<
                    bool,
                >>::initialize_state_and_update(
                    this.read_only,
                    |v| element.set_read_only(*v),
                    || dom_element.remove_attribute("readonly").unwrap(),
                ),
                required: <TypeDefs::required as ::frender_dom::props::MaybeUpdateValueWithState<
                    bool,
                >>::initialize_state_and_update(
                    this.required,
                    |v| element.set_required(*v),
                    || dom_element.remove_attribute("required").unwrap(),
                ),
                size: <TypeDefs::size as ::frender_dom::props::MaybeUpdateValueWithState<
                    u32,
                >>::initialize_state_and_update(
                    this.size,
                    |v| element.set_size(*v),
                    || dom_element.remove_attribute("size").unwrap(),
                ),
                src: <TypeDefs::src as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.src,
                    |v| element.set_src(v),
                    || dom_element.remove_attribute("src").unwrap(),
                ),
                step: <TypeDefs::step as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.step,
                    |v| element.set_step(v),
                    || dom_element.remove_attribute("step").unwrap(),
                ),
                type_: <TypeDefs::type_ as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.type_,
                    |v| element.set_type(v),
                    || dom_element.remove_attribute("type").unwrap(),
                ),
                value: <TypeDefs::value as ::frender_dom::props::MaybeUpdateValueWithState<
                    str,
                >>::initialize_state_and_update(
                    this.value,
                    |v| element.set_value(v),
                    || dom_element.remove_attribute("value").unwrap(),
                ),
                width: <TypeDefs::width as ::frender_dom::props::MaybeUpdateValueWithState<
                    u32,
                >>::initialize_state_and_update(
                    this.width,
                    |v| element.set_width(*v),
                    || dom_element.remove_attribute("width").unwrap(),
                ),
            }
        }
        fn update_element(
            this: Self,
            element: &web_sys::HtmlInputElement,
            children_ctx: &mut ::frender_dom::Dom,
            state: ::core::pin::Pin<&mut Self::State>,
        ) {
            let state = state.pin_project();
            let dom_element: &::web_sys::Element = element.as_ref();
            crate::props::UpdateElement::update_element(
                this.HtmlElementProps,
                element.as_ref(),
                children_ctx,
                state.HtmlElementProps,
            );
            <TypeDefs::accept as ::frender_dom::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.accept,
                state.accept,
                |v| element.set_accept(v),
                || dom_element.remove_attribute("accept").unwrap(),
            );
            <TypeDefs::alt as ::frender_dom::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.alt,
                state.alt,
                |v| element.set_alt(v),
                || dom_element.remove_attribute("alt").unwrap(),
            );
            <TypeDefs::auto_complete as ::frender_dom::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.auto_complete,
                state.auto_complete,
                |v| element.set_autocomplete(v),
                || dom_element.remove_attribute("autocomplete").unwrap(),
            );
            <TypeDefs::capture as ::frender_dom::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.capture,
                state.capture,
                |v| crate::props::UpdateElementAttribute::update_element_attribute(
                    v,
                    dom_element,
                    "capture",
                ),
                || dom_element.remove_attribute("capture").unwrap(),
            );
            <TypeDefs::checked as ::frender_dom::props::MaybeUpdateValueWithState<
                bool,
            >>::maybe_update_value_with_state(
                this.checked,
                state.checked,
                |v| element.set_checked(*v),
                || dom_element.remove_attribute("checked").unwrap(),
            );
            <TypeDefs::dirname as ::frender_dom::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.dirname,
                state.dirname,
                |v| crate::props::UpdateElementAttribute::update_element_attribute(
                    v,
                    dom_element,
                    "dirname",
                ),
                || dom_element.remove_attribute("dirname").unwrap(),
            );
            <TypeDefs::disabled as ::frender_dom::props::MaybeUpdateValueWithState<
                bool,
            >>::maybe_update_value_with_state(
                this.disabled,
                state.disabled,
                |v| element.set_disabled(*v),
                || dom_element.remove_attribute("disabled").unwrap(),
            );
            <TypeDefs::form as ::frender_dom::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.form,
                state.form,
                |v| crate::props::UpdateElementAttribute::update_element_attribute(
                    v,
                    dom_element,
                    "form",
                ),
                || dom_element.remove_attribute("form").unwrap(),
            );
            <TypeDefs::form_action as ::frender_dom::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.form_action,
                state.form_action,
                |v| element.set_form_action(v),
                || dom_element.remove_attribute("formaction").unwrap(),
            );
            <TypeDefs::form_enc_type as ::frender_dom::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.form_enc_type,
                state.form_enc_type,
                |v| element.set_form_enctype(v),
                || dom_element.remove_attribute("formenctype").unwrap(),
            );
            <TypeDefs::form_method as ::frender_dom::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.form_method,
                state.form_method,
                |v| element.set_form_method(v),
                || dom_element.remove_attribute("formmethod").unwrap(),
            );
            <TypeDefs::form_no_validate as ::frender_dom::props::MaybeUpdateValueWithState<
                bool,
            >>::maybe_update_value_with_state(
                this.form_no_validate,
                state.form_no_validate,
                |v| element.set_form_no_validate(*v),
                || dom_element.remove_attribute("formnovalidate").unwrap(),
            );
            <TypeDefs::form_target as ::frender_dom::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.form_target,
                state.form_target,
                |v| element.set_form_target(v),
                || dom_element.remove_attribute("formtarget").unwrap(),
            );
            <TypeDefs::height as ::frender_dom::props::MaybeUpdateValueWithState<
                u32,
            >>::maybe_update_value_with_state(
                this.height,
                state.height,
                |v| element.set_height(*v),
                || dom_element.remove_attribute("height").unwrap(),
            );
            <TypeDefs::list as ::frender_dom::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.list,
                state.list,
                |v| crate::props::UpdateElementAttribute::update_element_attribute(
                    v,
                    dom_element,
                    "list",
                ),
                || dom_element.remove_attribute("list").unwrap(),
            );
            <TypeDefs::max as ::frender_dom::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.max,
                state.max,
                |v| element.set_max(v),
                || dom_element.remove_attribute("max").unwrap(),
            );
            <TypeDefs::max_length as ::frender_dom::props::MaybeUpdateValueWithState<
                i32,
            >>::maybe_update_value_with_state(
                this.max_length,
                state.max_length,
                |v| element.set_max_length(*v),
                || dom_element.remove_attribute("maxlength").unwrap(),
            );
            <TypeDefs::min as ::frender_dom::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.min,
                state.min,
                |v| element.set_min(v),
                || dom_element.remove_attribute("min").unwrap(),
            );
            <TypeDefs::min_length as ::frender_dom::props::MaybeUpdateValueWithState<
                i32,
            >>::maybe_update_value_with_state(
                this.min_length,
                state.min_length,
                |v| element.set_min_length(*v),
                || dom_element.remove_attribute("minlength").unwrap(),
            );
            <TypeDefs::multiple as ::frender_dom::props::MaybeUpdateValueWithState<
                bool,
            >>::maybe_update_value_with_state(
                this.multiple,
                state.multiple,
                |v| element.set_multiple(*v),
                || dom_element.remove_attribute("multiple").unwrap(),
            );
            <TypeDefs::name as ::frender_dom::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.name,
                state.name,
                |v| element.set_name(v),
                || dom_element.remove_attribute("name").unwrap(),
            );
            <TypeDefs::pattern as ::frender_dom::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.pattern,
                state.pattern,
                |v| element.set_pattern(v),
                || dom_element.remove_attribute("pattern").unwrap(),
            );
            <TypeDefs::placeholder as ::frender_dom::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.placeholder,
                state.placeholder,
                |v| element.set_placeholder(v),
                || dom_element.remove_attribute("placeholder").unwrap(),
            );
            <TypeDefs::read_only as ::frender_dom::props::MaybeUpdateValueWithState<
                bool,
            >>::maybe_update_value_with_state(
                this.read_only,
                state.read_only,
                |v| element.set_read_only(*v),
                || dom_element.remove_attribute("readonly").unwrap(),
            );
            <TypeDefs::required as ::frender_dom::props::MaybeUpdateValueWithState<
                bool,
            >>::maybe_update_value_with_state(
                this.required,
                state.required,
                |v| element.set_required(*v),
                || dom_element.remove_attribute("required").unwrap(),
            );
            <TypeDefs::size as ::frender_dom::props::MaybeUpdateValueWithState<
                u32,
            >>::maybe_update_value_with_state(
                this.size,
                state.size,
                |v| element.set_size(*v),
                || dom_element.remove_attribute("size").unwrap(),
            );
            <TypeDefs::src as ::frender_dom::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.src,
                state.src,
                |v| element.set_src(v),
                || dom_element.remove_attribute("src").unwrap(),
            );
            <TypeDefs::step as ::frender_dom::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.step,
                state.step,
                |v| element.set_step(v),
                || dom_element.remove_attribute("step").unwrap(),
            );
            <TypeDefs::type_ as ::frender_dom::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.type_,
                state.type_,
                |v| element.set_type(v),
                || dom_element.remove_attribute("type").unwrap(),
            );
            <TypeDefs::value as ::frender_dom::props::MaybeUpdateValueWithState<
                str,
            >>::maybe_update_value_with_state(
                this.value,
                state.value,
                |v| element.set_value(v),
                || dom_element.remove_attribute("value").unwrap(),
            );
            <TypeDefs::width as ::frender_dom::props::MaybeUpdateValueWithState<
                u32,
            >>::maybe_update_value_with_state(
                this.width,
                state.width,
                |v| element.set_width(*v),
                || dom_element.remove_attribute("width").unwrap(),
            );
        }
    }
}
