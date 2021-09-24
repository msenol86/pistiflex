use fltk::frame::*;
use fltk::image::*;
use fltk::{enums::*, prelude::*};
use std::cell::RefCell;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

const POWER: &str = r#"<?xml version="1.0" encoding="iso-8859-1"?>
<!-- Generator: Adobe Illustrator 19.1.0, SVG Export Plug-In . SVG Version: 6.00 Build 0)  -->
<svg version="1.1" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" x="0px" y="0px"
	 viewBox="0 0 315.083 315.083" style="enable-background:new 0 0 315.083 315.083;" xml:space="preserve">
<g id="Layer_1">
	<linearGradient id="SVGID_1_" gradientUnits="userSpaceOnUse" x1="157.5417" y1="4.5417" x2="157.5417" y2="310.5417">
		<stop  offset="0" style="stop-color:#939598"/>
		<stop  offset="0.25" style="stop-color:#414042"/>
		<stop  offset="0.5" style="stop-color:#252223"/>
		<stop  offset="1" style="stop-color:#000000"/>
	</linearGradient>
	<circle style="fill:url(#SVGID_1_);" cx="157.542" cy="157.542" r="153"/>
</g>
<g id="Layer_2">
	<linearGradient id="SVGID_2_" gradientUnits="userSpaceOnUse" x1="157.5417" y1="292.5417" x2="157.5417" y2="22.5417">
		<stop  offset="0" style="stop-color:#58595B"/>
		<stop  offset="0.1" style="stop-color:#414042"/>
		<stop  offset="0.2" style="stop-color:#242122"/>
		<stop  offset="1" style="stop-color:#000000"/>
	</linearGradient>
	<circle style="fill:url(#SVGID_2_);stroke:#58595B;stroke-miterlimit:10;" cx="157.542" cy="157.542" r="135"/>
</g>
<g id="Layer_4">
	<radialGradient id="SVGID_3_" cx="157.5417" cy="89.9217" r="62.2727" gradientUnits="userSpaceOnUse">
		<stop  offset="0" style="stop-color:#58595B"/>
		<stop  offset="0.5" style="stop-color:#414042"/>
		<stop  offset="1" style="stop-color:#231F20"/>
	</radialGradient>
	<radialGradient id="SVGID_4_" cx="157.5417" cy="89.9217" r="62.7723" gradientUnits="userSpaceOnUse">
		<stop  offset="0" style="stop-color:#FFFFFF"/>
		<stop  offset="0.6561" style="stop-color:#231F20"/>
		<stop  offset="1" style="stop-color:#000000"/>
	</radialGradient>
	
		<ellipse style="fill:url(#SVGID_3_);stroke:url(#SVGID_4_);stroke-miterlimit:10;" cx="157.542" cy="89.922" rx="59.833" ry="64.62"/>
</g>
<g id="Layer_6">
	<path style="fill:none;stroke:red;stroke-width:10;stroke-linecap:round;stroke-miterlimit:10;" d="M119.358,119.358
		c-9.772,9.772-15.816,23.272-15.816,38.184c0,14.912,6.044,28.412,15.816,38.184s23.272,15.816,38.184,15.816
		c14.912,0,28.412-6.044,38.184-15.816s15.816-23.272,15.816-38.184c0-14.912-6.044-28.412-15.816-38.184"/>
	
		<line style="fill:none;stroke:red;stroke-width:10;stroke-linecap:round;stroke-miterlimit:10;" x1="157.542" y1="154.542" x2="157.542" y2="100.542"/>
</g>
</svg>"#;

const CARD: &str = r#"<?xml version="1.0" encoding="UTF-8" standalone="no"?>
<svg width="57" height="88" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink">
	<symbol id="suit" width="8" height="10">
		<path d="M 4 0 l 4 5 l -4 5 l -4 -5 z"></path>
	</symbol>
	<symbol id="rank" width="10" height="12">
		<rect width="2" height="12"></rect>
		<path d="M 6 0 h 2 l 2 2 v 8 l -2 2 h -2 l -2 -2 v -8 z m 0 2 v 8 h 2 v -8 z"></path>
	</symbol>
	<symbol id="index" width="57" height="88">
		<use xlink:href="url(#rank)" x="6" y="6"></use>
		<use xlink:href="url(#suit)" x="6" y="20"></use>
	</symbol>
	
	<rect x="1.5" y="1.5" width="54" height="85" fill='#fff'></rect>
	<path d="M 2 0 h 53 l 2 2 v 84 l -2 2 h -53 l -2 -2 v -84 z m 0 2 v 84 h 53 v -84 z"></path>
	<g style="fill: #f00;">
		<use xlink:href="url(#index)"></use>
		<use xlink:href="url(#index)" transform="rotate(180, 28.5, 44)"></use>
		<use xlink:href="url(#suit)" transform="translate(17, 11) scale(0.9)"></use>
		<use xlink:href="url(#suit)" transform="translate(32.8, 11) scale(0.9)"></use>
		<use xlink:href="url(#suit)" transform="translate(24.9, 20.5) scale(0.9)"></use>
		<use xlink:href="url(#suit)" transform="translate(17, 30) scale(0.9)"></use>
		<use xlink:href="url(#suit)" transform="translate(32.8, 30) scale(0.9)"></use>
		<g transform="rotate(180, 28.5, 44)">
			<use xlink:href="(#suit)" transform="translate(17, 11) scale(0.9)"></use>
			<use xlink:href="(#suit)" transform="translate(32.8, 11) scale(0.9)"></use>
			<use xlink:href="(#suit)" transform="translate(24.9, 20.5) scale(0.9)"></use>
			<use xlink:href="(#suit)" transform="translate(32.8, 30) scale(0.9)"></use>
			<use xlink:href="(#suit)" transform="translate(17, 30) scale(0.9)"></use>
		</g>
	</g>
</svg>"#;



#[derive(Clone)]
pub struct PowerButton {
    frm: Frame,
    on: Rc<RefCell<bool>>
}

impl PowerButton {
    pub fn new(x: i32, y: i32) -> Self {
        let mut frm = Frame::new(x, y, 80, 80, "");
        let on = Rc::from(RefCell::from(false));
        frm.draw({
            let on = on.clone();
            move |f| {
                let image_data = if *on.borrow() {
                    CARD.to_string().replace("red", "green")
                } else {
                    CARD.to_string()
                };
                let mut svg = SvgImage::from_data(&image_data).unwrap();
                svg.scale(f.width(), f.height(), true, true);
                svg.draw(f.x(), f.y(), f.width(), f.height());
            }
        });
        frm.handle({
            let on = on.clone();
            move |f, ev| match ev {
                Event::Push => {
                    let prev = *on.borrow();
                    *on.borrow_mut() = !prev;
                    f.do_callback();
                    f.redraw();
                    true
                }
                _ => false,
            }
        });
        Self { frm, on }
    }
}

impl Deref for PowerButton {
    type Target = Frame;

    fn deref(&self) -> &Self::Target {
        &self.frm
    }
}

impl DerefMut for PowerButton {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.frm
    }
}