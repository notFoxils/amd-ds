use scraper::selector::ToCss;
use snafu::{OptionExt, Snafu};

#[derive(Debug, Snafu)]
pub enum SelectFirstError {
    #[snafu(display("could not find a match using the provided selector: '{selector}'"))]
    SelectorNonMatching { selector: Box<str> },
}

pub trait SelectFirst<'a>: Sized + scraper::selectable::Selectable<'a> {
    fn select_first(
        self,
        selector: &scraper::Selector,
    ) -> Result<<Self::Select<'_> as Iterator>::Item, SelectFirstError> {
        self.select(selector)
            .next()
            .context(SelectorNonMatchingSnafu {
                selector: selector.clone().to_css_string(),
            })
    }
}

impl<'a, T: scraper::selectable::Selectable<'a>> SelectFirst<'a> for T {}
