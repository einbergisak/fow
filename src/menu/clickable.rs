use ggez::graphics;

#[derive(PartialEq, Debug)]
pub(crate) enum ClickableGroup {
    MainMenu = 0,
    MainMenuList = 1,
    InGame = 2,
    GameOverMenu = 3
}

pub(crate) struct Transform {
    pub(crate) x: i32,
    pub(crate) y: i32,
    pub(crate) width: i32,
    pub(crate) height: i32,
}
pub(crate) struct Clickable {
    pub(crate) id: String,
    pub(crate) transform: Transform,
    pub(crate) color: graphics::Color,
    pub(crate) text: String,
    pub(crate) hovered: bool,
    pub(crate) list_item: bool,
    pub(crate) group: ClickableGroup
}
