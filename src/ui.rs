use std::sync::{atomic::Ordering, Mutex};

use fltk::{
    app::{self},
    button::Button,
    enums::{self},
    frame::{self, Frame},
    prelude::*,
    window::DoubleWindow,
};

use crate::{
    calc::series_xy,
    game::{Card, Game, Player},
    widget::{
        activate_all_bottom_cards, button_constructor, deactivate_all_bottom_cards, draw_card,
        insert_new_item_into_window, sleep_and_awake,
    },
    ANIM_SPEED,
};

#[derive(Clone, Debug)]
pub enum Row {
    Top,
    Bottom,
}

#[derive(Clone, Debug)]
pub struct MoveCard {
    pub startx: i32,
    pub starty: i32,
    pub endx: i32,
    pub endy: i32,
    pub card: Card,
    pub row: Row,
    pub card_index: usize,
}
#[derive(Clone, Debug)]
pub struct CollectCards {
    pub player: Player,
}

#[derive(Clone, Debug)]
pub struct DistributeCards {
    pub bottom_hand: [Card; 4],
    pub top_hand: [Card; 4],
}

#[derive(Clone, Debug)]
pub enum ThreadMessage {
    MC(MoveCard),
    CC(CollectCards),
    DC(DistributeCards),
    GameOver(String),
}

#[derive(Copy, Clone, Debug)]
pub struct EventMessage {
    pub the_player: Player,
    pub card_index: usize,
}

#[derive(Clone, Debug)]
pub enum FltkMessage {
    EM(EventMessage),
    #[allow(dead_code)]
    UI(u32),
}

pub const WIN_WIDTH: i32 = 800;
pub const WIN_HEIGHT: i32 = 800;
pub const CARD_H: i32 = 204;
pub const CARD_W: i32 = 144;
pub const CARD_MARGIN: i32 = 110;

pub const MC_ANIM_TIME: f64 = 50.0; // move cards animation time
pub const CC_ANIM_TIME: f64 = 25.0; // collect cards animation time
pub const DC_ANIM_TIME: f64 = 25.0; // distribute cards animation time

