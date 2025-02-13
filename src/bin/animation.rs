use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};

use fltk::{app, button::Button, enums, frame::Frame, image::Pixmap, prelude::*, window::Window};

static A: AtomicBool = AtomicBool::new(true);
static B: AtomicI32 = AtomicI32::new(0);
static ANIMATON_RESUME: AtomicBool = AtomicBool::new(true);

const PXM: &[&str] = &[
    "50 34 4 1",
    "  c #000000",
    "o c #ff9900",
    "@ c #ffffff",
    "# c None",
    "##################################################",
    "###      ##############################       ####",
    "### ooooo  ###########################  ooooo ####",
    "### oo  oo  #########################  oo  oo ####",
    "### oo   oo  #######################  oo   oo ####",
    "### oo    oo  #####################  oo    oo ####",
    "### oo     oo  ###################  oo     oo ####",
    "### oo      oo                     oo      oo ####",
    "### oo       oo  ooooooooooooooo  oo       oo ####",
    "### oo        ooooooooooooooooooooo        oo ####",
    "### oo     ooooooooooooooooooooooooooo    ooo ####",
    "#### oo   ooooooo ooooooooooooo ooooooo   oo #####",
    "####  oo oooooooo ooooooooooooo oooooooo oo  #####",
    "##### oo oooooooo ooooooooooooo oooooooo oo ######",
    "#####  o ooooooooooooooooooooooooooooooo o  ######",
    "###### ooooooooooooooooooooooooooooooooooo #######",
    "##### ooooooooo     ooooooooo     ooooooooo ######",
    "##### oooooooo  @@@  ooooooo  @@@  oooooooo ######",
    "##### oooooooo @@@@@ ooooooo @@@@@ oooooooo ######",
    "##### oooooooo @@@@@ ooooooo @@@@@ oooooooo ######",
    "##### oooooooo  @@@  ooooooo  @@@  oooooooo ######",
    "##### ooooooooo     ooooooooo     ooooooooo ######",
    "###### oooooooooooooo       oooooooooooooo #######",
    "###### oooooooo@@@@@@@     @@@@@@@oooooooo #######",
    "###### ooooooo@@@@@@@@@   @@@@@@@@@ooooooo #######",
    "####### ooooo@@@@@@@@@@@ @@@@@@@@@@@ooooo ########",
    "######### oo@@@@@@@@@@@@ @@@@@@@@@@@@oo ##########",
    "########## o@@@@@@ @@@@@ @@@@@ @@@@@@o ###########",
    "########### @@@@@@@     @     @@@@@@@ ############",
    "############  @@@@@@@@@@@@@@@@@@@@@  #############",
    "##############  @@@@@@@@@@@@@@@@@  ###############",
    "################    @@@@@@@@@    #################",
    "####################         #####################",
    "##################################################",
];

fn move_image(mut frm: Frame, xxx: Vec<String>, handle: app::TimeoutHandle) {
    if ANIMATON_RESUME.load(Ordering::SeqCst) {
        let (x, y) = (frm.x(), frm.y());
        let newx = if A.load(Ordering::SeqCst) {
            if x < 440 {
                x + 5
            } else {
                A.store(false, Ordering::SeqCst);
                x
            }
        } else {
            if x < 50 {
                A.store(true, Ordering::SeqCst);
                x
            } else {
                x - 5
            }
        };
        frm.set_pos(newx, y);
        B.store(x, Ordering::Relaxed);
        app::redraw();
    }
    app::repeat_timeout3(0.016, handle);
    // if frm.x() > 440 {
    //     app::remove_timeout3(handle)
    // } else {
    //     app::repeat_timeout3(0.016, handle);
    // }
}

fn main() {
    let app = app::App::default();
    let xxx = vec!["Test".to_string(), "Test1".to_string()];
    let mut wind = Window::default()
        .with_label("timeout")
        .with_size(720, 486)
        .center_screen();
    let mut button = Button::new(10, 10, 100, 50, "Click Me!");
    let mut button2 = Button::new(130, 10, 100, 50, "");
    let mut button_stop = Button::new(250, 10, 100, 50, "Stop!");
    button.set_callback(move |_b| {
        button2.set_label(format!("{}", B.load(Ordering::Relaxed)).as_str());
    });
    button_stop.set_callback(move |b| {
        if ANIMATON_RESUME.load(Ordering::SeqCst) {
            ANIMATON_RESUME.store(false, Ordering::SeqCst);
            b.set_label("Resume!");
        } else {
            ANIMATON_RESUME.store(true, Ordering::SeqCst);
            b.set_label("Stop!");
        }
        
    });
    let mut frame = Frame::new(-200, 150, 200, 200, "");
    let mut pxm = Pixmap::new(PXM).unwrap();
    pxm.scale(200, 200, true, true);
    frame.set_image_scaled(Some(pxm));
    wind.set_color(enums::Color::White);
    wind.end();
    wind.show_with_env_args();

    app::add_timeout3(0.016, move |handle| {
        let frame = frame.clone();
        let yyy = xxx.clone();
        move_image(frame, yyy, handle);
    });
    app.run().unwrap();
}
