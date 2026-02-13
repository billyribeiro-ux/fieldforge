<script lang="ts">
	import TopBar from '$lib/components/layout/TopBar.svelte';
	import Card from '$lib/components/ui/Card.svelte';
	import Button from '$lib/components/ui/Button.svelte';
	import Input from '$lib/components/ui/Input.svelte';
	import Select from '$lib/components/ui/Select.svelte';
	import Textarea from '$lib/components/ui/Textarea.svelte';
	import { ArrowLeft, Save, Send, MapPin, Clock, User, Calendar } from 'lucide-svelte';

	let customerId = $state('');
	let propertyId = $state('');
	let title = $state('');
	let description = $state('');
	let jobType = $state('');
	let priority = $state('normal');
	let status = $state('lead');
	let assignedTo = $state('');
	let scheduledStart = $state('');
	let scheduledEnd = $state('');
	let estimatedDuration = $state('60');

	const customerOptions = [
		{ value: '1', label: 'Sarah Johnson' },
		{ value: '2', label: 'Mike Chen — Chen Properties LLC' },
		{ value: '3', label: 'Lisa Rodriguez' },
		{ value: '4', label: 'Tom Williams' },
		{ value: '5', label: 'Amy Foster — Foster Realty' },
	];

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
		{ value: 'emergency', label: '🔴 Emergency' },
		{ value: 'high', label: '🟠 High' },
		{ value: 'normal', label: '🟢 Normal' },
		{ value: 'low', label: '⚪ Low' },
	];

	const statusOptions = [
		{ value: 'lead', label: 'Lead' },
		{ value: 'estimated', label: 'Estimated' },
		{ value: 'scheduled', label: 'Scheduled' },
	];

	const technicianOptions = [
		{ value: '', label: 'Unassigned' },
		{ value: '1', label: 'Mike Johnson' },
		{ value: '2', label: 'Jake Rodriguez' },
		{ value: '3', label: 'Tom Williams' },
		{ value: '4', label: 'Sarah Chen' },
	];
</script>

<TopBar title="New Job">
	{#snippet actions()}
		<div class="flex items-center gap-2">
			<Button variant="outline" size="sm" href="/dashboard/jobs">
				<ArrowLeft class="w-4 h-4" />
				Cancel
			</Button>
			<Button variant="outline" size="sm">
				<Save class="w-4 h-4" />
				Save Draft
			</Button>
			<Button size="sm">
				<Send class="w-4 h-4" />
				Create Job
			</Button>
		</div>
	{/snippet}
</TopBar>

<div class="p-6 max-w-4xl mx-auto space-y-6">
	<!-- Customer & Property -->
	<Card>
		<h3 class="text-sm font-semibold text-surface-900 mb-4 flex items-center gap-2">
			<User class="w-4 h-4 text-surface-400" />
			Customer & Location
		</h3>
		<div class="grid grid-cols-1 md:grid-cols-2 gap-4">
			<Select
				label="Customer"
				options={customerOptions}
				bind:value={customerId}
				placeholder="Select a customer..."
			/>
			<Select
				label="Property"
				options={[{ value: '', label: 'Default property' }]}
				bind:value={propertyId}
				placeholder="Select property..."
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
			<Input label="Job Title" bind:value={title} placeholder="e.g., AC Unit Not Cooling — Emergency Repair" />

			<div class="grid grid-cols-1 md:grid-cols-3 gap-4">
				<Select
					label="Job Type"
					options={jobTypeOptions}
					bind:value={jobType}
					placeholder="Select type..."
				/>
				<Select
					label="Priority"
					options={priorityOptions}
					bind:value={priority}
				/>
				<Select
					label="Initial Status"
					options={statusOptions}
					bind:value={status}
				/>
			</div>

			<Textarea
				label="Description"
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
		<div class="grid grid-cols-1 md:grid-cols-3 gap-4">
			<Select
				label="Assign To"
				options={technicianOptions}
				bind:value={assignedTo}
			/>
			<Input label="Scheduled Start" type="datetime-local" bind:value={scheduledStart} />
			<Input label="Scheduled End" type="datetime-local" bind:value={scheduledEnd} />
		</div>
		<div class="mt-4">
			<Input label="Estimated Duration (minutes)" type="number" bind:value={estimatedDuration} />
		</div>
	</Card>

	<!-- Tags (placeholder) -->
	<Card>
		<h3 class="text-sm font-semibold text-surface-900 mb-4 flex items-center gap-2">
			<Clock class="w-4 h-4 text-surface-400" />
			Additional Options
		</h3>
		<div class="grid grid-cols-1 md:grid-cols-2 gap-4">
			<Input label="Source / Referral" placeholder="e.g., Google, Referral, Repeat customer" />
			<Input label="Purchase Order #" placeholder="Customer PO number (optional)" />
		</div>
	</Card>

	<!-- Actions -->
	<div class="flex justify-end gap-3 pb-8">
		<Button variant="outline" href="/dashboard/jobs">Cancel</Button>
		<Button variant="outline">Save as Draft</Button>
		<Button>Create Job</Button>
	</div>
</div>
