pub trait Html {
    fn tag(&self) -> String;
    fn closing_tag(&self) -> bool;
    fn attrs(&self) -> String;
    fn render(&self) -> String;
}


pub fn render_html(html: &Html) -> String {
    let closing_tag = if html.closing_tag() {
        String::from_str("</") + html.tag() + String::from_str(">")
    } else {"".to_string()};

    let mut v = vec!();
    v.push(String::from_str("<"));
    v.push(html.tag());
    v.push(html.attrs());
    v.push(String::from_str(">"));
    v.push(closing_tag);
    v.connect(" ")
}
