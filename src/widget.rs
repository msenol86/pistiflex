use fltk::{button::Button, enums::Color, frame::Frame, image, prelude::*, window::DoubleWindow};
use fltk_theme::widget_themes;

use std::sync::mpsc::Sender;

use crate::{ButtonAnimation, ChannelMessage, Row, game::{Card, Game, Suit}};

pub fn button_constructor(a_label: String) -> Frame {
    let x = Frame::default().with_label(&a_label);
    // x.to_owned().set_frame(widget_themes::OS_DEFAULT_BUTTON_UP_BOX);
    return x;
}

pub fn set_button_color(but: &Frame, suit: Suit) {
    but.to_owned().set_frame(widget_themes::OS_DEFAULT_BUTTON_UP_BOX);
    but.to_owned().set_label_size(24);
    match suit {
        Suit::Diamond | Suit::Heart => {  but.to_owned().set_label_color(Color::Red) },
        Suit::Club | Suit::Spade => { but.to_owned().set_label_color(Color::Black); }
    }
    
}


pub fn card_into_filename(card: Card) -> String {
    let first_letter = match card.rank {
        11 => {"J".to_string()},
        12 => {"Q".to_string()},
        13 => {"K".to_string()},
        1 => {"A".to_string()},
        10 => {"T".to_string()},
        x => {x.to_string()}
    };
    let second_letter = match card.suit {
        crate::game::Suit::Spade => "S".to_string(),
        crate::game::Suit::Heart => "H".to_string(),
        crate::game::Suit::Diamond => "D".to_string(),
        crate::game::Suit::Club => "C".to_string(),
    };
    return format!("{}{}", first_letter, second_letter);
}


pub fn draw_card(a_button: &mut Frame, card: Card) {
    let filename = card_into_filename(card);
        match image::PngImage::load(format!("src/img/{}.svg.png", filename)) {
            Ok(mut kk) => {a_button.to_owned().draw( {
                move |f | {
                    kk.scale(f.width(), f.height(), true, true);
                    kk.draw(f.x(), f.y(), f.width(), f.height());
                }
            })},
            Err(e) => {println!("error loading png for filename: {} with error: {}", filename, e)},
        };
}

pub fn draw_game(the_game: &Game, win: &mut DoubleWindow, animations: Vec<ButtonAnimation>, sender: Sender<ButtonAnimation>) {
    draw_animations(win, animations, sender);
    // redraw_game_state();
}

pub fn  draw_animations(win: &mut DoubleWindow, animations: Vec<ButtonAnimation>, sender: Sender<ButtonAnimation>) {
    for an_animation in animations {
        sender.send(an_animation);
    }
    // remove temp frames
}

pub fn deactivate_all_bottom_cards(bottom_card_frames: &mut Vec<Frame>) {
    for a_card in bottom_card_frames {
        a_card.deactivate();
    }
}


pub fn activate_all_bottom_cards(bottom_card_frames: &mut Vec<Frame>) {
    for a_card in bottom_card_frames {
        a_card.activate();
    }
}

// pub fn update_ui_on_button_press(
//     ai_cards: &Vec<Button>,
//     pl_cards: &Vec<Button>,
//     board: &Button,
//     the_game: &Game,
//     out1: &Output,
//     out2: &Output,
// ) {
//     let cards_len = the_game.player2_hand.len();
//     for i in 0..4 {
//         let ai_but = ai_cards.get(i).unwrap();
//         let pl_but = pl_cards.get(i).unwrap();
//         if i < cards_len {
//             let a_string = format!("{}", the_game.player2_hand.get(i).unwrap());
//             ai_but.to_owned().set_label(&a_string);
//             let b_string = format!("{}", the_game.player1_hand.get(i).unwrap());
//             pl_but.to_owned().set_label(&b_string);
//             pl_but.to_owned().activate();
//             pl_but.to_owned().clear_visible_focus();
//         } else {
//             ai_but.to_owned().set_label("");
//             ai_but.to_owned().deactivate();
//             pl_but.to_owned().set_label("");
//             pl_but.to_owned().deactivate();
//             pl_but.to_owned().clear_visible_focus();
//         }
//     }
//     if the_game.board.len() > 0 {
//         let c_string = format!("{}", the_game.board.last().unwrap());
//         board.to_owned().set_label(&c_string);
//     } else {
//         board.to_owned().set_label("");
//     }
//     let o_string1 = format!(
//         "{} Pist({}) PT({})",
//         the_game.player1_won_cards.len(),
//         the_game.player1_pisti_count,
//         the_game.player1_point,
//     );
//     let o_string2 = format!(
//         "{} Pist({}) PT({})",
//         the_game.player2_won_cards.len(),
//         the_game.player2_pisti_count,
//         the_game.player2_point,
//     );
//     out1.to_owned().set_value(&o_string1);
//     out2.to_owned().set_value(&o_string2);
//     println!("player1_won_cards: {:#?}", the_game.player1_won_cards);
//     println!("player2_won_cards: {:#?}", the_game.player2_won_cards);
// }
