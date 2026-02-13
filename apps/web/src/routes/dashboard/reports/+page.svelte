<script lang="ts">
	import TopBar from '$lib/components/layout/TopBar.svelte';
	import Card from '$lib/components/ui/Card.svelte';
	import Badge from '$lib/components/ui/Badge.svelte';
	import Button from '$lib/components/ui/Button.svelte';
	import {
		BarChart3,
		TrendingUp,
		TrendingDown,
		DollarSign,
		Users,
		Briefcase,
		Clock,
		Download,
		Calendar
	} from 'lucide-svelte';

	let dateRange = $state('this_month');

	const revenueData = {
		current: 47850,
		previous: 42500,
		change: 12.6,
		trend: 'up' as const
	};

	const kpis = [
		{ label: 'Revenue', value: '$47,850', change: '+12.6%', trend: 'up', icon: DollarSign, color: 'bg-green-50 text-green-600' },
		{ label: 'Jobs Completed', value: '42', change: '+8', trend: 'up', icon: Briefcase, color: 'bg-forge-50 text-forge-600' },
		{ label: 'New Customers', value: '18', change: '+5', trend: 'up', icon: Users, color: 'bg-purple-50 text-purple-600' },
		{ label: 'Avg Job Time', value: '2.4 hrs', change: '-15 min', trend: 'up', icon: Clock, color: 'bg-accent-50 text-accent-600' }
	];

	const topCustomers = [
		{ name: 'Karen White — White & Sons', revenue: 18500, jobs: 6 },
		{ name: 'Mike Chen — Chen Properties', revenue: 12400, jobs: 4 },
		{ name: 'Amy Foster — Foster Realty', revenue: 8200, jobs: 3 },
		{ name: 'Sarah Johnson', revenue: 4200, jobs: 1 },
		{ name: 'Robert Kim', revenue: 3500, jobs: 1 }
	];

	const revenueByTrade = [
		{ trade: 'HVAC', revenue: 22400, pct: 47 },
		{ trade: 'Plumbing', revenue: 14200, pct: 30 },
		{ trade: 'Electrical', revenue: 7800, pct: 16 },
		{ trade: 'General', revenue: 3450, pct: 7 }
	];

	const techPerformance = [
		{ name: 'Mike Thompson', jobs: 18, revenue: 22400, avg_rating: 4.9, avg_time: '2.1 hrs' },
		{ name: 'Jake Rodriguez', jobs: 14, revenue: 15200, avg_rating: 4.7, avg_time: '2.6 hrs' },
		{ name: 'Sarah Lee', jobs: 10, revenue: 10250, avg_rating: 4.8, avg_time: '2.3 hrs' }
	];

	function formatCurrency(n: number): string {
		return new Intl.NumberFormat('en-US', { style: 'currency', currency: 'USD', minimumFractionDigits: 0 }).format(n);
	}
</script>

<svelte:head>
	<title>Reports — FieldForge</title>
</svelte:head>

