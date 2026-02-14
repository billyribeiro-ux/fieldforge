<script lang="ts">
	import TopBar from '$lib/components/layout/TopBar.svelte';
	import Card from '$lib/components/ui/Card.svelte';
	import Button from '$lib/components/ui/Button.svelte';
	import Badge from '$lib/components/ui/Badge.svelte';
	import { Plus, Search, Package, AlertTriangle, MoreHorizontal, Truck } from 'lucide-svelte';


	let { data } = $props();
	let searchQuery = $state('');
	let categoryFilter = $state('all');

	const inventory = [
		{ id: '1', name: 'Copper Fittings (1/2")', sku: 'CF-050', category: 'Plumbing', quantity: 12, min_quantity: 20, unit_cost: 3.50, location: 'Warehouse A', status: 'low' },
		{ id: '2', name: 'PVC Pipe (2" x 10ft)', sku: 'PVC-210', category: 'Plumbing', quantity: 45, min_quantity: 20, unit_cost: 8.75, location: 'Warehouse A', status: 'ok' },
		{ id: '3', name: 'R-410A Refrigerant (25lb)', sku: 'REF-410', category: 'HVAC', quantity: 8, min_quantity: 5, unit_cost: 125.00, location: 'Truck #1', status: 'ok' },
		{ id: '4', name: '20A Circuit Breaker', sku: 'CB-020', category: 'Electrical', quantity: 3, min_quantity: 10, unit_cost: 12.50, location: 'Warehouse B', status: 'low' },
		{ id: '5', name: 'Drywall Sheet (4x8)', sku: 'DW-048', category: 'General', quantity: 30, min_quantity: 15, unit_cost: 15.00, location: 'Warehouse A', status: 'ok' },
		{ id: '6', name: 'Furnace Filter (16x25x1)', sku: 'FF-162', category: 'HVAC', quantity: 0, min_quantity: 10, unit_cost: 8.00, location: 'Warehouse A', status: 'out' },
		{ id: '7', name: 'Wire Nuts (Yellow)', sku: 'WN-YEL', category: 'Electrical', quantity: 200, min_quantity: 50, unit_cost: 0.15, location: 'All Trucks', status: 'ok' },
		{ id: '8', name: 'Solder (Lead-Free Roll)', sku: 'SLD-LF', category: 'Plumbing', quantity: 6, min_quantity: 5, unit_cost: 22.00, location: 'Truck #2', status: 'ok' }
	];

	let filteredInventory = $derived(
		inventory.filter((item) => {
			if (categoryFilter !== 'all' && item.category.toLowerCase() !== categoryFilter) return false;
			if (searchQuery) {
				const q = searchQuery.toLowerCase();
				return item.name.toLowerCase().includes(q) || item.sku.toLowerCase().includes(q);
			}
			return true;
		})
	);

	let lowStockCount = $derived(inventory.filter((i) => i.status === 'low' || i.status === 'out').length);

	function formatCurrency(n: number): string {
		return new Intl.NumberFormat('en-US', { style: 'currency', currency: 'USD', minimumFractionDigits: 2 }).format(n);
	}
</script>

<svelte:head>
	<title>Inventory — FieldForge</title>
</svelte:head>

