use futures::{channel::mpsc::UnboundedReceiver, StreamExt};

#[derive(Clone)]
/// The list of all possible commands the editor can perform
///
/// Currently very limitted - this will likely always be
/// non-exhaustive though for backwards compatibility purposes.
#[non_exhaustive]
pub(crate) enum EditorCommand {
    Continue,
    ScrollUp { count: usize },
    ScrollDown { count: usize },
    Stop,
}

pub(crate) struct CommandHandler {
    rx: UnboundedReceiver<EditorCommand>,
}

impl CommandHandler {
    pub(crate) fn new(rx: UnboundedReceiver<EditorCommand>) -> Self {
        Self { rx }
    }

    /// Constantly process incoming commands until one of them is `Stop`
    ///
    /// Note that this will only return once the command `Stop` has been
    /// sent to the handler's receiver. Even if no data has been received,
    /// this should only return when `Stop` is received.
    pub(crate) async fn handle(mut self) {
        loop {
            match self.rx.next().await {
                // This is the only case that handle should return
                Some(EditorCommand::Stop) => return,
                _ => (),
            }
        }
    }
}
