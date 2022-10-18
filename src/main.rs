use std::sync::Arc;

use druid::{PlatformError, Widget, widget::{
    Flex, RadioGroup, BackgroundBrush, Checkbox
}, AppLauncher, WindowDesc, Data, Color, WidgetExt};

#[derive(Data, Clone, PartialEq)]
struct ApplicationData {
    checked: Arc<Checked>
}

#[derive(Data, Clone, PartialEq)]
enum Checked {
    True,
    False
}

// TODO: Dont know what do to with that. Remove if it is not used.

/*impl From<bool> for Checked {
    fn from(this: bool) -> Self {
        match this {
            true => Self::True,
            false => Self::False,
        }
    }
}

impl From<Checked> for bool {
    fn from(this: Checked) -> Self {
        match this {
            True => true,
            False => false,
        }
    }
}*/

fn main() -> Result<(), PlatformError> {

    let data: ApplicationData = ApplicationData {
        checked: Arc::new(Checked::False)
    };

    let _checked = Checked::True;

    AppLauncher::with_window(WindowDesc::new(build_ui()).title("Hello World"))
        .launch(data)?;
        Ok(())
    }


fn build_ui() -> impl Widget<ApplicationData> {
    
    let background: BackgroundBrush<ApplicationData> = BackgroundBrush::Color(Color::grey(0.7));
    let radio_group = RadioGroup::column(vec!(("Hello World", ApplicationData {checked: Arc::new(Checked::False)}), ("Hello World2", ApplicationData {checked: Arc::new(Checked::True)})));

    // TODO: Get _checkboxes into the Flex layout.
    let _checkboxes: Checkbox = Checkbox::new("Hello World");

    Flex::column()
        .with_child(radio_group)
        .background(background)
}