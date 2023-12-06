/// Registers a signal handler that waits for a signal that indicates a shutdown request.
pub async fn signal_handler(svc: &str) {
    wait_for_signal_impl(svc).await
}

/// Waits for a signal that requests a graceful shutdown, like SIGTERM, SIGINT (Ctrl-C), or SIGQUIT.
// https://stackoverflow.com/questions/77585473/rust-tokio-how-to-handle-more-signals-than-just-sigint-i-e-sigquit?noredirect=1#comment136778587_77585473
#[cfg(unix)]
async fn wait_for_signal_impl(svc: &str) {
    use tokio::signal::unix::{signal, SignalKind};

    // Docs: https://www.gnu.org/software/libc/manual/html_node/Termination-Signals.html
    let mut signal_terminate = signal(SignalKind::terminate()).unwrap();
    let mut signal_interrupt = signal(SignalKind::interrupt()).unwrap();
    let mut signal_quit = signal(SignalKind::quit()).unwrap();
    let mut signal_hang = signal(SignalKind::hangup()).unwrap();

    // https://docs.rs/tokio/latest/tokio/macro.select.html
    tokio::select! {
        _ = signal_terminate.recv() => println!("* {svc} received SIGTERM"),
        _ = signal_interrupt.recv() => println!("* {svc} received SIGINT"),
        _ = signal_quit.recv() => println!("* {svc} received SIGQUIT"),
        _ = signal_hang.recv() => println!(" * {svc} received SIGHUP"),
    }
}

/// Waits for a signal that requests a graceful shutdown, like SIGTERM, SIGINT (Ctrl-C), or SIGQUIT.
#[cfg(windows)]
async fn wait_for_signal_impl(svc: &str) {
    use tokio::signal::windows;

    // Docs: https://learn.microsoft.com/en-us/windows/console/handlerroutine
    let mut signal_c = windows::ctrl_c().unwrap();
    let mut signal_break = windows::ctrl_break().unwrap();
    let mut signal_close = windows::ctrl_close().unwrap();
    let mut signal_shutdown = windows::ctrl_shutdown().unwrap();

    // https://docs.rs/tokio/latest/tokio/macro.select.html
    tokio::select! {
        _ = signal_c.recv() => println!("* {svc} received CTRL_C."),
        _ = signal_break.recv() => println!("* {svc} received CTRL_BREAK."),
        _ = signal_close.recv() => println!("* {svc} received CTRL_CLOSE."),
        _ = signal_shutdown.recv() => println!("* {svc} received CTRL_SHUTDOWN."),
    }
}
