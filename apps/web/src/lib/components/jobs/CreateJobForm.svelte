<script lang="ts">
	import Modal from '$lib/components/ui/Modal.svelte';
	import Button from '$lib/components/ui/Button.svelte';
	import Input from '$lib/components/ui/Input.svelte';
	import Select from '$lib/components/ui/Select.svelte';
	import Textarea from '$lib/components/ui/Textarea.svelte';

	interface Props {
		open: boolean;
		onclose: () => void;
		onsave?: (data: Record<string, unknown>) => void;
	}

	let { open = $bindable(false), onclose, onsave }: Props = $props();

	let title = $state('');
	let customerId = $state('');
	let priority = $state('normal');
	let trade = $state('');
	let jobType = $state('');
	let scheduledDate = $state('');
	let scheduledTime = $state('');
	let estimatedDuration = $state('');
	let description = $state('');
	let accessInstructions = $state('');
	let internalNotes = $state('');
	let assignedTo = $state('');
	let loading = $state(false);

	const priorityOptions = [
		{ value: 'low', label: 'Low' },
		{ value: 'normal', label: 'Normal' },
		{ value: 'high', label: 'High' },
		{ value: 'emergency', label: 'Emergency' }
	];

	const tradeOptions = [
		{ value: 'hvac', label: 'HVAC' },
		{ value: 'plumbing', label: 'Plumbing' },
		{ value: 'electrical', label: 'Electrical' },
		{ value: 'roofing', label: 'Roofing' },
		{ value: 'general', label: 'General' },
		{ value: 'painting', label: 'Painting' },
		{ value: 'carpentry', label: 'Carpentry' },
		{ value: 'landscaping', label: 'Landscaping' }
	];

	const jobTypeOptions = [
		{ value: 'service_call', label: 'Service Call' },
		{ value: 'repair', label: 'Repair' },
		{ value: 'installation', label: 'Installation' },
		{ value: 'replacement', label: 'Replacement' },
		{ value: 'maintenance', label: 'Maintenance' },
		{ value: 'inspection', label: 'Inspection' },
		{ value: 'estimate', label: 'Estimate Only' },
		{ value: 'remodel', label: 'Remodel' }
	];

	const durationOptions = [
		{ value: '30', label: '30 minutes' },
		{ value: '60', label: '1 hour' },
		{ value: '120', label: '2 hours' },
		{ value: '180', label: '3 hours' },
		{ value: '240', label: '4 hours' },
		{ value: '360', label: '6 hours' },
		{ value: '480', label: '8 hours (full day)' }
	];

	// Demo customers
	const customerOptions = [
		{ value: '1', label: 'Sarah Johnson' },
		{ value: '2', label: 'Mike Chen — Chen Properties LLC' },
		{ value: '3', label: 'Lisa Rodriguez' },
		{ value: '4', label: 'Tom Williams' },
		{ value: '5', label: 'Amy Foster — Foster Realty' },
		{ value: '6', label: 'David Park' },
		{ value: '7', label: 'Karen White — White & Sons' },
		{ value: '8', label: 'Robert Kim' }
	];

	const technicianOptions = [
		{ value: '', label: 'Unassigned' },
		{ value: '1', label: 'Mike Thompson' },
		{ value: '2', label: 'Jake Rodriguez' },
		{ value: '3', label: 'Sarah Lee' }
	];

	async function handleSubmit(e: SubmitEvent) {
		e.preventDefault();
		loading = true;

		const data = {
			title,
			customer_id: customerId,
			priority,
			trade,
			job_type: jobType,
			scheduled_date: scheduledDate || null,
			scheduled_start_time: scheduledTime || null,
			estimated_duration_minutes: estimatedDuration ? parseInt(estimatedDuration) : null,
			description: description || null,
			access_instructions: accessInstructions || null,
			internal_notes: internalNotes || null,
			assigned_to: assignedTo || null
		};

		onsave?.(data);
		loading = false;
		resetForm();
		onclose();
	}

	function resetForm() {
		title = '';
		customerId = '';
		priority = 'normal';
		trade = '';
		jobType = '';
		scheduledDate = '';
		scheduledTime = '';
		estimatedDuration = '';
		description = '';
		accessInstructions = '';
		internalNotes = '';
		assignedTo = '';
	}
</script>

<Modal {open} title="Create New Job" size="lg" {onclose}>
	<form onsubmit={handleSubmit} class="space-y-5">
		<!-- Row 1: Title + Customer -->
		<Input label="Job Title" bind:value={title} required placeholder="e.g. AC Unit Replacement" />

		<Select label="Customer" options={customerOptions} bind:value={customerId} required placeholder="Select customer..." />

		<!-- Row 2: Trade + Type + Priority -->
		<div class="grid grid-cols-3 gap-4">
			<Select label="Trade" options={tradeOptions} bind:value={trade} placeholder="Select trade..." />
			<Select label="Job Type" options={jobTypeOptions} bind:value={jobType} placeholder="Select type..." />
			<Select label="Priority" options={priorityOptions} bind:value={priority} />
		</div>

		<!-- Row 3: Schedule -->
		<div class="grid grid-cols-3 gap-4">
			<Input label="Date" type="date" bind:value={scheduledDate} />
			<Input label="Start Time" type="time" bind:value={scheduledTime} />
			<Select label="Est. Duration" options={durationOptions} bind:value={estimatedDuration} placeholder="Select..." />
		</div>

		<!-- Assigned -->
		<Select label="Assign To" options={technicianOptions} bind:value={assignedTo} placeholder="Unassigned" />

		<!-- Description -->
		<Textarea label="Description" bind:value={description} placeholder="Describe the work to be done..." rows={3} />

		<!-- Access Instructions -->
		<Textarea label="Access Instructions" bind:value={accessInstructions} placeholder="Gate code, parking, pet info..." rows={2} />

		<!-- Internal Notes -->
		<Textarea label="Internal Notes" bind:value={internalNotes} placeholder="Notes visible only to your team..." rows={2} />
	</form>

	{#snippet footer()}
		<Button variant="ghost" onclick={onclose}>Cancel</Button>
		<Button variant="primary" {loading} onclick={(e) => { const form = document.querySelector('form'); form?.requestSubmit(); }}>
			Create Job
		</Button>
	{/snippet}
</Modal>