pub fn game_over_on_ui(win_clone: &mut DoubleWindow, s: String) {
    let t_index = win_clone.children();
    let b = frame::Frame::default()
        .with_size(400, 50)
        .with_label(s.as_str());
    win_clone.insert(&b, t_index);
    b.center_of(win_clone);
    app::awake();
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

pub fn set_pos_and_size_and_draw_card_on_ui(
    a_card_frame: &mut Frame,
    my_index: usize,
    y: i32,
    a_card: Card,
    hidden: bool,
) {
    a_card_frame.set_size(CARD_W, CARD_H);
    a_card_frame.set_pos((my_index as i32 + 1) * CARD_MARGIN + (CARD_MARGIN / 2), y);
    draw_card(a_card_frame, a_card, hidden);
}

pub fn draw_and_set_callbacks_on_ui(
    top_cards: &mut Vec<Frame>,
    bottom_cards: &mut Vec<Frame>,
    my_game: &Game,
    bottom_cards_values: &mut Vec<Card>,
    but_inc: &mut Button,
    but_dec: &mut Button,
    speed_text: &mut Button,
    app_sender: &app::Sender<FltkMessage>,
) {
    let mut speed_text_inc_clone = speed_text.clone();
    let mut speed_text_dec_clone = speed_text.clone();

    but_inc.to_owned().set_callback(move |_b| {
        println!("Increase button pushed");
        if ANIM_SPEED.load(Ordering::Relaxed) > 1 {
            ANIM_SPEED.store(ANIM_SPEED.load(Ordering::Relaxed) - 1, Ordering::Relaxed);
            println!(
                "Animation Speed Increased to {}",
                ANIM_SPEED.load(Ordering::Relaxed)
            )
        }
        speed_text_inc_clone.set_label(format!("{}", ANIM_SPEED.load(Ordering::Relaxed)).as_str());
    });
    but_dec.to_owned().set_callback(move |_b| {
        println!("Decrease button pushed");
        if ANIM_SPEED.load(Ordering::Relaxed) < 9 {
            ANIM_SPEED.store(ANIM_SPEED.load(Ordering::Relaxed) + 1, Ordering::Relaxed);
            println!(
                "Animation Speed Decreased to {}",
                ANIM_SPEED.load(Ordering::Relaxed)
            )
        }
        speed_text_dec_clone.set_label(format!("{}", ANIM_SPEED.load(Ordering::Relaxed)).as_str());
    });
    for (j, a_vec) in [&top_cards, &bottom_cards].iter().enumerate() {
        for (i, a_but) in a_vec.iter().enumerate() {
            let fltk_sender = app_sender.clone();
            if j == 1 {
                set_pos_and_size_and_draw_card_on_ui(
                    &mut a_but.to_owned(),
                    i,
                    WIN_HEIGHT - 20 - CARD_H,
                    my_game.bottom_hand[i],
                    false,
                );
                bottom_cards_values.push(my_game.bottom_hand[i]);
                a_but.to_owned().set_callback(move |b| {
                    b.to_owned().emit(
                        fltk_sender.to_owned(),
                        FltkMessage::EM(EventMessage {
                            the_player: Player::Player1,
                            card_index: i,
                        }),
                    );
                });
            } else {
                set_pos_and_size_and_draw_card_on_ui(
                    &mut a_but.to_owned(),
                    i,
                    20,
                    my_game.top_hand[i],
                    true,
                );
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
}

pub fn create_4_cards_on_center() -> Vec<Frame> {
    (0..4)
        .map(|v| button_constructor(v.to_string()).center_of_parent())
        .collect()
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
    // let t_index = win_clone.children();
    let mut new_but = button_constructor(format!("{}", ba.card))
        .with_pos(ba.startx, ba.starty)
        .with_size(but_w, but_h);
    new_but.set_size(reference_card_frame.width(), reference_card_frame.height());
    draw_card(&mut new_but, ba.card, false);
    // win_clone.insert(&new_but, t_index);
    insert_new_item_into_window(win_clone, &new_but);
    let avaiable_top_cards: Vec<Frame> = get_avaiable_cards_on_ui(top_cards);
    let card_to_hide =
        get_player_cards_on_ui(ba.row, &avaiable_top_cards, bottom_cards, ba.card_index);
    card_to_hide.to_owned().hide();
    let (x, y) = get_pos_for_new_card_on_board(cards_on_board, boardx, boardy);
    let time_len = MC_ANIM_TIME as usize;
    let st = series_xy(ba.startx, x, ba.starty, y, MC_ANIM_TIME);
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
        sleep_and_awake(ANIM_SPEED.load(Ordering::Relaxed));
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
        Player::Player1 => (Row::Bottom, boardx, WIN_HEIGHT),
        Player::Player2 => (Row::Top, boardx, 0 - CARD_H),
    };
    deactivate_all_bottom_cards(bottom_cards);
    sleep_and_awake(5);
    for (i, _) in cards_on_board.iter().enumerate().rev() {
        let mut a_card_frame = cards_on_board[i].to_owned();

        let time_len = CC_ANIM_TIME as usize;
        let st = series_xy(a_card_frame.x(), endx, a_card_frame.y(), endy, CC_ANIM_TIME);
        let series_x = st.0;
        let series_y = st.1;

        for i in 0..time_len {
            a_card_frame.set_pos(*series_x.get(i).unwrap(), *series_y.get(i).unwrap());
            sleep_and_awake(ANIM_SPEED.load(Ordering::Relaxed));
            a_card_frame.parent().unwrap().redraw();
        }
    }
    activate_all_bottom_cards(bottom_cards);
    *cards_on_board = Vec::new();
}

pub fn distribute_cards_on_ui(
    dc: DistributeCards,
    p_bottom_cards: &mut Vec<Frame>,
    p_top_cards: &mut Vec<Frame>,
    cards_on_decs: &mut Vec<Frame>,
    win_clone: &mut DoubleWindow,
) {
    for (player_cards, player_hand, hidden) in [
        (p_top_cards, dc.top_hand, true),
        (p_bottom_cards, dc.bottom_hand, false),
    ] {
        deactivate_all_bottom_cards(player_cards);
        for i in 0..4 {
            let a_card = player_hand[i];
            let mut a_card_frame_old = cards_on_decs.pop().unwrap();
            let mut a_card_frame = button_constructor(format!("{}", a_card))
                .with_pos(a_card_frame_old.x(), a_card_frame_old.y())
                .with_size(a_card_frame_old.w(), a_card_frame_old.h());
            draw_card(&mut a_card_frame, a_card, true);
            insert_new_item_into_window(win_clone, &a_card_frame);
            a_card_frame_old.hide();

            let time_len = DC_ANIM_TIME as usize;
            let (series_x, series_y) = series_xy(
                a_card_frame.x(),
                player_cards[i].x(),
                a_card_frame.y(),
                player_cards[i].y(),
                DC_ANIM_TIME,
            );

            for i in 0..time_len {
                a_card_frame.set_pos(*series_x.get(i).unwrap(), *series_y.get(i).unwrap());
                sleep_and_awake(ANIM_SPEED.load(Ordering::Relaxed));
                a_card_frame.parent().unwrap().redraw();
            }
            a_card_frame.hide();
            draw_card(&mut player_cards[i], a_card, hidden);
            player_cards[i].show();
            player_cards[i].parent().unwrap().redraw();
        }
        activate_all_bottom_cards(player_cards);
    }

    app::awake();
}
