#[macro_use]
extern crate serde_derive;
extern crate preferences;
extern crate app_dirs;
use preferences::{AppInfo, PreferencesMap, Preferences, prefs_base_dir};
use std::collections::HashMap;

const APP_INFO: AppInfo = AppInfo{name: "Smart Messroom", author: "Miroslav Chakarov"};



use fltk::{enums::{Align, Color, Font, FrameType}, prelude::*, *, };
use fltk_theme::{widget_themes, WidgetTheme, ThemeType};
use fltk_theme::{WidgetScheme, SchemeType};

//use std::{thread, time};

use linux_embedded_hal::Pin;
use linux_embedded_hal::sysfs_gpio::Direction;
use hd44780_hal::HD44780;

//hx711
use rppal::{spi::{Spi, Bus, SlaveSelect, Mode, Error},hal::Delay};
use hx711_spi::Hx711;
use nb::block;

use std::{thread, time};

static mut ONE_KG_VALUE: f32 = 130670.0;
const N: f32 = 30.0;
const READ_LOOP_COUNT: u8 = 5;

const BLUE: Color = Color::from_hex(0x42A5F5);
const SEL_BLUE: Color = Color::from_hex(0x2196F3);
const GRAY: Color = Color::from_hex(0x757575);
const GREEN: Color = Color::from_hex(0x9CC28F);
const WIDTH: i32 = 1024;
const HEIGHT: i32 = 768;

static mut kgval : f32 = 0.0;
static mut calib_flag: bool = false;

struct Customer{
    id: u64,
    product: String,
    quantity: f32,
}

impl Customer{
    fn add_new_customer(id: u64, prod: String, quant: f32){

    }
}

struct AdcData{
    adc_raw_val: f32,
    adc_val : f32,
    zero_val: f32,
    tara_val: f32,
    kg_val: f32,
    previous_kg_val: f32,
}
#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct SettingsData(f32);

fn callback(app: app::App) {
    app.redraw();
    app::repeat_timeout(0.005, Box::new(move || {
        callback(app);
    }));
}

fn add_product(product: String, quantity: u32){
    //let mut s: &str = &format!("{}: \n{} g.", product, quantity).to_owned();
    
   // let mut bar4 = frame::Frame::new(0, 0, 200, 90, s);
}



