async fn run() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::try_init()?;

    let yaml = clap::load_yaml!("../cli.yaml");
    let app = clap::App::from_yaml(&yaml)
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .name(clap::crate_name!())
        .setting(clap::AppSettings::ArgRequiredElseHelp);
    let matches = app.get_matches();

    if let Some(input) = matches.value_of("input") {
        log::info!("input file: {}", input);

        let (width, height) = image::image_dimensions(input)?;
        println!("{} {}", width, height);
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    match run().await {
        Ok(_) => {}
        Err(error) => log::error!("{:?}", error),
    }
}
