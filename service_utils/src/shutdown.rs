// https://stackoverflow.com/questions/77585473/rust-tokio-how-to-handle-more-signals-than-just-sigint-i-e-sigquit?noredirect=1#comment136778587_77585473

/// Registers signal handlers and waits for a signal that indicates a shutdown request.
pub async fn wait_for_signal() {
    wait_for_signal_impl().await
}

/// Waits for a signal that requests a graceful shutdown, like SIGTERM, SIGINT (Ctrl-C), or SIGQUIT.
#[cfg(unix)]
async fn wait_for_signal_impl() {
    use tokio::signal::unix::{signal, SignalKind};

    // Docs: https://www.gnu.org/software/libc/manual/html_node/Termination-Signals.html
    let mut signal_terminate = signal(SignalKind::terminate()).unwrap();
    let mut signal_interrupt = signal(SignalKind::interrupt()).unwrap();
    let mut signal_quit = signal(SignalKind::quit()).unwrap();
    let mut signal_hang = signal(SignalKind::hangup()).unwrap();

    // https://docs.rs/tokio/latest/tokio/macro.select.html
    tokio::select! {
        _ = signal_terminate.recv() => println!("* Received SIGTERM"),
        _ = signal_interrupt.recv() => println!("* Received SIGINT"),
        _ = signal_quit.recv() => println!("* Received SIGQUIT"),
        _ = signal_hang.recv() => println!(" * Received SIGHUP"),
    }
}

/// Waits for a signal that requests a graceful shutdown, Ctrl-C (SIGINT).
#[cfg(windows)]
async fn wait_for_signal_impl() {
    use tokio::signal::windows;

    // Docs: https://learn.microsoft.com/en-us/windows/console/handlerroutine
    let mut signal_c = windows::ctrl_c().unwrap();
    let mut signal_break = windows::ctrl_break().unwrap();
    let mut signal_close = windows::ctrl_close().unwrap();
    let mut signal_shutdown = windows::ctrl_shutdown().unwrap();

    // https://docs.rs/tokio/latest/tokio/macro.select.html
    tokio::select! {
        _ = signal_c.recv() => println!("Received CTRL_C."),
        _ = signal_break.recv() => println!("Received CTRL_BREAK."),
        _ = signal_close.recv() => println!("Received CTRL_CLOSE."),
        _ = signal_shutdown.recv() => println!("Received CTRL_SHUTDOWN."),
    }
}
