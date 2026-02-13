import { api } from './client';
import type {
	Customer, CreateCustomerRequest, UpdateCustomerRequest,
	Job, CreateJobRequest, JobStatusTransition,
	Estimate, CreateEstimateRequest,
	Invoice, CreateInvoiceRequest, RecordPaymentRequest, Payment,
	TimeEntry, StartTimerRequest, StopTimerRequest,
	Photo, User, Team, Property, Note, Notification,
	InventoryItem, Vehicle, Checklist, ChecklistItem,
	PaginatedResponse, ApiResponse
} from '$lib/types';

// Types used only in services (not exported from types/index.ts yet)
interface InventoryStock { id: string; item_id: string; location_id: string; quantity: number; updated_at: string; }
interface InventoryLocation { id: string; team_id: string; name: string; location_type: string; vehicle_id: string | null; address: string | null; is_active: boolean; created_at: string; updated_at: string; }
interface VehicleMaintenance { id: string; vehicle_id: string; maintenance_type: string; description: string | null; provider: string | null; cost: number | null; odometer: number | null; performed_at: string; next_due_date: string | null; next_due_odometer: number | null; notes: string | null; created_at: string; }
interface Equipment { id: string; team_id: string; name: string; category: string | null; brand: string | null; model: string | null; serial_number: string | null; condition: string; assigned_to: string | null; is_active: boolean; created_at: string; updated_at: string; }

// ── Customers ──

export const customers = {
	list: (params?: Record<string, string>) =>
		api.get<Customer[]>('/customers', params),
	get: (id: string) =>
		api.get<Customer>(`/customers/${id}`),
	create: (data: CreateCustomerRequest) =>
		api.post<Customer>('/customers', data),
	update: (id: string, data: UpdateCustomerRequest) =>
		api.patch<Customer>(`/customers/${id}`, data),
	delete: (id: string) =>
		api.delete<null>(`/customers/${id}`),
};

// ── Jobs ──

export const jobs = {
	list: (params?: Record<string, string>) =>
		api.get<Job[]>('/jobs', params),
	get: (id: string) =>
		api.get<Job>(`/jobs/${id}`),
	create: (data: CreateJobRequest) =>
		api.post<Job>('/jobs', data),
	update: (id: string, data: Partial<CreateJobRequest>) =>
		api.patch<Job>(`/jobs/${id}`, data),
	transition: (id: string, data: JobStatusTransition) =>
		api.post<Job>(`/jobs/${id}/status`, data),
	delete: (id: string) =>
		api.delete<null>(`/jobs/${id}`),
};

// ── Estimates ──

export const estimates = {
	list: (params?: Record<string, string>) =>
		api.get<Estimate[]>('/estimates', params),
	get: (id: string) =>
		api.get<Estimate>(`/estimates/${id}`),
	create: (data: CreateEstimateRequest) =>
		api.post<Estimate>('/estimates', data),
	update: (id: string, data: Partial<CreateEstimateRequest>) =>
		api.patch<Estimate>(`/estimates/${id}`, data),
	send: (id: string) =>
		api.post<Estimate>(`/estimates/${id}/send`, {}),
	approve: (id: string, signature?: string) =>
		api.post<Estimate>(`/estimates/${id}/approve`, { customer_signature: signature }),
	decline: (id: string, reason?: string) =>
		api.post<Estimate>(`/estimates/${id}/decline`, { decline_reason: reason }),
	convertToInvoice: (id: string) =>
		api.post<Invoice>(`/estimates/${id}/convert`, {}),
	duplicate: (id: string) =>
		api.post<Estimate>(`/estimates/${id}/duplicate`, {}),
	delete: (id: string) =>
		api.delete<null>(`/estimates/${id}`),
};

// ── Invoices ──

