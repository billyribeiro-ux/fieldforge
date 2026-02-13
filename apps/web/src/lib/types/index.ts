// FieldForge TypeScript Types — mirrors Rust API models

// ── Common ──

export interface PaginationParams {
	cursor?: string;
	limit?: number;
}

export interface PaginatedResponse<T> {
	data: T[];
	meta: {
		has_more: boolean;
		cursor: string | null;
	};
	errors: null;
}

export interface ApiResponse<T> {
	data: T;
	meta: Record<string, unknown> | null;
	errors: null;
}

export type SyncStatus = 'synced' | 'pending' | 'conflict' | 'failed';

// ── User ──

export type UserRole = 'owner' | 'admin' | 'manager' | 'technician' | 'office_staff' | 'apprentice';

export interface User {
	id: string;
	team_id: string;
	email: string;
	first_name: string;
	last_name: string;
	phone: string | null;
	role: UserRole;
	avatar_url: string | null;
	hourly_rate: number | null;
	is_active: boolean;
	last_login_at: string | null;
	created_at: string;
	updated_at: string;
}

export interface AuthResponse {
	user: User;
	token: string;
	refresh_token: string;
}

// ── Team ──

export interface Team {
	id: string;
	name: string;
	slug: string;
	phone: string | null;
	email: string | null;
	website: string | null;
	logo_url: string | null;
	address_line1: string | null;
	address_line2: string | null;
	city: string | null;
	state: string | null;
	zip_code: string | null;
	country: string;
	timezone: string;
	tax_rate: number | null;
	default_hourly_rate: number | null;
	stripe_customer_id: string | null;
	subscription_plan: string;
	subscription_status: string;
	estimate_prefix: string;
	estimate_next_number: number;
	invoice_prefix: string;
	invoice_next_number: number;
	created_at: string;
	updated_at: string;
}

// ── Customer ──

export interface Customer {
	id: string;
	team_id: string;
	first_name: string;
	last_name: string;
	email: string | null;
	phone: string | null;
	phone_secondary: string | null;
	company_name: string | null;
	preferred_contact_method: string;
	referral_source: string | null;
	tags: string[];
	lifetime_value: number;
	outstanding_balance: number;
	total_jobs: number;
	notes: string | null;
	is_active: boolean;
	created_at: string;
	updated_at: string;
	deleted_at: string | null;
}

export interface CreateCustomerRequest {
	first_name: string;
	last_name: string;
	email?: string;
	phone?: string;
	phone_secondary?: string;
	company_name?: string;
	referral_source?: string;
	notes?: string;
}

export interface UpdateCustomerRequest extends Partial<CreateCustomerRequest> {}

// ── Property ──

export type PropertyType = 'residential' | 'commercial' | 'industrial' | 'government' | 'hoa';

export interface Property {
	id: string;
	team_id: string;
	customer_id: string;
	address_line1: string;
	address_line2: string | null;
	city: string;
	state: string;
	zip_code: string;
	country: string;
	latitude: number | null;
	longitude: number | null;
	property_type: PropertyType;
	square_footage: number | null;
	year_built: number | null;
	is_primary: boolean;
	access_instructions: string | null;
	gate_code: string | null;
	notes: string | null;
	created_at: string;
}

// ── Job ──

export type JobStatus =
	| 'lead' | 'estimated' | 'approved' | 'scheduled' | 'en_route'
	| 'in_progress' | 'paused' | 'completed' | 'invoiced' | 'paid'
	| 'cancelled' | 'on_hold' | 'declined' | 'closed';

export type JobPriority = 'low' | 'normal' | 'high' | 'emergency';

export type JobSource = 'phone' | 'email' | 'website' | 'referral' | 'walk_in' | 'social_media' | 'advertising' | 'repeat' | 'other';

