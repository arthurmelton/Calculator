use fltk::button::*;
use fltk::enums::*;
use fltk::frame::*;
use fltk::prelude::*;
use fltk::window::*;
use fltk::*;

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Add,
    Sub,
    Times,
    Dev,
    Clear,
    Neg,
    Percent,
    Dot,
    Equal,
}

fn main() {
    let app = app::App::default();
    let mut wind = Window::default()
        .with_size(400,675)
        .with_label("Calculator");
    wind.make_resizable(true);
    wind.set_color(Color::Black);
    
    let mut frame = Frame::new(375, 50, 350, 75, "0").with_align(Align::Left);
    frame.set_label_color(Color::White);
    frame.set_label_size(61);
    
    let mut one = Button::new(25, 475, 75, 75, "1");
    let mut two = Button::new(116, 475, 75, 75, "2");
    let mut three = Button::new(207, 475, 75, 75, "3");
    let mut plus = Button::new(300, 475, 75, 75, "+");
    let mut four = Button::new(25, 375, 75, 75, "4");
    let mut five = Button::new(116, 375, 75, 75, "5");
    let mut six = Button::new(207, 375, 75, 75, "6");
    let mut minus = Button::new(300, 375, 75, 75, "-");
    let mut seven = Button::new(25, 275, 75, 75, "7");
    let mut eight = Button::new(116, 275, 75, 75, "8");
    let mut nine = Button::new(207, 275, 75, 75, "9");
    let mut times = Button::new(300, 275, 75, 75, "x");
    let mut clear = Button::new(25, 175, 75, 75, "AC");
    let mut neg = Button::new(116, 175, 75, 75, "+/-");
    let mut percent = Button::new(207, 175, 75, 75, "%");
    let mut divide = Button::new(300, 175, 75, 75, "÷");
    let mut zero = Button::new(25, 575, 166, 75, "0");
    let mut dot = Button::new(207, 575, 75, 75, ".");
    let mut equal = Button::new(300, 575, 75, 75, "=");

    one.set_color(Color::rgb_color(51,51,51));
    one.set_label_color(Color::White);
    one.set_label_size(40);
    one.set_frame(FrameType::OvalBox);
    one.set_selection_color(Color::rgb_color(165,165,165));

    two.set_color(Color::rgb_color(51,51,51));
    two.set_label_color(Color::White);
    two.set_label_size(40);
    two.set_frame(FrameType::OvalBox);
    two.set_selection_color(Color::rgb_color(165,165,165));

    three.set_color(Color::rgb_color(51,51,51));
    three.set_label_color(Color::White);
    three.set_label_size(40);
    three.set_frame(FrameType::OvalBox);
    three.set_selection_color(Color::rgb_color(165,165,165));

    plus.set_color(Color::rgb_color(241,163,60));
    plus.set_label_color(Color::White);
    plus.set_label_size(40);
    plus.set_frame(FrameType::OvalBox);
    plus.set_selection_color(Color::rgb_color(255,200,120));

    zero.set_color(Color::rgb_color(51,51,51));
    zero.set_label_color(Color::White);
    zero.set_label_size(40);
    zero.set_frame(FrameType::RoundedBox);
    zero.set_selection_color(Color::rgb_color(165,165,165));

    dot.set_color(Color::rgb_color(51,51,51));
    dot.set_label_color(Color::White);
    dot.set_label_size(40);
    dot.set_frame(FrameType::OvalBox);
    dot.set_selection_color(Color::rgb_color(165,165,165));

    equal.set_color(Color::rgb_color(241,163,60));
    equal.set_label_color(Color::White);
    equal.set_label_size(40);
    equal.set_frame(FrameType::OvalBox);
    equal.set_selection_color(Color::rgb_color(255,200,120));

    four.set_color(Color::rgb_color(51,51,51));
    four.set_label_color(Color::White);
    four.set_label_size(40);
    four.set_frame(FrameType::OvalBox);
    four.set_selection_color(Color::rgb_color(165,165,165));

    five.set_color(Color::rgb_color(51,51,51));
    five.set_label_color(Color::White);
    five.set_label_size(40);
    five.set_frame(FrameType::OvalBox);
    five.set_selection_color(Color::rgb_color(165,165,165));

    six.set_color(Color::rgb_color(51,51,51));
    six.set_label_color(Color::White);
    six.set_label_size(40);
    six.set_frame(FrameType::OvalBox);
    six.set_selection_color(Color::rgb_color(165,165,165));

    minus.set_color(Color::rgb_color(241,163,60));
    minus.set_label_color(Color::White);
    minus.set_label_size(40);
    minus.set_frame(FrameType::OvalBox);
    minus.set_selection_color(Color::rgb_color(255,200,120));

    seven.set_color(Color::rgb_color(51,51,51));
    seven.set_label_color(Color::White);
    seven.set_label_size(40);
    seven.set_frame(FrameType::OvalBox);
    seven.set_selection_color(Color::rgb_color(165,165,165));

    eight.set_color(Color::rgb_color(51,51,51));
    eight.set_label_color(Color::White);
    eight.set_label_size(40);
    eight.set_frame(FrameType::OvalBox);
    eight.set_selection_color(Color::rgb_color(165,165,165));

    nine.set_color(Color::rgb_color(51,51,51));
    nine.set_label_color(Color::White);
    nine.set_label_size(40);
    nine.set_frame(FrameType::OvalBox);
    nine.set_selection_color(Color::rgb_color(165,165,165));

    times.set_color(Color::rgb_color(241,163,60));
    times.set_label_color(Color::White);
    times.set_label_size(40);
    times.set_frame(FrameType::OvalBox);
    times.set_selection_color(Color::rgb_color(255,200,120));

    clear.set_color(Color::rgb_color(165,165,165));
    clear.set_label_color(Color::White);
    clear.set_label_size(40);
    clear.set_frame(FrameType::OvalBox);
    clear.set_selection_color(Color::rgb_color(255,255,255));

    neg.set_color(Color::rgb_color(165,165,165));
    neg.set_label_color(Color::White);
    neg.set_label_size(40);
    neg.set_frame(FrameType::OvalBox);
    neg.set_selection_color(Color::rgb_color(255,255,255));

    percent.set_color(Color::rgb_color(165,165,165));
    percent.set_label_color(Color::White);
    percent.set_label_size(40);
    percent.set_frame(FrameType::OvalBox);
    percent.set_selection_color(Color::rgb_color(255,255,255));

    divide.set_color(Color::rgb_color(241,163,60));
    divide.set_label_color(Color::White);
    divide.set_label_size(40);
    divide.set_frame(FrameType::OvalBox);
    divide.set_selection_color(Color::rgb_color(255,200,120));

    let mut back_number:f64 = 0.0;
    let mut number:f64 = 0.0;
    let mut last_op:i8 = 0;

    let mut add_dot = false;

    wind.end();
    wind.show();

    let (s, r) = app::channel::<Message>();

    zero.emit(s, Message::Zero);
    one.emit(s, Message::One);
    two.emit(s, Message::Two);
    three.emit(s, Message::Three);
    four.emit(s, Message::Four);
    five.emit(s, Message::Five);
    six.emit(s, Message::Six);
    seven.emit(s, Message::Seven);
    eight.emit(s, Message::Eight);
    nine.emit(s, Message::Nine);
    plus.emit(s, Message::Add);
    minus.emit(s, Message::Sub);
    times.emit(s, Message::Times);
    divide.emit(s, Message::Dev);
    clear.emit(s, Message::Clear);
    percent.emit(s, Message::Percent);
    dot.emit(s, Message::Dot);
    neg.emit(s, Message::Neg);
    equal.emit(s, Message::Equal);

    while app.wait() {
        if let Some(msg) = r.recv() {
            match msg {
                Message::Zero => {if add_dot {number = (number.to_string() + &".0".to_string()).parse::<f64>().unwrap(); add_dot = false;} else {number = (number.to_string() + &"0".to_string()).parse::<f64>().unwrap();}},
                Message::One => {if add_dot {number = (number.to_string() + &".1".to_string()).parse::<f64>().unwrap(); add_dot = false;} else {number = (number.to_string() + &"1".to_string()).parse::<f64>().unwrap();}},
                Message::Two => {if add_dot {number = (number.to_string() + &".2".to_string()).parse::<f64>().unwrap(); add_dot = false;} else {number = (number.to_string() + &"2".to_string()).parse::<f64>().unwrap();}},
                Message::Three => {if add_dot {number = (number.to_string() + &".3".to_string()).parse::<f64>().unwrap(); add_dot = false;} else {number = (number.to_string() + &"3".to_string()).parse::<f64>().unwrap();}},
                Message::Four => {if add_dot {number = (number.to_string() + &".4".to_string()).parse::<f64>().unwrap(); add_dot = false;} else {number = (number.to_string() + &"4".to_string()).parse::<f64>().unwrap();}},
                Message::Five => {if add_dot {number = (number.to_string() + &".5".to_string()).parse::<f64>().unwrap(); add_dot = false;} else {number = (number.to_string() + &"5".to_string()).parse::<f64>().unwrap();}},
                Message::Six => {if add_dot {number = (number.to_string() + &".6".to_string()).parse::<f64>().unwrap(); add_dot = false;} else {number = (number.to_string() + &"6".to_string()).parse::<f64>().unwrap();}},
                Message::Seven => {if add_dot {number = (number.to_string() + &".7".to_string()).parse::<f64>().unwrap(); add_dot = false;} else {number = (number.to_string() + &"7".to_string()).parse::<f64>().unwrap();}},
                Message::Eight => {if add_dot {number = (number.to_string() + &".8".to_string()).parse::<f64>().unwrap(); add_dot = false;} else {number = (number.to_string() + &"8".to_string()).parse::<f64>().unwrap();}},
                Message::Nine => {if add_dot {number = (number.to_string() + &".9".to_string()).parse::<f64>().unwrap(); add_dot = false;} else {number = (number.to_string() + &"9".to_string()).parse::<f64>().unwrap();}},
                Message::Clear => {if number == 0.0 {back_number = 0.0;  last_op = 0;}; number = 0.0 as f64;},
                Message::Neg => {number = number * -1.0;},
                Message::Add => {if last_op == 0 {back_number = number} else if last_op == 1 {back_number += number} else if last_op == 2 {back_number -= number} else if last_op == 3 {back_number *= number} else if last_op == 4 {back_number /= number};last_op = 1; number = 0.0 as f64;},
                Message::Equal => {if last_op == 0 {number = back_number} else if last_op == 1 {number += back_number} else if last_op == 2 {number -= back_number} else if last_op == 3 {number *= back_number} else if last_op == 4 {number = back_number/number};},
                Message::Sub => {if last_op == 0 {back_number = number} else if last_op == 1 {back_number += number} else if last_op == 2 {back_number -= number} else if last_op == 3 {back_number *= number} else if last_op == 4 {back_number /= number}; last_op = 2; number = 0.0 as f64;},
                Message::Times => {if last_op == 0 {back_number = number} else if last_op == 1 {back_number += number} else if last_op == 2 {back_number -= number} else if last_op == 3 {back_number *= number} else if last_op == 4 {back_number /= number}; last_op = 3; number = 0.0 as f64;},
                Message::Dev => {if last_op == 0 {back_number = number} else if last_op == 1 {back_number += number} else if last_op == 2 {back_number -= number} else if last_op == 3 {back_number *= number} else if last_op == 4 {back_number /= number}; last_op = 4; number = 0.0 as f64;},
                Message::Percent => {number = number/100.0;},
                Message::Dot => {add_dot = true;},
            }
            frame.set_label("                      ");
            frame.set_label(number_to_string(number.to_string()).as_str());
        }
        zero.redraw();
        one.redraw();
        two.redraw();
        three.redraw();
        four.redraw();
        five.redraw();
        six.redraw();
        seven.redraw();
        eight.redraw();
        nine.redraw();
        plus.redraw();
        minus.redraw();
        times.redraw();
        divide.redraw();
        clear.redraw();
        percent.redraw();
        dot.redraw();
        neg.redraw();
        equal.redraw();


        zero.clear_visible_focus();
        one.clear_visible_focus();
        two.clear_visible_focus();
        three.clear_visible_focus();
        four.clear_visible_focus();
        five.clear_visible_focus();
        six.clear_visible_focus();
        seven.clear_visible_focus();
        eight.clear_visible_focus();
        nine.clear_visible_focus();
        plus.clear_visible_focus();
        minus.clear_visible_focus();
        times.clear_visible_focus();
        divide.clear_visible_focus();
        clear.clear_visible_focus();
        percent.clear_visible_focus();
        dot.clear_visible_focus();
        neg.clear_visible_focus();
        equal.clear_visible_focus();
    }
}

