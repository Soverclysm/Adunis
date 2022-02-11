// Macro to generate code to create buttons and associate function to be performed on button click
//
// Example:
// button_clicked!(
//     ui,
//     "Open" => { button.open() },
//     "Close" => { button.close() },
// )
//
// Expanded:
// if ui.button("Open").clicked() {
//     button.open();
// }
// if ui.button("Close").clicked() {
//     button.close();
// }
macro_rules! button_clicked {
    ( $ui:expr, $($name:expr => $action:block),+, ) => {{
        $(
            if $ui.button($name).clicked() {
                $action;
            }
        )+
    }};
}

pub(crate) use button_clicked;
