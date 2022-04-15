use fltk::{enums::{Align, Color, Font, FrameType}, prelude::*, *, };

const BLUE: Color = Color::from_hex(0x42A5F5);
const SEL_BLUE: Color = Color::from_hex(0x2196F3);
const GRAY: Color = Color::from_hex(0x757575);
const GREEN: Color = Color::from_hex(0x9CC28F);
const WIDTH: i32 = 600;
const HEIGHT: i32 = 400;

fn main() {
    let app = app::App::default();
    let mut win = window::Window::default()
        .with_size(WIDTH, HEIGHT)
        .with_label("Smart Messroom");
    let mut bar =
        frame::Frame::new(0, 0, WIDTH, 60, "  Customer #4").with_align(Align::Left | Align::Inside);
    let mut product_label = frame::Frame::default()
        .with_size(100, 40)
        .center_of(&win)
        .with_label("Almond nuts");
    let mut calc_status = frame::Frame::default()
        .size_of(&product_label)
        .below_of(&product_label, 0)
        .with_label("Almost there...");
    let mut weight_lbl = frame::Frame::default()
        .size_of(&calc_status)
        .below_of(&calc_status, 0)
        .with_label("0");
    let mut but = button::Button::new(WIDTH - 220, HEIGHT - 80, 200, 60, "Confirm"); //@+6plus
        
    win.end();
    win.make_resizable(true);
    win.show();

    // Theming
    app::background(255, 255, 255);
    app::set_visible_focus(false);

    bar.set_frame(FrameType::FlatBox);
    bar.set_label_size(22);
    bar.set_label_color(Color::White);
    bar.set_color(GREEN);
    bar.draw(|b| {
        draw::set_draw_rgb_color(211, 211, 211);
        draw::draw_rectf(0, b.height(), b.width(), 1);
    });

    calc_status.set_label_size(18);
    calc_status.set_label_font(Font::Times);

    product_label.set_label_size(30);

    weight_lbl.set_label_size(36);
    weight_lbl.set_label_color(GRAY);

    but.set_color(BLUE);
    but.set_selection_color(SEL_BLUE);
    but.set_label_color(Color::White);
    but.set_label_size(28);
    but.set_frame(FrameType::RFlatBox);
    // End theming

    but.set_callback(move |_| {
        let label = (weight_lbl.label().parse::<i32>().unwrap() + 1).to_string();
        weight_lbl.set_label(&label);
    });

    app.run().unwrap();
}
