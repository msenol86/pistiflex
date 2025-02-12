// #![windows_subsystem = "windows"]
mod calc;
mod game;
mod ui;
mod widget;

use std::sync::atomic::{AtomicU8, Ordering};
use std::{sync::Mutex, thread};

use fltk::{app, button::Button, frame::Frame, prelude::*, window::Window};

use game::Game;
use spin_sleep::SpinSleeper;
use std::sync::mpsc;

use crate::{
    game::{Card, Player, WinStatus},
    ui::*,
    widget::draw_game,
};

#[cfg(test)]
mod test;
static ANIM_SPEED: AtomicU8 = AtomicU8::new(6);

fn main() {
    let sleeper = SpinSleeper::new(1_000_000);
    println!("native sleep accuracy: {}", sleeper.native_accuracy_ns());
    // native sleep accuracy on linux: 125000
    // native sleep accuracy on windo: 1000000
    let mut my_game = Game::new();
    my_game.start_game_and_give_cards_to_players();
    let a = app::App::default();
    let (s, r) = app::channel::<FltkMessage>();
    let (t_s, t_r) = mpsc::channel::<ThreadMessage>();

    let mut win = Window::default()
        .with_size(WIN_WIDTH, WIN_HEIGHT)
        .with_label("Pisti");
    let mut frame = Frame::new(0, 0, 400, 300, "");
    let mut but_inc = Button::new(10, 10, 80, 40, "+");
    let mut but_dec = Button::new(10, 60, 80, 40, "-");
    let mut speed_text = Button::new(10, 120, 80, 40, "");
    speed_text.deactivate();
    speed_text.set_label(format!("{}", ANIM_SPEED.load(Ordering::Relaxed)).as_str());

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

    draw_and_set_callbacks_on_ui(
        &mut top_cards,
        &mut bottom_cards,
        &my_game,
        &mut bottom_cards_values,
        &mut but_inc,
        &mut but_dec,
        &mut speed_text,
        &s,
    );

    let mut win_clone = win.clone();
    let _animator = thread::spawn(move || loop {
        if let Ok(msg) = t_r.recv() {
            match msg {
                ThreadMessage::MC(ba) => move_card_animation(
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
                    sleeper,
                ),
                ThreadMessage::CC(cc) => collect_cards_on_ui(
                    cc,
                    boardx,
                    boardy,
                    &mut cards_on_board,
                    &mut bottom_cards,
                    sleeper,
                ),
                ThreadMessage::DC(dc) => distribute_cards_on_ui(
                    dc,
                    &mut bottom_cards,
                    &mut top_cards,
                    &mut cards_on_decs,
                    &mut win_clone,
                    sleeper,
                ),
                ThreadMessage::GameOver(s) => game_over_on_ui(&mut win_clone, s),
            }
        }
    });

    while a.wait() {
        if let Some(fltk_msg) = r.recv() {
            match fltk_msg {
                FltkMessage::UI(ui_code) => {
                    println!("recevied code: {}", ui_code);
                }
                FltkMessage::EM(msg) => {
                    // println!("eventmessage: {:#?}", msg);
                    let human_player_card = bottom_cards_values[msg.card_index];
                    let mut animations = Vec::new();
                    let bot_i = my_game.get_index_of_card(human_player_card, game::Player::Player1);
                    let a_card = my_game.bottom_hand.remove(bot_i);
                    let (endx, endy) = (boardx, boardy);
                    animations.push(ThreadMessage::MC(MoveCard {
                        startx: bottom_cards_immut[msg.card_index].x(),
                        starty: bottom_cards_immut[msg.card_index].y(),
                        endx,
                        endy,
                        card: a_card,
                        row: Row::Bottom,
                        card_index: msg.card_index,
                    }));
                    let stat = my_game.play_card(a_card);
                    my_game.move_cards_if_win(stat, Player::Player1);
                    match stat {
                        WinStatus::Pisti | WinStatus::Win => {
                            animations.push(ThreadMessage::CC(CollectCards {
                                player: Player::Player1,
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
                    animations.push(ThreadMessage::MC(MoveCard {
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
                            animations.push(ThreadMessage::CC(CollectCards {
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
                            animations.push(ThreadMessage::DC(DistributeCards {
                                bottom_hand: bot_a,
                                top_hand: top_a,
                            }))
                        } else {
                            // last player get all remaining cards on board
                            my_game
                                .move_cards_if_win(game::WinStatus::Win, my_game.get_last_player());
                            animations.push(ThreadMessage::CC(CollectCards {
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

                            animations.push(ThreadMessage::GameOver(msg));
                        }
                    }

                    draw_game(animations, t_s.clone());
                }
            }
        }
    }
}
