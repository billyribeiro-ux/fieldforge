export const colors = {
	forge: {
		50: '#eff6ff',
		100: '#dbeafe',
		200: '#bfdbfe',
		300: '#93c5fd',
		400: '#60a5fa',
		500: '#3b82f6',
		600: '#2563eb',
		700: '#1d4ed8',
		800: '#1e40af',
		900: '#1e3a8a'
	},
	surface: {
		50: '#f8fafc',
		100: '#f1f5f9',
		200: '#e2e8f0',
		300: '#cbd5e1',
		400: '#94a3b8',
		500: '#64748b',
		600: '#475569',
		700: '#334155',
		800: '#1e293b',
		900: '#0f172a'
	},
	success: '#22c55e',
	warning: '#f59e0b',
	danger: '#ef4444',
	info: '#3b82f6'
} as const;

export const breakpoints = {
	sm: 640,
	md: 768,
	lg: 1024,
	xl: 1280,
	'2xl': 1536
} as const;

export const spacing = {
	sidebar: '260px',
	sidebarCollapsed: '72px',
	topbar: '64px'
} as const;

export const transitions = {
	fast: '150ms',
	normal: '200ms',
	slow: '300ms'
} as const;

export const jobStatuses = [
	'lead', 'estimated', 'approved', 'scheduled', 'en_route',
	'in_progress', 'paused', 'completed', 'invoiced', 'paid',
	'cancelled', 'on_hold', 'declined', 'closed'
] as const;

export const estimateStatuses = [
	'draft', 'sent', 'viewed', 'approved', 'declined', 'expired', 'converted'
] as const;

export const invoiceStatuses = [
	'draft', 'sent', 'viewed', 'partially_paid', 'paid', 'overdue', 'void'
] as const;

export const trades = [
	'hvac', 'plumbing', 'electrical', 'roofing', 'painting',
	'landscaping', 'general', 'carpentry', 'flooring', 'concrete'
] as const;

export type JobStatus = (typeof jobStatuses)[number];
export type EstimateStatus = (typeof estimateStatuses)[number];
export type InvoiceStatus = (typeof invoiceStatuses)[number];
export type Trade = (typeof trades)[number];
