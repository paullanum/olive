use std::collections::HashMap;

use crossterm::event::{Event, EventStream, KeyCode, KeyEvent, KeyModifiers};
use futures::{
    channel::mpsc::{TrySendError, UnboundedSender},
    FutureExt, StreamExt,
};

use crate::commands::EditorCommand;

/// Manage key bindings
struct KeyMap {
    map: HashMap<(KeyCode, KeyModifiers), EditorCommand>,
}

trait Mapping {
    fn event(&self, event: KeyEvent) -> EditorCommand;
}

const DEFAULTS: &[(char, EditorCommand)] = &[
    ('q', EditorCommand::Stop),
    ('k', EditorCommand::ScrollUp { count: 1 }),
    ('j', EditorCommand::ScrollDown { count: 1 }),
];

impl KeyMap {
    fn new() -> Self {
        Self {
            map: DEFAULTS
                .iter()
                .map(|tup| ((KeyCode::Char(tup.0), KeyModifiers::NONE), tup.1.clone()))
                .collect(),
        }
    }
}

impl Mapping for KeyMap {
    /// Process `event` given the current keymappings
    fn event(&self, event: KeyEvent) -> EditorCommand {
        let KeyEvent {
            code, modifiers, ..
        } = event;
        self.map
            .get(&(code, modifiers))
            .cloned()
            .unwrap_or(EditorCommand::Continue)
    }
}

pub struct InputHandler {
    keys: KeyMap,
    tx: UnboundedSender<EditorCommand>,
}

impl InputHandler {
    pub(crate) fn new(tx: UnboundedSender<EditorCommand>) -> Self {
        Self {
            keys: KeyMap::new(),
            tx,
        }
    }

    pub(crate) async fn handle(&self) -> Result<(), TrySendError<EditorCommand>> {
        let mut reader = EventStream::new();

        loop {
            let test = match reader.next().fuse().await {
                Some(Ok(Event::Key(event))) => self.keys.event(event),
                _ => EditorCommand::Continue,
            };
            self.tx.unbounded_send(test)?;
        }
    }
}
