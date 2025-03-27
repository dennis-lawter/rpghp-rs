use handlebars::Handlebars;

#[allow(dead_code)]
#[derive(Clone)]
pub struct FrontendSharedState<'a> {
    pub hb: Handlebars<'a>,
}
impl FrontendSharedState<'static> {
    pub fn new() -> Self {
        let hb = Handlebars::new();
        Self { hb }
    }
}