<TopBar title="Reports">
	{#snippet actions()}
		<div class="flex items-center gap-2">
			<select
				bind:value={dateRange}
				class="px-3 py-2 text-sm bg-white border border-surface-200 rounded-lg focus:outline-none focus:ring-2 focus:ring-forge-500 text-surface-700 cursor-pointer"
			>
				<option value="today">Today</option>
				<option value="this_week">This Week</option>
				<option value="this_month">This Month</option>
				<option value="this_quarter">This Quarter</option>
				<option value="this_year">This Year</option>
				<option value="last_month">Last Month</option>
				<option value="custom">Custom Range</option>
			</select>
			<Button variant="outline" size="md">
				<Download class="w-4 h-4" />
				Export
			</Button>
		</div>
	{/snippet}
</TopBar>

<div class="p-6 space-y-6">
	<!-- KPI Cards -->
	<div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4">
		{#each kpis as kpi}
			<Card>
				<div class="flex items-start justify-between">
					<div class="space-y-1">
						<p class="text-sm text-surface-500">{kpi.label}</p>
						<p class="text-2xl font-bold text-surface-900">{kpi.value}</p>
						<div class="flex items-center gap-1">
							{#if kpi.trend === 'up'}
								<TrendingUp class="w-3.5 h-3.5 text-green-500" />
							{:else}
								<TrendingDown class="w-3.5 h-3.5 text-red-500" />
							{/if}
							<span class="text-sm font-medium {kpi.trend === 'up' ? 'text-green-600' : 'text-red-600'}">{kpi.change}</span>
							<span class="text-xs text-surface-400">vs last period</span>
						</div>
					</div>
					<div class="p-2.5 rounded-xl {kpi.color}">
						<kpi.icon class="w-5 h-5" />
					</div>
				</div>
			</Card>
		{/each}
	</div>

	<div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
		<!-- Revenue by Trade -->
		<Card>
			<h3 class="text-base font-semibold text-surface-900 mb-4">Revenue by Trade</h3>
			<div class="space-y-4">
				{#each revenueByTrade as item}
					<div>
						<div class="flex items-center justify-between mb-1">
							<span class="text-sm font-medium text-surface-700">{item.trade}</span>
							<div class="flex items-center gap-2">
								<span class="text-sm font-semibold text-surface-900">{formatCurrency(item.revenue)}</span>
								<span class="text-xs text-surface-400">{item.pct}%</span>
							</div>
						</div>
						<div class="w-full bg-surface-100 rounded-full h-2">
							<div class="bg-forge-500 h-2 rounded-full transition-all" style="width: {item.pct}%"></div>
						</div>
					</div>
				{/each}
			</div>
		</Card>

		<!-- Top Customers -->
		<Card padding={false}>
			<div class="px-6 py-4 border-b border-surface-200">
				<h3 class="text-base font-semibold text-surface-900">Top Customers</h3>
			</div>
			<div class="divide-y divide-surface-100">
				{#each topCustomers as customer, i}
					<div class="flex items-center justify-between px-6 py-3">
						<div class="flex items-center gap-3">
							<span class="text-sm font-bold text-surface-400 w-5">{i + 1}</span>
							<span class="text-sm font-medium text-surface-900">{customer.name}</span>
						</div>
						<div class="flex items-center gap-4">
							<span class="text-xs text-surface-400">{customer.jobs} jobs</span>
							<span class="text-sm font-semibold text-surface-900">{formatCurrency(customer.revenue)}</span>
						</div>
					</div>
				{/each}
			</div>
		</Card>
	</div>

	<!-- Technician Performance -->
	<Card padding={false}>
		<div class="px-6 py-4 border-b border-surface-200">
			<h3 class="text-base font-semibold text-surface-900">Technician Performance</h3>
		</div>
		<div class="overflow-x-auto">
			<table class="w-full">
				<thead>
					<tr class="border-b border-surface-200 bg-surface-50">
						<th class="text-left text-xs font-medium text-surface-500 uppercase tracking-wider px-6 py-3">Technician</th>
						<th class="text-center text-xs font-medium text-surface-500 uppercase tracking-wider px-6 py-3">Jobs</th>
						<th class="text-right text-xs font-medium text-surface-500 uppercase tracking-wider px-6 py-3">Revenue</th>
						<th class="text-center text-xs font-medium text-surface-500 uppercase tracking-wider px-6 py-3">Avg Rating</th>
						<th class="text-center text-xs font-medium text-surface-500 uppercase tracking-wider px-6 py-3">Avg Time</th>
					</tr>
				</thead>
				<tbody class="divide-y divide-surface-100">
					{#each techPerformance as tech}
						<tr class="hover:bg-surface-50">
							<td class="px-6 py-4">
								<div class="flex items-center gap-3">
									<div class="w-8 h-8 bg-forge-100 text-forge-600 rounded-full flex items-center justify-center text-xs font-bold">
										{tech.name.split(' ').map(n => n[0]).join('')}
									</div>
									<span class="text-sm font-medium text-surface-900">{tech.name}</span>
								</div>
							</td>
							<td class="px-6 py-4 text-center text-sm text-surface-700">{tech.jobs}</td>
							<td class="px-6 py-4 text-right text-sm font-medium text-surface-900">{formatCurrency(tech.revenue)}</td>
							<td class="px-6 py-4 text-center">
								<span class="text-sm font-medium text-surface-900">{tech.avg_rating} ★</span>
							</td>
							<td class="px-6 py-4 text-center text-sm text-surface-700">{tech.avg_time}</td>
						</tr>
					{/each}
				</tbody>
			</table>
		</div>
	</Card>

	<!-- Revenue Chart Placeholder -->
	<Card>
		<h3 class="text-base font-semibold text-surface-900 mb-4">Revenue Trend</h3>
		<div class="h-64 bg-surface-50 rounded-lg flex items-center justify-center border border-surface-200">
			<div class="text-center">
				<BarChart3 class="w-12 h-12 text-surface-300 mx-auto mb-2" />
				<p class="text-sm text-surface-400">Chart.js integration — coming in Phase 3</p>
				<p class="text-xs text-surface-400 mt-1">Revenue trend visualization with monthly breakdown</p>
			</div>
		</div>
	</Card>
</div>
