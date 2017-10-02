
pub struct WindowBuilderBase {
    title : Option<String>,
    size : Option<(i32, i32)>
}

impl WindowBuilderBase<W> {

    pub fn new() -> WindowBuilderBase {
        WindowBuilderBase {
            title : None,
            size : None
        }
    }

    pub fn with_title<'a>(&'a mut self, string : String) -> &'a mut WindowBuilder {
        self.title = Some(string.clone());
        self
    }

    pub fn with_size<'a>(&'a mut self, width : i32, height : i32) -> &'a mut WindowBuilder {
        self.size = Some((width, height));
        self
    }

    pub fn build(&self) -> W;

}