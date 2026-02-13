<script lang="ts">
	import TopBar from '$lib/components/layout/TopBar.svelte';
	import Card from '$lib/components/ui/Card.svelte';
	import Button from '$lib/components/ui/Button.svelte';
	import Badge from '$lib/components/ui/Badge.svelte';
	import Input from '$lib/components/ui/Input.svelte';
	import CreateJobForm from '$lib/components/jobs/CreateJobForm.svelte';
	import {
		Plus,
		Filter,
		Search,
		MoreHorizontal,
		Clock,
		MapPin,
		User,
		ChevronDown
	} from 'lucide-svelte';

	let showCreateJob = $state(false);
	let searchQuery = $state('');
	let statusFilter = $state('all');
	let priorityFilter = $state('all');

	const statusOptions = [
		{ value: 'all', label: 'All Statuses' },
		{ value: 'lead', label: 'Lead' },
		{ value: 'estimated', label: 'Estimated' },
		{ value: 'approved', label: 'Approved' },
		{ value: 'scheduled', label: 'Scheduled' },
		{ value: 'en_route', label: 'En Route' },
		{ value: 'in_progress', label: 'In Progress' },
		{ value: 'completed', label: 'Completed' },
		{ value: 'invoiced', label: 'Invoiced' },
		{ value: 'paid', label: 'Paid' }
	];

	function statusVariant(status: string): 'success' | 'warning' | 'danger' | 'info' | 'default' {
		switch (status) {
			case 'completed': case 'paid': return 'success';
			case 'in_progress': case 'en_route': return 'info';
			case 'scheduled': case 'approved': return 'default';
			case 'lead': case 'estimated': return 'warning';
			case 'cancelled': return 'danger';
			default: return 'default';
		}
	}

	function statusLabel(status: string): string {
		return status.replace(/_/g, ' ').replace(/\b\w/g, (c) => c.toUpperCase());
	}

	function priorityVariant(priority: string): 'danger' | 'warning' | 'default' | 'info' {
		switch (priority) {
			case 'emergency': return 'danger';
			case 'high': return 'warning';
			case 'low': return 'info';
			default: return 'default';
		}
	}

	// Demo data
	const jobs = [
		{ id: '1', title: 'AC Unit Replacement', customer: 'Sarah Johnson', status: 'in_progress', priority: 'high', scheduled_date: '2024-12-15', scheduled_time: '9:00 AM', assigned: 'Mike T.', amount: '$4,200', address: '123 Oak St, Austin TX', tags: ['HVAC', 'Residential'] },
		{ id: '2', title: 'Plumbing Inspection', customer: 'Mike Chen', status: 'scheduled', priority: 'normal', scheduled_date: '2024-12-15', scheduled_time: '1:00 PM', assigned: 'Jake R.', amount: '$350', address: '456 Elm Ave, Austin TX', tags: ['Plumbing'] },
		{ id: '3', title: 'Emergency Leak Repair', customer: 'Lisa Rodriguez', status: 'en_route', priority: 'emergency', scheduled_date: '2024-12-15', scheduled_time: '11:30 AM', assigned: 'Mike T.', amount: '$800', address: '789 Pine Dr, Austin TX', tags: ['Plumbing', 'Emergency'] },
		{ id: '4', title: 'Furnace Maintenance', customer: 'Tom Williams', status: 'completed', priority: 'normal', scheduled_date: '2024-12-14', scheduled_time: '3:00 PM', assigned: 'Sarah L.', amount: '$275', address: '321 Maple Ln, Austin TX', tags: ['HVAC'] },
		{ id: '5', title: 'Electrical Panel Upgrade', customer: 'Amy Foster', status: 'approved', priority: 'normal', scheduled_date: '2024-12-16', scheduled_time: '8:00 AM', assigned: 'Unassigned', amount: '$3,500', address: '654 Cedar Blvd, Austin TX', tags: ['Electrical'] },
		{ id: '6', title: 'Water Heater Install', customer: 'David Park', status: 'estimated', priority: 'normal', scheduled_date: '', scheduled_time: '', assigned: 'Unassigned', amount: '$2,100', address: '987 Birch Ct, Austin TX', tags: ['Plumbing'] },
		{ id: '7', title: 'Roof Leak Assessment', customer: 'Karen White', status: 'lead', priority: 'high', scheduled_date: '', scheduled_time: '', assigned: 'Unassigned', amount: '', address: '147 Walnut Way, Austin TX', tags: ['Roofing'] },
		{ id: '8', title: 'Kitchen Remodel — Phase 2', customer: 'Robert Kim', status: 'invoiced', priority: 'normal', scheduled_date: '2024-12-10', scheduled_time: '7:00 AM', assigned: 'Full Crew', amount: '$18,500', address: '258 Spruce St, Austin TX', tags: ['General', 'Remodel'] }
	];

	let filteredJobs = $derived(
		jobs.filter((j) => {
			if (statusFilter !== 'all' && j.status !== statusFilter) return false;
			if (priorityFilter !== 'all' && j.priority !== priorityFilter) return false;
			if (searchQuery) {
				const q = searchQuery.toLowerCase();
				return (
					j.title.toLowerCase().includes(q) ||
					j.customer.toLowerCase().includes(q) ||
					j.address.toLowerCase().includes(q)
				);
			}
			return true;
		})
	);
</script>

