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

use crate::{game::{Card, Suit, WinStatus, rank_to_str}, widget::{activate_all_bottom_cards, deactivate_all_bottom_cards, draw_card, draw_game}};

#[cfg(test)]
mod test;

#[derive(Clone, Debug)]
pub enum Row {
    Top,
    Bottom,
}

#[derive(Clone, Debug)]
pub struct MoveCard {
    startx: i32,
    starty: i32,
    endx: i32,
    endy: i32,
    card: Card,
    row: Row,
    card_index: usize,
}
#[derive(Clone, Debug)]
pub struct CollectCards {
    player: game::Player,
}

#[derive(Clone, Debug)]
pub struct DistributeCards {
    bottom_hand: [Card; 4],
    top_hand: [Card; 4]
}

#[derive(Clone, Debug)]
pub enum ButtonAnimation {
    MC(MoveCard),
    CC(CollectCards),
    DC(DistributeCards),
}

#[derive(Copy, Clone, Debug)]
pub struct EventMessage {
    pub the_player: game::Player,
    pub card_index: usize,
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

    let mut cards_on_decs = Vec::new();
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
        cards_on_decs.push(_deck);
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

    let top_cards_immut = vec![ai_1.clone(), ai_2.clone(), ai_3.clone(), ai_4.clone()];
    let mut top_cards = vec![ai_1, ai_2, ai_3, ai_4];
    let mut bottom_cards_values: Vec<Card> = vec![];
    let bottom_cards_immut = vec![pl_1.clone(), pl_2.clone(), pl_3.clone(), pl_4.clone()];
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
        bottom_cards_values.push(my_game.bottom_hand[my_index]);
    }

    for (j, a_vec) in [&top_cards, &bottom_cards].iter().enumerate() {
        for (i, a_but) in a_vec.iter().enumerate() {
            let endx = boardx;
            let endy = boardy;
            let t_hand = my_game.top_hand.clone();
            let b_hand = my_game.bottom_hand.clone();
            let fltk_sender = s.clone();
            if j == 1 {
                a_but.to_owned().set_callback(move |b| {
                    b.to_owned().emit(
                        fltk_sender.to_owned(),
                        ChannelMessage::EM(EventMessage {
                            the_player: game::Player::Player1,
                            card_index: i,
                        })
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
        if let Ok(msg) = t_r.recv() {
            match msg {
                ButtonAnimation::MC(ba) => {
                    println!("animation mesg: {:?}", ba);
                    let t_index = win_clone.children();
                    let mut new_but = button_constructor(format!(
                        "{}{}",
                        rank_to_str(ba.card.rank),
                        ba.card.suit
                    ))
                    .with_pos(ba.startx, ba.starty)
                    .with_size(but_w, but_h);
                    new_but.set_size(tmp_b.width(), tmp_b.height());
                    draw_card(&mut new_but, ba.card);
                    win_clone.insert(&new_but, t_index);
                    let avaiable_top_cards: Vec<&Frame> =
                        top_cards.iter().filter(|t_f| t_f.visible()).collect();
                    match ba.row {
                        Row::Top => match avaiable_top_cards.get(ba.card_index) {
                            Some(a_b) => {let xxxx = *a_b; xxxx.to_owned().hide()},
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
                    cards_on_board.push(new_but);
                    deactivate_all_bottom_cards(&mut bottom_cards);
                    for i in 0..time_len {
                        cards_on_board.last().unwrap().to_owned().set_pos(*series_x.get(i).unwrap(), *series_y.get(i).unwrap());
                        app::sleep(0.01);
                        cards_on_board.last().unwrap().to_owned().parent().unwrap().redraw();
                    }
                    activate_all_bottom_cards(&mut bottom_cards);
                }
                ButtonAnimation::CC(cc) => {
                    let (_,endx,endy) = match cc.player {
                        game::Player::Player1 => {
                            (Row::Bottom, boardx, WIN_HEIGHT)
                        },
                        game::Player::Player2 => {
                            (Row::Top, boardx, 0 - CARD_H)
                        },
                    };
                    for mut a_card_frame in cards_on_board.to_owned() {
                        let time = 10.0;
                        let time_len = time as usize;
                        let st = series_xy(a_card_frame.x(), endx, a_card_frame.y(), endy, time);
                        let series_x = st.0;
                        let series_y = st.1;
                        deactivate_all_bottom_cards(&mut bottom_cards);
                        for i in 0..time_len {
                            a_card_frame.set_pos(*series_x.get(i).unwrap(), *series_y.get(i).unwrap());
                            app::sleep(0.01);
                            a_card_frame.parent().unwrap().redraw();
                        }
                        activate_all_bottom_cards(&mut bottom_cards);
                    }
                    cards_on_board = Vec::new();
                },
                ButtonAnimation::DC(dc) => {
                    println!("dc message received: {:?}", dc);
                    for (i, a_card) in dc.bottom_hand.iter().enumerate() {
                        let mut a_card_frame = cards_on_decs.pop().unwrap();
                        let endx = bottom_cards[i].x();
                        let endy = bottom_cards[i].y();
                        let time = 10.0;
                        let time_len = time as usize;
                        let st = series_xy(a_card_frame.x(), endx, a_card_frame.y(), endy, time);
                        let series_x = st.0;
                        let series_y = st.1;
                        deactivate_all_bottom_cards(&mut bottom_cards);
                        for i in 0..time_len {
                            a_card_frame.set_pos(*series_x.get(i).unwrap(), *series_y.get(i).unwrap());
                            app::sleep(0.01);
                            a_card_frame.parent().unwrap().redraw();
                        }
                        activate_all_bottom_cards(&mut bottom_cards);
                        a_card_frame.hide();
                        bottom_cards[i].show();
                        draw_card(&mut bottom_cards[i], *a_card);
                        bottom_cards[i].parent().unwrap().redraw();
                    }
                    for (i, a_card) in dc.top_hand.iter().enumerate() {
                        let mut a_card_frame = cards_on_decs.pop().unwrap();
                        let endx = top_cards[i].x();
                        let endy = top_cards[i].y();
                        let time = 10.0;
                        let time_len = time as usize;
                        let st = series_xy(a_card_frame.x(), endx, a_card_frame.y(), endy, time);
                        let series_x = st.0;
                        let series_y = st.1;
                        deactivate_all_bottom_cards(&mut bottom_cards);
                        for i in 0..time_len {
                            a_card_frame.set_pos(*series_x.get(i).unwrap(), *series_y.get(i).unwrap());
                            app::sleep(0.01);
                            a_card_frame.parent().unwrap().redraw();
                        }
                        activate_all_bottom_cards(&mut bottom_cards);
                        a_card_frame.hide();
                        top_cards[i].show();
                        draw_card(&mut top_cards[i], *a_card);
                        top_cards[i].parent().unwrap().redraw();
                    }
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
                    let human_player_card = bottom_cards_values[msg.card_index];
                    let mut animations = Vec::new();
                    let bot_i = my_game.get_index_of_card(human_player_card, game::Player::Player1);
                    let a_card = my_game.bottom_hand.remove(bot_i);
                    println!("you played: {}", a_card);
                    animations.push(ButtonAnimation::MC(MoveCard {
                        startx: bottom_cards_immut[msg.card_index].x(),
                        starty: bottom_cards_immut[msg.card_index].y(),
                        endx: boardx,
                        endy: boardy,
                        card: a_card,
                        row: Row::Bottom,
                        card_index: msg.card_index,
                    }));
                    let stat = my_game.play_card(a_card);
                    my_game.move_cards_if_win(stat, game::Player::Player1);
                    match stat {
                        WinStatus::Pisti | WinStatus::Win => {animations.push(ButtonAnimation::CC(CollectCards {
                            player: game::Player::Player1,
                        }))},
                        _ => {}
                    }
                    let ai_card_index = my_game.pick_card_for_ai();
                    let a_card = my_game.top_hand.remove(ai_card_index);
                    println!("ai played: {}", a_card);
                    let avaiable_cards: Vec<&Frame> =
                        top_cards_immut.iter().filter(|t_f| t_f.visible()).collect();
                    println!("avaiable_cards {:?}", avaiable_cards.len());
                    animations.push(ButtonAnimation::MC(MoveCard {
                        startx: avaiable_cards[ai_card_index].x(),
                        starty: avaiable_cards[ai_card_index].y(),
                        endx: boardx,
                        endy: boardy,
                        card: a_card,
                        row: Row::Top,
                        card_index: ai_card_index,
                    }));
                    let stat = my_game.play_card(a_card);
                    my_game.move_cards_if_win(stat, game::Player::Player2);
                    match stat {
                        WinStatus::Pisti | WinStatus::Win => {animations.push(ButtonAnimation::CC(CollectCards {
                            player: game::Player::Player2,
                        }))},
                        _ => {}
                    }
                    if my_game.bottom_hand.len() < 1 && my_game.top_hand.len() < 1 {
                        if my_game.deck.len() > 7 {
                            let (bot_hand, top_hand) = my_game.give_cards_to_players();
                            let bot_a = [bot_hand[0], bot_hand[1], bot_hand[2], bot_hand[3]];
                            let top_a = [top_hand[0], top_hand[1], top_hand[2], top_hand[3]];
                            bottom_cards_values = bot_hand;
                            animations.push(ButtonAnimation::DC(DistributeCards {
                                bottom_hand: bot_a,
                                top_hand: top_a,
                            }))
                        } else {
                            // last player get all remaining cards on board
                            my_game
                                .move_cards_if_win(game::WinStatus::Win, my_game.get_last_player());
                            animations.push(ButtonAnimation::CC(CollectCards {
                                player: game::Player::Player2,
                            }))
                        }
                    }

                    draw_game(&my_game, &mut win, animations, t_s.clone());
                }
            }
        }
    }
}
