use std::collections::HashMap;

use crossterm::{
    event::{Event, EventStream, KeyCode, KeyEvent},
    Result,
};
use futures::{select, FutureExt, StreamExt};

use crate::commands::EditorCommand;

// TODO: Make this a trait to handle events
// TODO: Take a `KeyEvent` object as map key
/// Manage key bindings
struct KeyMap {
    map: HashMap<char, EditorCommand>,
}

impl KeyMap {
    fn new() -> Self {
        Self {
            map: HashMap::from([
                ('q', EditorCommand::Stop),
                ('k', EditorCommand::ScrollUp),
                ('j', EditorCommand::ScrollDown),
            ]),
        }
    }

    /// Process `event` given the current keymappings
    fn event(&self, event: KeyEvent) -> EditorCommand {
        match event {
            KeyEvent {
                code: KeyCode::Char(c),
                ..
            } => self.map.get(&c).cloned().unwrap_or(EditorCommand::Continue),
            _ => EditorCommand::Continue,
        }
    }
}

// TODO: Make this run in a separate thread, or at least run fully async with command processor
pub async fn handle_input() -> Result<()> {
    let mut reader = EventStream::new();
    let keys = KeyMap::new();

    Ok(loop {
        let mut event = reader.next().fuse();

        let test = select! {
            maybe_event = event => {
                match maybe_event {
                    Some(Ok(event)) => {
                        match event {
                            Event::Key(event) => keys.event(event),
                            _ => EditorCommand::Continue
                        }
                    },
                    _ => EditorCommand::Continue
                }
            },
        };
        match test {
            EditorCommand::Continue => (),
            EditorCommand::Stop => break,
            // TODO: Implement command handler
            _ => (),
        }
    })
}