export interface Job {
	id: string;
	team_id: string;
	customer_id: string;
	property_id: string | null;
	assigned_to: string | null;
	title: string;
	description: string | null;
	status: JobStatus;
	priority: JobPriority;
	trade: string | null;
	job_type: string | null;
	source: JobSource | null;
	po_number: string | null;
	scheduled_date: string | null;
	scheduled_start_time: string | null;
	scheduled_end_time: string | null;
	estimated_duration_minutes: number | null;
	actual_duration_minutes: number | null;
	budget: number | null;
	total: number | null;
	access_instructions: string | null;
	internal_notes: string | null;
	tags: string[];
	started_at: string | null;
	completed_at: string | null;
	created_at: string;
	updated_at: string;
	deleted_at: string | null;
}

export interface CreateJobRequest {
	customer_id: string;
	property_id?: string;
	assigned_to?: string;
	title: string;
	description?: string;
	priority?: JobPriority;
	trade?: string;
	job_type?: string;
	source?: JobSource;
	scheduled_date?: string;
	scheduled_start_time?: string;
	estimated_duration_minutes?: number;
	budget?: number;
	access_instructions?: string;
	internal_notes?: string;
	tags?: string[];
}

export interface JobStatusTransition {
	to_status: JobStatus;
	notes?: string;
}

// ── Estimate ──

export type EstimateStatus = 'draft' | 'sent' | 'viewed' | 'approved' | 'declined' | 'expired' | 'converted';

export interface Estimate {
	id: string;
	team_id: string;
	customer_id: string;
	job_id: string | null;
	property_id: string | null;
	estimate_number: string;
	title: string | null;
	scope_of_work: string | null;
	status: EstimateStatus;
	subtotal: number;
	discount_amount: number;
	discount_pct: number | null;
	tax_amount: number;
	tax_rate: number;
	total: number;
	valid_until: string | null;
	sent_at: string | null;
	viewed_at: string | null;
	approved_at: string | null;
	declined_at: string | null;
	customer_signature: string | null;
	decline_reason: string | null;
	payment_terms: string | null;
	warranty_terms: string | null;
	created_at: string;
	updated_at: string;
	deleted_at: string | null;
}

export interface LineItemInput {
	description: string;
	category?: string;
	quantity: number;
	unit?: string;
	unit_price: number;
	taxable?: boolean;
	sort_order?: number;
}

export interface CreateEstimateRequest {
	customer_id: string;
	job_id?: string;
	property_id?: string;
	title?: string;
	scope_of_work?: string;
	valid_until?: string;
	discount_amount?: number;
	discount_pct?: number;
	payment_terms?: string;
	warranty_terms?: string;
	line_items: LineItemInput[];
}

// ── Line Item ──

export type LineItemCategory = 'labor' | 'materials' | 'equipment' | 'permits' | 'disposal' | 'subcontractor' | 'overhead' | 'other';

export interface LineItem {
	id: string;
	team_id: string;
	estimate_id: string | null;
	invoice_id: string | null;
	description: string;
	category: LineItemCategory;
	quantity: number;
	unit: string;
	unit_price: number;
	total: number;
	cost_price: number | null;
	taxable: boolean;
	sort_order: number;
	created_at: string;
}

// ── Invoice ──

export type InvoiceStatus = 'draft' | 'sent' | 'viewed' | 'partially_paid' | 'paid' | 'overdue' | 'void';

export interface Invoice {
	id: string;
	team_id: string;
	job_id: string | null;
	estimate_id: string | null;
	customer_id: string;
	property_id: string | null;
	invoice_number: string;
	status: InvoiceStatus;
	subtotal: number;
	discount_amount: number;
	tax_amount: number;
	tax_rate: number;
	total: number;
	amount_paid: number;
	amount_due: number;
	due_date: string | null;
	sent_at: string | null;
	viewed_at: string | null;
	paid_at: string | null;
	voided_at: string | null;
	payment_terms: string | null;
	notes: string | null;
	created_at: string;
	updated_at: string;
	deleted_at: string | null;
}

export interface CreateInvoiceRequest {
	customer_id: string;
	job_id?: string;
	estimate_id?: string;
	property_id?: string;
	due_date?: string;
	payment_terms?: string;
	notes?: string;
	line_items: LineItemInput[];
}

// ── Payment ──

