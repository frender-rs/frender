/// 'none' | 'inline' | 'list' | 'both'
pub enum AriaAutoComplete {
    None,
    Inline,
    List,
    Both,
}

pub enum MixedBool {
    Bool(bool),
    Mixed,
}

pub enum AriaCurrent {
    Bool(bool),
    Page,
    Step,
    Location,
    Date,
    Time,
}

/// 'none' | 'copy' | 'execute' | 'link' | 'move' | 'popup'
pub enum AriaDropEffect {
    None,
    Copy,
    Execute,
    Link,
    Move,
    Popup,
}

/// boolean | 'false' | 'true' | 'menu' | 'listbox' | 'tree' | 'grid' | 'dialog' | undefined
pub enum AriaHasPopup {
    Bool(bool),
    Menu,
    ListBox,
    Tree,
    Grid,
    Dialog,
}

pub enum AriaInvalid {
    Bool(bool),
    Grammar,
    Spelling,
}

/// 'off' | 'assertive' | 'polite'
pub enum AriaLive {
    Off,
    Assertive,
    Polite,
}

/// 'horizontal' | 'vertical'
pub enum AriaOrientation {
    Horizontal,
    Vertical,
}

/// 'additions' | 'additions removals' | 'additions text' | 'all' | 'removals' | 'removals additions' | 'removals text' | 'text' | 'text additions' | 'text removals'
pub enum AriaRelevant {
    Additions,
    AdditionsRemovals,
    AdditionsText,
    All,
    Removals,
    RemovalsAdditions,
    RemovalsText,
    Text,
    TextAdditions,
    TextRemovals,
}

/// 'none' | 'ascending' | 'descending' | 'other'
pub enum AriaSort {
    None,
    Ascending,
    Descending,
    Other,
}

pub enum AriaRole {
    Alert,
    AlertDialog,
    Application,
    Article,
    Banner,
    Button,
    Cell,
    Checkbox,
    ColumnHeader,
    ComboBox,
    Complementary,
    ContentInfo,
    Definition,
    Dialog,
    Directory,
    Document,
    Feed,
    Figure,
    Form,
    Grid,
    GridCell,
    Group,
    Heading,
    Img,
    Link,
    List,
    ListBox,
    ListItem,
    Log,
    Main,
    Marquee,
    Math,
    Menu,
    Menubar,
    MenuItem,
    MenuItemCheckbox,
    MenuItemRadio,
    Navigation,
    None,
    Note,
    Option,
    Presentation,
    ProgressBar,
    Radio,
    RadioGroup,
    Region,
    Row,
    RowGroup,
    RowHeader,
    Scrollbar,
    Search,
    SearchBox,
    Separator,
    Slider,
    SpinButton,
    Status,
    Switch,
    Tab,
    Table,
    TabList,
    TabPanel,
    Term,
    TextBox,
    Timer,
    Toolbar,
    Tooltip,
    Tree,
    TreeGrid,
    TreeItem,
    Custom(String),
}

