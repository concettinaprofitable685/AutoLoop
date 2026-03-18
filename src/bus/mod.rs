use std::sync::Arc;

use tokio::sync::{Mutex, mpsc};

#[derive(Debug, Clone)]
pub struct InboundMessage {
    pub session_id: String,
    pub content: String,
}

#[derive(Debug, Clone)]
pub struct OutboundMessage {
    pub session_id: String,
    pub content: String,
}

#[derive(Clone)]
pub struct MessageBus {
    inbound_tx: mpsc::Sender<InboundMessage>,
    inbound_rx: Arc<Mutex<mpsc::Receiver<InboundMessage>>>,
    outbound_tx: mpsc::Sender<OutboundMessage>,
    outbound_rx: Arc<Mutex<mpsc::Receiver<OutboundMessage>>>,
}

impl Default for MessageBus {
    fn default() -> Self {
        let (inbound_tx, inbound_rx) = mpsc::channel(256);
        let (outbound_tx, outbound_rx) = mpsc::channel(256);

        Self {
            inbound_tx,
            inbound_rx: Arc::new(Mutex::new(inbound_rx)),
            outbound_tx,
            outbound_rx: Arc::new(Mutex::new(outbound_rx)),
        }
    }
}

impl MessageBus {
    pub async fn publish_inbound(&self, message: InboundMessage) -> Result<(), mpsc::error::SendError<InboundMessage>> {
        self.inbound_tx.send(message).await
    }

    pub async fn consume_inbound(&self) -> Option<InboundMessage> {
        self.inbound_rx.lock().await.recv().await
    }

    pub async fn publish_outbound(&self, message: OutboundMessage) -> Result<(), mpsc::error::SendError<OutboundMessage>> {
        self.outbound_tx.send(message).await
    }

    pub async fn consume_outbound(&self) -> Option<OutboundMessage> {
        self.outbound_rx.lock().await.recv().await
    }
}

