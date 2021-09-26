use fltk::{enums::Color, frame::Frame, image, prelude::*, window::DoubleWindow, app};
use fltk_theme::widget_themes;

use std::sync::mpsc::Sender;

use crate::{
    game::{Card, Game, Suit},
    ButtonAnimation, 
};

pub fn button_constructor(a_label: String) -> Frame {
    let x = Frame::default().with_label(&a_label);
    // x.to_owned().set_frame(widget_themes::OS_DEFAULT_BUTTON_UP_BOX);
    return x;
}

pub fn set_button_color(but: &Frame, suit: Suit) {
    but.to_owned()
        .set_frame(widget_themes::OS_DEFAULT_BUTTON_UP_BOX);
    but.to_owned().set_label_size(24);
    match suit {
        Suit::Diamond | Suit::Heart => but.to_owned().set_label_color(Color::Red),
        Suit::Club | Suit::Spade => {
            but.to_owned().set_label_color(Color::Black);
        }
    }
}

pub fn card_into_filename(card: Card) -> String {
    let first_letter = match card.rank {
        11 => "J".to_string(),
        12 => "Q".to_string(),
        13 => "K".to_string(),
        1 => "A".to_string(),
        10 => "T".to_string(),
        x => x.to_string(),
    };
    let second_letter = match card.suit {
        crate::game::Suit::Spade => "S".to_string(),
        crate::game::Suit::Heart => "H".to_string(),
        crate::game::Suit::Diamond => "D".to_string(),
        crate::game::Suit::Club => "C".to_string(),
    };
    return format!("{}{}", first_letter, second_letter);
}

pub fn draw_card(a_button: &mut Frame, card: Card, hidden: bool) {
    let filename = card_into_filename(card);
    let mut path = format!("src/img/{}.svg.png", filename);
    if hidden {
        path = "src/img/2B.svg.png".to_string();
    }
    match image::PngImage::load(path) {
        Ok(mut kk) => a_button.to_owned().draw({
            move |f| {
                kk.scale(f.width(), f.height(), true, true);
                kk.draw(f.x(), f.y(), f.width(), f.height());
            }
        }),
        Err(e) => {
            println!(
                "error loading png for filename: {} with error: {}",
                filename, e
            )
        }
    };
}

pub fn draw_game(
    the_game: &Game,
    win: &mut DoubleWindow,
    animations: Vec<ButtonAnimation>,
    sender: Sender<ButtonAnimation>,
) {
    draw_animations(win, animations, sender);
    // redraw_game_state();
}

pub fn draw_animations(
    win: &mut DoubleWindow,
    animations: Vec<ButtonAnimation>,
    sender: Sender<ButtonAnimation>,
) {
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

pub fn sleep_and_awake(anim_speed: f64) {
    app::sleep(anim_speed);
    app::awake();
}