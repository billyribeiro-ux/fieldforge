<script lang="ts">
	import { enhance } from '$app/forms';
	import TopBar from '$lib/components/layout/TopBar.svelte';
	import Card from '$lib/components/ui/Card.svelte';
	import Button from '$lib/components/ui/Button.svelte';
	import Input from '$lib/components/ui/Input.svelte';
	import Select from '$lib/components/ui/Select.svelte';
	import Textarea from '$lib/components/ui/Textarea.svelte';
	import { ArrowLeft, Send, MapPin, Clock, User, Calendar, Loader2 } from 'lucide-svelte';

	let { form } = $props();

	let submitting = $state(false);

	let customerId = $state('');
	let title = $state('');
	let description = $state('');
	let jobType = $state('');
	let priority = $state('normal');
	let assignedTo = $state('');
	let scheduledStart = $state('');
	let estimatedDuration = $state('60');
	let source = $state('');
	let poNumber = $state('');

	const jobTypeOptions = [
		{ value: 'hvac_repair', label: 'HVAC Repair' },
		{ value: 'hvac_install', label: 'HVAC Installation' },
		{ value: 'hvac_maintenance', label: 'HVAC Maintenance' },
		{ value: 'plumbing_repair', label: 'Plumbing Repair' },
		{ value: 'plumbing_install', label: 'Plumbing Installation' },
		{ value: 'electrical_repair', label: 'Electrical Repair' },
		{ value: 'electrical_install', label: 'Electrical Installation' },
		{ value: 'general', label: 'General Contracting' },
		{ value: 'inspection', label: 'Inspection' },
		{ value: 'other', label: 'Other' },
	];

	const priorityOptions = [
		{ value: 'emergency', label: 'Emergency' },
		{ value: 'high', label: 'High' },
		{ value: 'normal', label: 'Normal' },
		{ value: 'low', label: 'Low' },
	];

	const technicianOptions = [
		{ value: '', label: 'Unassigned' },
		{ value: '1', label: 'Mike Johnson' },
		{ value: '2', label: 'Jake Rodriguez' },
		{ value: '3', label: 'Tom Williams' },
		{ value: '4', label: 'Sarah Chen' },
	];
</script>

<svelte:head>
	<title>New Job — FieldForge</title>
</svelte:head>

<TopBar title="New Job">
	{#snippet actions()}
		<Button variant="outline" size="sm" href="/dashboard/jobs">
			<ArrowLeft class="w-4 h-4" />
			Cancel
		</Button>
	{/snippet}
</TopBar>

<form method="POST" action="?/create" use:enhance={() => {
	submitting = true;
	return async ({ update }) => {
		submitting = false;
		await update();
	};
}}>
	<div class="p-6 max-w-4xl mx-auto space-y-6">
		{#if form?.error}
			<div class="bg-danger-50 border border-danger-200 text-danger-700 px-4 py-3 rounded-xl text-sm">
				{form.error}
			</div>
		{/if}

		<!-- Customer & Property -->
		<Card>
			<h3 class="text-sm font-semibold text-surface-900 mb-4 flex items-center gap-2">
				<User class="w-4 h-4 text-surface-400" />
				Customer & Location
			</h3>
			<div class="grid grid-cols-1 md:grid-cols-2 gap-4">
				<Select
					label="Customer"
					name="customer_id"
					options={[{ value: '', label: 'Select a customer...' }, { value: '1', label: 'Sarah Johnson' }, { value: '2', label: 'Mike Chen — Chen Properties LLC' }, { value: '3', label: 'Lisa Rodriguez' }]}
					bind:value={customerId}
				/>
			</div>
		</Card>

		<!-- Job Details -->
		<Card>
			<h3 class="text-sm font-semibold text-surface-900 mb-4 flex items-center gap-2">
				<MapPin class="w-4 h-4 text-surface-400" />
				Job Details
			</h3>
			<div class="space-y-4">
				<Input label="Job Title" name="title" bind:value={title} placeholder="e.g., AC Unit Not Cooling — Emergency Repair" required />

				<div class="grid grid-cols-1 md:grid-cols-2 gap-4">
					<Select
						label="Job Type"
						name="job_type"
						options={[{ value: '', label: 'Select type...' }, ...jobTypeOptions]}
						bind:value={jobType}
					/>
					<Select
						label="Priority"
						name="priority"
						options={priorityOptions}
						bind:value={priority}
					/>
				</div>

				<Textarea
					label="Description"
					name="description"
					bind:value={description}
					placeholder="Describe the job scope, customer concerns, and any relevant details..."
					rows={4}
				/>
			</div>
		</Card>

		<!-- Scheduling -->
		<Card>
			<h3 class="text-sm font-semibold text-surface-900 mb-4 flex items-center gap-2">
				<Calendar class="w-4 h-4 text-surface-400" />
				Scheduling
			</h3>
			<div class="grid grid-cols-1 md:grid-cols-2 gap-4">
				<Select
					label="Assign To"
					name="assigned_to"
					options={technicianOptions}
					bind:value={assignedTo}
				/>
				<Input label="Scheduled Start" name="scheduled_start" type="datetime-local" bind:value={scheduledStart} />
			</div>
			<div class="mt-4">
				<Input label="Estimated Duration (minutes)" name="estimated_duration" type="number" bind:value={estimatedDuration} />
			</div>
		</Card>

		<!-- Additional Options -->
		<Card>
			<h3 class="text-sm font-semibold text-surface-900 mb-4 flex items-center gap-2">
				<Clock class="w-4 h-4 text-surface-400" />
				Additional Options
			</h3>
			<div class="grid grid-cols-1 md:grid-cols-2 gap-4">
				<Input label="Source / Referral" name="source" bind:value={source} placeholder="e.g., Google, Referral, Repeat customer" />
				<Input label="Purchase Order #" name="po_number" bind:value={poNumber} placeholder="Customer PO number (optional)" />
			</div>
		</Card>

		<!-- Actions -->
		<div class="flex justify-end gap-3 pb-8">
			<Button variant="outline" href="/dashboard/jobs">Cancel</Button>
			<Button type="submit" disabled={submitting}>
				{#if submitting}
					<Loader2 class="w-4 h-4 animate-spin" />
					Creating...
				{:else}
					<Send class="w-4 h-4" />
					Create Job
				{/if}
			</Button>
		</div>
	</div>
</form>