export const invoices = {
	list: (params?: Record<string, string>) =>
		api.get<Invoice[]>('/invoices', params),
	get: (id: string) =>
		api.get<Invoice>(`/invoices/${id}`),
	create: (data: CreateInvoiceRequest) =>
		api.post<Invoice>('/invoices', data),
	send: (id: string) =>
		api.post<Invoice>(`/invoices/${id}/send`, {}),
	void: (id: string) =>
		api.post<Invoice>(`/invoices/${id}/void`, {}),
	recordPayment: (id: string, data: RecordPaymentRequest) =>
		api.post<Payment>(`/invoices/${id}/payments`, data),
	listPayments: (id: string) =>
		api.get<Payment[]>(`/invoices/${id}/payments`),
	delete: (id: string) =>
		api.delete<null>(`/invoices/${id}`),
};

// ── Time Entries ──

export const timeEntries = {
	startTimer: (data: StartTimerRequest) =>
		api.post<TimeEntry>('/time-entries/start', data),
	stopTimer: (id: string, data?: StopTimerRequest) =>
		api.post<TimeEntry>(`/time-entries/${id}/stop`, data ?? {}),
	activeTimers: () =>
		api.get<TimeEntry[]>('/time-entries/active'),
	listForJob: (jobId: string) =>
		api.get<TimeEntry[]>(`/jobs/${jobId}/time-entries`),
};

// ── Photos ──

export const photos = {
	getPresignedUrl: (data: { filename: string; content_type: string; job_id?: string }) =>
		api.post<{ upload_url: string; file_key: string; expires_in: number }>('/photos/presigned-url', data),
	create: (jobId: string, data: { file_key: string; filename: string; content_type: string; file_size?: number; category?: string; caption?: string }) =>
		api.post<Photo>(`/jobs/${jobId}/photos`, data),
	listForJob: (jobId: string) =>
		api.get<Photo[]>(`/jobs/${jobId}/photos`),
	get: (id: string) =>
		api.get<{ photo: Photo; view_url: string }>(`/photos/${id}`),
	delete: (id: string) =>
		api.delete<null>(`/photos/${id}`),
};

// ── Properties ──

export const properties = {
	listForCustomer: (customerId: string) =>
		api.get<Property[]>(`/customers/${customerId}/properties`),
	get: (id: string) =>
		api.get<Property>(`/properties/${id}`),
	create: (customerId: string, data: Partial<Property>) =>
		api.post<Property>(`/customers/${customerId}/properties`, data),
	update: (id: string, data: Partial<Property>) =>
		api.patch<Property>(`/properties/${id}`, data),
	delete: (id: string) =>
		api.delete<null>(`/properties/${id}`),
};

// ── Notes ──

export const notes = {
	listForJob: (jobId: string) =>
		api.get<Note[]>(`/jobs/${jobId}/notes`),
	listForCustomer: (customerId: string) =>
		api.get<Note[]>(`/customers/${customerId}/notes`),
	create: (jobId: string, data: { content: string; is_internal?: boolean }) =>
		api.post<Note>(`/jobs/${jobId}/notes`, data),
	delete: (id: string) =>
		api.delete<null>(`/notes/${id}`),
};

// ── Team ──

export const team = {
	get: () =>
		api.get<Team>('/team'),
	update: (data: Partial<Team>) =>
		api.patch<Team>('/team', data),
	listMembers: () =>
		api.get<User[]>('/team/members'),
	inviteMember: (data: { email: string; first_name: string; last_name: string; role: string }) =>
		api.post<User>('/team/members', data),
	updateMember: (id: string, data: { role?: string; hourly_rate?: number }) =>
		api.patch<User>(`/team/members/${id}`, data),
	deactivateMember: (id: string) =>
		api.post<null>(`/team/members/${id}/deactivate`, {}),
};

// ── Inventory ──

