#![windows_subsystem = "windows"]
mod animation;
mod calc;
mod game;
mod widget;

use std::{sync::Mutex, thread};

use calc::series_xy;
use fltk::{
    app::{self},
    enums::{self},
    frame::{self, Frame},
    prelude::*,
    window::{DoubleWindow, Window},
};
use fltk_theme::{ThemeType, WidgetTheme};
use widget::button_constructor;

use game::Game;
use std::sync::mpsc;

use crate::{
    game::{Card, WinStatus},
    widget::{
        activate_all_bottom_cards, deactivate_all_bottom_cards, draw_card, draw_game,
        sleep_and_awake,
    },
};

#[cfg(test)]
mod test;

const WIN_WIDTH: i32 = 800;
const WIN_HEIGHT: i32 = 800;
const CARD_H: i32 = 204;
const CARD_W: i32 = 144;
const CARD_MARGIN: i32 = 110;
const ANIM_SPEED: f64 = 0.01;

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
    top_hand: [Card; 4],
}

#[derive(Clone, Debug)]
pub enum ButtonAnimation {
    MC(MoveCard),
    CC(CollectCards),
    DC(DistributeCards),
    GameOver(String),
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

pub fn get_avaiable_cards_on_ui(card_frames: &Vec<Frame>) -> Vec<Frame> {
    return card_frames
        .iter()
        .filter(|&t_f| t_f.visible())
        .map(|t_f| t_f.to_owned())
        .collect();
}

pub fn get_player_cards_on_ui<'a>(
    row: Row,
    top_cards: &'a Vec<Frame>,
    bottom_cards: &'a Vec<Frame>,
    card_index: usize,
) -> &'a Frame {
    match row {
        Row::Top => &top_cards[card_index],
        Row::Bottom => &bottom_cards[card_index],
    }
}

pub fn move_card_animation(
    win_clone: &mut DoubleWindow,
    ba: MoveCard,
    top_cards: &Vec<Frame>,
    bottom_cards: &Vec<Frame>,
    cards_on_board: &mut Vec<Frame>,
    but_w: i32,
    but_h: i32,
    reference_card_frame: &Frame,
    cards_on_board_lastx: &Mutex<i32>,
    cards_on_board_lasty: &Mutex<i32>,
    boardx: i32,
    boardy: i32,
) {
    let t_index = win_clone.children();
    let mut new_but = button_constructor(format!("{}", ba.card))
        .with_pos(ba.startx, ba.starty)
        .with_size(but_w, but_h);
    new_but.set_size(reference_card_frame.width(), reference_card_frame.height());
    draw_card(&mut new_but, ba.card, false);
    win_clone.insert(&new_but, t_index);
    let avaiable_top_cards: Vec<Frame> = get_avaiable_cards_on_ui(top_cards);
    let card_to_hide =
        get_player_cards_on_ui(ba.row, &avaiable_top_cards, bottom_cards, ba.card_index);
    card_to_hide.to_owned().hide();
    let (x, y) = get_pos_for_new_card_on_board(cards_on_board, boardx, boardy);
    let time = 50.0;
    let time_len = time as usize;
    let st = series_xy(ba.startx, x, ba.starty, y, time);
    let series_x = st.0;
    let series_y = st.1;
    cards_on_board.push(new_but);
    let mut bottom_cards_clone = bottom_cards.clone();
    deactivate_all_bottom_cards(&mut bottom_cards_clone);
    for i in 0..time_len {
        cards_on_board
            .last()
            .unwrap()
            .to_owned()
            .set_pos(*series_x.get(i).unwrap(), *series_y.get(i).unwrap());
        sleep_and_awake(ANIM_SPEED);
        cards_on_board
            .last()
            .unwrap()
            .to_owned()
            .parent()
            .unwrap()
            .redraw();
    }
    let mut lastx = cards_on_board_lastx.lock().unwrap();
    let mut lasty = cards_on_board_lasty.lock().unwrap();
    *lastx = x;
    *lasty = y;
    cards_on_board
        .last()
        .unwrap()
        .to_owned()
        .set_pos(*lastx, *lasty);
    cards_on_board
        .last()
        .unwrap()
        .to_owned()
        .parent()
        .unwrap()
        .redraw();
    activate_all_bottom_cards(&mut bottom_cards_clone);
}

