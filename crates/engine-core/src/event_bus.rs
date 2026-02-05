//! Event bus: distribución de eventos globales

use crate::{SharedState, StateEvent};
use tokio::sync::broadcast::Receiver;

/// Suscriptor a eventos globales
pub struct EventSubscriber {
    rx: Receiver<StateEvent>,
}

impl EventSubscriber {
    /// Esperar próximo evento
    pub async fn next(&mut self) -> Option<StateEvent> {
        self.rx.recv().await.ok()
    }
}

/// Crear nuevo suscriptor de eventos
pub fn subscribe(state: &SharedState) -> EventSubscriber {
    let rx = state.event_tx.subscribe();
    EventSubscriber { rx }
}

/// Emitir evento
pub fn emit(state: &SharedState, event: StateEvent) -> Result<(), tokio::sync::broadcast::error::SendError<StateEvent>> {
    state.event_tx.send(event).map(|_| ())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[tokio::test]
    async fn test_event_subscription() {
        let config = Config {
            version: "1.0".to_string(),
            widgets: vec![],
            styles: Default::default(),
            displays: vec![],
            global: Default::default(),
        };

        let state = crate::init(config).unwrap();
        
        // Crear suscriptor
        let mut sub = subscribe(&state);

        // Emitir evento en background
        let state_clone = state.clone();
        tokio::spawn(async move {
            tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
            let _ = emit(
                &state_clone,
                StateEvent::MonitorsChanged { monitors: vec![] },
            );
        });

        // Esperar evento
        let event = tokio::time::timeout(
            tokio::time::Duration::from_secs(1),
            sub.next(),
        )
        .await;

        assert!(event.is_ok());
    }
}