fn number_to_string(number:String) -> String {
    return if number.chars().count() < 10 {
        let mut returns = "".to_string();
        let mut index = number.chars().count();
        let mut offset_sub = 0;
        let mut offset_add = 0;
        let mut seen_point = false;
        for i in number.chars() {
            if i == '.' {
                seen_point = true;
            }
            if seen_point {
                offset_add+=1;
            }
        }
        seen_point = false;
        for i in number.chars() {
            if i == '.' {
                seen_point = true;
            }
            else if i == '-' {
                offset_sub += 1;
            }
            if index > offset_add && (index - offset_add) % 3 == 0 && index < number.chars().count() - offset_sub && number.chars().count() > index && !seen_point {
                returns.push_str(",");
            }
            returns.push_str(i.to_string().as_str());
            index -= 1;
        }
        returns
    } else {
        let mut index = 0;
        let mut number_f64 = number.parse::<f64>().unwrap();
        while number_f64 > 10.0 || number_f64 < -10.0  {
            index +=1;
            number_f64 *= 0.1;
        }
        if &number_f64.to_string().chars().count() < &(10 as usize) {
            [number_f64.to_string().as_str(), &"000000000000".to_string()[0..(9 - index.to_string().chars().count()) - number_f64.to_string().chars().count()], "e", (index+1).to_string().as_str()].join("")
        }
        else {
            [&number_f64.to_string()[0..(9 - index.to_string().chars().count())], "e", index.to_string().as_str()].join("")
        }
    }
}