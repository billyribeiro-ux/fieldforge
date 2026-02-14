<script lang="ts">
	import TopBar from '$lib/components/layout/TopBar.svelte';
	import StatCard from '$lib/components/dashboard/StatCard.svelte';
	import RecentJobsList from '$lib/components/dashboard/RecentJobsList.svelte';
	import Card from '$lib/components/ui/Card.svelte';
	import Button from '$lib/components/ui/Button.svelte';
	import Badge from '$lib/components/ui/Badge.svelte';
	import {
		Briefcase,
		DollarSign,
		Users,
		Clock,
		Plus,
		TrendingUp,
		AlertTriangle,
		CheckCircle2
	} from 'lucide-svelte';

	let { data } = $props();

	// Use server data when available, fallback to demo
	const serverJobs = (data?.recentJobs ?? []) as any[];
	const serverCustomers = (data?.recentCustomers ?? []) as any[];

	const stats = [
		{
			title: 'Active Jobs',
			value: '24',
			change: '+3 from yesterday',
			changeType: 'positive' as const,
			icon: Briefcase,
			iconColor: 'bg-forge-50 text-forge-600'
		},
		{
			title: 'Revenue (MTD)',
			value: '$47,850',
			change: '+12.5% vs last month',
			changeType: 'positive' as const,
			icon: DollarSign,
			iconColor: 'bg-green-50 text-green-600'
		},
		{
			title: 'Customers',
			value: '312',
			change: '+8 this week',
			changeType: 'positive' as const,
			icon: Users,
			iconColor: 'bg-purple-50 text-purple-600'
		},
		{
			title: 'Avg. Completion',
			value: '2.4 hrs',
			change: '-15 min vs avg',
			changeType: 'positive' as const,
			icon: Clock,
			iconColor: 'bg-accent-50 text-accent-600'
		}
	];

	// Demo fallback data
	const demoRecentJobs = [
		{
			id: '1',
			title: 'AC Unit Replacement',
			customer: 'Sarah Johnson',
			status: 'in_progress',
			priority: 'high',
			scheduled: 'Today, 9:00 AM',
			address: '123 Oak St, Austin TX'
		},
		{
			id: '2',
			title: 'Plumbing Inspection',
			customer: 'Mike Chen',
			status: 'scheduled',
			priority: 'normal',
			scheduled: 'Today, 1:00 PM',
			address: '456 Elm Ave, Austin TX'
		},
		{
			id: '3',
			title: 'Emergency Leak Repair',
			customer: 'Lisa Rodriguez',
			status: 'en_route',
			priority: 'emergency',
			scheduled: 'Today, 11:30 AM',
			address: '789 Pine Dr, Austin TX'
		},
		{
			id: '4',
			title: 'Furnace Maintenance',
			customer: 'Tom Williams',
			status: 'completed',
			priority: 'normal',
			scheduled: 'Yesterday, 3:00 PM',
			address: '321 Maple Ln, Austin TX'
		},
		{
			id: '5',
			title: 'Electrical Panel Upgrade',
			customer: 'Amy Foster',
			status: 'scheduled',
			priority: 'normal',
			scheduled: 'Tomorrow, 8:00 AM',
			address: '654 Cedar Blvd, Austin TX'
		}
	];

	// Use server data when available, fallback to demo
	const recentJobs = serverJobs.length > 0 ? serverJobs.map((j: any) => ({
		id: j.id,
		title: j.title ?? '',
		customer: `${j.customer_first_name ?? ''} ${j.customer_last_name ?? ''}`.trim() || 'Unknown',
		status: j.status ?? 'lead',
		priority: j.priority ?? 'normal',
		scheduled: j.scheduled_date ?? '',
		address: ''
	})) : demoRecentJobs;

	const todaySchedule = [
		{ time: '8:00 AM', title: 'Team standup', type: 'meeting' },
		{ time: '9:00 AM', title: 'AC Unit Replacement — Sarah Johnson', type: 'job' },
		{ time: '11:30 AM', title: 'Emergency Leak Repair — Lisa Rodriguez', type: 'emergency' },
		{ time: '1:00 PM', title: 'Plumbing Inspection — Mike Chen', type: 'job' },
		{ time: '3:30 PM', title: 'Estimate: Kitchen Remodel — David Park', type: 'estimate' }
	];

	const alerts = [
		{ message: '3 invoices overdue (>30 days)', type: 'danger' as const },
		{ message: 'License renewal due in 14 days', type: 'warning' as const },
		{ message: 'Low stock: Copper fittings (12 remaining)', type: 'warning' as const }
	];
