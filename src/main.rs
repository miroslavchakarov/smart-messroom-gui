use fltk::{enums::{Align, Color, Font, FrameType}, prelude::*, *, };

use std::{thread, time};
use linux_embedded_hal::Pin;
use linux_embedded_hal::sysfs_gpio::Direction;
use hd44780_hal::HD44780;

//hx711
use rppal::{spi::{Spi, Bus, SlaveSelect, Mode, Error},hal::Delay};
use hx711_spi::Hx711;
use nb::block;

const ONE_KG_VALUE: f32 = 130670.0;
const N: f32 = 30.0;
const READ_LOOP_COUNT: u8 = 5;



const BLUE: Color = Color::from_hex(0x42A5F5);
const SEL_BLUE: Color = Color::from_hex(0x2196F3);
const GRAY: Color = Color::from_hex(0x757575);
const GREEN: Color = Color::from_hex(0x9CC28F);
const WIDTH: i32 = 800;
const HEIGHT: i32 = 500;

struct AdcData{
    adc_raw_val: f32,
    adc_val : f32,
    zero_val: f32,
    tara_val: f32,
    kg_val: f32,
    previous_kg_val: f32,
}

fn callback(app: app::App) {
    app.redraw();
    // println!("TICK");
    app::repeat_timeout(0.005, Box::new(move || {
        callback(app);
    }));
}

fn main() -> Result<(), Error> {
    let app = app::App::default().with_scheme(app::Scheme::Gleam);
    let mut win = window::Window::default()
        .with_size(WIDTH, HEIGHT)
        .with_label("Smart Messroom");
    let mut bar =
        frame::Frame::new(0, 0, WIDTH, 60, "  Customer #4").with_align(Align::Left | Align::Inside);
    let mut product_label = frame::Frame::default()
        .with_size(200, 40)
        .center_of(&win)
        .with_label("Hazelnuts");
    let mut calc_status = frame::Frame::default()
        .with_size(200, 40)
        .below_of(&product_label, 0)
        .with_label("");
    let mut weight_lbl = frame::Frame::default()
        .with_size(200, 40)
        //.size_of(&calc_status)
        .below_of(&calc_status, 0)
        .with_label("--- g.");
    let mut but = button::Button::new(WIDTH - 220, HEIGHT - 80, 200, 60, "Confirm"); //@+6plus
        
    win.end();
    win.make_resizable(true);
    win.show();
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
        //let label = (weight_lbl.label().parse::<i32>().unwrap() + 1).to_string();
       // weight_lbl.set_label(&label);
       println!("Button pressed");
    });

    //app.run().unwrap();
    //done with GUI inits
    
    let mut adc = AdcData{
        adc_raw_val: 0.0,
        adc_val : 0.0,
        zero_val: 0.0,
        tara_val: 0.0,
        kg_val: 0.0,
        previous_kg_val: 0.0,
    };

    //hx711 declarations
    let spi = Spi::new(Bus::Spi0, SlaveSelect::Ss0, 1_000_000, Mode::Mode0)?;
    let mut hx711 = Hx711::new(spi, Delay::new());
    
    //init the screen
    hx711.reset()?;

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
    lcd.write_str("Customer #4");

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
        
        for _ in 0..READ_LOOP_COUNT {
            adc.adc_raw_val = block!(hx711.read()).unwrap() as f32;
            adc.adc_val += adc.adc_raw_val;
        }
        adc.adc_val /= READ_LOOP_COUNT as f32;
        adc.tara_val = adc.adc_val-adc.zero_val;
        adc.kg_val = adc.tara_val/ONE_KG_VALUE;
        println!(
            "Read: {} --- Tara val: {} --- kg: {:.3}", 
            adc.adc_val as i32, 
            adc.tara_val as i32, 
            adc.kg_val);
                
        if (adc.kg_val - adc.previous_kg_val) > 0.002 {
            println!("--- START LISTENING --- {}", adc.kg_val - adc.previous_kg_val);
            lcd.set_cursor_pos(40);
            lcd.write_str("Calculating...");
            calc_status.set_label("Calculating...");
        }

        if (adc.previous_kg_val - adc.kg_val) > 0.002 {
            println!("---STOP LISTENING--- To be added: {:.3}", adc.kg_val);
            lcd.set_cursor_pos(40);
            lcd.write_str("Almost there...");
            calc_status.set_label("Almost there...");

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
                println!("{:.3} added to customer", adc.kg_val);
                
                lcd.set_cursor_pos(40);
                lcd.write_str("Hazelnuts:      ");
                calc_status.set_label("Done. Press Confirm.");
                flag = false;
                counter = 0;
            }
        }
        thread::sleep(time::Duration::from_millis(50));
        
    }   
    Ok(()) 
      
}
    


