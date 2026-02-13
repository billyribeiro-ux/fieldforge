<script lang="ts">
	import TopBar from '$lib/components/layout/TopBar.svelte';
	import Card from '$lib/components/ui/Card.svelte';
	import Button from '$lib/components/ui/Button.svelte';
	import Badge from '$lib/components/ui/Badge.svelte';
	import { MapPin, Clock, Phone, Navigation, User, ChevronRight, AlertTriangle, CheckCircle2 } from 'lucide-svelte';

	const technicians = [
		{
			id: '1', name: 'Mike Thompson', status: 'on_job', phone: '(512) 555-0101',
			currentJob: { title: 'AC Unit Replacement', customer: 'Sarah Johnson', address: '123 Oak St', eta: null, progress: 65 },
			nextJob: { title: 'Plumbing Inspection', customer: 'Mike Chen', address: '456 Elm Ave', time: '1:00 PM' },
			jobsToday: 3, jobsCompleted: 1, lat: 30.2672, lng: -97.7431
		},
		{
			id: '2', name: 'Jake Rodriguez', status: 'available', phone: '(512) 555-0102',
			currentJob: null,
			nextJob: { title: 'Thermostat Install', customer: 'Amy Foster', address: '654 Cedar Blvd', time: '2:00 PM' },
			jobsToday: 2, jobsCompleted: 2, lat: 30.2849, lng: -97.7341
		},
		{
			id: '3', name: 'Sarah Lee', status: 'en_route', phone: '(512) 555-0103',
			currentJob: { title: 'Emergency Leak Repair', customer: 'Lisa Rodriguez', address: '789 Pine Dr', eta: '12 min', progress: 0 },
			nextJob: { title: 'Water Heater Check', customer: 'David Park', address: '987 Birch Ct', time: '3:30 PM' },
			jobsToday: 4, jobsCompleted: 1, lat: 30.2500, lng: -97.7500
		}
	];

	const unassignedJobs = [
		{ id: '1', title: 'Roof Leak Assessment', customer: 'Karen White', address: '147 Walnut Way', priority: 'high', scheduledTime: 'Today, 4:00 PM' },
		{ id: '2', title: 'Electrical Panel Upgrade', customer: 'Amy Foster', address: '654 Cedar Blvd', priority: 'normal', scheduledTime: 'Tomorrow, 8:00 AM' },
		{ id: '3', title: 'Furnace Tune-Up', customer: 'New Lead', address: '321 Maple Ln', priority: 'low', scheduledTime: 'Tomorrow, 10:00 AM' }
	];

	function statusColor(s: string): string {
		switch (s) {
			case 'available': return 'bg-success-500';
			case 'on_job': return 'bg-forge-500';
			case 'en_route': return 'bg-accent-500';
			case 'on_break': return 'bg-warning-500';
			default: return 'bg-surface-400';
		}
	}

	function statusLabel(s: string): string {
		return s.replace(/_/g, ' ').replace(/\b\w/g, (c) => c.toUpperCase());
	}

	function priorityVariant(p: string): 'danger' | 'warning' | 'info' | 'default' {
		switch (p) {
			case 'emergency': return 'danger';
			case 'high': return 'warning';
			case 'low': return 'info';
			default: return 'default';
		}
	}

	function initials(name: string): string {
		return name.split(' ').map((n) => n[0]).join('').toUpperCase();
	}
</script>

