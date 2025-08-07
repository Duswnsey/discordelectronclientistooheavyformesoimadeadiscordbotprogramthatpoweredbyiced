use iced::{
    border,
    widget::{button, column, container, row, svg, text, text_input},
    Border, Length, Padding,
};

use crate::{
    frontend::{state::State, update::Message},
    japda::from_hex,
};

pub fn view(state: &State) -> iced::Element<'_, Message> {
    container(column(vec![
        container(row(vec![
            text!("대충 서버 사진").into(),
            text!("대충 서버 이름").into(),
        ]))
        .center(Length::Shrink)
        .width(Length::Fill)
        .style(|_| container::Style::from(from_hex("@02659E")))
        .padding(Padding::new(5.0))
        .into(),
        row(vec![
            container(
                column(vec![
                    container("섭")
                        .width(Length::Fixed(40.0))
                        .height(Length::Fixed(40.0))
                        .style(|_| {
                            container::Style::from(from_hex("#248790"))
                                .border(Border::default().rounded(10))
                        })
                        .into(),
                    container("섭")
                        .width(Length::Fixed(40.0))
                        .height(Length::Fixed(40.0))
                        .style(|_| {
                            container::Style::from(from_hex("#248790"))
                                .border(Border::default().rounded(10))
                        })
                        .into(),
                ])
                .spacing(10),
            )
            .style(|_| container::Style::from(from_hex("@02659E")))
            .height(Length::Fill)
            .padding(5)
            .into(),
            column(vec![
                container("대충 체팅 들어갈거들").height(Length::Fill).into(),
                container(
                    container(
                        row(vec![
                            svg("insert.svg").into(),
                            text_input("Message Here", &state.message_input).on_input(Message::MessageInput).into(),
                                ]))
                        .style(|_| {
                            container::Style::from(from_hex("#02659E"))
                                .border(Border::default().rounded(10))
                        })
                        .center(Length::Fill)
                        .height(50)
                        .width(Length::Fill)
                ).padding(10)
                .into(),
            ])
            .height(Length::Fill)
            .into(),
        ])
        .into(),
    ]))
    .width(Length::Fill)
    .height(Length::Fill)
    .style(|_theme| container::Style::from(from_hex("#248790")).color(iced::Color::WHITE))
    .into()
}
