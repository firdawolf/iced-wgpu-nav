#[derive(Debug, Clone)]
pub enum Message {
    CurrentScreen(i8),
    UpdateHomeScreen(HomeScreenState),
}
#[derive(Debug, Clone)]
pub enum Home {
    Loading,
    Loaded(PublicScreenState),
}
#[derive(Debug, Clone)]
pub struct PublicScreenState {
    pub currentscreen: i8,
    pub home_screen_state: HomeScreenState,
}
#[derive(Debug, Clone)]
pub struct HomeScreenState {
    pub fps: String,
}