/// All the WAI-ARIA 1.1 attributes from https://www.w3.org/TR/wai-aria-1.1/
pub struct AriaAttributes {
    /** Identifies the currently active element when DOM focus is on a composite widget, textbox, group, or application. */
    activedescendant: Option<String>,
    /** Indicates whether assistive technologies will present all, or only parts of, the changed region based on the change notifications defined by the aria-relevant attribute. */
    atomic: Option<bool>,
    /**
     * Indicates whether inputting text could trigger display of one or more predictions of the user's intended value for an input and specifies how predictions would be
     * presented if they are made.
     */
    autocomplete: Option<AriaAutoComplete>,
    /** Indicates an element is being modified and that assistive technologies MAY want to wait until the modifications are complete before exposing them to the user. */
    busy: Option<bool>,
    /**
     * Indicates the current "checked" state of checkboxes, radio buttons, and other widgets.
     * @see aria-pressed @see aria-selected.
     */
    checked: Option<MixedBool>,
    /**
     * Defines the total number of columns in a table, grid, or treegrid.
     * @see aria-colindex.
     */
    colcount: Option<i32>,
    /**
     * Defines an element's column index or position with respect to the total number of columns within a table, grid, or treegrid.
     * @see aria-colcount @see aria-colspan.
     */
    colindex: Option<i32>,
    /**
     * Defines the number of columns spanned by a cell or gridcell within a table, grid, or treegrid.
     * @see aria-colindex @see aria-rowspan.
     */
    colspan: Option<i32>,
    /**
     * Identifies the element (or elements) whose contents or presence are controlled by the current element.
     * @see aria-owns.
     */
    controls: Option<String>,
    /** Indicates the element that represents the current item within a container or set of related elements. */
    current: Option<AriaCurrent>,
    /**
     * Identifies the element (or elements) that describes the object.
     * @see aria-labelledby
     */
    describedby: Option<String>,
    /**
     * Identifies the element that provides a detailed, extended description for the object.
     * @see aria-describedby.
     */
    details: Option<String>,
    /**
     * Indicates that the element is perceivable but disabled, so it is not editable or otherwise operable.
     * @see aria-hidden @see aria-readonly.
     */
    disabled: Option<bool>,
    /**
     * Indicates what functions can be performed when a dragged object is released on the drop target.
     * @deprecated in ARIA 1.1
     */
    dropeffect: Option<AriaDropEffect>,
    /**
     * Identifies the element that provides an error message for the object.
     * @see aria-invalid @see aria-describedby.
     */
    errormessage: Option<String>,
    /** Indicates whether the element, or another grouping element it controls, is currently expanded or collapsed. */
    expanded: Option<bool>,
    /**
     * Identifies the next element (or elements) in an alternate reading order of content which, at the user's discretion,
     * allows assistive technology to override the general default of reading in document source order.
     */
    flowto: Option<String>,
    /**
     * Indicates an element's "grabbed" state in a drag-and-drop operation.
     * @deprecated in ARIA 1.1
     */
    grabbed: Option<bool>,
    /** Indicates the availability and type of interactive popup element, such as menu or dialog, that can be triggered by an element. */
    haspopup: Option<AriaHasPopup>,
    /**
     * Indicates whether the element is exposed to an accessibility API.
     * @see aria-disabled.
     */
    hidden: Option<bool>,
    /**
     * Indicates the entered value does not conform to the format expected by the application.
     * @see aria-errormessage.
     */
    invalid: Option<AriaInvalid>,
    /** Indicates keyboard shortcuts that an author has implemented to activate or give focus to an element. */
    keyshortcuts: Option<String>,
    /**
     * Defines a string value that labels the current element.
     * @see aria-labelledby.
     */
    label: Option<String>,
    /**
     * Identifies the element (or elements) that labels the current element.
     * @see aria-describedby.
     */
    labelledby: Option<String>,
    /** Defines the hierarchical level of an element within a structure. */
    level: Option<i32>,
    /** Indicates that an element will be updated, and describes the types of updates the user agents, assistive technologies, and user can expect from the live region. */
    live: Option<AriaLive>,
    /** Indicates whether an element is modal when displayed. */
    modal: Option<bool>,
    /** Indicates whether a text box accepts multiple lines of input or only a single line. */
    multiline: Option<bool>,
    /** Indicates that the user may select more than one item from the current selectable descendants. */
    multiselectable: Option<bool>,
    /** Indicates whether the element's orientation is horizontal, vertical, or unknown/ambiguous. */
    orientation: Option<AriaOrientation>,
    /**
     * Identifies an element (or elements) in order to define a visual, functional, or contextual parent/child relationship
     * between DOM elements where the DOM hierarchy cannot be used to represent the relationship.
     * @see aria-controls.
     */
    owns: Option<String>,
    /**
     * Defines a short hint (a word or short phrase) intended to aid the user with data entry when the control has no value.
     * A hint could be a sample value or a brief description of the expected format.
     */
    placeholder: Option<String>,
    /**
     * Defines an element's number or position in the current set of listitems or treeitems. Not required if all elements in the set are present in the DOM.
     * @see aria-setsize.
     */
    posinset: Option<i32>,
    /**
     * Indicates the current "pressed" state of toggle buttons.
     * @see aria-checked @see aria-selected.
     */
    pressed: Option<MixedBool>,
    /**
     * Indicates that the element is not editable, but is otherwise operable.
     * @see aria-disabled.
     */
    readonly: Option<bool>,
    /**
     * Indicates what notifications the user agent will trigger when the accessibility tree within a live region is modified.
     * @see aria-atomic.
     */
    relevant: Option<AriaRelevant>,
    /** Indicates that user input is required on the element before a form may be submitted. */
    required: Option<bool>,
    /** Defines a human-readable, author-localized description for the role of an element. */
    roledescription: Option<String>,
    /**
     * Defines the total number of rows in a table, grid, or treegrid.
     * @see aria-rowindex.
     */
    rowcount: Option<i32>,
    /**
     * Defines an element's row index or position with respect to the total number of rows within a table, grid, or treegrid.
     * @see aria-rowcount @see aria-rowspan.
     */
    rowindex: Option<i32>,
    /**
     * Defines the number of rows spanned by a cell or gridcell within a table, grid, or treegrid.
     * @see aria-rowindex @see aria-colspan.
     */
    rowspan: Option<i32>,
    /**
     * Indicates the current "selected" state of various widgets.
     * @see aria-checked @see aria-pressed.
     */
    selected: Option<bool>,
    /**
     * Defines the number of items in the current set of listitems or treeitems. Not required if all elements in the set are present in the DOM.
     * @see aria-posinset.
     */
    setsize: Option<i32>,
    /** Indicates if items in a table or grid are sorted in ascending or descending order. */
    sort: Option<AriaSort>,
    /** Defines the maximum allowed value for a range widget. */
    valuemax: Option<f64>,
    /** Defines the minimum allowed value for a range widget. */
    valuemin: Option<f64>,
    /**
     * Defines the current value for a range widget.
     * @see aria-valuetext.
     */
    valuenow: Option<f64>,
    /** Defines the human readable text alternative of aria-valuenow for a range widget. */
    valuetext: Option<String>,
}
