extern crate gtk;

pub mod stack;
pub mod calc;

// use gdk::prelude::*;
use gtk::prelude::*;
use gtk::{Builder, Button, Window, TextView, TextBuffer};

fn map_number_btns(text_view: &TextView, nums_vec: &Vec<Button>) {
    let mut idx = 0;

    for num in nums_vec {
        let tv = text_view.clone();

        num.connect_clicked(move |_| {
            let _ = tv.get_buffer().unwrap().insert_at_cursor(&idx.to_string());
        });

        idx += 1;
    }
}

fn map_btn_insert_at_cursor(text_view: &TextView, btn: &Button, txt: &str) {
    let tv = text_view.clone();
    let txt_cpy: String = (*txt).to_string();

    btn.connect_clicked(move |_| {
        let buf = tv.get_buffer().unwrap();
        buf.insert_at_cursor(&txt_cpy);
    });
}

fn calc_and_set_result(in_buf: &TextBuffer, out_buf: &TextBuffer){
    let (begin, end) = in_buf.get_bounds();

    match calc::calc(&(in_buf.get_text(&begin, &end, true)).unwrap()) {
        Ok(x) => {
            let stringed = format!("= {}", x);
            out_buf.set_text(&stringed);
        },
        Err(x) => out_buf.set_text(&x)
    }
}

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let glade_src = include_str!("calc_win.glade");
    let builder = Builder::new_from_string(glade_src);


    let window: Window = builder.get_object("calc_app_win").unwrap();

    let text_view: TextView = builder.get_object("input_view").unwrap();
    let output_view: TextView = builder.get_object("output_view").unwrap();

    let nums_vec: Vec<Button> = vec![builder.get_object("btn_0").unwrap(),

                                     builder.get_object("btn_1").unwrap(),
                                     builder.get_object("btn_2").unwrap(),
                                     builder.get_object("btn_3").unwrap(),

                                     builder.get_object("btn_4").unwrap(),
                                     builder.get_object("btn_5").unwrap(),
                                     builder.get_object("btn_6").unwrap(),

                                     builder.get_object("btn_7").unwrap(),
                                     builder.get_object("btn_8").unwrap(),
                                     builder.get_object("btn_9").unwrap()];

    let btn_calc: Button = builder.get_object("btn_calc").unwrap();
    let btn_clear: Button = builder.get_object("btn_clear").unwrap();

    let btn_comma: Button = builder.get_object("btn_comma").unwrap();
    let btn_sub: Button = builder.get_object("btn_sub").unwrap();
    let btn_add: Button = builder.get_object("btn_add").unwrap();
    let btn_mul: Button = builder.get_object("btn_mul").unwrap();
    let btn_div: Button = builder.get_object("btn_div").unwrap();
    let btn_percent: Button = builder.get_object("btn_percent").unwrap();
    let btn_par_left: Button = builder.get_object("btn_par_left").unwrap();
    let btn_par_right: Button = builder.get_object("btn_par_right").unwrap();


    // let btn: Button = builder.get_object("btn1").unwrap();
    // let image: Image = builder.get_object("image1").unwrap();

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    //Clear output view on input view changed
    let tv = text_view.clone();
    let tv_out = output_view.clone();

    text_view.connect_key_press_event(move |_, key|{
        match key.get_keyval(){
            65293 | 65421 => { //ENTER, num lock enter button
                let buf = tv.get_buffer().unwrap();
                let buf_out = tv_out.get_buffer().unwrap();
                calc_and_set_result(&buf, &buf_out);
                Inhibit(true)
            },
            48...57 | 65...90 | 97...122 | 94 | 37 | 
            46 | 42 | 47 | 65288 | 65456...65465 | 40 | 41 |
            65455 | 65450 | 65453 | 65451 | 65454
            => { //nums, letters, ^, %, ., *, /, space, backspace, num lock, (, )  keys
                tv_out.get_buffer().unwrap().set_text("");
                Inhibit(false)
            }, 
            _ =>{  
                //println!("VAL {}",  key.get_keyval());
                Inhibit(true)
            }
        }
    });

    // Map buttons
    map_number_btns(&text_view, &nums_vec);

    // Calc result
    let tv = text_view.clone();
    let tv_out = output_view.clone();

    btn_calc.connect_clicked(move |_| {
        let buf = tv.get_buffer().unwrap();
        let buf_out = tv_out.get_buffer().unwrap();


        calc_and_set_result(&buf, &buf_out);
        // let stringed = format!("{}",
        //                        calc::calc(&(buf.get_text(&begin, &end, true)).unwrap()).unwrap());

    });

    // Clear text view
    let tv = text_view.clone();
    let tv_out = output_view.clone();

    btn_clear.connect_clicked(move |_| {
        let buf = tv.get_buffer().unwrap();
        let buf_out = tv_out.get_buffer().unwrap();

        buf.set_text("");
        buf_out.set_text("");
    });

    // Operators
    map_btn_insert_at_cursor(&text_view, &btn_comma, ".");

    map_btn_insert_at_cursor(&text_view, &btn_percent, "%");

    map_btn_insert_at_cursor(&text_view, &btn_sub, "-");
    map_btn_insert_at_cursor(&text_view, &btn_add, "+");

    map_btn_insert_at_cursor(&text_view, &btn_mul, "*");
    map_btn_insert_at_cursor(&text_view, &btn_div, "/");

    map_btn_insert_at_cursor(&text_view, &btn_par_left, "(");
    map_btn_insert_at_cursor(&text_view, &btn_par_right, ")");


    window.show_all();

    gtk::main();
}