<TopBar title="Inventory">
	{#snippet actions()}
		<Button variant="primary" size="md">
			<Plus class="w-4 h-4" />
			Add Item
		</Button>
	{/snippet}
</TopBar>

<div class="p-6 space-y-4">
	<!-- Low stock alert -->
	{#if lowStockCount > 0}
		<div class="flex items-center gap-3 p-4 bg-yellow-50 border border-yellow-200 rounded-lg">
			<AlertTriangle class="w-5 h-5 text-yellow-600 flex-shrink-0" />
			<p class="text-sm text-yellow-800"><strong>{lowStockCount} items</strong> are low or out of stock and need reordering.</p>
		</div>
	{/if}

	<!-- Filters -->
	<div class="flex items-center gap-3">
		<div class="relative flex-1 max-w-sm">
			<Search class="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-surface-400" />
			<input
				type="text"
				placeholder="Search inventory..."
				bind:value={searchQuery}
				class="w-full pl-9 pr-4 py-2 text-sm bg-white border border-surface-200 rounded-lg focus:outline-none focus:ring-2 focus:ring-forge-500 placeholder:text-surface-400"
			/>
		</div>
		<select
			bind:value={categoryFilter}
			class="px-3 py-2 text-sm bg-white border border-surface-200 rounded-lg focus:outline-none focus:ring-2 focus:ring-forge-500 text-surface-700 cursor-pointer"
		>
			<option value="all">All Categories</option>
			<option value="hvac">HVAC</option>
			<option value="plumbing">Plumbing</option>
			<option value="electrical">Electrical</option>
			<option value="general">General</option>
		</select>
		<span class="text-sm text-surface-400">{filteredInventory.length} items</span>
	</div>

	<!-- Table -->
	<Card padding={false}>
		<div class="overflow-x-auto">
			<table class="w-full">
				<thead>
					<tr class="border-b border-surface-200 bg-surface-50">
						<th class="text-left text-xs font-medium text-surface-500 uppercase tracking-wider px-6 py-3">Item</th>
						<th class="text-left text-xs font-medium text-surface-500 uppercase tracking-wider px-6 py-3">Category</th>
						<th class="text-center text-xs font-medium text-surface-500 uppercase tracking-wider px-6 py-3">Qty</th>
						<th class="text-center text-xs font-medium text-surface-500 uppercase tracking-wider px-6 py-3">Min</th>
						<th class="text-right text-xs font-medium text-surface-500 uppercase tracking-wider px-6 py-3">Unit Cost</th>
						<th class="text-left text-xs font-medium text-surface-500 uppercase tracking-wider px-6 py-3">Location</th>
						<th class="text-left text-xs font-medium text-surface-500 uppercase tracking-wider px-6 py-3">Status</th>
						<th class="w-12 px-3 py-3"></th>
					</tr>
				</thead>
				<tbody class="divide-y divide-surface-100">
					{#each filteredInventory as item (item.id)}
						<tr class="hover:bg-surface-50 transition-colors">
							<td class="px-6 py-4">
								<p class="text-sm font-medium text-surface-900">{item.name}</p>
								<p class="text-xs text-surface-400 font-mono">{item.sku}</p>
							</td>
							<td class="px-6 py-4">
								<Badge variant="outline" size="sm">{item.category}</Badge>
							</td>
							<td class="px-6 py-4 text-center">
								<span class="text-sm font-medium {item.quantity <= item.min_quantity ? 'text-danger-500' : 'text-surface-900'}">{item.quantity}</span>
							</td>
							<td class="px-6 py-4 text-center">
								<span class="text-sm text-surface-400">{item.min_quantity}</span>
							</td>
							<td class="px-6 py-4 text-right">
								<span class="text-sm text-surface-700">{formatCurrency(item.unit_cost)}</span>
							</td>
							<td class="px-6 py-4">
								<div class="flex items-center gap-1.5 text-sm text-surface-600">
									<Truck class="w-3.5 h-3.5" />
									<span>{item.location}</span>
								</div>
							</td>
							<td class="px-6 py-4">
								{#if item.status === 'out'}
									<Badge variant="danger" size="sm">Out of Stock</Badge>
								{:else if item.status === 'low'}
									<Badge variant="warning" size="sm">Low Stock</Badge>
								{:else}
									<Badge variant="success" size="sm">In Stock</Badge>
								{/if}
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

		{#if filteredInventory.length === 0}
			<div class="px-6 py-16 text-center">
				<Package class="w-12 h-12 text-surface-300 mx-auto mb-3" />
				<p class="text-sm font-medium text-surface-500">No inventory items found</p>
			</div>
		{/if}
	</Card>
</div>
