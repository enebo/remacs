use remacs_macros::lisp_fn;
use lisp::{LispObject, ExternalPtr};
use remacs_sys::{Lisp_Symbol, Qsetting_constant};

pub type LispSymbolRef = ExternalPtr<Lisp_Symbol>;

impl LispSymbolRef {
    pub fn symbol_name(&self) -> LispObject {
        LispObject::from_raw(self.name)
    }

    pub fn get_function(&self) -> LispObject {
        LispObject::from_raw(self.function)
    }

    pub fn get_plist(&self) -> LispObject {
        LispObject::from_raw(self.plist)
    }

    pub fn set_plist(&mut self, plist: LispObject) {
        self.plist = plist.to_raw();
    }

    pub fn set_function(&mut self, function: LispObject) {
        self.function = function.to_raw();
    }
}

/// Return t if OBJECT is a symbol.
#[lisp_fn]
fn symbolp(object: LispObject) -> LispObject {
    LispObject::from_bool(object.is_symbol())
}

/// Return SYMBOL's name, a string.
#[lisp_fn]
pub fn symbol_name(symbol: LispObject) -> LispObject {
    symbol.as_symbol_or_error().symbol_name()
}


/* It has been previously suggested to make this function an alias for
   symbol-function, but upon discussion at Bug#23957, there is a risk
   breaking backward compatibility, as some users of fboundp may
   expect `t' in particular, rather than any true value.  */

/// Return t if SYMBOL's function definition is not void.
#[lisp_fn]
pub fn fboundp(symbol: LispObject) -> LispObject {
    let symbol = symbol.as_symbol_or_error();
    LispObject::from_bool(symbol.get_function().is_not_nil())
}

/// Return SYMBOL's function definition, or nil if that is void.
#[lisp_fn]
fn symbol_function(symbol: LispObject) -> LispObject {
    symbol.as_symbol_or_error().get_function()
}

/// Return SYMBOL's property list.
#[lisp_fn]
fn symbol_plist(symbol: LispObject) -> LispObject {
    symbol.as_symbol_or_error().get_plist()
}

/// Set SYMBOL's property list to NEWPLIST, and return NEWPLIST.
#[lisp_fn]
fn setplist(symbol: LispObject, newplist: LispObject) -> LispObject {
    symbol.as_symbol_or_error().set_plist(newplist);
    newplist
}

/// Make SYMBOL's function definition be nil.
/// Return SYMBOL.
#[lisp_fn]
fn fmakunbound(symbol: LispObject) -> LispObject {
    let mut sym = symbol.as_symbol_or_error();
    if symbol.is_nil() || symbol.is_t() {
        xsignal!(Qsetting_constant, symbol);
    }
    sym.set_function(LispObject::constant_nil());
    symbol
}
