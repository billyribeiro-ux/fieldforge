<script lang="ts">
	import TopBar from '$lib/components/layout/TopBar.svelte';
	import Card from '$lib/components/ui/Card.svelte';
	import Button from '$lib/components/ui/Button.svelte';
	import Badge from '$lib/components/ui/Badge.svelte';
	import Modal from '$lib/components/ui/Modal.svelte';
	import Input from '$lib/components/ui/Input.svelte';
	import Select from '$lib/components/ui/Select.svelte';
	import Textarea from '$lib/components/ui/Textarea.svelte';
	import { Plus, Search, DollarSign, TrendingUp, Receipt, Filter } from 'lucide-svelte';


	let { data } = $props();
	let searchQuery = $state('');
	let categoryFilter = $state('all');
	let showAddModal = $state(false);

	const expenses = [
		{ id: '1', description: 'Carrier 24ACC636 AC Unit', category: 'materials', amount: 2400, vendor: 'HVAC Supply Co', expense_date: '2024-12-10', job: 'AC Unit Replacement', is_billable: true, reimbursed: false },
		{ id: '2', description: 'Copper pipe fittings (assorted)', category: 'materials', amount: 185.50, vendor: 'Home Depot', expense_date: '2024-12-09', job: 'Kitchen Remodel Phase 2', is_billable: true, reimbursed: false },
		{ id: '3', description: 'Fuel — Service Van #1', category: 'fuel', amount: 78.42, vendor: 'Shell', expense_date: '2024-12-09', job: null, is_billable: false, reimbursed: false },
		{ id: '4', description: 'Electrical panel (200A)', category: 'materials', amount: 650, vendor: 'Electrical Wholesale', expense_date: '2024-12-08', job: 'Electrical Panel Upgrade', is_billable: true, reimbursed: false },
		{ id: '5', description: 'Permit fee — City of Austin', category: 'permits', amount: 125, vendor: 'City of Austin', expense_date: '2024-12-07', job: 'Electrical Panel Upgrade', is_billable: true, reimbursed: false },
		{ id: '6', description: 'Tool rental — Pipe threader', category: 'equipment', amount: 95, vendor: 'Sunbelt Rentals', expense_date: '2024-12-06', job: 'Commercial HVAC Install', is_billable: true, reimbursed: false },
		{ id: '7', description: 'Office supplies', category: 'overhead', amount: 42.30, vendor: 'Staples', expense_date: '2024-12-05', job: null, is_billable: false, reimbursed: false },
		{ id: '8', description: 'Mileage — 47 miles', category: 'mileage', amount: 30.55, vendor: null, expense_date: '2024-12-04', job: 'Furnace Maintenance', is_billable: true, reimbursed: true }
	];

	const categories = [
		{ value: 'all', label: 'All Categories' },
		{ value: 'materials', label: 'Materials' },
		{ value: 'fuel', label: 'Fuel' },
		{ value: 'equipment', label: 'Equipment' },
		{ value: 'permits', label: 'Permits' },
		{ value: 'mileage', label: 'Mileage' },
		{ value: 'overhead', label: 'Overhead' },
		{ value: 'subcontractor', label: 'Subcontractor' },
		{ value: 'other', label: 'Other' }
	];

	let filteredExpenses = $derived(
		expenses.filter((exp) => {
			if (categoryFilter !== 'all' && exp.category !== categoryFilter) return false;
			if (searchQuery) {
				const q = searchQuery.toLowerCase();
				return exp.description.toLowerCase().includes(q) || (exp.vendor?.toLowerCase().includes(q) ?? false) || (exp.job?.toLowerCase().includes(q) ?? false);
			}
			return true;
		})
	);

	let totalAmount = $derived(filteredExpenses.reduce((sum, e) => sum + e.amount, 0));
	let billableAmount = $derived(filteredExpenses.filter((e) => e.is_billable).reduce((sum, e) => sum + e.amount, 0));

	function formatCurrency(n: number): string {
		return new Intl.NumberFormat('en-US', { style: 'currency', currency: 'USD', minimumFractionDigits: 2 }).format(n);
	}

	function categoryColor(cat: string): 'success' | 'warning' | 'danger' | 'info' | 'default' {
		switch (cat) {
			case 'materials': return 'info';
			case 'fuel': return 'warning';
			case 'equipment': return 'default';
			case 'permits': return 'success';
			case 'mileage': return 'default';
			case 'overhead': return 'danger';
			default: return 'default';
		}
	}

	// Add expense form state
	let newDescription = $state('');
	let newCategory = $state('materials');
	let newAmount = $state('');
	let newVendor = $state('');
	let newDate = $state('');
	let newNotes = $state('');
</script>

<svelte:head>
	<title>Expenses — FieldForge</title>
</svelte:head>

