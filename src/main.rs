#![allow(non_snake_case)]

use gtk4 as gtk;
use gtk::prelude::*;

pub mod rom;
pub mod cpu;

fn main()
{
    let mut _cpu: cpu::CPU = Default::default();
    _cpu.qm_x = 0xDE;
    _cpu.exec(&[0xDE, 0xAD]);
    _cpu.dump_qms();
}

// fn main() -> gtk::glib::ExitCode {
//     let app: gtk::Application = gtk::Application::builder().application_id("org.opencollectives.pirovm").build(); 
//     app.connect_activate(|app: &gtk::Application| {
//         let window: gtk::ApplicationWindow = 
//                     gtk::ApplicationWindow::builder()
//                     .application(app)
//                     .default_width(600)
//                     .default_height(400)
//                     .title("PiroVM")
//                     .build();
//         
//         let button: gtk::Button = gtk::Button::with_label("Click");
//         button.connect_clicked(|_| {
//             eprintln!("Clicked");
//         });
//         window.set_child(Some(&button));
//         window.show();
//     });
//     app.run()
// }
