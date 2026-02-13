/// Valid state transitions for the job lifecycle FSM.
/// Lead → Estimated → Approved → Scheduled → EnRoute → InProgress → Paused → Completed → Invoiced → Paid → Closed
pub fn is_valid_transition(from: &str, to: &str) -> bool {
    matches!(
        (from, to),
        ("lead", "estimated")
            | ("lead", "scheduled")
            | ("lead", "cancelled")
            | ("estimated", "approved")
            | ("estimated", "declined")
            | ("estimated", "cancelled")
            | ("approved", "scheduled")
            | ("approved", "cancelled")
            | ("scheduled", "en_route")
            | ("scheduled", "in_progress")
            | ("scheduled", "cancelled")
            | ("en_route", "in_progress")
            | ("en_route", "scheduled") // re-route
            | ("in_progress", "paused")
            | ("in_progress", "completed")
            | ("paused", "in_progress")
            | ("paused", "cancelled")
            | ("completed", "invoiced")
            | ("invoiced", "paid")
            | ("invoiced", "completed") // undo
            | ("paid", "closed")
            | ("closed", "lead") // callback creates new linked job
    )
}

/// Side effects triggered by status transitions
pub fn get_transition_side_effects(from: &str, to: &str) -> Vec<&'static str> {
    match (from, to) {
        ("approved", "scheduled") => vec!["send_confirmation", "add_to_calendar"],
        ("scheduled", "en_route") => vec!["start_gps_tracking", "send_customer_eta"],
        ("en_route", "in_progress") => vec!["stop_navigation", "start_time_tracking", "geofence_checkin"],
        ("in_progress", "paused") => vec!["pause_time_tracking"],
        ("paused", "in_progress") => vec!["resume_time_tracking"],
        ("in_progress", "completed") => vec!["stop_time_tracking", "prompt_completion_photos", "generate_invoice_draft"],
        ("completed", "invoiced") => vec!["create_invoice", "send_invoice"],
        ("invoiced", "paid") => vec!["record_payment", "send_receipt", "schedule_review_request"],
        _ => vec![],
    }
}
