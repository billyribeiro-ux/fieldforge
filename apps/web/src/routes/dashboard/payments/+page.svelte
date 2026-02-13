<script lang="ts">
	import TopBar from '$lib/components/layout/TopBar.svelte';
	import Card from '$lib/components/ui/Card.svelte';
	import Button from '$lib/components/ui/Button.svelte';
	import Badge from '$lib/components/ui/Badge.svelte';
	import { Search, DollarSign, CreditCard, Banknote, TrendingUp, ArrowDownRight, ArrowUpRight, Download } from 'lucide-svelte';

	let searchQuery = $state('');
	let methodFilter = $state('all');
	let dateRange = $state('this_month');

	const payments = [
		{ id: '1', invoice_number: 'FF-002', customer: 'Robert Kim', amount: 18500, method: 'card', status: 'completed', date: '2024-12-12', last4: '4242', description: 'Kitchen Remodel Phase 2' },
		{ id: '2', invoice_number: 'FF-005', customer: 'Tom Williams', amount: 275, method: 'card', status: 'completed', date: '2024-12-11', last4: '1234', description: 'Furnace Maintenance' },
		{ id: '3', invoice_number: 'FF-004', customer: 'Mike Chen', amount: 3400, method: 'ach', status: 'completed', date: '2024-12-10', last4: null, description: 'Office Plumbing — Partial' },
		{ id: '4', invoice_number: null, customer: 'Lisa Rodriguez', amount: 800, method: 'cash', status: 'completed', date: '2024-12-09', last4: null, description: 'Emergency Leak Repair' },
		{ id: '5', invoice_number: 'FF-001', customer: 'Sarah Johnson', amount: 2100, method: 'card', status: 'completed', date: '2024-12-08', last4: '5678', description: 'AC Unit — Deposit' },
		{ id: '6', invoice_number: null, customer: 'Amy Foster', amount: 150, method: 'check', status: 'completed', date: '2024-12-07', last4: null, description: 'Electrical Inspection Fee' },
		{ id: '7', invoice_number: 'FF-003', customer: 'Karen White', amount: 6200, method: 'card', status: 'refunded', date: '2024-12-06', last4: '9012', description: 'Commercial HVAC — Partial Refund' },
		{ id: '8', invoice_number: null, customer: 'David Park', amount: 500, method: 'card', status: 'failed', date: '2024-12-05', last4: '3456', description: 'Water Heater Deposit' }
	];

	const methodOptions = [
		{ value: 'all', label: 'All Methods' },
		{ value: 'card', label: 'Card' },
		{ value: 'ach', label: 'ACH / Bank' },
		{ value: 'cash', label: 'Cash' },
		{ value: 'check', label: 'Check' }
	];

	function statusVariant(s: string): 'success' | 'danger' | 'warning' | 'default' {
		switch (s) {
			case 'completed': return 'success';
			case 'refunded': return 'danger';
			case 'failed': return 'danger';
			case 'pending': return 'warning';
			default: return 'default';
		}
	}

	function methodIcon(m: string): string {
		switch (m) {
			case 'card': return '💳';
			case 'ach': return '🏦';
			case 'cash': return '💵';
			case 'check': return '📝';
			default: return '💰';
		}
	}

	function formatCurrency(n: number): string {
		return new Intl.NumberFormat('en-US', { style: 'currency', currency: 'USD', minimumFractionDigits: 0 }).format(n);
	}

	let filteredPayments = $derived(
		payments.filter((p) => {
			if (methodFilter !== 'all' && p.method !== methodFilter) return false;
			if (searchQuery) {
				const q = searchQuery.toLowerCase();
				return p.customer.toLowerCase().includes(q) || p.description.toLowerCase().includes(q) || (p.invoice_number?.toLowerCase().includes(q) ?? false);
			}
			return true;
		})
	);

	let totalCollected = $derived(payments.filter((p) => p.status === 'completed').reduce((s, p) => s + p.amount, 0));
	let cardTotal = $derived(payments.filter((p) => p.method === 'card' && p.status === 'completed').reduce((s, p) => s + p.amount, 0));
	let cashCheckTotal = $derived(payments.filter((p) => (p.method === 'cash' || p.method === 'check') && p.status === 'completed').reduce((s, p) => s + p.amount, 0));
	let processingFees = $derived(Math.round(cardTotal * 0.029 + payments.filter((p) => p.method === 'card' && p.status === 'completed').length * 0.30));
</script>

<svelte:head>
	<title>Payments — FieldForge</title>
</svelte:head>