pub fn collect_cards_on_ui(
    cc: CollectCards,
    boardx: i32,
    _boardy: i32,
    cards_on_board: &mut Vec<Frame>,
    bottom_cards: &mut Vec<Frame>,
) {
    let (_, endx, endy) = match cc.player {
        game::Player::Player1 => (Row::Bottom, boardx, WIN_HEIGHT),
        game::Player::Player2 => (Row::Top, boardx, 0 - CARD_H),
    };

    for (i, _) in cards_on_board.iter().enumerate().rev() {
        let mut a_card_frame = cards_on_board[i].to_owned();
        let time = 10.0;
        let time_len = time as usize;
        let st = series_xy(a_card_frame.x(), endx, a_card_frame.y(), endy, time);
        let series_x = st.0;
        let series_y = st.1;
        deactivate_all_bottom_cards(bottom_cards);
        for i in 0..time_len {
            a_card_frame.set_pos(*series_x.get(i).unwrap(), *series_y.get(i).unwrap());
            sleep_and_awake(ANIM_SPEED);
            a_card_frame.parent().unwrap().redraw();
        }
        activate_all_bottom_cards(bottom_cards);
    }
    *cards_on_board = Vec::new();
}

pub fn distribute_cards_on_ui(
    dc: DistributeCards,
    bottom_cards: &mut Vec<Frame>,
    top_cards: &mut Vec<Frame>,
    cards_on_decs: &mut Vec<Frame>,
) {
    for (i, a_card) in dc.bottom_hand.iter().enumerate() {
        let mut a_card_frame = cards_on_decs.pop().unwrap();
        let endx = bottom_cards[i].x();
        let endy = bottom_cards[i].y();
        let time = 10.0;
        let time_len = time as usize;
        let st = series_xy(a_card_frame.x(), endx, a_card_frame.y(), endy, time);
        let series_x = st.0;
        let series_y = st.1;
        deactivate_all_bottom_cards(bottom_cards);
        for i in 0..time_len {
            a_card_frame.set_pos(*series_x.get(i).unwrap(), *series_y.get(i).unwrap());
            sleep_and_awake(ANIM_SPEED);
            a_card_frame.parent().unwrap().redraw();
        }
        activate_all_bottom_cards(bottom_cards);
        a_card_frame.hide();
        bottom_cards[i].show();
        draw_card(&mut bottom_cards[i], *a_card, false);
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
        deactivate_all_bottom_cards(bottom_cards);
        for i in 0..time_len {
            a_card_frame.set_pos(*series_x.get(i).unwrap(), *series_y.get(i).unwrap());
            sleep_and_awake(ANIM_SPEED);
            a_card_frame.parent().unwrap().redraw();
        }
        activate_all_bottom_cards(bottom_cards);
        a_card_frame.hide();
        top_cards[i].show();
        draw_card(&mut top_cards[i], *a_card, true);
        top_cards[i].parent().unwrap().redraw();
    }
}

pub fn game_over_on_ui(win_clone: &mut DoubleWindow, s: String) {
    let t_index = win_clone.children();
    let b = frame::Frame::default()
        .with_size(400, 50)
        .with_label(s.as_str());
    win_clone.insert(&b, t_index);
    b.center_of_parent();
}

pub fn generate_card_frames_on_deck_on_ui(
    game_deck: &Vec<Card>,
    card_w: i32,
    card_h: i32,
    reference_card_frame: &Frame,
) -> Vec<Frame> {
    let mut tmp_deck = Vec::new();
    for i in 0..game_deck.len() {
        let mut _deck = frame::Frame::default()
            .with_size(card_w, card_h)
            .center_of_parent();

        _deck.set_pos(_deck.x() - 200, _deck.y());
        draw_card(&mut _deck, game_deck[i], true);
        _deck.to_owned().set_pos(
            _deck.x() - reference_card_frame.width() - (reference_card_frame.width() / 2)
                + (i as i32 * 1),
            _deck.y(),
        );
        _deck.to_owned().deactivate();
        tmp_deck.push(_deck);
    }
    return tmp_deck;
}

