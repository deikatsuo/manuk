use {Rc, Tera};

/// App State
pub struct AppState {
    pub context: Rc<ContextState>,
}

/// State Wrapper
pub struct ContextState {
    pub tera: Rc<Tera>,
}