fn create_scene() -> glane::Scene {
    let ((mut scene, _), left, right) = {
        let (root, left, right) = glane::widgets::VerticalPanes::new(
            glane::widgets::Column::new(),
            glane::widgets::Column::new(),
            0.5,
        );
        (glane::Scene::new(root), left, right)
    };
    let row_button = scene.push_child(&left, glane::widgets::Row::new());
    scene.push_child(&row_button, glane::widgets::Label::new("Button"));
    scene.push_child(&row_button, glane::widgets::Button::new("Push"));
    let row_text_box = scene.push_child(&left, glane::widgets::Row::new());
    scene.push_child(&row_text_box, glane::widgets::Label::new("TextBox"));
    scene.push_child(&row_text_box, glane::widgets::TextBox::new());
    let row_scroll_bar = scene.push_child(&left, glane::widgets::Row::new());
    scene.push_child(&row_scroll_bar, glane::widgets::Label::new("ScrollBar"));
    let scroll_bar = glane::widgets::ScrollBar::new(100, 10);
    scene.push_child(
        &row_scroll_bar,
        glane::widgets::MaxSize::new(None, Some(200.0), scroll_bar),
    );
    let scroll_bar = glane::widgets::ScrollBar::new(1000, 10);
    scene.push_child(
        &row_scroll_bar,
        glane::widgets::MaxSize::new(None, Some(200.0), scroll_bar),
    );
    let row_slider = scene.push_child(&left, glane::widgets::Row::new());
    scene.push_child(&row_slider, glane::widgets::Label::new("Slider"));
    let slider = glane::widgets::Slider::new();
    scene.push_child(
        &row_slider,
        glane::widgets::MaxSize::new(Some(150.0), None, slider),
    );
    let row_dropdown_box = scene.push_child(&right, glane::widgets::Row::new());
    scene.push_child(&row_dropdown_box, glane::widgets::Label::new("DropdownBox"));
    let dropdown_box = scene.push_child(&row_dropdown_box, glane::widgets::DropdownBox::new());
    for c in 'A'..='C' {
        scene.push_child(&dropdown_box, glane::widgets::Text::new(c.to_string()));
    }
    let row_list_box = scene.push_child(&right, glane::widgets::Row::new());
    scene.push_child(&row_list_box, glane::widgets::Label::new("ListBox"));
    let list_box = {
        let list_box = glane::widgets::ListBox::new();
        let handle = glane::Handle::new(&list_box);
        scene.push_child(
            &row_list_box,
            glane::widgets::MaxSize::new(None, Some(200.0), list_box),
        );
        handle
    };
    for c in 'a'..='z' {
        scene.push_child(&list_box, glane::widgets::Text::new(c.to_string()));
    }
    scene
}

#[divan::bench]
fn input(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| {
            let mouse_input = glane::Input::MouseInput(glane::MouseInput {
                button: glane::MouseButton::Left,
                button_state: glane::ButtonState::Pressed,
                mouse_state: glane::MouseState {
                    position: (0.0, 0.0).into(),
                    buttons: [glane::MouseButton::Left].into(),
                },
            });
            (create_scene(), mouse_input)
        })
        .bench_values(|(mut scene, input)| {
            let mut events = glane::Events::new();
            scene.input(input, &mut events);
        });
}

#[divan::bench]
fn layout(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| {
            let mouse_input = glane::Input::MouseInput(glane::MouseInput {
                button: glane::MouseButton::Left,
                button_state: glane::ButtonState::Pressed,
                mouse_state: glane::MouseState {
                    position: (0.0, 0.0).into(),
                    buttons: [glane::MouseButton::Left].into(),
                },
            });
            (create_scene(), mouse_input)
        })
        .bench_values(|(mut scene, input)| {
            let mut events = glane::Events::new();
            scene.input(input, &mut events);
            divan::black_box(scene.layout());
        });
}

fn main() {
    divan::main();
}