export type PaymentMethod = 'cash' | 'check' | 'card' | 'ach' | 'venmo' | 'zelle' | 'apple_pay' | 'google_pay' | 'financing' | 'other';
export type PaymentStatus = 'pending' | 'processing' | 'succeeded' | 'failed' | 'refunded' | 'partially_refunded';

export interface Payment {
	id: string;
	team_id: string;
	invoice_id: string;
	customer_id: string;
	amount: number;
	tip_amount: number;
	net_amount: number;
	payment_method: PaymentMethod;
	status: PaymentStatus;
	stripe_payment_id: string | null;
	check_number: string | null;
	reference_number: string | null;
	notes: string | null;
	collected_by: string | null;
	collected_at: string;
	created_at: string;
}

export interface RecordPaymentRequest {
	amount: number;
	payment_method: PaymentMethod;
	tip_amount?: number;
	check_number?: string;
	reference_number?: string;
	notes?: string;
}

// ── Time Entry ──

export type TimeEntryType = 'work' | 'travel' | 'break' | 'admin';

export interface TimeEntry {
	id: string;
	team_id: string;
	job_id: string;
	user_id: string;
	entry_type: TimeEntryType;
	started_at: string;
	ended_at: string | null;
	duration_minutes: number | null;
	hourly_rate: number | null;
	total_cost: number | null;
	latitude_start: number | null;
	longitude_start: number | null;
	latitude_end: number | null;
	longitude_end: number | null;
	notes: string | null;
	created_at: string;
}

export interface StartTimerRequest {
	job_id: string;
	entry_type?: TimeEntryType;
	latitude?: number;
	longitude?: number;
}

export interface StopTimerRequest {
	latitude?: number;
	longitude?: number;
	notes?: string;
}

// ── Photo ──

export type PhotoCategory = 'before' | 'during' | 'after' | 'damage' | 'equipment' | 'receipt' | 'general';

export interface Photo {
	id: string;
	team_id: string;
	job_id: string | null;
	customer_id: string | null;
	uploaded_by: string;
	file_key: string;
	filename: string;
	content_type: string;
	file_size: number | null;
	original_url: string | null;
	thumbnail_url: string | null;
	category: PhotoCategory;
	caption: string | null;
	latitude: number | null;
	longitude: number | null;
	taken_at: string | null;
	sort_order: number;
	created_at: string;
	deleted_at: string | null;
}

// ── Note ──

export interface Note {
	id: string;
	team_id: string;
	job_id: string | null;
	customer_id: string | null;
	author_id: string;
	content: string;
	is_pinned: boolean;
	is_internal: boolean;
	created_at: string;
	updated_at: string;
}

// ── Checklist ──

export interface Checklist {
	id: string;
	team_id: string;
	job_id: string;
	title: string;
	items: ChecklistItem[];
	created_at: string;
}

export interface ChecklistItem {
	id: string;
	checklist_id: string;
	text: string;
	is_completed: boolean;
	completed_by: string | null;
	completed_at: string | null;
	sort_order: number;
}

// ── Tag ──

export interface Tag {
	id: string;
	team_id: string;
	name: string;
	color: string;
	created_at: string;
}

// ── Inventory ──

export interface InventoryItem {
	id: string;
	team_id: string;
	name: string;
	sku: string | null;
	description: string | null;
	category: string | null;
	quantity: number;
	min_quantity: number;
	unit: string;
	unit_cost: number | null;
	unit_price: number | null;
	location: string | null;
	supplier: string | null;
	reorder_point: number | null;
	is_active: boolean;
	created_at: string;
	updated_at: string;
}

// ── Vehicle ──

export interface Vehicle {
	id: string;
	team_id: string;
	name: string;
	make: string | null;
	model: string | null;
	year: number | null;
	vin: string | null;
	license_plate: string | null;
	assigned_to: string | null;
	status: 'active' | 'maintenance' | 'retired';
	mileage: number;
	fuel_type: string | null;
	insurance_policy: string | null;
	insurance_expiry: string | null;
	registration_expiry: string | null;
	next_service_date: string | null;
	notes: string | null;
	created_at: string;
	updated_at: string;
}

// ── Expense ──

