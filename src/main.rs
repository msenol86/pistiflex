mod animation;
mod calc;
mod game;
mod widget;

use std::{sync::Arc, thread};

use calc::series_xy;
use fltk::{
    app,
    button::{self, Button},
    enums::{self, CallbackTrigger},
    frame::{self, Frame},
    group::{self, Pack},
    prelude::*,
    window::Window,
};
use fltk_theme::{ThemeType, WidgetTheme};
use widget::{button_constructor};

use game::Game;

use std::sync::mpsc;

use crate::{
    game::{rank_to_str, Card, Suit},
    widget::draw_card,
};

#[cfg(test)]
mod test;

#[derive(Clone, Debug)]
pub enum Row {
    Top,
    Bottom,
}

#[derive(Clone, Debug)]
pub struct ButtonAnimation {
    startx: i32,
    starty: i32,
    endx: i32,
    endy: i32,
    card: Card,
    row: Row,
    card_index: usize,
}

#[derive(Clone, Debug)]
pub enum ChannelMessage {
    UI(u32),
    BR(ButtonAnimation),
}

fn main() {
    let mut my_game = Game::new();
    my_game.create_deck();
    my_game.shuffle_deck();
    my_game.put_cards_onto_board();
    while my_game.is_reshuffle_required() {
        println!("J is the top card on board. Reshuffling");
        my_game.create_deck();
        my_game.shuffle_deck();
        my_game.put_cards_onto_board();
    }
    my_game.give_cards_to_players();
    let a = app::App::default();
    let (s, r) = app::channel::<ChannelMessage>();
    let (t_s, t_r) = mpsc::channel::<ButtonAnimation>();
    let theme = WidgetTheme::new(ThemeType::Metro);
    theme.apply();
    const WIN_WIDTH: i32 = 600;
    const WIN_HEIGHT: i32 = 600;
    const CARD_H: i32 = 315 / 2;
    const CARD_W: i32 = 225 / 2;
    const CARD_MARGIN: i32 = 80;

    let mut win = Window::default()
        .with_size(WIN_WIDTH, WIN_HEIGHT)
        .with_label("My Window");
    let ai_1 = button_constructor("1".to_string()).center_of_parent();
    let ai_2 = button_constructor("2".to_string()).center_of_parent();
    let ai_3 = button_constructor("3".to_string()).center_of_parent();
    let ai_4 = button_constructor("4".to_string()).center_of_parent();
    let pl_1 = button_constructor("1".to_string()).center_of_parent();
    let pl_2 = button_constructor("2".to_string()).center_of_parent();
    let pl_3 = button_constructor("3".to_string()).center_of_parent();
    let pl_4 = button_constructor("4".to_string()).center_of_parent();

    let tmp_b = pl_1.clone();
    for i in 0..my_game.deck.len() {
        let mut _deck = frame::Frame::default()
            .with_size(CARD_W, CARD_H)
            .center_of_parent();

        _deck.set_pos(_deck.x() - 200, _deck.y());
        draw_card(&mut _deck, my_game.deck[i]);
        _deck.to_owned().set_pos(
            _deck.x() - tmp_b.width() - (tmp_b.width() / 2) + (i as i32 * 1),
            _deck.y(),
        );
        _deck.to_owned().deactivate();
    }
    let mut board = frame::Frame::default()
        .with_label("B")
        .with_size(CARD_W, CARD_H)
        .center_of_parent();
    draw_card(&mut board, my_game.board[my_game.board.len() - 1]);

    let start = button::Button::default()
        .with_label("Start")
        .with_size(100, 50)
        .center_of_parent();
    start.to_owned().set_pos(start.x() + 150, start.y());

    win.end();
    win.show();

    let but_w = tmp_b.w();
    let but_h = tmp_b.h();

    let mut top_cards = vec![ai_1, ai_2, ai_3, ai_4];
    let mut bottom_cards = vec![pl_1, pl_2, pl_3, pl_4];

    for my_index in 0..4 {
        let mut ai_but = top_cards.get_mut(my_index).unwrap();
        ai_but.set_size(CARD_W, CARD_H);
        ai_but.set_pos((my_index as i32 + 1) * CARD_MARGIN + (CARD_MARGIN / 2), 20);
        draw_card(&mut ai_but, my_game.player2_hand[my_index]);

        let mut pl_but = bottom_cards.get_mut(my_index).unwrap();
        pl_but.set_size(CARD_W, CARD_H);
        pl_but.set_pos((my_index as i32 + 1) * CARD_MARGIN + (CARD_MARGIN / 2), WIN_HEIGHT - 20 - CARD_H);
        draw_card(&mut pl_but, my_game.player1_hand[my_index]);
    }

    let board_card = my_game.board[3];
    board
        .to_owned()
        .set_label(format!("{}{}", rank_to_str(board_card.rank), board_card.suit).as_str());
    // set_button_color(&board, board_card.suit);

    let my_game_arc = Arc::new(my_game);
    for (j, a_vec) in [&top_cards, &bottom_cards].iter().enumerate() {
        for (i, a_but) in a_vec.iter().enumerate() {
            let endx = board.x();
            let endy = board.y();
            let t_game = my_game_arc.clone();
            let t_s = s.clone();
            a_but.to_owned().set_callback(move |b| {
                b.to_owned().emit(
                    t_s.to_owned(),
                    ChannelMessage::BR(ButtonAnimation {
                        startx: b.x(),
                        starty: b.y(),
                        endx: endx,
                        endy: endy,
                        card: if j == 0 {
                            *t_game.player2_hand.get(i).unwrap()
                        } else {
                            *t_game.player1_hand.get(i).unwrap()
                        },
                        row: if j == 0 { Row::Top } else { Row::Bottom },
                        card_index: i,
                    }),
                );
            });
            a_but.to_owned().handle(|b, ev| match ev {
                enums::Event::Push => {
                    b.do_callback();
                    true
                }
                _ => false,
            })
        }
    }

    let _animator = thread::spawn(move || loop {
        if let Ok(ba) = t_r.recv() {
            println!("animation mesg: {:?}", ba);
            let t_index = win.children();
            let mut new_but =
                button_constructor(format!("{}{}", rank_to_str(ba.card.rank), ba.card.suit))
                    .with_pos(ba.startx, ba.starty)
                    .with_size(but_w, but_h);
            new_but.set_size(tmp_b.width(), tmp_b.height());
            draw_card(&mut new_but, ba.card);
            win.insert(&new_but, t_index);
            match ba.row {
                Row::Top => match top_cards.get(ba.card_index) {
                    Some(a_b) => a_b.to_owned().hide(),
                    None => {}
                },
                Row::Bottom => match bottom_cards.get(ba.card_index) {
                    Some(a_b) => a_b.to_owned().hide(),
                    None => {}
                },
            }
            let time = 50.0;
            let time_len = time as usize;
            let st = series_xy(ba.startx, ba.endx, ba.starty, ba.endy, time);
            let series_x = st.0;
            let series_y = st.1;
            for i in 0..time_len {
                new_but.set_pos(*series_x.get(i).unwrap(), *series_y.get(i).unwrap());
                app::sleep(0.01);
                new_but.parent().unwrap().redraw();
            }
        }
    });

    while a.wait() {
        if let Some(c_msg) = r.recv() {
            match c_msg {
                ChannelMessage::UI(ui_code) => {
                    println!("recevied code: {}", ui_code);
                }
                ChannelMessage::BR(bm) => {
                    let tmp_sender = t_s.clone();
                    match tmp_sender.send(bm) {
                        Ok(_) => {}
                        Err(e) => {
                            println!("send error: {}", e)
                        }
                    }
                }
            }
        }
    }
}
