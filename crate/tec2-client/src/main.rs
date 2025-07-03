use tec2_client::App;

use tec2_client::ui::Tec2ClientRouter;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();

    let result = App::new::<Tec2ClientRouter>().run(terminal);
    ratatui::restore();
    result
}