export const inventory = {
	listItems: (params?: Record<string, string>) =>
		api.get<InventoryItem[]>('/inventory/items', params),
	getItem: (id: string) =>
		api.get<{ item: InventoryItem; stock: InventoryStock[] }>(`/inventory/items/${id}`),
	createItem: (data: { name: string; sku?: string; description?: string; category?: string; unit_of_measure?: string; min_stock_level?: number; cost_price?: number; sell_price?: number }) =>
		api.post<InventoryItem>('/inventory/items', data),
	updateItem: (id: string, data: Partial<InventoryItem>) =>
		api.patch<InventoryItem>(`/inventory/items/${id}`, data),
	deleteItem: (id: string) =>
		api.delete<null>(`/inventory/items/${id}`),
	listLocations: () =>
		api.get<InventoryLocation[]>('/inventory/locations'),
	createLocation: (data: { name: string; location_type?: string; vehicle_id?: string; address?: string }) =>
		api.post<InventoryLocation>('/inventory/locations', data),
	getItemStock: (id: string) =>
		api.get<InventoryStock[]>(`/inventory/items/${id}/stock`),
	adjustStock: (id: string, data: { location_id: string; quantity: number; txn_type: string; job_id?: string; notes?: string }) =>
		api.post<null>(`/inventory/items/${id}/adjust`, data),
};

// ── Vehicles ──

export const vehicles = {
	list: () =>
		api.get<Vehicle[]>('/vehicles'),
	get: (id: string) =>
		api.get<{ vehicle: Vehicle; maintenance: VehicleMaintenance[] }>(`/vehicles/${id}`),
	create: (data: { make: string; model: string; year: number; vin?: string; license_plate?: string; color?: string; assigned_to?: string; odometer?: number; notes?: string }) =>
		api.post<Vehicle>('/vehicles', data),
	update: (id: string, data: Partial<Vehicle>) =>
		api.patch<Vehicle>(`/vehicles/${id}`, data),
	delete: (id: string) =>
		api.delete<null>(`/vehicles/${id}`),
	listMaintenance: (id: string) =>
		api.get<VehicleMaintenance[]>(`/vehicles/${id}/maintenance`),
	createMaintenance: (id: string, data: { maintenance_type: string; description?: string; provider?: string; cost?: number; odometer?: number; performed_at: string; next_due_date?: string; notes?: string }) =>
		api.post<VehicleMaintenance>(`/vehicles/${id}/maintenance`, data),
};

// ── Checklists ──

export const checklists = {
	listForJob: (jobId: string) =>
		api.get<{ checklists: Checklist[]; items: ChecklistItem[] }>(`/jobs/${jobId}/checklists`),
	get: (id: string) =>
		api.get<{ checklist: Checklist; items: ChecklistItem[] }>(`/checklists/${id}`),
	create: (jobId: string, data: { title: string; checklist_type?: string; is_required?: boolean; items?: { description: string; sort_order?: number }[] }) =>
		api.post<Checklist>(`/jobs/${jobId}/checklists`, data),
	delete: (id: string) =>
		api.delete<null>(`/checklists/${id}`),
	addItem: (checklistId: string, data: { description: string; sort_order?: number }) =>
		api.post<ChecklistItem>(`/checklists/${checklistId}/items`, data),
	toggleItem: (checklistId: string, itemId: string) =>
		api.post<ChecklistItem>(`/checklists/${checklistId}/items/${itemId}/toggle`, {}),
};

// ── Equipment ──

export const equipment = {
	list: () =>
		api.get<Equipment[]>('/equipment'),
	get: (id: string) =>
		api.get<Equipment>(`/equipment/${id}`),
	create: (data: { name: string; category?: string; brand?: string; model?: string; serial_number?: string; condition?: string; notes?: string }) =>
		api.post<Equipment>('/equipment', data),
	update: (id: string, data: Partial<Equipment>) =>
		api.patch<Equipment>(`/equipment/${id}`, data),
	delete: (id: string) =>
		api.delete<null>(`/equipment/${id}`),
};

// ── Search ──

export const search = {
	global: (q: string, type?: string) =>
		api.get<Record<string, unknown[]>>('/search', { q, ...(type ? { type } : {}) }),
};

// ── Notifications ──

export const notifications = {
	list: (params?: Record<string, string>) =>
		api.get<Notification[]>('/notifications', params),
	markRead: (id: string) =>
		api.post<null>(`/notifications/${id}/read`, {}),
	markAllRead: () =>
		api.post<null>('/notifications/read-all', {}),
};
