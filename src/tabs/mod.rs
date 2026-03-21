pub mod page;
pub mod tree;

#[derive(Copy, Clone, Debug)]
pub enum ActiveTab {
    // Showing the webpage
    Page,
    // Showing the HTML tree
    Tree,
    // Showing the JavaScript console
    Console,
}

impl ActiveTab {
    pub fn get_title(&self) -> String {
        match self {
            Self::Page => "Web Page",
            Self::Tree => "HTML Tree",
            Self::Console => "JavaScript Console",
        }
        .into()
    }
}
