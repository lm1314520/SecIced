use iced::widget::{column, container, row, scrollable, text, Container};
use iced::Theme;
use iced::Alignment::Center;

use crate::components::square::square;
use crate::styles::myicon::Icon;
use crate::{App, Message};

pub fn content(app: &App) -> Container<Message, Theme> {    
    container(
    scrollable(
            column![
                text("内容页:按上下方向键试试"),
                Icon::get_hourglass(app.value).size(40),
                row((1..10).map(|i| square(if i % 2 == 0 { 80 } else { 160 })))
                    .spacing(20)
                    .align_y(Center)
                    .wrap()
            ]
            .spacing(40)
            .align_x(Center)
            .width(iced::Fill),
        )
        .height(iced::Fill),
    )
    .padding(10)
}