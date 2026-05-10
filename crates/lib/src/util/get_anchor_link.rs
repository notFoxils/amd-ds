use snafu::Snafu;

#[derive(Debug, Snafu)]
pub enum GetAnchorLinkError {
    #[snafu(display("the provided element was not an anchor"))]
    ElementNotAnchor,
    #[snafu(display("the provided anchor-element does not have an href"))]
    AnchorHrefMissing,
}

pub fn get_anchor_link(element: &scraper::node::Element) -> Result<&str, GetAnchorLinkError> {
    const ANCHOR_ELEMENT_NAME: &str = "a";
    const ANCHOR_HREF_ATTR_NAME: &str = "href";

    if element.name() != ANCHOR_ELEMENT_NAME {
        return ElementNotAnchorSnafu.fail();
    }

    element
        .attr(ANCHOR_HREF_ATTR_NAME)
        .ok_or(AnchorHrefMissingSnafu.build())
}
