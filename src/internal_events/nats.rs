use crate::emit;
use metrics::counter;
use vector_common::internal_event::{
    error_stage, error_type, ComponentEventsDropped, UNINTENTIONAL,
};
use vector_core::internal_event::InternalEvent;

#[derive(Debug)]
pub struct NatsEventSendError {
    pub error: async_nats::Error,
}

impl InternalEvent for NatsEventSendError {
    fn emit(self) {
        let reason = "Failed to send message.";
        error!(
            message = reason,
            error = %self.error,
            error_type = error_type::WRITER_FAILED,
            stage = error_stage::SENDING,
            internal_log_rate_limit = true,
        );
        counter!(
            "component_errors_total", 1,
            "error_type" => error_type::WRITER_FAILED,
            "stage" => error_stage::SENDING,
        );
        emit!(ComponentEventsDropped::<UNINTENTIONAL> { count: 1, reason });

        // deprecated
        counter!("send_errors_total", 1);
    }
}
