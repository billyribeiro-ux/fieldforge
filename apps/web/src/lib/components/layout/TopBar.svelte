<script lang="ts">
	import { Search, Bell, Plus, X, Briefcase, FileText, DollarSign, Calendar, CheckCircle2, AlertTriangle } from 'lucide-svelte';
	import Button from '$lib/components/ui/Button.svelte';
	import Badge from '$lib/components/ui/Badge.svelte';

	interface Props {
		title?: string;
		children?: import('svelte').Snippet;
		actions?: import('svelte').Snippet;
	}

	let { title = '', children, actions }: Props = $props();

	let searchQuery = $state('');
	let showNotifications = $state(false);

	const notifications = [
		{ id: '1', type: 'job_assigned', title: 'New job assigned', body: 'AC Unit Replacement for Sarah Johnson', time: '5 min ago', read: false, icon: Briefcase },
		{ id: '2', type: 'estimate_approved', title: 'Estimate approved', body: 'EST-001 approved by Sarah Johnson', time: '1 hour ago', read: false, icon: CheckCircle2 },
		{ id: '3', type: 'payment_received', title: 'Payment received', body: '$275.00 from Tom Williams (FF-012)', time: '2 hours ago', read: false, icon: DollarSign },
		{ id: '4', type: 'schedule_reminder', title: 'Tomorrow\'s schedule', body: '3 jobs scheduled for tomorrow', time: '3 hours ago', read: true, icon: Calendar },
		{ id: '5', type: 'invoice_overdue', title: 'Invoice overdue', body: 'FF-008 for Lisa Rodriguez is 5 days past due', time: '1 day ago', read: true, icon: AlertTriangle }
	];

	let unreadCount = $derived(notifications.filter(n => !n.read).length);

	function handleClickOutside(e: MouseEvent) {
		const target = e.target as HTMLElement;
		if (!target.closest('.notification-panel') && !target.closest('.notification-trigger')) {
			showNotifications = false;
		}
	}
</script>

<svelte:window onclick={showNotifications ? handleClickOutside : undefined} />

<header class="sticky top-0 z-30 bg-white/80 backdrop-blur-md border-b border-surface-200">
	<div class="flex items-center justify-between h-16 px-6">
		<div class="flex items-center gap-4">
			{#if title}
				<h1 class="text-xl font-semibold text-surface-900">{title}</h1>
			{/if}
			{#if children}
				{@render children()}
			{/if}
		</div>

		<div class="flex items-center gap-3">
			<!-- Global search -->
			<div class="relative">
				<Search class="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-surface-400" />
				<input
					type="text"
					placeholder="Search jobs, customers..."
					bind:value={searchQuery}
					class="w-64 pl-9 pr-4 py-2 text-sm bg-surface-50 border border-surface-200 rounded-lg focus:outline-none focus:ring-2 focus:ring-forge-500 focus:border-forge-500 placeholder:text-surface-400"
				/>
				<kbd class="absolute right-3 top-1/2 -translate-y-1/2 text-xs text-surface-400 bg-surface-100 px-1.5 py-0.5 rounded border border-surface-200">⌘K</kbd>
			</div>

			<!-- Notifications -->
			<div class="relative">
				<button
					onclick={() => (showNotifications = !showNotifications)}
					class="notification-trigger relative p-2 text-surface-500 hover:text-surface-700 hover:bg-surface-100 rounded-lg transition-colors cursor-pointer"
				>
					<Bell class="w-5 h-5" />
					{#if unreadCount > 0}
						<span class="absolute top-1 right-1 w-4 h-4 bg-danger-500 rounded-full text-[10px] font-bold text-white flex items-center justify-center">{unreadCount}</span>
					{/if}
				</button>

				{#if showNotifications}
					<div class="notification-panel absolute right-0 top-full mt-2 w-96 bg-white rounded-xl shadow-2xl border border-surface-200 overflow-hidden z-50">
						<div class="flex items-center justify-between px-4 py-3 border-b border-surface-200">
							<h3 class="text-sm font-semibold text-surface-900">Notifications</h3>
							<button class="text-xs text-forge-600 hover:text-forge-700 font-medium cursor-pointer">Mark all read</button>
						</div>
						<div class="max-h-80 overflow-y-auto divide-y divide-surface-100">
							{#each notifications as notif}
								<div class="flex items-start gap-3 px-4 py-3 hover:bg-surface-50 transition-colors cursor-pointer {notif.read ? '' : 'bg-forge-50/30'}">
									<div class="p-1.5 rounded-lg {notif.read ? 'bg-surface-100 text-surface-500' : 'bg-forge-100 text-forge-600'} flex-shrink-0 mt-0.5">
										<notif.icon class="w-4 h-4" />
									</div>
									<div class="flex-1 min-w-0">
										<p class="text-sm font-medium text-surface-900 {notif.read ? '' : 'font-semibold'}">{notif.title}</p>
										<p class="text-xs text-surface-500 truncate">{notif.body}</p>
										<p class="text-xs text-surface-400 mt-0.5">{notif.time}</p>
									</div>
									{#if !notif.read}
										<div class="w-2 h-2 bg-forge-500 rounded-full flex-shrink-0 mt-2"></div>
									{/if}
								</div>
							{/each}
						</div>
						<div class="px-4 py-3 border-t border-surface-200 text-center">
							<a href="/dashboard/settings" class="text-xs text-forge-600 hover:text-forge-700 font-medium">View all notifications</a>
						</div>
					</div>
				{/if}
			</div>

			<!-- Quick actions -->
			{#if actions}
				{@render actions()}
			{/if}
		</div>
	</div>
</header>
