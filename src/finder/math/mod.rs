use std::rc::Rc;

use crate::finder::{Entry, Finder, math::entry::MathEntry};

mod entry;

pub struct MathFinder {}

impl Finder for MathFinder {
    fn new() -> Self {
        Self {}
    }

    fn default_entries(&self) -> Option<Vec<Rc<dyn Entry>>> {
        None
    }

    fn filtered_entries(
        &self,
        _matcher: &mut nucleo_matcher::Matcher,
        search_term: &str,
    ) -> Option<Vec<Rc<dyn Entry>>> {
        let search_term = search_term.strip_prefix('=')?;
        let value = evalexpr::eval(search_term).ok()?;

        Some(vec![Rc::new(MathEntry {
            result: value.to_string().into(),
        })])
    }
}