</script>

<svelte:head>
	<title>Dashboard — FieldForge</title>
</svelte:head>

<TopBar title="Dashboard">
	{#snippet actions()}
		<Button variant="primary" size="md" href="/dashboard/jobs/new">
			<Plus class="w-4 h-4" />
			New Job
		</Button>
	{/snippet}
</TopBar>

<div class="p-6 space-y-6">
	<!-- Stats Grid -->
	<div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4">
		{#each stats as stat}
			<StatCard {...stat} />
		{/each}
	</div>

	<!-- Main Content Grid -->
	<div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
		<!-- Recent Jobs (2/3 width) -->
		<div class="lg:col-span-2">
			<RecentJobsList jobs={recentJobs} />
		</div>

		<!-- Right Column -->
		<div class="space-y-6">
			<!-- Today's Schedule -->
			<Card padding={false}>
				<div class="px-6 py-4 border-b border-surface-200">
					<div class="flex items-center justify-between">
						<h3 class="text-base font-semibold text-surface-900">Today's Schedule</h3>
						<a href="/dashboard/schedule" class="text-sm font-medium text-forge-600 hover:text-forge-700">View →</a>
					</div>
				</div>
				<div class="divide-y divide-surface-100">
					{#each todaySchedule as item}
						<div class="flex items-start gap-3 px-6 py-3">
							<span class="text-xs font-mono text-surface-400 w-16 flex-shrink-0 pt-0.5">{item.time}</span>
							<div class="flex-1 min-w-0">
								<p class="text-sm text-surface-700 truncate">{item.title}</p>
							</div>
							{#if item.type === 'emergency'}
								<Badge variant="danger" size="sm">URGENT</Badge>
							{:else if item.type === 'estimate'}
								<Badge variant="info" size="sm">Estimate</Badge>
							{/if}
						</div>
					{/each}
				</div>
			</Card>

			<!-- Alerts -->
			<Card padding={false}>
				<div class="px-6 py-4 border-b border-surface-200">
					<h3 class="text-base font-semibold text-surface-900">Alerts</h3>
				</div>
				<div class="divide-y divide-surface-100">
					{#each alerts as alert}
						<div class="flex items-start gap-3 px-6 py-3">
							{#if alert.type === 'danger'}
								<AlertTriangle class="w-4 h-4 text-danger-500 flex-shrink-0 mt-0.5" />
							{:else}
								<AlertTriangle class="w-4 h-4 text-warning-500 flex-shrink-0 mt-0.5" />
							{/if}
							<p class="text-sm text-surface-700">{alert.message}</p>
						</div>
					{/each}
				</div>
			</Card>

			<!-- Quick Stats -->
			<Card>
				<h3 class="text-base font-semibold text-surface-900 mb-4">This Week</h3>
				<div class="space-y-3">
					<div class="flex items-center justify-between">
						<span class="text-sm text-surface-500">Jobs Completed</span>
						<div class="flex items-center gap-1.5">
							<CheckCircle2 class="w-4 h-4 text-green-500" />
							<span class="text-sm font-semibold text-surface-900">18</span>
						</div>
					</div>
					<div class="flex items-center justify-between">
						<span class="text-sm text-surface-500">Estimates Sent</span>
						<span class="text-sm font-semibold text-surface-900">7</span>
					</div>
					<div class="flex items-center justify-between">
						<span class="text-sm text-surface-500">Invoices Collected</span>
						<div class="flex items-center gap-1.5">
							<TrendingUp class="w-4 h-4 text-green-500" />
							<span class="text-sm font-semibold text-surface-900">$12,450</span>
						</div>
					</div>
					<div class="flex items-center justify-between">
						<span class="text-sm text-surface-500">Customer Rating</span>
						<span class="text-sm font-semibold text-surface-900">4.8 ★</span>
					</div>
				</div>
			</Card>
		</div>
	</div>
</div>