<TopBar title="Dispatch Board">
	{#snippet actions()}
		<div class="flex items-center gap-2">
			<Badge variant="success" size="md">{technicians.filter((t) => t.status === 'available').length} Available</Badge>
			<Badge variant="info" size="md">{technicians.filter((t) => t.status === 'on_job' || t.status === 'en_route').length} Active</Badge>
			<Badge variant="warning" size="md">{unassignedJobs.length} Unassigned</Badge>
		</div>
	{/snippet}
</TopBar>

<div class="p-6">
	<div class="grid grid-cols-1 xl:grid-cols-3 gap-6">
		<!-- Technician Cards -->
		<div class="xl:col-span-2 space-y-4">
			<h2 class="text-sm font-semibold text-surface-500 uppercase tracking-wider">Technicians</h2>

			{#each technicians as tech (tech.id)}
				<Card>
					<div class="flex items-start justify-between mb-4">
						<div class="flex items-center gap-3">
							<div class="relative">
								<div class="w-11 h-11 bg-forge-100 text-forge-700 rounded-full flex items-center justify-center text-sm font-bold">
									{initials(tech.name)}
								</div>
								<div class="absolute -bottom-0.5 -right-0.5 w-3.5 h-3.5 rounded-full border-2 border-white {statusColor(tech.status)}"></div>
							</div>
							<div>
								<p class="text-sm font-semibold text-surface-900">{tech.name}</p>
								<p class="text-xs text-surface-400">{statusLabel(tech.status)} • {tech.jobsCompleted}/{tech.jobsToday} jobs</p>
							</div>
						</div>
						<div class="flex items-center gap-2">
							<a href="tel:{tech.phone}" class="p-1.5 text-surface-400 hover:text-forge-600 rounded-lg hover:bg-surface-100 transition-colors">
								<Phone class="w-4 h-4" />
							</a>
						</div>
					</div>

					{#if tech.currentJob}
						<div class="p-3 bg-forge-50 rounded-lg border border-forge-100 mb-3">
							<div class="flex items-center justify-between mb-1">
								<p class="text-xs font-medium text-forge-700 uppercase tracking-wider">
									{tech.status === 'en_route' ? 'En Route To' : 'Currently Working'}
								</p>
								{#if tech.currentJob.eta}
									<Badge variant="info" size="sm">ETA {tech.currentJob.eta}</Badge>
								{/if}
							</div>
							<p class="text-sm font-semibold text-surface-900">{tech.currentJob.title}</p>
							<p class="text-xs text-surface-500">{tech.currentJob.customer} • {tech.currentJob.address}</p>
							{#if tech.currentJob.progress > 0}
								<div class="mt-2 h-1.5 bg-forge-100 rounded-full overflow-hidden">
									<div class="h-full bg-forge-500 rounded-full transition-all" style="width: {tech.currentJob.progress}%"></div>
								</div>
								<p class="text-xs text-surface-400 mt-1">{tech.currentJob.progress}% complete</p>
							{/if}
						</div>
					{/if}

					{#if tech.nextJob}
						<div class="p-3 bg-surface-50 rounded-lg border border-surface-100">
							<p class="text-xs font-medium text-surface-400 uppercase tracking-wider mb-1">Next Up</p>
							<div class="flex items-center justify-between">
								<div>
									<p class="text-sm font-medium text-surface-700">{tech.nextJob.title}</p>
									<p class="text-xs text-surface-400">{tech.nextJob.customer} • {tech.nextJob.time}</p>
								</div>
								<button class="p-1.5 text-surface-400 hover:text-forge-600 rounded cursor-pointer">
									<Navigation class="w-4 h-4" />
								</button>
							</div>
						</div>
					{:else if !tech.currentJob}
						<div class="p-3 bg-success-50 rounded-lg border border-success-100 text-center">
							<CheckCircle2 class="w-5 h-5 text-success-500 mx-auto mb-1" />
							<p class="text-sm text-success-700 font-medium">All jobs complete!</p>
						</div>
					{/if}
				</Card>
			{/each}
		</div>

		<!-- Unassigned Jobs -->
		<div class="space-y-4">
			<h2 class="text-sm font-semibold text-surface-500 uppercase tracking-wider">Unassigned Jobs</h2>

			{#each unassignedJobs as job (job.id)}
				<Card class="border-l-4 {job.priority === 'high' ? 'border-l-warning-400' : job.priority === 'emergency' ? 'border-l-danger-400' : 'border-l-surface-300'}">
					<div class="flex items-start justify-between mb-2">
						<div>
							<p class="text-sm font-semibold text-surface-900">{job.title}</p>
							<p class="text-xs text-surface-500">{job.customer}</p>
						</div>
						<Badge variant={priorityVariant(job.priority)} size="sm">
							{job.priority.charAt(0).toUpperCase() + job.priority.slice(1)}
						</Badge>
					</div>
					<div class="flex items-center gap-3 text-xs text-surface-400 mb-3">
						<span class="flex items-center gap-1"><MapPin class="w-3 h-3" /> {job.address}</span>
						<span class="flex items-center gap-1"><Clock class="w-3 h-3" /> {job.scheduledTime}</span>
					</div>
					<div class="flex gap-2">
						{#each technicians.filter((t) => t.status === 'available') as tech}
							<Button variant="outline" size="sm" class="text-xs">
								Assign {tech.name.split(' ')[0]}
							</Button>
						{/each}
					</div>
				</Card>
			{/each}

			{#if unassignedJobs.length === 0}
				<Card>
					<div class="text-center py-6">
						<CheckCircle2 class="w-8 h-8 text-success-400 mx-auto mb-2" />
						<p class="text-sm text-surface-500">All jobs assigned!</p>
					</div>
				</Card>
			{/if}

			<!-- Map Placeholder -->
			<Card>
				<h3 class="text-sm font-semibold text-surface-500 uppercase tracking-wider mb-3">Live Map</h3>
				<div class="aspect-[4/3] bg-surface-100 rounded-lg flex items-center justify-center">
					<div class="text-center">
						<MapPin class="w-8 h-8 text-surface-300 mx-auto mb-2" />
						<p class="text-sm text-surface-400">Map integration</p>
						<p class="text-xs text-surface-300">MapLibre GL + Mapbox tiles</p>
					</div>
				</div>
			</Card>
		</div>
	</div>
</div>
