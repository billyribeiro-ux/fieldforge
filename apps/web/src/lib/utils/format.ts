// ── Currency ──

export function formatCurrency(
	amount: number,
	currency = 'USD',
	locale = 'en-US'
): string {
	return new Intl.NumberFormat(locale, {
		style: 'currency',
		currency,
		minimumFractionDigits: 2,
		maximumFractionDigits: 2
	}).format(amount);
}

export function formatCurrencyCompact(amount: number): string {
	if (amount >= 1_000_000) return `$${(amount / 1_000_000).toFixed(1)}M`;
	if (amount >= 1_000) return `$${(amount / 1_000).toFixed(1)}K`;
	return formatCurrency(amount);
}

// ── Date / Time ──

export function formatDate(
	date: string | Date,
	style: 'short' | 'medium' | 'long' | 'relative' = 'medium'
): string {
	const d = typeof date === 'string' ? new Date(date) : date;

	if (style === 'relative') return formatRelativeTime(d);

	const options: Intl.DateTimeFormatOptions =
		style === 'short'
			? { month: 'numeric', day: 'numeric', year: '2-digit' }
			: style === 'long'
				? { weekday: 'long', month: 'long', day: 'numeric', year: 'numeric' }
				: { month: 'short', day: 'numeric', year: 'numeric' };

	return d.toLocaleDateString('en-US', options);
}

export function formatTime(date: string | Date, includeSeconds = false): string {
	const d = typeof date === 'string' ? new Date(date) : date;
	return d.toLocaleTimeString('en-US', {
		hour: 'numeric',
		minute: '2-digit',
		second: includeSeconds ? '2-digit' : undefined,
		hour12: true
	});
}

export function formatDateTime(date: string | Date): string {
	return `${formatDate(date)} at ${formatTime(date)}`;
}

export function formatRelativeTime(date: Date): string {
	const now = new Date();
	const diffMs = now.getTime() - date.getTime();
	const diffSec = Math.floor(diffMs / 1000);
	const diffMin = Math.floor(diffSec / 60);
	const diffHr = Math.floor(diffMin / 60);
	const diffDay = Math.floor(diffHr / 24);

	if (diffSec < 60) return 'just now';
	if (diffMin < 60) return `${diffMin}m ago`;
	if (diffHr < 24) return `${diffHr}h ago`;
	if (diffDay < 7) return `${diffDay}d ago`;
	if (diffDay < 30) return `${Math.floor(diffDay / 7)}w ago`;
	return formatDate(date, 'short');
}

// ── Duration ──

export function formatDuration(minutes: number): string {
	if (minutes < 60) return `${minutes}m`;
	const hrs = Math.floor(minutes / 60);
	const mins = minutes % 60;
	return mins > 0 ? `${hrs}h ${mins}m` : `${hrs}h`;
}

export function formatDurationLong(minutes: number): string {
	const hrs = Math.floor(minutes / 60);
	const mins = minutes % 60;
	const parts: string[] = [];
	if (hrs > 0) parts.push(`${hrs} hour${hrs !== 1 ? 's' : ''}`);
	if (mins > 0) parts.push(`${mins} minute${mins !== 1 ? 's' : ''}`);
	return parts.join(' ') || '0 minutes';
}

// ── Phone ──

export function formatPhone(phone: string): string {
	const cleaned = phone.replace(/\D/g, '');
	if (cleaned.length === 10) {
		return `(${cleaned.slice(0, 3)}) ${cleaned.slice(3, 6)}-${cleaned.slice(6)}`;
	}
	if (cleaned.length === 11 && cleaned[0] === '1') {
		return `+1 (${cleaned.slice(1, 4)}) ${cleaned.slice(4, 7)}-${cleaned.slice(7)}`;
	}
	return phone;
}

// ── Names ──

export function formatName(first: string, last: string): string {
	return `${first} ${last}`.trim();
}

export function getInitials(first: string, last: string): string {
	return `${first?.[0] ?? ''}${last?.[0] ?? ''}`.toUpperCase();
}

// ── Numbers ──

export function formatNumber(n: number, decimals = 0): string {
	return new Intl.NumberFormat('en-US', {
		minimumFractionDigits: decimals,
		maximumFractionDigits: decimals
	}).format(n);
}

export function formatPercent(n: number, decimals = 1): string {
	return `${n.toFixed(decimals)}%`;
}

// ── File Size ──

export function formatFileSize(bytes: number): string {
	if (bytes < 1024) return `${bytes} B`;
	if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
	if (bytes < 1024 * 1024 * 1024) return `${(bytes / (1024 * 1024)).toFixed(1)} MB`;
	return `${(bytes / (1024 * 1024 * 1024)).toFixed(1)} GB`;
}

// ── Address ──

export function formatAddress(parts: {
	line1?: string;
	line2?: string;
	city?: string;
	state?: string;
	zip?: string;
}): string {
	const lines: string[] = [];
	if (parts.line1) lines.push(parts.line1);
	if (parts.line2) lines.push(parts.line2);
	const cityStateZip = [parts.city, parts.state].filter(Boolean).join(', ');
	if (cityStateZip || parts.zip) {
		lines.push([cityStateZip, parts.zip].filter(Boolean).join(' '));
	}
	return lines.join(', ');
}
