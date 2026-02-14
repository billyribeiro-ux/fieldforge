<script lang="ts">
	import TopBar from '$lib/components/layout/TopBar.svelte';
	import Card from '$lib/components/ui/Card.svelte';
	import Button from '$lib/components/ui/Button.svelte';
	import Badge from '$lib/components/ui/Badge.svelte';
	import { Plus, Search, Receipt, MoreHorizontal } from 'lucide-svelte';


	let { data } = $props();
	let searchQuery = $state('');
	let statusFilter = $state('all');

	const invoices = [
		{ id: '1', number: 'FF-001', customer: 'Sarah Johnson', status: 'sent', total: 4200, amount_paid: 0, due_date: '2024-12-25', job: 'AC Unit Replacement' },
		{ id: '2', number: 'FF-002', customer: 'Robert Kim', status: 'paid', total: 18500, amount_paid: 18500, due_date: '2024-12-20', job: 'Kitchen Remodel Phase 2' },
		{ id: '3', number: 'FF-003', customer: 'Karen White', status: 'overdue', total: 12400, amount_paid: 0, due_date: '2024-11-30', job: 'Commercial HVAC Install' },
		{ id: '4', number: 'FF-004', customer: 'Mike Chen', status: 'partially_paid', total: 6800, amount_paid: 3400, due_date: '2024-12-28', job: 'Office Plumbing Overhaul' },
		{ id: '5', number: 'FF-005', customer: 'Tom Williams', status: 'paid', total: 275, amount_paid: 275, due_date: '2024-12-14', job: 'Furnace Maintenance' },
		{ id: '6', number: 'FF-006', customer: 'Amy Foster', status: 'draft', total: 3500, amount_paid: 0, due_date: '', job: 'Electrical Panel Upgrade' }
	];

	function statusVariant(status: string): 'success' | 'warning' | 'danger' | 'info' | 'default' {
		switch (status) {
			case 'paid': return 'success';
			case 'sent': case 'viewed': return 'info';
			case 'partially_paid': return 'warning';
			case 'overdue': return 'danger';
			default: return 'default';
		}
	}

	function statusLabel(s: string): string {
		return s.replace(/_/g, ' ').replace(/\b\w/g, (c) => c.toUpperCase());
	}

	function formatCurrency(amount: number): string {
		return new Intl.NumberFormat('en-US', { style: 'currency', currency: 'USD', minimumFractionDigits: 0 }).format(amount);
	}

	let filteredInvoices = $derived(
		invoices.filter((inv) => {
			if (statusFilter !== 'all' && inv.status !== statusFilter) return false;
			if (searchQuery) {
				const q = searchQuery.toLowerCase();
				return inv.customer.toLowerCase().includes(q) || inv.number.toLowerCase().includes(q) || inv.job.toLowerCase().includes(q);
			}
			return true;
		})
	);
</script>

<svelte:head>
	<title>Invoices — FieldForge</title>
</svelte:head>

<TopBar title="Invoices">
	{#snippet actions()}
		<a href="/dashboard/invoices/new">
			<Button variant="primary" size="md">
				<Plus class="w-4 h-4" />
				New Invoice
			</Button>
		</a>
	{/snippet}
</TopBar>

<div class="p-6 space-y-4">
	<div class="flex items-center gap-3">
		<div class="relative flex-1 max-w-sm">
			<Search class="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-surface-400" />
			<input
				type="text"
				placeholder="Search invoices..."
				bind:value={searchQuery}
				class="w-full pl-9 pr-4 py-2 text-sm bg-white border border-surface-200 rounded-lg focus:outline-none focus:ring-2 focus:ring-forge-500 focus:border-forge-500 placeholder:text-surface-400"
			/>
		</div>
		<select
			bind:value={statusFilter}
			class="px-3 py-2 text-sm bg-white border border-surface-200 rounded-lg focus:outline-none focus:ring-2 focus:ring-forge-500 text-surface-700 cursor-pointer"
		>
			<option value="all">All Statuses</option>
			<option value="draft">Draft</option>
			<option value="sent">Sent</option>
			<option value="partially_paid">Partially Paid</option>
			<option value="paid">Paid</option>
			<option value="overdue">Overdue</option>
			<option value="void">Void</option>
		</select>
		<span class="text-sm text-surface-400">{filteredInvoices.length} invoices</span>
	</div>

	<Card padding={false}>
		<div class="overflow-x-auto">
			<table class="w-full">
				<thead>
					<tr class="border-b border-surface-200 bg-surface-50">
						<th class="text-left text-xs font-medium text-surface-500 uppercase tracking-wider px-6 py-3">Invoice</th>
						<th class="text-left text-xs font-medium text-surface-500 uppercase tracking-wider px-6 py-3">Customer</th>
						<th class="text-left text-xs font-medium text-surface-500 uppercase tracking-wider px-6 py-3">Status</th>
						<th class="text-right text-xs font-medium text-surface-500 uppercase tracking-wider px-6 py-3">Total</th>
						<th class="text-right text-xs font-medium text-surface-500 uppercase tracking-wider px-6 py-3">Paid</th>
						<th class="text-right text-xs font-medium text-surface-500 uppercase tracking-wider px-6 py-3">Balance</th>
						<th class="text-left text-xs font-medium text-surface-500 uppercase tracking-wider px-6 py-3">Due Date</th>
						<th class="w-12 px-3 py-3"></th>
					</tr>
				</thead>
				<tbody class="divide-y divide-surface-100">
					{#each filteredInvoices as inv (inv.id)}
						<tr class="hover:bg-surface-50 transition-colors">
							<td class="px-6 py-4">
								<a href="/dashboard/invoices/{inv.id}" class="block">
									<p class="text-xs text-surface-400 font-mono">{inv.number}</p>
									<p class="text-sm font-medium text-surface-900 hover:text-forge-600">{inv.job}</p>
								</a>
							</td>
							<td class="px-6 py-4">
								<span class="text-sm text-surface-700">{inv.customer}</span>
							</td>
							<td class="px-6 py-4">
								<Badge variant={statusVariant(inv.status)}>{statusLabel(inv.status)}</Badge>
							</td>
							<td class="px-6 py-4 text-right">
								<span class="text-sm font-medium text-surface-900">{formatCurrency(inv.total)}</span>
							</td>
							<td class="px-6 py-4 text-right">
								<span class="text-sm text-surface-700">{formatCurrency(inv.amount_paid)}</span>
							</td>
							<td class="px-6 py-4 text-right">
								<span class="text-sm font-medium {(inv.total - inv.amount_paid) > 0 ? 'text-danger-500' : 'text-green-600'}">
									{formatCurrency(inv.total - inv.amount_paid)}
								</span>
							</td>
							<td class="px-6 py-4">
								<span class="text-sm text-surface-500">{inv.due_date || '—'}</span>
							</td>
							<td class="px-3 py-4">
								<button class="p-1 text-surface-400 hover:text-surface-600 rounded cursor-pointer">
									<MoreHorizontal class="w-4 h-4" />
								</button>
							</td>
						</tr>
					{/each}
				</tbody>
			</table>
		</div>

		{#if filteredInvoices.length === 0}
			<div class="px-6 py-16 text-center">
				<Receipt class="w-12 h-12 text-surface-300 mx-auto mb-3" />
				<p class="text-sm font-medium text-surface-500">No invoices found</p>
			</div>
		{/if}
	</Card>
</div>
