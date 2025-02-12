use std::sync::atomic::{AtomicBool, Ordering, AtomicI32};

use fltk::{app, enums, frame::Frame, image::Pixmap, prelude::*, window::Window, button::Button};

static A: AtomicBool = AtomicBool::new(true);
static B: AtomicI32 = AtomicI32::new(0);


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

fn move_image(mut frm: Frame, handle: app::TimeoutHandle) {
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
            x -5 
        }
    };
    // let newx = if (x < 50 && !A.load(Ordering::SeqCst)) || (x < 440 && A.load(Ordering::SeqCst)) {
    //     x + 5
    // } else {
    //     A.store(false, Ordering::SeqCst);
    //     x - 5
    // };
    frm.set_pos(newx, y);
    B.store(x, Ordering::Relaxed);
    app::redraw();
    app::repeat_timeout3(0.016, handle);
    // if frm.x() > 440 {
    //     app::remove_timeout3(handle)
    // } else {
    //     app::repeat_timeout3(0.016, handle);
    // }
    
}

fn main() {
    // let is_anim_direct_posit = Arc::new(Mutex::new(true));
    let app = app::App::default();
    let mut wind = Window::default()
        .with_label("timeout")
        .with_size(720, 486)
        .center_screen();
    let mut button = Button::new(10, 10, 100, 50, "Click Me!");
    let mut button2 = Button::new(130, 10, 100, 50, "Test");
    button.set_callback(move |_b| {
        button2.set_label(format!("{}", B.load(Ordering::Relaxed)).as_str());
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
        // let a_clone = is_anim_direct_posit.clone();
        move_image(frame, handle);
    });
    app.run().unwrap();
}