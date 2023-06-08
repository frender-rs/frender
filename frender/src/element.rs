use frender_csr::CsrElement;
use frender_ssr::SsrElement;

/// A trait alias for <code>[CsrElement] + [SsrElement]</code>
pub trait Element: CsrElement + SsrElement {}

impl<E: CsrElement + SsrElement> Element for E {}
