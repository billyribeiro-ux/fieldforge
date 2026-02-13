<script lang="ts">
	import TopBar from '$lib/components/layout/TopBar.svelte';
	import Card from '$lib/components/ui/Card.svelte';
	import Button from '$lib/components/ui/Button.svelte';
	import Badge from '$lib/components/ui/Badge.svelte';
	import Switch from '$lib/components/ui/Switch.svelte';
	import { Zap, Plus, Clock, ArrowRight, Mail, MessageSquare, Bell } from 'lucide-svelte';

	const rules = [
		{
			id: '1', name: 'Send follow-up after job completion', trigger_event: 'job.completed',
			actions: { type: 'send_email', template: 'follow_up' }, delay_minutes: 1440,
			is_active: true, trigger_count: 47, last_triggered_at: '2024-12-12T10:00:00Z'
		},
		{
			id: '2', name: 'Request review after 3 days', trigger_event: 'job.completed',
			actions: { type: 'send_sms', template: 'review_request' }, delay_minutes: 4320,
			is_active: true, trigger_count: 32, last_triggered_at: '2024-12-11T14:00:00Z'
		},
		{
			id: '3', name: 'Notify office on emergency job', trigger_event: 'job.created',
			actions: { type: 'send_notification', priority: 'emergency' }, delay_minutes: 0,
			is_active: true, trigger_count: 8, last_triggered_at: '2024-12-10T08:30:00Z'
		},
		{
			id: '4', name: 'Auto-schedule recurring maintenance', trigger_event: 'recurring_rule.due',
			actions: { type: 'create_job', auto_assign: true }, delay_minutes: 0,
			is_active: false, trigger_count: 120, last_triggered_at: '2024-12-09T06:00:00Z'
		},
		{
			id: '5', name: 'Send payment reminder on overdue invoice', trigger_event: 'invoice.overdue',
			actions: { type: 'send_email', template: 'payment_reminder' }, delay_minutes: 0,
			is_active: true, trigger_count: 15, last_triggered_at: '2024-12-08T09:00:00Z'
		},
	];

	let activeCount = $derived(rules.filter(r => r.is_active).length);

	function triggerLabel(event: string): string {
		const map: Record<string, string> = {
			'job.completed': 'Job Completed',
			'job.created': 'Job Created',
			'recurring_rule.due': 'Recurring Due',
			'invoice.overdue': 'Invoice Overdue',
			'estimate.approved': 'Estimate Approved',
		};
		return map[event] ?? event;
	}

	function actionIcon(type: string) {
		if (type === 'send_email') return Mail;
		if (type === 'send_sms') return MessageSquare;
		return Bell;
	}

	function delayLabel(minutes: number): string {
		if (minutes === 0) return 'Immediately';
		if (minutes < 60) return `${minutes}m delay`;
		if (minutes < 1440) return `${Math.round(minutes / 60)}h delay`;
		return `${Math.round(minutes / 1440)}d delay`;
	}
</script>

<svelte:head>
	<title>Automation — FieldForge</title>
</svelte:head>

<TopBar title="Automation">
	{#snippet actions()}
		<Button size="sm">
			<Plus class="w-4 h-4" />
			New Rule
		</Button>
	{/snippet}
</TopBar>

<div class="p-6 space-y-6">
	<!-- Summary -->
	<div class="grid grid-cols-1 md:grid-cols-3 gap-4">
		<Card>
			<div class="flex items-center gap-3">
				<div class="w-10 h-10 bg-forge-50 text-forge-600 rounded-lg flex items-center justify-center">
					<Zap class="w-5 h-5" />
				</div>
				<div>
					<p class="text-2xl font-bold text-surface-900">{rules.length}</p>
					<p class="text-xs text-surface-500">Total Rules</p>
				</div>
			</div>
		</Card>
		<Card>
			<div class="flex items-center gap-3">
				<div class="w-10 h-10 bg-success-50 text-success-600 rounded-lg flex items-center justify-center">
					<Zap class="w-5 h-5" />
				</div>
				<div>
					<p class="text-2xl font-bold text-surface-900">{activeCount}</p>
					<p class="text-xs text-surface-500">Active Rules</p>
				</div>
			</div>
		</Card>
		<Card>
			<div class="flex items-center gap-3">
				<div class="w-10 h-10 bg-accent-50 text-accent-600 rounded-lg flex items-center justify-center">
					<Clock class="w-5 h-5" />
				</div>
				<div>
					<p class="text-2xl font-bold text-surface-900">{rules.reduce((sum, r) => sum + r.trigger_count, 0)}</p>
					<p class="text-xs text-surface-500">Total Triggers</p>
				</div>
			</div>
		</Card>
	</div>

	<!-- Rules List -->
	<div class="space-y-3">
		{#each rules as rule (rule.id)}
			{@const ActionIcon = actionIcon(rule.actions.type)}
			<Card>
				<div class="flex items-center justify-between">
					<div class="flex items-center gap-4">
						<div class="w-10 h-10 rounded-lg flex items-center justify-center {rule.is_active ? 'bg-forge-50 text-forge-600' : 'bg-surface-100 text-surface-400'}">
							<Zap class="w-5 h-5" />
						</div>
						<div>
							<p class="text-sm font-semibold text-surface-900">{rule.name}</p>
							<div class="flex items-center gap-2 mt-1">
								<Badge variant="default" size="sm">{triggerLabel(rule.trigger_event)}</Badge>
								<ArrowRight class="w-3 h-3 text-surface-400" />
								<div class="flex items-center gap-1 text-xs text-surface-500">
									<ActionIcon class="w-3.5 h-3.5" />
									{rule.actions.type.replace('send_', '').replace('create_', 'Create ')}
								</div>
								{#if rule.delay_minutes > 0}
									<span class="text-xs text-surface-400">· <Clock class="w-3 h-3 inline" /> {delayLabel(rule.delay_minutes)}</span>
								{/if}
							</div>
						</div>
					</div>
					<div class="flex items-center gap-4">
						<div class="text-right">
							<p class="text-sm font-medium text-surface-700">{rule.trigger_count} triggers</p>
						</div>
						<Switch checked={rule.is_active} onchange={() => {}} />
					</div>
				</div>
			</Card>
		{/each}
	</div>
</div>
