<script lang="ts">
	import Badge from './Badge.svelte';

	interface Props {
		status: string;
		entity?: 'job' | 'estimate' | 'invoice' | 'payment';
		size?: 'sm' | 'md';
	}

	let { status, entity = 'job', size = 'sm' }: Props = $props();

	const jobVariants: Record<string, 'success' | 'warning' | 'danger' | 'info' | 'default'> = {
		lead: 'default',
		estimated: 'info',
		approved: 'info',
		scheduled: 'info',
		en_route: 'warning',
		in_progress: 'warning',
		paused: 'warning',
		completed: 'success',
		invoiced: 'success',
		paid: 'success',
		cancelled: 'danger',
		on_hold: 'default',
		declined: 'danger',
		closed: 'default'
	};

	const estimateVariants: Record<string, 'success' | 'warning' | 'danger' | 'info' | 'default'> = {
		draft: 'default',
		sent: 'info',
		viewed: 'warning',
		approved: 'success',
		declined: 'danger',
		expired: 'danger',
		converted: 'success'
	};

	const invoiceVariants: Record<string, 'success' | 'warning' | 'danger' | 'info' | 'default'> = {
		draft: 'default',
		sent: 'info',
		viewed: 'info',
		partially_paid: 'warning',
		paid: 'success',
		overdue: 'danger',
		void: 'default'
	};

	const paymentVariants: Record<string, 'success' | 'warning' | 'danger' | 'info' | 'default'> = {
		pending: 'warning',
		processing: 'info',
		succeeded: 'success',
		failed: 'danger',
		refunded: 'default',
		partially_refunded: 'warning'
	};

	function getVariant(): 'success' | 'warning' | 'danger' | 'info' | 'default' {
		const map = entity === 'estimate' ? estimateVariants
			: entity === 'invoice' ? invoiceVariants
			: entity === 'payment' ? paymentVariants
			: jobVariants;
		return map[status] ?? 'default';
	}

	function formatLabel(s: string): string {
		return s.replace(/_/g, ' ').replace(/\b\w/g, (c) => c.toUpperCase());
	}
</script>

<Badge variant={getVariant()} {size}>{formatLabel(status)}</Badge>
