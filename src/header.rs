use iced::widget::{container, horizontal_space, row, text, Container};
use iced::{border, Theme};
use iced::Alignment::Center;

use crate::components::square::square;
use crate::styles::myfont::{BIGTITLE, MAX_SIZE};
use crate::{App, Message};

pub fn header(app: &App) -> Container<Message> {    
    container(
    row![
            square(40),
            horizontal_space(),
            text("带布局的脚手架").font(BIGTITLE).size(MAX_SIZE),
            horizontal_space(),
            square(40),
        ]
        .padding(10)
        .align_y(Center),
    )
    .style(|theme| {
        let palette = theme.extended_palette();

        container::Style::default()
            .border(border::color(palette.background.strong.color).width(1))
    })
}