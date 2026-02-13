<script lang="ts">
	import TopBar from '$lib/components/layout/TopBar.svelte';
	import Card from '$lib/components/ui/Card.svelte';
	import Button from '$lib/components/ui/Button.svelte';
	import Badge from '$lib/components/ui/Badge.svelte';
	import { Plus, Search, FileText, MoreHorizontal, Eye, Send } from 'lucide-svelte';

	let searchQuery = $state('');
	let statusFilter = $state('all');

	const estimates = [
		{ id: '1', number: 'EST-001', title: 'AC Unit Replacement', customer: 'Sarah Johnson', status: 'approved', total: 4200, sent_at: '2024-12-10', valid_until: '2024-12-25' },
		{ id: '2', number: 'EST-002', title: 'Kitchen Remodel Phase 2', customer: 'Robert Kim', status: 'sent', total: 18500, sent_at: '2024-12-12', valid_until: '2024-12-27' },
		{ id: '3', number: 'EST-003', title: 'Water Heater Install', customer: 'David Park', status: 'viewed', total: 2100, sent_at: '2024-12-13', valid_until: '2024-12-28' },
		{ id: '4', number: 'EST-004', title: 'Roof Leak Assessment', customer: 'Karen White', status: 'draft', total: 0, sent_at: '', valid_until: '' },
		{ id: '5', number: 'EST-005', title: 'Electrical Panel Upgrade', customer: 'Amy Foster', status: 'approved', total: 3500, sent_at: '2024-12-08', valid_until: '2024-12-23' },
		{ id: '6', number: 'EST-006', title: 'Bathroom Renovation', customer: 'Tom Williams', status: 'declined', total: 8900, sent_at: '2024-12-05', valid_until: '2024-12-20' }
	];

	function statusVariant(status: string): 'success' | 'warning' | 'danger' | 'info' | 'default' {
		switch (status) {
			case 'approved': case 'converted': return 'success';
			case 'sent': return 'info';
			case 'viewed': return 'warning';
			case 'declined': case 'expired': return 'danger';
			default: return 'default';
		}
	}

	function statusLabel(s: string): string {
		return s.charAt(0).toUpperCase() + s.slice(1);
	}

	function formatCurrency(amount: number): string {
		return new Intl.NumberFormat('en-US', { style: 'currency', currency: 'USD', minimumFractionDigits: 0 }).format(amount);
	}

	let filteredEstimates = $derived(
		estimates.filter((e) => {
			if (statusFilter !== 'all' && e.status !== statusFilter) return false;
			if (searchQuery) {
				const q = searchQuery.toLowerCase();
				return e.title.toLowerCase().includes(q) || e.customer.toLowerCase().includes(q) || e.number.toLowerCase().includes(q);
			}
			return true;
		})
	);
</script>

<TopBar title="Estimates">
	{#snippet actions()}
		<Button variant="primary" size="md">
			<Plus class="w-4 h-4" />
			New Estimate
		</Button>
	{/snippet}
</TopBar>

<div class="p-6 space-y-4">
	<div class="flex items-center gap-3">
		<div class="relative flex-1 max-w-sm">
			<Search class="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-surface-400" />
			<input
				type="text"
				placeholder="Search estimates..."
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
			<option value="viewed">Viewed</option>
			<option value="approved">Approved</option>
			<option value="declined">Declined</option>
		</select>
		<span class="text-sm text-surface-400">{filteredEstimates.length} estimates</span>
	</div>

	<Card padding={false}>
		<div class="overflow-x-auto">
			<table class="w-full">
				<thead>
					<tr class="border-b border-surface-200 bg-surface-50">
						<th class="text-left text-xs font-medium text-surface-500 uppercase tracking-wider px-6 py-3">Estimate</th>
						<th class="text-left text-xs font-medium text-surface-500 uppercase tracking-wider px-6 py-3">Customer</th>
						<th class="text-left text-xs font-medium text-surface-500 uppercase tracking-wider px-6 py-3">Status</th>
						<th class="text-right text-xs font-medium text-surface-500 uppercase tracking-wider px-6 py-3">Total</th>
						<th class="text-left text-xs font-medium text-surface-500 uppercase tracking-wider px-6 py-3">Sent</th>
						<th class="text-left text-xs font-medium text-surface-500 uppercase tracking-wider px-6 py-3">Valid Until</th>
						<th class="w-12 px-3 py-3"></th>
					</tr>
				</thead>
				<tbody class="divide-y divide-surface-100">
					{#each filteredEstimates as est (est.id)}
						<tr class="hover:bg-surface-50 transition-colors">
							<td class="px-6 py-4">
								<a href="/dashboard/estimates/{est.id}" class="block">
									<p class="text-xs text-surface-400 font-mono">{est.number}</p>
									<p class="text-sm font-medium text-surface-900 hover:text-forge-600">{est.title}</p>
								</a>
							</td>
							<td class="px-6 py-4">
								<span class="text-sm text-surface-700">{est.customer}</span>
							</td>
							<td class="px-6 py-4">
								<Badge variant={statusVariant(est.status)}>{statusLabel(est.status)}</Badge>
							</td>
							<td class="px-6 py-4 text-right">
								<span class="text-sm font-medium text-surface-900">{est.total > 0 ? formatCurrency(est.total) : '—'}</span>
							</td>
							<td class="px-6 py-4">
								<span class="text-sm text-surface-500">{est.sent_at || '—'}</span>
							</td>
							<td class="px-6 py-4">
								<span class="text-sm text-surface-500">{est.valid_until || '—'}</span>
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

		{#if filteredEstimates.length === 0}
			<div class="px-6 py-16 text-center">
				<FileText class="w-12 h-12 text-surface-300 mx-auto mb-3" />
				<p class="text-sm font-medium text-surface-500">No estimates found</p>
			</div>
		{/if}
	</Card>
</div>
