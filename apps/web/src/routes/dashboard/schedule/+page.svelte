<script lang="ts">
	import TopBar from '$lib/components/layout/TopBar.svelte';
	import Card from '$lib/components/ui/Card.svelte';
	import Badge from '$lib/components/ui/Badge.svelte';
	import Button from '$lib/components/ui/Button.svelte';
	import { Plus, ChevronLeft, ChevronRight, Clock, MapPin, User } from 'lucide-svelte';


	let { data } = $props();
	let currentDate = $state(new Date());
	let viewMode = $state<'day' | 'week' | 'month'>('week');

	const hours = Array.from({ length: 12 }, (_, i) => i + 7); // 7 AM to 6 PM

	const weekDays = ['Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat', 'Sun'];

	// Demo fallback data
	const demoScheduledJobs = [
		{ id: '1', title: 'AC Unit Replacement', customer: 'Sarah Johnson', technician: 'Mike T.', day: 0, startHour: 9, duration: 4, color: 'bg-forge-100 border-forge-400 text-forge-800' },
		{ id: '2', title: 'Plumbing Inspection', customer: 'Mike Chen', technician: 'Jake R.', day: 0, startHour: 13, duration: 2, color: 'bg-blue-100 border-blue-400 text-blue-800' },
		{ id: '3', title: 'Emergency Leak Repair', customer: 'Lisa Rodriguez', technician: 'Mike T.', day: 0, startHour: 11, duration: 2, color: 'bg-red-100 border-red-400 text-red-800' },
		{ id: '4', title: 'Electrical Panel Upgrade', customer: 'Amy Foster', technician: 'Sarah L.', day: 1, startHour: 8, duration: 6, color: 'bg-yellow-100 border-yellow-400 text-yellow-800' },
		{ id: '5', title: 'Furnace Tune-up', customer: 'Tom Williams', technician: 'Jake R.', day: 2, startHour: 10, duration: 2, color: 'bg-green-100 border-green-400 text-green-800' },
		{ id: '6', title: 'Water Heater Install', customer: 'David Park', technician: 'Mike T.', day: 3, startHour: 9, duration: 5, color: 'bg-purple-100 border-purple-400 text-purple-800' },
		{ id: '7', title: 'Duct Cleaning', customer: 'Karen White', technician: 'Sarah L.', day: 4, startHour: 8, duration: 3, color: 'bg-orange-100 border-orange-400 text-orange-800' }
	];

	// Use server data when available, fallback to demo
	const serverScheduledJobs = (data?.scheduledJobs ?? []) as any[];
	const scheduledJobs = serverScheduledJobs.length > 0 ? serverScheduledJobs : demoScheduledJobs;

	function getDateString(): string {
		return currentDate.toLocaleDateString('en-US', { month: 'long', year: 'numeric' });
	}

	function getWeekRange(): string {
		const start = new Date(currentDate);
		start.setDate(start.getDate() - start.getDay() + 1);
		const end = new Date(start);
		end.setDate(end.getDate() + 6);
		return `${start.toLocaleDateString('en-US', { month: 'short', day: 'numeric' })} — ${end.toLocaleDateString('en-US', { month: 'short', day: 'numeric', year: 'numeric' })}`;
	}

	function prevWeek() {
		const d = new Date(currentDate);
		d.setDate(d.getDate() - 7);
		currentDate = d;
	}

	function nextWeek() {
		const d = new Date(currentDate);
		d.setDate(d.getDate() + 7);
		currentDate = d;
	}
</script>

<svelte:head>
	<title>Schedule — FieldForge</title>
</svelte:head>

<TopBar title="Schedule">
	{#snippet actions()}
		<Button variant="primary" size="md">
			<Plus class="w-4 h-4" />
			Schedule Job
		</Button>
	{/snippet}
</TopBar>

<div class="p-6 space-y-4">
	<!-- Calendar Controls -->
	<div class="flex items-center justify-between">
		<div class="flex items-center gap-3">
			<button onclick={prevWeek} class="p-2 hover:bg-surface-100 rounded-lg transition-colors cursor-pointer">
				<ChevronLeft class="w-5 h-5 text-surface-600" />
			</button>
			<h2 class="text-lg font-semibold text-surface-900">{getWeekRange()}</h2>
			<button onclick={nextWeek} class="p-2 hover:bg-surface-100 rounded-lg transition-colors cursor-pointer">
				<ChevronRight class="w-5 h-5 text-surface-600" />
			</button>
			<Button variant="ghost" size="sm" onclick={() => (currentDate = new Date())}>Today</Button>
		</div>

		<div class="flex rounded-lg bg-surface-100 p-1">
			{#each ['day', 'week', 'month'] as mode}
				<button
					onclick={() => (viewMode = mode as 'day' | 'week' | 'month')}
					class="px-3 py-1.5 text-sm font-medium rounded-md transition-all cursor-pointer {viewMode === mode
						? 'bg-white text-surface-900 shadow-sm'
						: 'text-surface-500 hover:text-surface-700'}"
				>
					{mode.charAt(0).toUpperCase() + mode.slice(1)}
				</button>
			{/each}
		</div>
	</div>

	<!-- Week View Calendar -->
	<Card padding={false}>
		<div class="overflow-x-auto">
			<div class="min-w-[800px]">
				<!-- Day headers -->
				<div class="grid grid-cols-[80px_repeat(7,1fr)] border-b border-surface-200">
					<div class="p-3"></div>
					{#each weekDays as day, i}
						<div class="p-3 text-center border-l border-surface-200">
							<p class="text-xs font-medium text-surface-500 uppercase">{day}</p>
							<p class="text-lg font-semibold text-surface-900 mt-0.5">{15 + i}</p>
						</div>
					{/each}
				</div>

				<!-- Time grid -->
				<div class="relative">
					{#each hours as hour}
						<div class="grid grid-cols-[80px_repeat(7,1fr)] border-b border-surface-100 h-16">
							<div class="p-2 text-right pr-3">
								<span class="text-xs text-surface-400 font-mono">
									{hour > 12 ? hour - 12 : hour}:00 {hour >= 12 ? 'PM' : 'AM'}
								</span>
							</div>
							{#each weekDays as _, dayIdx}
								<div class="border-l border-surface-100 relative">
									{#each scheduledJobs.filter((j) => j.day === dayIdx && j.startHour === hour) as job}
										<div
											class="absolute inset-x-1 rounded-md border-l-3 px-2 py-1 text-xs overflow-hidden cursor-pointer hover:opacity-90 transition-opacity {job.color}"
											style="height: {job.duration * 64 - 4}px; z-index: 10;"
										>
											<p class="font-medium truncate">{job.title}</p>
											<p class="opacity-70 truncate">{job.customer}</p>
											<div class="flex items-center gap-1 mt-0.5 opacity-60">
												<User class="w-3 h-3" />
												<span>{job.technician}</span>
											</div>
										</div>
									{/each}
								</div>
							{/each}
						</div>
					{/each}
				</div>
			</div>
		</div>
	</Card>
</div>
