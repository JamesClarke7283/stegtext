use fltk::{
    app, button::Button, enums::*, frame::Frame, input::MultilineInput, menu::*, prelude::*,
    window::Window,
};
use std::{cell::RefCell, rc::Rc};
use stego_wps::{compare, decode, encode};

fn main() {
    let app = app::App::default();
    let mut win = Window::default().with_size(800, 600).with_label("StegText");

    let mut menu_bar = MenuBar::new(0, 0, 800, 30, "");
    let (sender, receiver) = app::channel::<()>(); // Corrected line

    menu_bar.add_emit("&File/Exit", Shortcut::None, MenuFlag::Normal, sender, ());

    let secret_input = Rc::new(RefCell::new(MultilineInput::new(
        10,
        40,
        380,
        200,
        "Secret Text:",
    )));
    let cover_input = Rc::new(RefCell::new(MultilineInput::new(
        410,
        40,
        380,
        200,
        "Cover Text:",
    )));

    let output_display = Rc::new(RefCell::new(MultilineInput::new(
        10, 250, 780, 290, "Output:",
    )));
    output_display
        .borrow_mut()
        .set_value("The output will appear here");
    output_display.borrow_mut().set_readonly(true);
    output_display.borrow_mut().set_frame(FrameType::FlatBox);

    let compare_output = Rc::new(RefCell::new(Frame::new(10, 550, 780, 40, "")));

    let toggle_button = Rc::new(RefCell::new(Button::new(10, 500, 200, 40, "Encode")));

    let encode_mode = Rc::new(RefCell::new(true));

    // You need to clone outside of the closure to avoid moving the original Rc into the closure
    let toggle_button_clone = toggle_button.clone();
    let encode_mode_clone = encode_mode.clone();
    let cover_input_clone = cover_input.clone();
    let secret_input_clone = secret_input.clone();
    let output_display_clone = output_display.clone();
    let compare_output_clone = compare_output.clone();

    toggle_button.borrow_mut().set_callback({
        let encode_mode_clone = encode_mode.clone();
        let cover_input_clone = cover_input.clone();
        let secret_input_clone = secret_input.clone();
        let output_display_clone = output_display.clone();
        let compare_output_clone = compare_output.clone();

        move |_| {
            let mut mode = encode_mode_clone.borrow_mut();
            *mode = !*mode;
            let label = if *mode { "Encode On" } else { "Decode Off" };
            toggle_button_clone.borrow_mut().set_label(label);

            // Call the update_output function with dereferenced mode
            // and borrow the Rc<RefCell<>> here inside the closure
            update_output(
                &cover_input_clone.borrow(),
                &secret_input_clone.borrow(),
                output_display_clone.clone(),
                compare_output_clone.clone(),
                *mode,
            );
        }
    });

    win.end();
    win.show();

    while app.wait() {
        if receiver.recv().is_some() {
            app.quit();
        }
    }
}

fn update_output(
    cover_input: &MultilineInput,
    secret_input: &MultilineInput,
    output_display: Rc<RefCell<MultilineInput>>,
    compare_output: Rc<RefCell<Frame>>,
    encode_mode: bool,
) {
    let cover_text_val = cover_input.value();
    let secret_text_val = secret_input.value();
    let cs = "ABCDEFGHIJKLMNOPQRSTUVWXYZ"; // This will be your character set.

    if encode_mode {
        // Encode mode
        match encode(&cover_text_val) {
            Ok(encoded) => {
                let encoded_str = encoded
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
                    .join(",");
                output_display.borrow_mut().set_value(&encoded_str);
            }
            Err(_) => {
                output_display
                    .borrow_mut()
                    .set_value("Error encoding the text.");
            }
        }
    } else {
        // Decode mode
        match decode(
            &secret_text_val
                .split_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect::<Vec<_>>(),
            cs,
        ) {
            Ok(decoded) => {
                output_display.borrow_mut().set_value(&decoded);
            }
            Err(_) => {
                output_display
                    .borrow_mut()
                    .set_value("Error decoding the text.");
            }
        }
    }

    // Update compare output
    match compare(&secret_text_val, &cover_text_val, cs) {
        Ok(changes) => {
            let changes_str = changes
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(", ");
            compare_output
                .borrow_mut()
                .set_label(&format!("Changes: {}", changes_str));
        }
        Err(e) => {
            compare_output
                .borrow_mut()
                .set_label(&format!("Comparison error: {}", e));
        }
    }
}