fn main() -> Result<(), Error> {
    //let mut sets = PreferencesMap::new();
    //sets.insert("onekg".into(),  1.5.to_string());
    // let prefs_key = "Documents/rust-projects/smart-messroom-gui";
    // //let save_result = sets.save(&APP_INFO, prefs_key);
    // //assert!(save_result.is_ok());

    // // Method `load` is from trait `Preferences`.
    // let load_result: HashMap<String, f32> = PreferencesMap::load(&APP_INFO, prefs_key).unwrap();
  

    // let a: f32 = *load_result.get("onekg".into()).unwrap();
    // print!("{}", &a);

    // print!("{:?}", prefs_base_dir().unwrap());


    let mut adc = AdcData {
        adc_raw_val: 0.0,
        adc_val : 0.0,
        zero_val: 0.0,
        tara_val: 0.0,
        kg_val: 0.0,
        previous_kg_val: 0.0,
    };

    let app = app::App::default().with_scheme(app::Scheme::Gleam);
    let widget_scheme = WidgetScheme::new(SchemeType::Aqua);
    widget_scheme.apply();
    let mut win = window::Window::default()
        .with_size(WIDTH, HEIGHT)
        .with_label("Smart Coffeteria");
    
    let mut bar = frame::Frame::new(0, 0, WIDTH, 80, "  Customer #4")
        .with_align(Align::Left | Align::Inside);
    let mut new_customer_btn = button::Button::new(60, 80, 200, 110, "New customer")
        //.with_align(Align::Left | Align::Inside)
        .below_of(&bar, 10);
        new_customer_btn.hide();
    let mut calibration_btn = button::Button::new(60, 160, 200, 110, "Calibration")
        .below_of(&new_customer_btn, 10);
    let mut calib_label = frame::Frame::new(220, 170, 220, 40, "Put 1 kg. weight\nto calibrate.")
        .below_of(&calibration_btn, 30);
    //calib_label.hide();    
    let mut product_label = frame::Frame::new(300, 170, 300, 40, "Hazelnuts");
        //.with_size(300, 40)
        //.below_of(&bar, 200);
        //.center_of(&win)
        //.with_align(Align::Center | Align::Inside);
        //.with_label("Hazelnuts");
    let mut calc_status = frame::Frame::default()
        .with_size(300, 60)
        .below_of(&product_label, 90)
       // .with_align(Align::Center)
        .with_label("");
    let mut weight_lbl = frame::Frame::default()
        .with_size(350, 60)
        //.size_of(&calc_status)
        .below_of(&calc_status, 90)
        //.with_align(Align::Center)
        .with_label("--- g.");
    let mut confirm_btn = button::Button::new(350, HEIGHT - 120, 180, 80, "Confirm");
       // .below_of(&weight_lbl, 30);
        confirm_btn.hide();
    let mut products_bar = frame::Frame::new(WIDTH - 250, 90, 200, 90, "Products added:")
        .with_align(Align::Right | Align::Inside);
    let mut pay_btn = button::Button::new(WIDTH - 230, HEIGHT - 120, 220, 110, "Pay"); //@+6plus
    //let mut spin = Progress::new(0,0,0);

    // let mut scroll = group::Scroll::new(WIDTH - 240, 30, 250, HEIGHT-150, "Products taken")
    //     .with_type(group::ScrollType::Vertical)
    //     .below_of(&products_bar, 10);
    
    let mut scroll = group::Scroll::new(WIDTH - 320, 50, 250, HEIGHT-300, None)
        .with_type(group::ScrollType::Vertical)
        .below_of(&products_bar, 0);
    let mut scrollbar = scroll.scrollbar();
        scrollbar.set_type(valuator::ScrollbarType::VerticalNice);
    let mut pack = group::Pack::default_fill();
    pack.begin();
    
    
    pack.end();
    //scroll.scroll_to(0, 0);
    scroll.end();

    win.end();
    win.make_resizable(true);
    win.show();

    let mut payment_win = window::OverlayWindow::default()
        .with_size(800, 530)
        .center_of(&win)
        .with_label("Payment - Smart Coffeteria");
    // let mut progress = misc::Progress::new(200, 200, 300, 20, "Sending payment request...")
    //     .set_value(100.0)
    let mut pay_title_lbl = frame::Frame::new(0, 50, 800, 80, "Payment request sent.");
    pay_title_lbl.set_label_size(35);
    pay_title_lbl.set_label_color(BLUE);
    let mut pay_amount_lbl = frame::Frame::new(0, 0, 800, 80, "0 BGN")
        .below_of(&pay_title_lbl, 50);
    pay_amount_lbl.set_label_size(50);

    let mut info_lbl = frame::Frame::new(0, 0, 800, 80, "Open your wallet app and scan the QR code on the tablet.
                                                        Waiting for payment confirmation...")
        .below_of(&pay_amount_lbl, 50);
    info_lbl.set_label_size(22);
    let mut close_pay_dial_btn = button::Button::new (450, 450, 300, 80, "Close")
        .with_align(Align::Right);

        close_pay_dial_btn.set_color(BLUE);
        close_pay_dial_btn.set_selection_color(SEL_BLUE);
        close_pay_dial_btn.set_label_color(Color::White);
        close_pay_dial_btn.set_label_size(25);

        
    payment_win.end();
    
    close_pay_dial_btn.set_callback(move |_| {
        
        &payment_win.hide();
        
    });
    app::add_timeout(0.005, Box::new(move || {
        callback(app);
    }));

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

   
    
    

    products_bar.set_frame(FrameType::FlatBox);
    products_bar.set_label_size(22);
    // products_bar.set_label_color(Color::White);
    // products_bar.set_color(GREEN);
    // products_bar.draw(|b| {
    //     draw::set_draw_rgb_color(211, 211, 211);
    //     draw::draw_rectf(0, b.height(), b.width(), 1);
    // });

    calc_status.set_label_size(24);
    calc_status.set_label_font(Font::Times);

    product_label.set_label_size(50);

    weight_lbl.set_label_size(46);
    weight_lbl.set_label_color(GRAY);
    calib_label.set_label_size(26);
    pay_btn.set_color(BLUE);
    pay_btn.set_selection_color(SEL_BLUE);
    pay_btn.set_label_color(Color::White);
    pay_btn.set_label_size(35);
   // pay_btn.set_frame(FrameType::GtkUpBox);
  
    confirm_btn.set_color(BLUE);
    confirm_btn.set_selection_color(SEL_BLUE);
    confirm_btn.set_label_color(Color::White);
    confirm_btn.set_label_size(32);

    new_customer_btn.set_color(BLUE);
    new_customer_btn.set_selection_color(SEL_BLUE);
    new_customer_btn.set_label_color(Color::White);
    new_customer_btn.set_label_size(25);
    //new_customer_btn.set_frame(FrameType::GtkUpBox);
    // End theming

    calibration_btn.set_color(BLUE);
    calibration_btn.set_selection_color(SEL_BLUE);
    calibration_btn.set_label_color(Color::White);
    calibration_btn.set_label_size(25);

    pay_btn.set_callback(move |_| {
        
        println!("Pay pressed");
        &payment_win.show();
       
    });
    confirm_btn.set_callback(move |_| {
       
        println!("Confirm pressed");
        pack.begin();
        //let un = format!("{}", kgval).as_str();

        let mut bar_x = frame::Frame::new(0, 0, 200, 40, "Hazelnuts");
        bar_x.set_label_size(21);
        unsafe{
            let mut bar_y = frame::Frame::default()
                .with_label(format!("{:.0} g.", (kgval)).as_str())
                .with_size(200, 40);
                bar_y.set_label_size(21);
                    
        }
        pack.end();

        
    });
    new_customer_btn.set_callback(move |_| {
       println!("New Customer pressed");
    });
    //hx711 declarations
    let spi = Spi::new(Bus::Spi0, SlaveSelect::Ss0, 1_000_000, Mode::Mode0)?;
    let mut hx711 = Hx711::new(spi, Delay::new());

    //init the ADC
    hx711.reset()?;
    

    //app.run().unwrap();
    //done with GUI inits
    
    

    calibration_btn.set_callback(move |_| {
        unsafe{calib_flag = true;}
        calib_label.set_label("Put 1 kg. weight\nto calibrate.");
    });

    //calibration (tara)
    for i in 0..N as i32 {
        let reading = block!(hx711.read()).unwrap() as f32;
        println!("{:>2}: {}", i, reading);
        adc.zero_val += reading;
    }
    adc.zero_val /= N; //tara
    println!("Tara: {}", adc.zero_val);

    

    

    //screen declarations
    let rs = Pin::new(13);
    let en = Pin::new(19);

    let db4 = Pin::new(26);
    let db5 = Pin::new(16);
    let db6 = Pin::new(20);
    let db7 = Pin::new(21);

    rs.export().unwrap();
    en.export().unwrap();
    
    db4.export().unwrap();
    db5.export().unwrap();
    db6.export().unwrap();
    db7.export().unwrap();

    rs.set_direction(Direction::Low).unwrap();
    en.set_direction(Direction::Low).unwrap();
    
    db4.set_direction(Direction::Low).unwrap();
    db5.set_direction(Direction::Low).unwrap();
    db6.set_direction(Direction::Low).unwrap();
    db7.set_direction(Direction::Low).unwrap();

    //4-bit communication with display
    let mut lcd = HD44780::new_4bit(
        rs,
        en,
    
        db4,
        db5,
        db6,
        db7,
        linux_embedded_hal::Delay,
    );
    
    lcd.reset();
    lcd.clear();
    lcd.set_display_mode(true, false, false);
    lcd.write_str("Smart Coffeteria");

    lcd.set_cursor_pos(40);
    lcd.write_str("WELCOME!");
    lcd.set_cursor_pos(30);
    lcd.write_str("--- g.");

    
    let mut counter: u8 = 0;
    let mut flag: bool = false;
    
    
    while app.wait()
    {
        adc.previous_kg_val = adc.kg_val;
        adc.adc_val = 0.0;
        let prefs_key = "Documents/rust-projects/smart-messroom-gui";
    
    // Method `load` is from trait `Preferences`.
        let load_result: HashMap<String, f32> = PreferencesMap::load(&APP_INFO, prefs_key).unwrap();
        let a: f32 = *load_result.get("onekg".into()).unwrap();
        
        print!("{}", &a);

        print!("{:?}", prefs_base_dir().unwrap());
        unsafe{ONE_KG_VALUE = a;}
        
        for _ in 0..READ_LOOP_COUNT {
            adc.adc_raw_val = block!(hx711.read()).unwrap() as f32;
            adc.adc_val += adc.adc_raw_val;
        }
        adc.adc_val /= READ_LOOP_COUNT as f32;
        adc.tara_val = adc.adc_val - adc.zero_val;
        unsafe{adc.kg_val = adc.tara_val / ONE_KG_VALUE;}
       
        println!(
            "Read: {} --- Tara val: {} --- kg: {:.3}", 
            adc.adc_val as i32, 
            adc.tara_val as i32, 
            adc.kg_val
            );
                
        if (adc.kg_val - adc.previous_kg_val) > 0.0015 {
            println!("--- START LISTENING --- {}", adc.kg_val - adc.previous_kg_val);
            lcd.set_cursor_pos(40);
            lcd.write_str("Calculating...");
            calc_status.set_label("Calculating...");
            confirm_btn.hide();
            flag = true;
            counter = 0;
        }

        if (adc.previous_kg_val - adc.kg_val) > 0.0015 {
            println!("---STOP LISTENING--- To be added: {:.3}", adc.kg_val);
            lcd.set_cursor_pos(40);
            lcd.write_str("Almost there...");
            calc_status.set_label("Almost there...");
            confirm_btn.hide();

            flag = true;
            counter = 0;
        }
        
        if flag == true {
            counter += 1;
            lcd.set_cursor_pos(30);
            let s = format!("{:.0} g.   ", (adc.kg_val * 1000.0).abs());
            lcd.write_str(&s);
            weight_lbl.set_label(&s);
            if counter == 5{
                unsafe{if calib_flag == true{
                    calc_status.set_label("Calibration done.\n\nContinue using the system\nas usual.");
                    calib_flag = false;
                    //calib_label.set_label("");

                    let mut sets = PreferencesMap::new();
                    sets.insert("onekg".into(),  adc.tara_val.to_string());
                    let prefs_key = "Documents/rust-projects/smart-messroom-gui";
                    let save_result = sets.save(&APP_INFO, prefs_key);
                    assert!(save_result.is_ok());
                }
                
                else if adc.kg_val.abs() > 0.001 {
                    println!("{:.3} added to customer", adc.kg_val);
                    //unsafe{
                        kgval = (adc.kg_val*1000.0).abs();
                        println!("{}", kgval);
                    //}
                    lcd.set_cursor_pos(40);
                    lcd.write_str("Hazelnuts:      ");
                    calc_status.set_label("Done.\n\nPress Confirm to continue or\nreturn the food to reset.");
                    confirm_btn.show();
                    
                }
                else{
                    println!("Quantity returned");
                    lcd.set_cursor_pos(40);
                    lcd.write_str("Returned      ");
                    calc_status.set_label("Food returned.\n\nContinue with another product\n or tap Pay to continue.");
                }
                }
                flag = false;
                counter = 0;
            }
        }
        //thread::sleep(time::Duration::from_millis(50));   
    }   
    Ok(()) 
}
    