export interface Expense {
	id: string;
	team_id: string;
	user_id: string | null;
	job_id: string | null;
	vehicle_id: string | null;
	category: string;
	description: string;
	amount: number;
	tax_amount: number | null;
	vendor: string | null;
	receipt_url: string | null;
	expense_date: string;
	is_billable: boolean;
	is_reimbursable: boolean;
	reimbursed: boolean;
	notes: string | null;
	created_at: string;
	updated_at: string;
}

export interface CreateExpenseRequest {
	category: string;
	description: string;
	amount: number;
	tax_amount?: number;
	vendor?: string;
	expense_date: string;
	job_id?: string;
	vehicle_id?: string;
	is_billable?: boolean;
	is_reimbursable?: boolean;
	notes?: string;
}

// ── Service Plan ──

export interface ServicePlan {
	id: string;
	team_id: string;
	name: string;
	description: string | null;
	price_monthly: number | null;
	price_quarterly: number | null;
	price_annual: number | null;
	visits_per_year: number;
	discount_pct: number | null;
	priority_scheduling: boolean;
	is_active: boolean;
	created_at: string;
	updated_at: string;
}

export interface CustomerServicePlan {
	id: string;
	service_plan_id: string;
	customer_id: string;
	team_id: string;
	billing_frequency: string;
	start_date: string;
	end_date: string | null;
	auto_renew: boolean;
	status: string;
	visits_used: number;
	created_at: string;
	updated_at: string;
}

// ── Message ──

export type MessageDirection = 'inbound' | 'outbound';
export type MessageChannel = 'sms' | 'email' | 'phone' | 'in_app';
export type MessageStatus = 'queued' | 'sent' | 'delivered' | 'failed' | 'received';

export interface Message {
	id: string;
	team_id: string;
	customer_id: string;
	job_id: string | null;
	user_id: string | null;
	direction: MessageDirection;
	channel: MessageChannel;
	status: MessageStatus;
	subject: string | null;
	body: string;
	sent_at: string | null;
	delivered_at: string | null;
	read_at: string | null;
	created_at: string;
}

export interface SendMessageRequest {
	customer_id: string;
	channel: MessageChannel;
	body: string;
	subject?: string;
	job_id?: string;
}

// ── Review ──

export interface Review {
	id: string;
	team_id: string;
	customer_id: string | null;
	job_id: string | null;
	platform: string;
	rating: number;
	content: string | null;
	reviewer_name: string | null;
	review_url: string | null;
	response: string | null;
	responded_at: string | null;
	created_at: string;
	updated_at: string;
}

export interface CreateReviewRequest {
	platform: string;
	rating: number;
	content?: string;
	reviewer_name?: string;
	customer_id?: string;
	job_id?: string;
}

// ── Notification ──

export type NotificationType = 'job_assigned' | 'job_status_changed' | 'estimate_approved' | 'estimate_declined'
	| 'invoice_paid' | 'payment_received' | 'schedule_reminder' | 'low_stock' | 'license_expiring'
	| 'review_received' | 'message_received' | 'system';

export interface Notification {
	id: string;
	team_id: string;
	user_id: string;
	type: NotificationType;
	title: string;
	body: string;
	data: Record<string, unknown> | null;
	read_at: string | null;
	created_at: string;
}

// ── Audit Log ──

export interface AuditLogEntry {
	id: string;
	team_id: string;
	user_id: string | null;
	action: string;
	entity_type: string;
	entity_id: string | null;
	old_values: Record<string, unknown> | null;
	new_values: Record<string, unknown> | null;
	ip_address: string | null;
	user_agent: string | null;
	created_at: string;
}

// ── Webhook ──

export interface Webhook {
	id: string;
	team_id: string;
	url: string;
	secret: string;
	events: string[];
	is_active: boolean;
	last_triggered_at: string | null;
	last_status_code: number | null;
	failure_count: number;
	created_at: string;
	updated_at: string;
}

// ── Search ──

export interface SearchResults {
	jobs?: Job[];
	customers?: Customer[];
	estimates?: Estimate[];
	invoices?: Invoice[];
}