<TopBar title="Expenses">
	{#snippet actions()}
		<Button variant="primary" size="md" onclick={() => (showAddModal = true)}>
			<Plus class="w-4 h-4" />
			Add Expense
		</Button>
	{/snippet}
</TopBar>

<div class="p-6 space-y-6">
	<!-- Summary Cards -->
	<div class="grid grid-cols-1 sm:grid-cols-3 gap-4">
		<Card>
			<div class="flex items-center gap-3">
				<div class="w-10 h-10 bg-surface-100 rounded-lg flex items-center justify-center">
					<DollarSign class="w-5 h-5 text-surface-600" />
				</div>
				<div>
					<p class="text-xs text-surface-400 uppercase tracking-wider">Total Expenses</p>
					<p class="text-xl font-bold text-surface-900">{formatCurrency(totalAmount)}</p>
				</div>
			</div>
		</Card>
		<Card>
			<div class="flex items-center gap-3">
				<div class="w-10 h-10 bg-forge-50 rounded-lg flex items-center justify-center">
					<Receipt class="w-5 h-5 text-forge-600" />
				</div>
				<div>
					<p class="text-xs text-surface-400 uppercase tracking-wider">Billable</p>
					<p class="text-xl font-bold text-forge-600">{formatCurrency(billableAmount)}</p>
				</div>
			</div>
		</Card>
		<Card>
			<div class="flex items-center gap-3">
				<div class="w-10 h-10 bg-success-50 rounded-lg flex items-center justify-center">
					<TrendingUp class="w-5 h-5 text-success-600" />
				</div>
				<div>
					<p class="text-xs text-surface-400 uppercase tracking-wider">This Month</p>
					<p class="text-xl font-bold text-surface-900">{filteredExpenses.length} entries</p>
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
				placeholder="Search expenses..."
				bind:value={searchQuery}
				class="w-full pl-9 pr-4 py-2 text-sm bg-white border border-surface-200 rounded-lg focus:outline-none focus:ring-2 focus:ring-forge-500 placeholder:text-surface-400"
			/>
		</div>
		<select
			bind:value={categoryFilter}
			class="px-3 py-2 text-sm bg-white border border-surface-200 rounded-lg focus:outline-none focus:ring-2 focus:ring-forge-500 text-surface-700 cursor-pointer"
		>
			{#each categories as cat}
				<option value={cat.value}>{cat.label}</option>
			{/each}
		</select>
		<span class="text-sm text-surface-400">{filteredExpenses.length} expenses</span>
	</div>

	<!-- Expenses Table -->
	<Card padding={false}>
		<div class="overflow-x-auto">
			<table class="w-full">
				<thead>
					<tr class="border-b border-surface-200 bg-surface-50">
						<th class="text-left text-xs font-medium text-surface-500 uppercase tracking-wider px-6 py-3">Date</th>
						<th class="text-left text-xs font-medium text-surface-500 uppercase tracking-wider px-6 py-3">Description</th>
						<th class="text-left text-xs font-medium text-surface-500 uppercase tracking-wider px-6 py-3">Category</th>
						<th class="text-left text-xs font-medium text-surface-500 uppercase tracking-wider px-6 py-3">Vendor</th>
						<th class="text-left text-xs font-medium text-surface-500 uppercase tracking-wider px-6 py-3">Job</th>
						<th class="text-right text-xs font-medium text-surface-500 uppercase tracking-wider px-6 py-3">Amount</th>
						<th class="text-center text-xs font-medium text-surface-500 uppercase tracking-wider px-6 py-3">Billable</th>
					</tr>
				</thead>
				<tbody class="divide-y divide-surface-100">
					{#each filteredExpenses as expense}
						<tr class="hover:bg-surface-50 transition-colors">
							<td class="px-6 py-4 text-sm text-surface-500 whitespace-nowrap">{expense.expense_date}</td>
							<td class="px-6 py-4">
								<p class="text-sm font-medium text-surface-900">{expense.description}</p>
							</td>
							<td class="px-6 py-4">
								<Badge variant={categoryColor(expense.category)} size="sm">
									{expense.category.charAt(0).toUpperCase() + expense.category.slice(1)}
								</Badge>
							</td>
							<td class="px-6 py-4 text-sm text-surface-600">{expense.vendor ?? '—'}</td>
							<td class="px-6 py-4 text-sm text-surface-600">{expense.job ?? '—'}</td>
							<td class="px-6 py-4 text-sm font-semibold text-surface-900 text-right">{formatCurrency(expense.amount)}</td>
							<td class="px-6 py-4 text-center">
								{#if expense.is_billable}
									<span class="inline-block w-2 h-2 bg-success-500 rounded-full"></span>
								{:else}
									<span class="inline-block w-2 h-2 bg-surface-300 rounded-full"></span>
								{/if}
							</td>
						</tr>
					{/each}
				</tbody>
			</table>
		</div>
	</Card>
</div>

<!-- Add Expense Modal -->
<Modal bind:open={showAddModal} title="Add Expense" size="md">
	<div class="space-y-4">
		<Input label="Description" bind:value={newDescription} required placeholder="What was the expense for?" />
		<div class="grid grid-cols-2 gap-4">
			<Select label="Category" options={categories.filter((c) => c.value !== 'all')} bind:value={newCategory} />
			<Input label="Amount" type="number" bind:value={newAmount} required placeholder="0.00" />
		</div>
		<div class="grid grid-cols-2 gap-4">
			<Input label="Vendor" bind:value={newVendor} placeholder="Vendor name" />
			<Input label="Date" type="date" bind:value={newDate} required />
		</div>
		<Textarea label="Notes" bind:value={newNotes} rows={2} placeholder="Additional notes..." />
	</div>
	{#snippet footer()}
		<div class="flex justify-end gap-3">
			<Button variant="outline" size="md" onclick={() => (showAddModal = false)}>Cancel</Button>
			<Button variant="primary" size="md" onclick={() => (showAddModal = false)}>Add Expense</Button>
		</div>
	{/snippet}
</Modal>
