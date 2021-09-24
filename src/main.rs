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
use widget::button_constructor;

use game::Game;

use std::sync::mpsc;

use crate::{
    game::{rank_to_str, Card, Suit},
    widget::{draw_card, draw_game},
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

#[derive(Copy, Clone, Debug)]
pub struct EventMessage {
    pub the_player: game::Player,
    pub card: Card,
}

#[derive(Clone, Debug)]
pub enum ChannelMessage {
    EM(EventMessage),
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

    let hidden_board = frame::Frame::default()
        .with_label("B")
        .with_size(CARD_W, CARD_H)
        .center_of_parent();
    let boardx = hidden_board.x();
    let boardy = hidden_board.y();
    hidden_board.to_owned().hide();

    let mut cards_on_board = Vec::new();
    for (i, a_card) in my_game.board.iter().enumerate() {
        let mut board = frame::Frame::default()
            .with_size(CARD_W, CARD_H)
            .center_of_parent();

        board.set_pos(board.x() + i as i32, board.y());
        draw_card(&mut board, *a_card);
        cards_on_board.push(board);
    }

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
        draw_card(&mut ai_but, my_game.top_hand[my_index]);

        let mut pl_but = bottom_cards.get_mut(my_index).unwrap();
        pl_but.set_size(CARD_W, CARD_H);
        pl_but.set_pos(
            (my_index as i32 + 1) * CARD_MARGIN + (CARD_MARGIN / 2),
            WIN_HEIGHT - 20 - CARD_H,
        );
        draw_card(&mut pl_but, my_game.bottom_hand[my_index]);
    }

    for (j, a_vec) in [&top_cards, &bottom_cards].iter().enumerate() {
        for (i, a_but) in a_vec.iter().enumerate() {
            let endx = boardx;
            let endy = boardy;
            let t_hand = my_game.top_hand.clone();
            let b_hand = my_game.bottom_hand.clone();
            let t_s = s.clone();
            if j == 1 {
                a_but.to_owned().set_callback(move |b| {
                    b.to_owned().emit(
                        t_s.to_owned(),
                        ChannelMessage::BR(ButtonAnimation {
                            startx: b.x(),
                            starty: b.y(),
                            endx: endx,
                            endy: endy,
                            card: if j == 0 { t_hand[i] } else { b_hand[i] },
                            row: if j == 0 { Row::Top } else { Row::Bottom },
                            card_index: i,
                        }),
                    );
                });
            }
            a_but.to_owned().handle(|b, ev| match ev {
                enums::Event::Push => {
                    b.do_callback();
                    true
                }
                _ => false,
            })
        }
    }

    let mut win_clone = win.clone();
    let _animator = thread::spawn(move || loop {
        if let Ok(ba) = t_r.recv() {
            println!("animation mesg: {:?}", ba);
            let t_index = win_clone.children();
            let mut new_but =
                button_constructor(format!("{}{}", rank_to_str(ba.card.rank), ba.card.suit))
                    .with_pos(ba.startx, ba.starty)
                    .with_size(but_w, but_h);
            new_but.set_size(tmp_b.width(), tmp_b.height());
            draw_card(&mut new_but, ba.card);
            win_clone.insert(&new_but, t_index);
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

            let player = match ba.row {
                Row::Bottom => game::Player::Player1,
                Row::Top => game::Player::Player2,
            };

            s.send(ChannelMessage::EM(EventMessage {
                the_player: player,
                card: ba.card,
            }));
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
                ChannelMessage::EM(msg) => {
                    println!("eventmessage: {:#?}", msg);
                    let bot_i = my_game.get_index_of_card(msg.card, game::Player::Player1);
                    let a_card = my_game.bottom_hand.remove(bot_i);
                    println!("you played: {}", a_card);
                    let stat = my_game.play_card(a_card);
                    let mut moved = my_game.move_cards_if_win(stat, game::Player::Player1);
                    if moved {
                        draw_game(&my_game, &mut win);
                    } else {
                        let ai_card_index = my_game.pick_card_for_ai();
                        let a_card = my_game.top_hand.remove(ai_card_index);
                        println!("ai played: {}", a_card);
                        let stat = my_game.play_card(a_card);
                        my_game.move_cards_if_win(stat, game::Player::Player2);
                        if my_game.bottom_hand.len() < 1 && my_game.top_hand.len() < 1 {
                            if my_game.deck.len() > 7 {
                                my_game.give_cards_to_players();
                            } else {
                                // last player get all remaining cards on board
                                my_game.move_cards_if_win(
                                    game::WinStatus::Win,
                                    my_game.get_last_player(),
                                );
                            }
                        }
                    }
                    // update_ui_on_button_press(
                    //     &ai_cards,
                    //     &player_cards,
                    //     &board,
                    //     &my_game,
                    //     &out1,
                    //     &out2,
                    // );
                }
            }
        }
    }
    // while a.wait() {
    //     if let Some(c_msg) = r.recv() {
    //         match c_msg {
    //             ChannelMessage::EM(msg) => {
    //                 println!("{:#?}", msg);
    //                 let a_card = my_game.player1_hand.remove(msg.card_index as usize);
    //                 println!("you played: {}", a_card);
    //                 let stat = my_game.play_card(a_card);
    //                 my_game.move_cards_if_win(stat, Player1);
    //                 let ai_card_index = my_game.pick_card_for_ai();
    //                 let a_card = my_game.player2_hand.remove(ai_card_index);
    //                 println!("ai played: {}", a_card);
    //                 let stat = my_game.play_card(a_card);
    //                 my_game.move_cards_if_win(stat, Player2);
    //                 if my_game.player1_hand.len() < 1 && my_game.player2_hand.len() < 1 {
    //                     if my_game.deck.len() > 7 {
    //                         my_game.give_cards_to_players();
    //                     } else {
    //                         // last player get all remaining cards on board
    //                         my_game.move_cards_if_win(Win, my_game.get_last_player());
    //                     }
    //                 }
    //                 update_ui_on_button_press(
    //                     &ai_cards,
    //                     &player_cards,
    //                     &board,
    //                     &my_game,
    //                     &out1,
    //                     &out2,
    //                 );
    //             }
    //             ChannelMessage::NM(a_msg) => {
    //                 process_network_msg(a_msg, my_local_ip, tmp_ip.to_owned(), &mut wind);
    //             }
    //             ChannelMessage::Dialog(an_ip4) => {
    //                 thread::spawn(move || {
    //                     let r = send_invite_request(my_local_ip.unwrap(), an_ip4, BRD_PORT);
    //                     match r {
    //                         Ok(_) => {}
    //                         Err(e) => {
    //                             println!("error invite: {}", e)
    //                         }
    //                     }
    //                 });
    //             }
    //         }
    //     }
    // }
}
