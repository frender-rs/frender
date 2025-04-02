/// ```
/// assert!(true)
/// ```
///
/// ```compile_fail
/// ::frender_attr_value::attr_value!(());
/// ```
enum EmptyShouldFail {}
