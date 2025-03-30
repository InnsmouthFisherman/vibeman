use comandline_processor::ComandlineProcessor;
use player::Player;
use druid::widget::{Button, Flex, Label};
use druid::{AppLauncher, Widget, WidgetExt, WindowDesc};

mod comandline_processor;
mod player;

fn main() {
    let mut cmd = ComandlineProcessor::new();
    cmd.process();
    
    let track_path = cmd.locate_directories();
    
    let player = Player::new(track_path);

    let main_window = WindowDesc::new(build_ui())
        .title("Музыкальный плеер")
        .window_size((400.0, 200.0));
    AppLauncher::with_window(main_window)
        .launch(player)
        .expect("Не удалось запустить приложение");
}

fn build_ui() -> impl Widget<Player> {

    let track_label = Label::dynamic(|data: &Player, _env| {
        format!("Сейчас играет: {}", data.current_track.display())
    })
    .padding(10.0);

    let play_pause_button = Button::dynamic(|data: &Player, _env| {
        if data.is_playing {
            "Пауза".to_string()
        } else {
            "Воспроизведение".to_string()
        }
    })
    .on_click(|_ctx, data: &mut Player, _env| {
        data.toggle_playback(); 
        println!("Состояние: {}", data.is_playing);
    });

    let stop_button = Button::new("Стоп")
        .on_click(|_ctx, data: &mut Player, _env| {
            data.stop();
            println!("Воспроизведение остановлено");
        })
        .padding(10.0);

    Flex::column()
        .with_child(track_label)
        .with_child(play_pause_button)
        .with_child(stop_button)
        .padding(20.0)
}
