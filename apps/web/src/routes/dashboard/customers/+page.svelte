<script lang="ts">
	import TopBar from '$lib/components/layout/TopBar.svelte';
	import Card from '$lib/components/ui/Card.svelte';
	import Button from '$lib/components/ui/Button.svelte';
	import Badge from '$lib/components/ui/Badge.svelte';
	import CreateCustomerForm from '$lib/components/customers/CreateCustomerForm.svelte';
	import {
		Plus,
		Search,
		MoreHorizontal,
		Phone,
		Mail,
		MapPin,
		DollarSign,
		Users
	} from 'lucide-svelte';


	let { data } = $props();
	let showCreateCustomer = $state(false);
	let searchQuery = $state('');
	let viewMode = $state<'table' | 'grid'>('table');

	const customers = [
		{ id: '1', first_name: 'Sarah', last_name: 'Johnson', email: 'sarah@email.com', phone: '(512) 555-0101', company: '', lifetime_value: 12450, outstanding: 4200, total_jobs: 8, tags: ['VIP', 'HVAC'], last_service: '2024-12-10' },
		{ id: '2', first_name: 'Mike', last_name: 'Chen', email: 'mike.chen@email.com', phone: '(512) 555-0102', company: 'Chen Properties LLC', lifetime_value: 34200, outstanding: 0, total_jobs: 15, tags: ['Commercial', 'Property Manager'], last_service: '2024-12-12' },
		{ id: '3', first_name: 'Lisa', last_name: 'Rodriguez', email: 'lisa.r@email.com', phone: '(512) 555-0103', company: '', lifetime_value: 2800, outstanding: 800, total_jobs: 3, tags: ['Residential'], last_service: '2024-12-14' },
		{ id: '4', first_name: 'Tom', last_name: 'Williams', email: 'tom.w@email.com', phone: '(512) 555-0104', company: '', lifetime_value: 8900, outstanding: 0, total_jobs: 6, tags: ['Residential', 'Maintenance Plan'], last_service: '2024-12-14' },
		{ id: '5', first_name: 'Amy', last_name: 'Foster', email: 'amy.foster@email.com', phone: '(512) 555-0105', company: 'Foster Realty', lifetime_value: 22100, outstanding: 3500, total_jobs: 12, tags: ['Commercial'], last_service: '2024-12-08' },
		{ id: '6', first_name: 'David', last_name: 'Park', email: 'david.p@email.com', phone: '(512) 555-0106', company: '', lifetime_value: 1500, outstanding: 2100, total_jobs: 2, tags: ['New'], last_service: '2024-12-13' },
		{ id: '7', first_name: 'Karen', last_name: 'White', email: 'karen.w@email.com', phone: '(512) 555-0107', company: 'White & Sons Construction', lifetime_value: 67800, outstanding: 18500, total_jobs: 28, tags: ['VIP', 'Commercial', 'Subcontractor'], last_service: '2024-12-15' },
		{ id: '8', first_name: 'Robert', last_name: 'Kim', email: 'robert.kim@email.com', phone: '(512) 555-0108', company: '', lifetime_value: 19200, outstanding: 0, total_jobs: 4, tags: ['Residential', 'Remodel'], last_service: '2024-12-10' }
	];

	let filteredCustomers = $derived(
		customers.filter((c) => {
			if (!searchQuery) return true;
			const q = searchQuery.toLowerCase();
			return (
				`${c.first_name} ${c.last_name}`.toLowerCase().includes(q) ||
				c.email.toLowerCase().includes(q) ||
				c.phone.includes(q) ||
				c.company.toLowerCase().includes(q)
			);
		})
	);

	function formatCurrency(amount: number): string {
		return new Intl.NumberFormat('en-US', { style: 'currency', currency: 'USD', minimumFractionDigits: 0 }).format(amount);
	}
</script>

<svelte:head>
	<title>Customers — FieldForge</title>
</svelte:head>

