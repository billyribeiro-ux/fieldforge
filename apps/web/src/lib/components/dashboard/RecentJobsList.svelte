<script lang="ts">
	import Badge from '$lib/components/ui/Badge.svelte';
	import Card from '$lib/components/ui/Card.svelte';
	import { Clock, MapPin } from 'lucide-svelte';

	interface Job {
		id: string;
		title: string;
		customer: string;
		status: string;
		priority: string;
		scheduled: string;
		address?: string;
	}

	interface Props {
		jobs: Job[];
	}

	let { jobs }: Props = $props();

	function statusVariant(status: string): 'success' | 'warning' | 'danger' | 'info' | 'default' {
		switch (status) {
			case 'completed': return 'success';
			case 'in_progress': return 'info';
			case 'scheduled': return 'default';
			case 'en_route': return 'warning';
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
			case 'normal': return 'default';
			case 'low': return 'info';
			default: return 'default';
		}
	}
</script>

<Card padding={false}>
	<div class="px-6 py-4 border-b border-surface-200">
		<div class="flex items-center justify-between">
			<h3 class="text-base font-semibold text-surface-900">Recent Jobs</h3>
			<a href="/dashboard/jobs" class="text-sm font-medium text-forge-600 hover:text-forge-700">View all →</a>
		</div>
	</div>

	<div class="divide-y divide-surface-100">
		{#each jobs as job (job.id)}
			<a href="/dashboard/jobs/{job.id}" class="flex items-center gap-4 px-6 py-4 hover:bg-surface-50 transition-colors">
				<div class="flex-1 min-w-0">
					<div class="flex items-center gap-2 mb-1">
						<p class="text-sm font-medium text-surface-900 truncate">{job.title}</p>
						<Badge variant={statusVariant(job.status)} size="sm">{statusLabel(job.status)}</Badge>
						{#if job.priority !== 'normal'}
							<Badge variant={priorityVariant(job.priority)} size="sm">{job.priority}</Badge>
						{/if}
					</div>
					<p class="text-sm text-surface-500">{job.customer}</p>
				</div>

				<div class="flex flex-col items-end gap-1 text-right flex-shrink-0">
					<div class="flex items-center gap-1 text-xs text-surface-400">
						<Clock class="w-3.5 h-3.5" />
						<span>{job.scheduled}</span>
					</div>
					{#if job.address}
						<div class="flex items-center gap-1 text-xs text-surface-400">
							<MapPin class="w-3.5 h-3.5" />
							<span class="truncate max-w-[150px]">{job.address}</span>
						</div>
					{/if}
				</div>
			</a>
		{/each}

		{#if jobs.length === 0}
			<div class="px-6 py-12 text-center">
				<p class="text-sm text-surface-400">No recent jobs</p>
			</div>
		{/if}
	</div>
</Card>
