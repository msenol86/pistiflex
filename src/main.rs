mod animation;
mod calc;
mod game;
mod network;
mod widget;

use std::{cmp::{max, min}, sync::Arc, thread, time::Duration};

use calc::series_xy;
use fltk::{app, button::{self, Button}, enums::{self, CallbackTrigger}, frame::{self, Frame}, group::{self, Pack}, prelude::*, window::Window};
use fltk_flex::{Flex, FlexType};
use fltk_theme::{widget_themes, ThemeType, WidgetTheme};
use widget::{button_constructor, card_into_filename, set_button_color};

use game::Game;

use std::sync::mpsc;

use crate::{
    game::{rank_to_str, Card, Suit},
    widget::{draw_card},
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
    // let array_of_s = Arc::new([&s.clone(), &s.clone(), &s.clone(), &s.clone()]);
    let (t_s, t_r) = mpsc::channel::<ButtonAnimation>();
    let (network_s, network_r) = mpsc::channel::<network::NetworkMessage>();
    let theme = WidgetTheme::new(ThemeType::Metro);
    theme.apply();
    const WIN_WIDTH: i32 = 600;
    const WIN_HEIGHT: i32 = 600;

    let mut win = Window::default()
        .with_size(600, 600)
        .with_label("My Window");
    let mut flex = Flex::default()
        .with_size(WIN_WIDTH - 10, WIN_HEIGHT - 10)
        .center_of_parent()
        .column();

    let mut upper_flex = Flex::default().row();
    let mut upper2_flex = Flex::default().with_size(200, 100).center_of_parent().row();
    upper2_flex.set_margin(10);
    let ai_1 = button_constructor("1".to_string());
    let ai_2 = button_constructor("2".to_string());
    let ai_3 = button_constructor("3".to_string());
    let ai_4 = button_constructor("4".to_string());
    upper2_flex.end();
    upper_flex.end();

    let mut pack = Pack::default();
    flex.set_size(&mut pack, 200);
    pack.set_spacing(10);
    pack.end();

    let mut bottom_flex = Flex::default().row();
    let mut bottom2_flex = Flex::default().with_size(200, 100).center_of_parent().row();
    bottom2_flex.set_margin(10);
    let pl_1 = button_constructor("1".to_string());
    let pl_2 = button_constructor("2".to_string());
    let pl_3 = button_constructor("3".to_string());
    let pl_4 = button_constructor("4".to_string());


    bottom2_flex.end();
    bottom_flex.end();

    // flex.debug(true);
    flex.end();
    let tmp_b = pl_1.clone();
    for i in 0..my_game.deck.len() {
        let mut _deck = frame::Frame::default()
            .with_size(225 / 2, 315 / 2)
            .center_of_parent();
        draw_card(&mut _deck, my_game.deck[i]);
        _deck.to_owned().set_pos(
            _deck.x() - tmp_b.width() - (tmp_b.width() / 2) + (i as i32 * 1),
            _deck.y(),
        );
        _deck.to_owned().deactivate();
    }
    let board = button::Button::default()
        .with_label("B")
        .with_size(tmp_b.width(), tmp_b.height())
        .center_of_parent();

    let start = button::Button::default()
        .with_label("Start")
        .with_size(100, 50)
        .center_of_parent();
    start.to_owned().set_pos(start.x() + 150, start.y());

    win.end();
    win.show();

    let but_w = tmp_b.w();
    let but_h = tmp_b.h();

    let top_cards = vec![ai_1, ai_2, ai_3, ai_4];
    let bottom_cards = vec![pl_1, pl_2, pl_3, pl_4];

    for my_index in 0..4 {
        let ai_but = top_cards.get(my_index).unwrap();
        ai_but.to_owned().deactivate();
        let a_string = format!("{}", my_game.player2_hand.get(my_index).unwrap());
        ai_but.to_owned().set_label(&a_string);
        set_button_color(&ai_but, my_game.player2_hand.get(my_index).unwrap().suit);

        let pl_but = bottom_cards.get(my_index).unwrap();
        let b_string = format!("{}", my_game.player1_hand.get(my_index).unwrap());
        pl_but.to_owned().set_label(&b_string);
        set_button_color(&pl_but, my_game.player1_hand.get(my_index).unwrap().suit);
    }

    let board_card = my_game.board[3];
    board
        .to_owned()
        .set_label(format!("{}{}", rank_to_str(board_card.rank), board_card.suit).as_str());
    // set_button_color(&board, board_card.suit);


    let my_box = Arc::new(my_game);
    // let my_s_box = Arc::new(array_of_s);
    for (j, a_vec) in [&top_cards, &bottom_cards].iter().enumerate() {
        for (i, a_but) in a_vec.iter().enumerate() {
            // a_but.to_owned().set_callback(|x| {
            //     println!("callback");
            // });
            let endx = board.x();
            let endy = board.y();
            let t_game = my_box.clone();
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
            a_but.to_owned().handle(|b, ev| {
                match ev {
                    enums::Event::Push => {b.do_callback() ;true}
                    _ => {false}
                }
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
            set_button_color(&new_but, ba.card.suit);
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
            println!("sx: {:?}", series_x);
            println!("sy: {:?}", series_y);
            for i in 0..time_len {
                new_but.set_pos(*series_x.get(i).unwrap(), *series_y.get(i).unwrap());
                app::sleep(0.01);
                new_but.parent().unwrap().redraw();
            }
        }
    });

    let _networking = thread::spawn(move || {
        if let Ok(nm) = network_r.recv() {
            match nm {
                network::NetworkMessage::StartBroadcast(port) => {
                    network::broadcast_port_and_username("msenol".to_string(), port.to_string())
                }
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
                    println!("app channel receive msg: {:?}", bm);
                    let tmp_sender = t_s.clone();
                    println!("thread still alive: {:?}", _animator.thread());
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