<TopBar title="Customers">
	{#snippet actions()}
		<Button variant="primary" size="md" onclick={() => (showCreateCustomer = true)}>
			<Plus class="w-4 h-4" />
			Add Customer
		</Button>
	{/snippet}
</TopBar>

<CreateCustomerForm bind:open={showCreateCustomer} onclose={() => (showCreateCustomer = false)} />

<div class="p-6 space-y-4">
	<!-- Search & Filters -->
	<div class="flex items-center gap-3">
		<div class="relative flex-1 max-w-sm">
			<Search class="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-surface-400" />
			<input
				type="text"
				placeholder="Search customers..."
				bind:value={searchQuery}
				class="w-full pl-9 pr-4 py-2 text-sm bg-white border border-surface-200 rounded-lg focus:outline-none focus:ring-2 focus:ring-forge-500 focus:border-forge-500 placeholder:text-surface-400"
			/>
		</div>
		<span class="text-sm text-surface-400">{filteredCustomers.length} customers</span>
	</div>

	<!-- Customers Table -->
	<Card padding={false}>
		<div class="overflow-x-auto">
			<table class="w-full">
				<thead>
					<tr class="border-b border-surface-200 bg-surface-50">
						<th class="text-left text-xs font-medium text-surface-500 uppercase tracking-wider px-6 py-3">Customer</th>
						<th class="text-left text-xs font-medium text-surface-500 uppercase tracking-wider px-6 py-3">Contact</th>
						<th class="text-left text-xs font-medium text-surface-500 uppercase tracking-wider px-6 py-3">Tags</th>
						<th class="text-right text-xs font-medium text-surface-500 uppercase tracking-wider px-6 py-3">Lifetime Value</th>
						<th class="text-right text-xs font-medium text-surface-500 uppercase tracking-wider px-6 py-3">Outstanding</th>
						<th class="text-center text-xs font-medium text-surface-500 uppercase tracking-wider px-6 py-3">Jobs</th>
						<th class="text-left text-xs font-medium text-surface-500 uppercase tracking-wider px-6 py-3">Last Service</th>
						<th class="w-12 px-3 py-3"></th>
					</tr>
				</thead>
				<tbody class="divide-y divide-surface-100">
					{#each filteredCustomers as customer (customer.id)}
						<tr class="hover:bg-surface-50 transition-colors">
							<td class="px-6 py-4">
								<a href="/dashboard/customers/{customer.id}" class="block">
									<div class="flex items-center gap-3">
										<div class="w-9 h-9 bg-forge-100 text-forge-600 rounded-full flex items-center justify-center text-sm font-bold flex-shrink-0">
											{customer.first_name[0]}{customer.last_name[0]}
										</div>
										<div>
											<p class="text-sm font-medium text-surface-900 hover:text-forge-600">{customer.first_name} {customer.last_name}</p>
											{#if customer.company}
												<p class="text-xs text-surface-400">{customer.company}</p>
											{/if}
										</div>
									</div>
								</a>
							</td>
							<td class="px-6 py-4">
								<div class="space-y-1">
									<div class="flex items-center gap-1.5 text-xs text-surface-500">
										<Phone class="w-3 h-3" />
										<span>{customer.phone}</span>
									</div>
									<div class="flex items-center gap-1.5 text-xs text-surface-500">
										<Mail class="w-3 h-3" />
										<span>{customer.email}</span>
									</div>
								</div>
							</td>
							<td class="px-6 py-4">
								<div class="flex items-center gap-1 flex-wrap">
									{#each customer.tags as tag}
										<Badge variant="outline" size="sm">{tag}</Badge>
									{/each}
								</div>
							</td>
							<td class="px-6 py-4 text-right">
								<span class="text-sm font-medium text-surface-900">{formatCurrency(customer.lifetime_value)}</span>
							</td>
							<td class="px-6 py-4 text-right">
								{#if customer.outstanding > 0}
									<span class="text-sm font-medium text-danger-500">{formatCurrency(customer.outstanding)}</span>
								{:else}
									<span class="text-sm text-surface-400">—</span>
								{/if}
							</td>
							<td class="px-6 py-4 text-center">
								<span class="text-sm text-surface-700">{customer.total_jobs}</span>
							</td>
							<td class="px-6 py-4">
								<span class="text-sm text-surface-500">{customer.last_service}</span>
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

		{#if filteredCustomers.length === 0}
			<div class="px-6 py-16 text-center">
				<Users class="w-12 h-12 text-surface-300 mx-auto mb-3" />
				<p class="text-sm font-medium text-surface-500">No customers found</p>
				<p class="text-xs text-surface-400 mt-1">Try adjusting your search or add a new customer</p>
			</div>
		{/if}
	</Card>
</div>