pub fn generate_card_frames_on_board_ui(
    game_board: &Vec<Card>,
    card_w: i32,
    card_h: i32,
) -> (Vec<Frame>, usize, i32, i32) {
    let mut tmp_board = Vec::new();
    let mut lastx = 0;
    let mut lasty = 0;
    let mut last_len = 0;
    for (i, a_card) in game_board.iter().enumerate() {
        let mut board = frame::Frame::default()
            .with_size(card_w, card_h)
            .center_of_parent();

        board.set_pos(board.x() + (i as i32) * 10, board.y());
        if i < 3 {
            draw_card(&mut board, *a_card, true);
        } else {
            draw_card(&mut board, *a_card, false);
        }
        lastx = board.x();
        lasty = board.y();
        tmp_board.push(board);
        last_len = tmp_board.len();
    }
    return (tmp_board, last_len, lastx, lasty);
}

pub fn generate_hidden_board_card_frame(card_w: i32, card_h: i32) -> (i32, i32, Frame) {
    let hidden_board = frame::Frame::default()
        .with_label("B")
        .with_size(card_w, card_h)
        .center_of_parent();
    let boardx = hidden_board.x();
    let boardy = hidden_board.y();
    hidden_board.to_owned().hide();
    return (boardx, boardy, hidden_board);
}

pub fn get_pos_for_new_card_on_board(
    cards_on_board: &Vec<Frame>,
    boardx: i32,
    boardy: i32,
) -> (i32, i32) {
    match cards_on_board.last() {
        Some(a_f) => (a_f.to_owned().x() + 5, a_f.to_owned().y()),
        None => (boardx, boardy),
    }
}

pub fn set_pos_and_size_and_draw_card(a_card_frame: &mut Frame, my_index: usize, y: i32, a_card: Card, hidden: bool) {
    a_card_frame.set_size(CARD_W, CARD_H);
    a_card_frame.set_pos((my_index as i32 + 1) * CARD_MARGIN + (CARD_MARGIN / 2), y);
    draw_card(a_card_frame, a_card, hidden);
}

pub fn create_4_cards_on_center() -> Vec<Frame> {
    (0..4)
        .map(|v| button_constructor(v.to_string()).center_of_parent())
        .collect()
}

