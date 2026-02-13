<script lang="ts">
	import TopBar from '$lib/components/layout/TopBar.svelte';
	import Card from '$lib/components/ui/Card.svelte';
	import Button from '$lib/components/ui/Button.svelte';
	import Badge from '$lib/components/ui/Badge.svelte';
	import { Bell, CheckCheck, Briefcase, Receipt, Calendar, MessageSquare, AlertTriangle, Users, Trash2 } from 'lucide-svelte';

	let filter = $state<'all' | 'unread'>('all');

	const notifications = [
		{ id: '1', type: 'job_completed', title: 'Job Completed', body: 'AC Unit Replacement for Sarah Johnson has been marked complete.', read: false, created_at: '2024-12-12T14:30:00Z' },
		{ id: '2', type: 'payment_received', title: 'Payment Received', body: '$3,500.00 payment received for Invoice #INV-2024-042.', read: false, created_at: '2024-12-12T12:15:00Z' },
		{ id: '3', type: 'new_message', title: 'New Message', body: 'Tom Williams sent a message about his upcoming appointment.', read: false, created_at: '2024-12-12T10:00:00Z' },
		{ id: '4', type: 'schedule_reminder', title: 'Schedule Reminder', body: 'You have 4 jobs scheduled for tomorrow.', read: true, created_at: '2024-12-11T18:00:00Z' },
		{ id: '5', type: 'license_expiring', title: 'License Expiring', body: 'Your HVAC Contractor License expires in 30 days.', read: true, created_at: '2024-12-11T09:00:00Z' },
		{ id: '6', type: 'team_update', title: 'Team Update', body: 'Jake Rodriguez has been assigned to the Dispatch team.', read: true, created_at: '2024-12-10T16:45:00Z' },
		{ id: '7', type: 'inventory_alert', title: 'Low Inventory', body: 'Copper Pipe 3/4" is below minimum stock level (5 remaining).', read: true, created_at: '2024-12-10T11:30:00Z' },
		{ id: '8', type: 'review_received', title: 'New Review', body: 'Sarah J. left a 5-star review on Google.', read: true, created_at: '2024-12-10T08:00:00Z' }
	];

	let filtered = $derived(filter === 'unread' ? notifications.filter((n) => !n.read) : notifications);
	let unreadCount = $derived(notifications.filter((n) => !n.read).length);

	function typeIcon(type: string) {
		switch (type) {
			case 'job_completed': return Briefcase;
			case 'payment_received': return Receipt;
			case 'schedule_reminder': return Calendar;
			case 'new_message': return MessageSquare;
			case 'license_expiring': return AlertTriangle;
			case 'team_update': return Users;
			case 'inventory_alert': return AlertTriangle;
			case 'review_received': return MessageSquare;
			default: return Bell;
		}
	}

	function typeColor(type: string): string {
		switch (type) {
			case 'job_completed': return 'bg-success-50 text-success-600';
			case 'payment_received': return 'bg-forge-50 text-forge-600';
			case 'schedule_reminder': return 'bg-accent-50 text-accent-600';
			case 'new_message': return 'bg-info-50 text-info-600';
			case 'license_expiring': return 'bg-warning-50 text-warning-600';
			case 'team_update': return 'bg-surface-100 text-surface-600';
			case 'inventory_alert': return 'bg-danger-50 text-danger-600';
			case 'review_received': return 'bg-warning-50 text-warning-600';
			default: return 'bg-surface-100 text-surface-600';
		}
	}

	function timeAgo(dateStr: string): string {
		const now = new Date();
		const date = new Date(dateStr);
		const diff = Math.floor((now.getTime() - date.getTime()) / 1000);
		if (diff < 60) return 'just now';
		if (diff < 3600) return `${Math.floor(diff / 60)}m ago`;
		if (diff < 86400) return `${Math.floor(diff / 3600)}h ago`;
		return `${Math.floor(diff / 86400)}d ago`;
	}
</script>

<TopBar title="Notifications">
	{#snippet actions()}
		<div class="flex items-center gap-2">
			{#if unreadCount > 0}
				<Badge variant="danger" size="md">{unreadCount} unread</Badge>
			{/if}
			<Button variant="outline" size="sm">
				<CheckCheck class="w-4 h-4" />
				Mark All Read
			</Button>
		</div>
	{/snippet}
</TopBar>

<div class="p-6 space-y-4">
	<!-- Filter Tabs -->
	<div class="flex gap-2">
		<button
			class="px-3 py-1.5 text-sm font-medium rounded-lg transition-colors cursor-pointer {filter === 'all' ? 'bg-forge-100 text-forge-700' : 'text-surface-500 hover:bg-surface-100'}"
			onclick={() => (filter = 'all')}
		>
			All ({notifications.length})
		</button>
		<button
			class="px-3 py-1.5 text-sm font-medium rounded-lg transition-colors cursor-pointer {filter === 'unread' ? 'bg-forge-100 text-forge-700' : 'text-surface-500 hover:bg-surface-100'}"
			onclick={() => (filter = 'unread')}
		>
			Unread ({unreadCount})
		</button>
	</div>

	<!-- Notification List -->
	<div class="space-y-2">
		{#each filtered as notification (notification.id)}
			{@const Icon = typeIcon(notification.type)}
			<div class="flex items-start gap-4 p-4 rounded-xl border transition-colors {notification.read ? 'bg-white border-surface-200' : 'bg-forge-50/30 border-forge-200'}">
				<div class="w-10 h-10 rounded-lg flex items-center justify-center flex-shrink-0 {typeColor(notification.type)}">
					<Icon class="w-5 h-5" />
				</div>
				<div class="flex-1 min-w-0">
					<div class="flex items-center gap-2 mb-0.5">
						<p class="text-sm font-semibold text-surface-900">{notification.title}</p>
						{#if !notification.read}
							<div class="w-2 h-2 bg-forge-500 rounded-full flex-shrink-0"></div>
						{/if}
					</div>
					<p class="text-sm text-surface-600 leading-relaxed">{notification.body}</p>
					<p class="text-xs text-surface-400 mt-1">{timeAgo(notification.created_at)}</p>
				</div>
				<button class="p-1.5 text-surface-300 hover:text-danger-500 rounded-lg hover:bg-surface-100 transition-colors cursor-pointer flex-shrink-0">
					<Trash2 class="w-4 h-4" />
				</button>
			</div>
		{/each}

		{#if filtered.length === 0}
			<Card>
				<div class="text-center py-12">
					<Bell class="w-10 h-10 text-surface-300 mx-auto mb-3" />
					<p class="text-sm font-medium text-surface-500">No notifications</p>
					<p class="text-xs text-surface-400 mt-1">You're all caught up!</p>
				</div>
			</Card>
		{/if}
	</div>
</div>
