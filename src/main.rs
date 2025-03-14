use std::borrow::Cow;

use content::content;
use header::header;
use iced::{keyboard, Element, Subscription, Task};
use iced::widget::{column, row};
use sidebar::sidebar;
use styles::myfont::{BIGTITLE_BYTES, ICONS_BYTES, NORMAL_SIZE, TITLE_BYTES, TXT, TXT_BYTES};

mod styles;
mod components;
mod header;
mod sidebar;
mod content;

//by LM
//QQ:195244527
pub const APP_NAME: &str = "脚手架";

pub fn main() -> iced::Result {
    let app_set = iced::Settings{
        // id:Some(APP_NAME.to_string()),
        fonts: vec![
            Cow::Borrowed(BIGTITLE_BYTES),
            Cow::Borrowed(TITLE_BYTES),
            Cow::Borrowed(TXT_BYTES),
            Cow::Borrowed(ICONS_BYTES)
        ],
        default_font:TXT, //默认字体为：仿宋_GB2312
        default_text_size:NORMAL_SIZE,
        ..Default::default()
    };

    iced::application(APP_NAME, App::update, App::view)
        .subscription(App::subscription)
        .settings(app_set)
        .run()
}

#[derive(Default)]
struct App{
    value: i64,
}

#[derive(Debug, Clone)]
enum Message {
    Increment,
    Decrement
}

impl App {
    fn update(&mut self, message: Message) ->Task<Message> {
        match message {
            Message::Increment => self.value = add_one(self.value),
            Message::Decrement => self.value = subtract_one(self.value),
        }
        Task::none()
    }

    fn view(&self) ->   Element<Message> {
        let header = header(self);
        let sidebar = sidebar(self);
        let content = content(self);
    
        column![header, row![sidebar, content]].into()
    }

    fn subscription(&self) -> Subscription<Message> {
        use keyboard::key;

        keyboard::on_key_press(|key, modifiers| {
            let keyboard::Key::Named(key) = key else {
                return None;
            };

            match (key, modifiers) {
                // (key::Named::ArrowUp, keyboard::Modifiers::SHIFT) => {
                //     Some(Message::Increment)
                // }
                (key::Named::ArrowUp, _) => {
                    Some(Message::Increment)
                }
                (key::Named::ArrowDown, _) => {
                    Some(Message::Decrement)
                }
                _ => None,
            }
        })
    }
}

fn add_one(value: i64) -> i64 {
    match value {
        0 => 1,
        1 => 2,
        2 => 3,
        3 => 0,
        _ => 0, // 默认情况，防止意外输入
    }
}

fn subtract_one(value: i64) -> i64 {
    match value {
        0 => 3,
        1 => 0,
        2 => 1,
        3 => 2,
        _ => 3, // 默认情况，防止意外输入
    }
}