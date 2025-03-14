use iced::widget::{container, text, Column, Container};
use iced::Theme;
use iced::Alignment::Center;

use crate::components::square::square;
use crate::styles::myfont::{BIG_SIZE, TITLE};
use crate::{App, Message};

pub fn sidebar(app: &App) -> Container<Message, Theme> {    
    Container::new(
        Column::new().push(text("菜单栏").font(TITLE).size(BIG_SIZE))
                        .push(square(50))
                        .push(square(50))
                        .padding(10)
                        .spacing(40)
                        .width(200)
                        .align_x(Center)
    ).style(container::rounded_box)
    .center_y(iced::Fill)
}