<TopBar title="Payments">
	{#snippet actions()}
		<Button variant="outline" size="md">
			<Download class="w-4 h-4" />
			Export
		</Button>
	{/snippet}
</TopBar>

<div class="p-6 space-y-6">
	<!-- Summary Cards -->
	<div class="grid grid-cols-1 sm:grid-cols-4 gap-4">
		<Card>
			<div class="flex items-center gap-3">
				<div class="w-10 h-10 bg-success-50 rounded-lg flex items-center justify-center">
					<DollarSign class="w-5 h-5 text-success-600" />
				</div>
				<div>
					<p class="text-xs text-surface-400 uppercase tracking-wider">Collected</p>
					<p class="text-xl font-bold text-surface-900">{formatCurrency(totalCollected)}</p>
				</div>
			</div>
		</Card>
		<Card>
			<div class="flex items-center gap-3">
				<div class="w-10 h-10 bg-forge-50 rounded-lg flex items-center justify-center">
					<CreditCard class="w-5 h-5 text-forge-600" />
				</div>
				<div>
					<p class="text-xs text-surface-400 uppercase tracking-wider">Card Payments</p>
					<p class="text-xl font-bold text-forge-600">{formatCurrency(cardTotal)}</p>
				</div>
			</div>
		</Card>
		<Card>
			<div class="flex items-center gap-3">
				<div class="w-10 h-10 bg-accent-50 rounded-lg flex items-center justify-center">
					<Banknote class="w-5 h-5 text-accent-600" />
				</div>
				<div>
					<p class="text-xs text-surface-400 uppercase tracking-wider">Cash / Check</p>
					<p class="text-xl font-bold text-surface-900">{formatCurrency(cashCheckTotal)}</p>
				</div>
			</div>
		</Card>
		<Card>
			<div class="flex items-center gap-3">
				<div class="w-10 h-10 bg-surface-100 rounded-lg flex items-center justify-center">
					<TrendingUp class="w-5 h-5 text-surface-600" />
				</div>
				<div>
					<p class="text-xs text-surface-400 uppercase tracking-wider">Processing Fees</p>
					<p class="text-xl font-bold text-surface-500">{formatCurrency(processingFees)}</p>
				</div>
			</div>
		</Card>
	</div>

	<!-- Filters -->
	<div class="flex items-center gap-3">
		<div class="relative flex-1 max-w-sm">
			<Search class="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-surface-400" />
			<input
				type="text"
				placeholder="Search payments..."
				bind:value={searchQuery}
				class="w-full pl-9 pr-4 py-2 text-sm bg-white border border-surface-200 rounded-lg focus:outline-none focus:ring-2 focus:ring-forge-500 placeholder:text-surface-400"
			/>
		</div>
		<select
			bind:value={methodFilter}
			class="px-3 py-2 text-sm bg-white border border-surface-200 rounded-lg focus:outline-none focus:ring-2 focus:ring-forge-500 text-surface-700 cursor-pointer"
		>
			{#each methodOptions as opt}
				<option value={opt.value}>{opt.label}</option>
			{/each}
		</select>
		<select
			bind:value={dateRange}
			class="px-3 py-2 text-sm bg-white border border-surface-200 rounded-lg focus:outline-none focus:ring-2 focus:ring-forge-500 text-surface-700 cursor-pointer"
		>
			<option value="today">Today</option>
			<option value="this_week">This Week</option>
			<option value="this_month">This Month</option>
			<option value="last_month">Last Month</option>
			<option value="this_quarter">This Quarter</option>
			<option value="this_year">This Year</option>
		</select>
		<span class="text-sm text-surface-400">{filteredPayments.length} transactions</span>
	</div>

	<!-- Payments Table -->
	<Card padding={false}>
		<div class="overflow-x-auto">
			<table class="w-full">
				<thead>
					<tr class="border-b border-surface-200 bg-surface-50">
						<th class="text-left text-xs font-medium text-surface-500 uppercase tracking-wider px-6 py-3">Date</th>
						<th class="text-left text-xs font-medium text-surface-500 uppercase tracking-wider px-6 py-3">Customer</th>
						<th class="text-left text-xs font-medium text-surface-500 uppercase tracking-wider px-6 py-3">Description</th>
						<th class="text-left text-xs font-medium text-surface-500 uppercase tracking-wider px-6 py-3">Invoice</th>
						<th class="text-left text-xs font-medium text-surface-500 uppercase tracking-wider px-6 py-3">Method</th>
						<th class="text-left text-xs font-medium text-surface-500 uppercase tracking-wider px-6 py-3">Status</th>
						<th class="text-right text-xs font-medium text-surface-500 uppercase tracking-wider px-6 py-3">Amount</th>
					</tr>
				</thead>
				<tbody class="divide-y divide-surface-100">
					{#each filteredPayments as payment (payment.id)}
						<tr class="hover:bg-surface-50 transition-colors">
							<td class="px-6 py-4 text-sm text-surface-500 whitespace-nowrap">{payment.date}</td>
							<td class="px-6 py-4 text-sm font-medium text-surface-900">{payment.customer}</td>
							<td class="px-6 py-4 text-sm text-surface-600">{payment.description}</td>
							<td class="px-6 py-4">
								{#if payment.invoice_number}
									<a href="/dashboard/invoices/{payment.id}" class="text-sm text-forge-600 hover:underline font-mono">{payment.invoice_number}</a>
								{:else}
									<span class="text-sm text-surface-400">—</span>
								{/if}
							</td>
							<td class="px-6 py-4">
								<span class="text-sm text-surface-700 flex items-center gap-1.5">
									<span>{methodIcon(payment.method)}</span>
									<span class="capitalize">{payment.method}</span>
									{#if payment.last4}
										<span class="text-surface-400">•••• {payment.last4}</span>
									{/if}
								</span>
							</td>
							<td class="px-6 py-4">
								<Badge variant={statusVariant(payment.status)} size="sm">
									{payment.status.charAt(0).toUpperCase() + payment.status.slice(1)}
								</Badge>
							</td>
							<td class="px-6 py-4 text-right">
								<span class="text-sm font-semibold {payment.status === 'refunded' ? 'text-danger-500' : 'text-surface-900'}">
									{payment.status === 'refunded' ? '-' : ''}{formatCurrency(payment.amount)}
								</span>
							</td>
						</tr>
					{/each}
				</tbody>
			</table>
		</div>
	</Card>
</div>