<svelte:head>
	<title>Jobs — FieldForge</title>
</svelte:head>

<TopBar title="Jobs">
	{#snippet actions()}
		<Button variant="primary" size="md" onclick={() => (showCreateJob = true)}>
			<Plus class="w-4 h-4" />
			New Job
		</Button>
	{/snippet}
</TopBar>

<CreateJobForm bind:open={showCreateJob} onclose={() => (showCreateJob = false)} />

<div class="p-6 space-y-4">
	<!-- Filters -->
	<div class="flex items-center gap-3 flex-wrap">
		<div class="relative flex-1 max-w-sm">
			<Search class="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-surface-400" />
			<input
				type="text"
				placeholder="Search jobs..."
				bind:value={searchQuery}
				class="w-full pl-9 pr-4 py-2 text-sm bg-white border border-surface-200 rounded-lg focus:outline-none focus:ring-2 focus:ring-forge-500 focus:border-forge-500 placeholder:text-surface-400"
			/>
		</div>

		<select
			bind:value={statusFilter}
			class="px-3 py-2 text-sm bg-white border border-surface-200 rounded-lg focus:outline-none focus:ring-2 focus:ring-forge-500 text-surface-700 cursor-pointer"
		>
			{#each statusOptions as opt}
				<option value={opt.value}>{opt.label}</option>
			{/each}
		</select>

		<select
			bind:value={priorityFilter}
			class="px-3 py-2 text-sm bg-white border border-surface-200 rounded-lg focus:outline-none focus:ring-2 focus:ring-forge-500 text-surface-700 cursor-pointer"
		>
			<option value="all">All Priorities</option>
			<option value="emergency">Emergency</option>
			<option value="high">High</option>
			<option value="normal">Normal</option>
			<option value="low">Low</option>
		</select>

		<span class="text-sm text-surface-400">{filteredJobs.length} jobs</span>
	</div>

	<!-- Jobs Table -->
	<Card padding={false}>
		<div class="overflow-x-auto">
			<table class="w-full">
				<thead>
					<tr class="border-b border-surface-200 bg-surface-50">
						<th class="text-left text-xs font-medium text-surface-500 uppercase tracking-wider px-6 py-3">Job</th>
						<th class="text-left text-xs font-medium text-surface-500 uppercase tracking-wider px-6 py-3">Customer</th>
						<th class="text-left text-xs font-medium text-surface-500 uppercase tracking-wider px-6 py-3">Status</th>
						<th class="text-left text-xs font-medium text-surface-500 uppercase tracking-wider px-6 py-3">Scheduled</th>
						<th class="text-left text-xs font-medium text-surface-500 uppercase tracking-wider px-6 py-3">Assigned</th>
						<th class="text-right text-xs font-medium text-surface-500 uppercase tracking-wider px-6 py-3">Amount</th>
						<th class="w-12 px-3 py-3"></th>
					</tr>
				</thead>
				<tbody class="divide-y divide-surface-100">
					{#each filteredJobs as job (job.id)}
						<tr class="hover:bg-surface-50 transition-colors cursor-pointer" onclick={() => {}}>
							<td class="px-6 py-4">
								<div>
									<a href="/dashboard/jobs/{job.id}" class="text-sm font-medium text-surface-900 hover:text-forge-600">{job.title}</a>
									<div class="flex items-center gap-1.5 mt-1">
										{#each job.tags as tag}
											<Badge variant="outline" size="sm">{tag}</Badge>
										{/each}
									</div>
								</div>
							</td>
							<td class="px-6 py-4">
								<p class="text-sm text-surface-700">{job.customer}</p>
								<div class="flex items-center gap-1 mt-0.5 text-xs text-surface-400">
									<MapPin class="w-3 h-3" />
									<span class="truncate max-w-[180px]">{job.address}</span>
								</div>
							</td>
							<td class="px-6 py-4">
								<div class="flex items-center gap-1.5">
									<Badge variant={statusVariant(job.status)}>{statusLabel(job.status)}</Badge>
									{#if job.priority !== 'normal'}
										<Badge variant={priorityVariant(job.priority)} size="sm">{job.priority}</Badge>
									{/if}
								</div>
							</td>
							<td class="px-6 py-4">
								{#if job.scheduled_date}
									<p class="text-sm text-surface-700">{job.scheduled_date}</p>
									<p class="text-xs text-surface-400">{job.scheduled_time}</p>
								{:else}
									<span class="text-sm text-surface-400">Unscheduled</span>
								{/if}
							</td>
							<td class="px-6 py-4">
								<div class="flex items-center gap-2">
									<div class="w-6 h-6 bg-forge-100 text-forge-600 rounded-full flex items-center justify-center text-xs font-medium">
										<User class="w-3.5 h-3.5" />
									</div>
									<span class="text-sm text-surface-700">{job.assigned}</span>
								</div>
							</td>
							<td class="px-6 py-4 text-right">
								<span class="text-sm font-medium text-surface-900">{job.amount || '—'}</span>
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

		{#if filteredJobs.length === 0}
			<div class="px-6 py-16 text-center">
				<Briefcase class="w-12 h-12 text-surface-300 mx-auto mb-3" />
				<p class="text-sm font-medium text-surface-500">No jobs found</p>
				<p class="text-xs text-surface-400 mt-1">Try adjusting your filters or create a new job</p>
			</div>
		{/if}
	</Card>
</div>
