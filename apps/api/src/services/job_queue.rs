use std::sync::Arc;
use tokio::sync::mpsc;
use tracing;

use crate::AppState;

/// Background job types that can be queued for async processing
#[derive(Debug, Clone)]
pub enum BackgroundJob {
    SendEmail {
        to: String,
        subject: String,
        body: String,
    },
    SendSms {
        to: String,
        body: String,
    },
    GenerateInvoicePdf {
        invoice_id: uuid::Uuid,
    },
    ProcessPhoto {
        photo_id: uuid::Uuid,
    },
    SendPushNotification {
        user_id: uuid::Uuid,
        title: String,
        body: String,
    },
    SyncQuickBooks {
        team_id: uuid::Uuid,
        entity_type: String,
        entity_id: uuid::Uuid,
    },
    RequestReview {
        customer_id: uuid::Uuid,
        job_id: uuid::Uuid,
    },
    ScheduleReminder {
        job_id: uuid::Uuid,
        remind_at: chrono::DateTime<chrono::Utc>,
    },
    RecurringJobGeneration {
        recurring_rule_id: uuid::Uuid,
    },
}

/// Job queue backed by a Tokio mpsc channel
pub struct JobQueue {
    sender: mpsc::UnboundedSender<BackgroundJob>,
}

impl JobQueue {
    pub fn new(state: Arc<AppState>) -> Self {
        let (tx, rx) = mpsc::unbounded_channel();
        tokio::spawn(process_jobs(rx, state));
        Self { sender: tx }
    }

    pub fn enqueue(&self, job: BackgroundJob) -> Result<(), String> {
        self.sender
            .send(job)
            .map_err(|e| format!("Failed to enqueue job: {}", e))
    }
}

async fn process_jobs(mut rx: mpsc::UnboundedReceiver<BackgroundJob>, state: Arc<AppState>) {
    tracing::info!("Background job processor started");

    while let Some(job) = rx.recv().await {
        let state = state.clone();
        tokio::spawn(async move {
            if let Err(e) = execute_job(&job, &state).await {
                tracing::error!(?job, error = %e, "Background job failed");
                // TODO: implement retry with exponential backoff
                // TODO: move to dead letter queue after max retries
            }
        });
    }

    tracing::warn!("Background job processor stopped");
}

async fn execute_job(job: &BackgroundJob, _state: &AppState) -> Result<(), String> {
    match job {
        BackgroundJob::SendEmail { to, subject, .. } => {
            tracing::info!(to = %to, subject = %subject, "Sending email");
            // TODO: integrate with SendGrid
            Ok(())
        }
        BackgroundJob::SendSms { to, .. } => {
            tracing::info!(to = %to, "Sending SMS");
            // TODO: integrate with Twilio
            Ok(())
        }
        BackgroundJob::GenerateInvoicePdf { invoice_id } => {
            tracing::info!(invoice_id = %invoice_id, "Generating invoice PDF");
            // TODO: generate PDF with WeasyPrint or similar
            Ok(())
        }
        BackgroundJob::ProcessPhoto { photo_id } => {
            tracing::info!(photo_id = %photo_id, "Processing photo (resize, watermark, thumbnail)");
            // TODO: process with image crate
            Ok(())
        }
        BackgroundJob::SendPushNotification { user_id, title, .. } => {
            tracing::info!(user_id = %user_id, title = %title, "Sending push notification");
            // TODO: integrate with Firebase Cloud Messaging
            Ok(())
        }
        BackgroundJob::SyncQuickBooks { team_id, entity_type, entity_id } => {
            tracing::info!(team_id = %team_id, entity_type = %entity_type, entity_id = %entity_id, "Syncing to QuickBooks");
            // TODO: integrate with QuickBooks Online API
            Ok(())
        }
        BackgroundJob::RequestReview { customer_id, job_id } => {
            tracing::info!(customer_id = %customer_id, job_id = %job_id, "Requesting review from customer");
            // TODO: send review request SMS/email
            Ok(())
        }
        BackgroundJob::ScheduleReminder { job_id, remind_at } => {
            tracing::info!(job_id = %job_id, remind_at = %remind_at, "Scheduling reminder");
            // TODO: schedule delayed task
            Ok(())
        }
        BackgroundJob::RecurringJobGeneration { recurring_rule_id } => {
            tracing::info!(recurring_rule_id = %recurring_rule_id, "Generating recurring job instances");
            // TODO: generate next job instances from RRULE
            Ok(())
        }
    }
}