fn main() {
    let mut my_game = Game::new();
    my_game.start_game_and_give_cards_to_players();
    let a = app::App::default();
    let (s, r) = app::channel::<ChannelMessage>();
    let (t_s, t_r) = mpsc::channel::<ButtonAnimation>();
    WidgetTheme::new(ThemeType::Metro).apply();

    let mut win = Window::default()
        .with_size(WIN_WIDTH, WIN_HEIGHT)
        .with_label("Pisti");

    let mut top_cards = create_4_cards_on_center();
    let mut bottom_cards = create_4_cards_on_center();
    let top_cards_immut: Vec<Frame> = top_cards.iter().map(|f| f.clone()).collect();
    let bottom_cards_immut: Vec<Frame> = bottom_cards.iter().map(|f| f.clone()).collect();
    let mut bottom_cards_values: Vec<Card> = vec![];

    let reference_card_frame = bottom_cards_immut[0].clone();

    let mut cards_on_decs =
        generate_card_frames_on_deck_on_ui(&my_game.deck, CARD_W, CARD_H, &reference_card_frame);

    let (boardx, boardy, _hidden_board) = generate_hidden_board_card_frame(CARD_W, CARD_H);

    let (mut cards_on_board, _, lastx, lasty) =
        generate_card_frames_on_board_ui(&my_game.board, CARD_W, CARD_H);
    let cards_on_board_lastx = Mutex::new(lastx);
    let cards_on_board_lasty = Mutex::new(lasty);

    win.end();
    win.show();

    let (but_w, but_h) = (reference_card_frame.w(), reference_card_frame.h());

    for (j, a_vec) in [&top_cards, &bottom_cards].iter().enumerate() {
        for (i, a_but) in a_vec.iter().enumerate() {
            let fltk_sender = s.clone();
            if j == 1 {
                set_pos_and_size_and_draw_card( &mut a_but.to_owned(), i, WIN_HEIGHT - 20 - CARD_H, my_game.bottom_hand[i], false);
                bottom_cards_values.push(my_game.bottom_hand[i]);
                a_but.to_owned().set_callback(move |b| {
                    b.to_owned().emit(
                        fltk_sender.to_owned(),
                        ChannelMessage::EM(EventMessage {
                            the_player: game::Player::Player1,
                            card_index: i,
                        }),
                    );
                });
            } else {
                set_pos_and_size_and_draw_card( &mut a_but.to_owned(), i, 20, my_game.top_hand[i], true);
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
                ButtonAnimation::MC(ba) => move_card_animation(
                    &mut win_clone,
                    ba,
                    &top_cards,
                    &bottom_cards,
                    &mut cards_on_board,
                    but_w,
                    but_h,
                    &reference_card_frame,
                    &cards_on_board_lastx,
                    &cards_on_board_lasty,
                    boardx,
                    boardy,
                ),
                ButtonAnimation::CC(cc) => {
                    collect_cards_on_ui(cc, boardx, boardy, &mut cards_on_board, &mut bottom_cards)
                }
                ButtonAnimation::DC(dc) => {
                    distribute_cards_on_ui(dc, &mut bottom_cards, &mut top_cards, &mut cards_on_decs)
                }
                ButtonAnimation::GameOver(s) => game_over_on_ui(&mut win_clone, s),
            }
        }
    });

    while a.wait() {
        if let Some(c_msg) = r.recv() {
            match c_msg {
                ChannelMessage::UI(_ui_code) => {
                    // println!("recevied code: {}", ui_code);
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
                    // println!("eventmessage: {:#?}", msg);
                    let human_player_card = bottom_cards_values[msg.card_index];
                    let mut animations = Vec::new();
                    let bot_i = my_game.get_index_of_card(human_player_card, game::Player::Player1);
                    let a_card = my_game.bottom_hand.remove(bot_i);
                    let (endx, endy) = (boardx, boardy);
                    animations.push(ButtonAnimation::MC(MoveCard {
                        startx: bottom_cards_immut[msg.card_index].x(),
                        starty: bottom_cards_immut[msg.card_index].y(),
                        endx,
                        endy,
                        card: a_card,
                        row: Row::Bottom,
                        card_index: msg.card_index,
                    }));
                    let stat = my_game.play_card(a_card);
                    my_game.move_cards_if_win(stat, game::Player::Player1);
                    match stat {
                        WinStatus::Pisti | WinStatus::Win => {
                            animations.push(ButtonAnimation::CC(CollectCards {
                                player: game::Player::Player1,
                            }))
                        }
                        _ => {}
                    }
                    let ai_card_index = my_game.pick_card_for_ai();
                    let a_card = my_game.top_hand.remove(ai_card_index);
                    // println!("ai played: {}", a_card);
                    let avaiable_cards: Vec<&Frame> =
                        top_cards_immut.iter().filter(|t_f| t_f.visible()).collect();
                    // println!("avaiable_cards {:?}", avaiable_cards.len());
                    let (endx, endy) = (boardx, boardy);
                    animations.push(ButtonAnimation::MC(MoveCard {
                        startx: avaiable_cards[ai_card_index].x(),
                        starty: avaiable_cards[ai_card_index].y(),
                        endx,
                        endy,
                        card: a_card,
                        row: Row::Top,
                        card_index: ai_card_index,
                    }));
                    let stat = my_game.play_card(a_card);
                    my_game.move_cards_if_win(stat, game::Player::Player2);
                    match stat {
                        WinStatus::Pisti | WinStatus::Win => {
                            animations.push(ButtonAnimation::CC(CollectCards {
                                player: game::Player::Player2,
                            }))
                        }
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
                            }));
                            my_game.calculate_points();
                            let mytxt = if my_game.player1_point > my_game.player2_point {
                                "You won"
                            } else {
                                "You lost"
                            };
                            let msg = format!(
                                "{} - Your points: {}({} Pisti) -- AI points: {}({} Pisti)",
                                mytxt,
                                my_game.player1_point,
                                my_game.player1_pisti_count,
                                my_game.player2_point,
                                my_game.player2_pisti_count
                            );

                            animations.push(ButtonAnimation::GameOver(msg));
                        }
                    }

                    draw_game(&my_game, &mut win, animations, t_s.clone());
                }
            }
        }
    }
}
