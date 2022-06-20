use glib::clone;
use gtk::{gio, glib};
use gtk::{prelude::*, Align};
use gtk::{Application,ApplicationWindow, Button, Label, Orientation};

use std::env;
use std::sync::{Arc, Mutex};

use crate::config::{APP_ID,GETTEXT_PACKAGE};
use crate::ihm::ihm_utilits;

pub fn ui() {
    // Create a new application
    let app = Application::new(Some(APP_ID), Default::default());

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    let mut args: Vec<String> = env::args().collect();
    let size = args.len();
    for _i in 1..size {
        args.swap_remove(1);
    }

    println!("{:?}", args);

    // Run the application
    app.run_with_args(&args);
}

fn build_ui(app: &Application) {
    let original_state = 0;
    let label = Label::builder()
        .label(&format!("Counter: {}", original_state))
        .build();
    // Create a button with label and action
    let button_incr = Button::builder()
        .label("+")
        .build();
    let button_decr = Button::builder()
        .label("-")
        .build();
    let button_reset = Button::builder()
        .label("Reset")
        .build();

    let gtk_box_sub = gtk::Box::builder()
        .orientation(Orientation::Horizontal)
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .spacing(12)
        .halign(Align::Center)
        .build();

    gtk_box_sub.append(&button_incr);
    gtk_box_sub.append(&button_decr);

    // Create `gtk_box` and add `button` and `label` to it
    let gtk_box = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .spacing(12)
        .halign(Align::Center)
        .build();
    gtk_box.append(&gtk_box_sub);
    gtk_box.append(&label);
    gtk_box.append(&button_reset);

    // Create a window and set the title
    let window = ApplicationWindow::builder()
        .application(app)
        .title(GETTEXT_PACKAGE)
        .width_request(360)
        .child(&gtk_box)
        .build();

    let counter = Arc::new(Mutex::new(0));


    button_incr.connect_clicked(clone!(@strong label, @strong counter => move |_| {
        counter_hange(counter.clone(), 1, label.clone())
    }));
    button_decr.connect_clicked(clone!(@strong label, @strong counter => move |_| {
        counter_hange(counter.clone(), -1, label.clone())
    }));
    button_reset.connect_clicked(clone!(@strong label, @strong counter => move |_| {
        counter_hange(counter.clone(), 0, label.clone())
    }));

    // Present window
    window.present();
}

fn counter_hange(mut count: Arc<Mutex<isize>>, add: isize , change_lable: Label) {

    let mut counter = count.lock().unwrap();
    let old = *counter;

    if add != 0 {
        *counter += add;
    } else {
        *counter = 0;
    }

    change_lable.set_label(&counter.to_string());

    ihm_utilits::info_col(1, &format!("value Counter {} -> {} ", old, counter));


}
