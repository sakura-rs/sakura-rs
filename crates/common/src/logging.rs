pub fn init(level: tracing::Level) {
    println!("  __  __                   _ _           _____   _____ \n |  \\/  |                 (_) |         |  __ \\ / ____|\n | \\  / | __ ___   ___   _ _| | ____ _  | |__) | (___  \n | |\\/| |/ _` \\ \\ / / | | | | |/ / _` | |  _  / \\___ \\ \n | |  | | (_| |\\ V /| |_| | |   < (_| | | | \\ \\ ____) |\n |_|  |_|\\__,_| \\_/  \\__,_|_|_|\\_\\__,_| |_|  \\_\\_____/ \n                                                       ");

    tracing_subscriber::fmt()
        .with_max_level(level)
        .without_time()
        .with_target(false)
        .init();
}
