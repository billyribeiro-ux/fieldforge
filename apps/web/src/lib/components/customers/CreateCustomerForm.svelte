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

	let firstName = $state('');
	let lastName = $state('');
	let email = $state('');
	let phone = $state('');
	let companyName = $state('');
	let referralSource = $state('');
	let addressLine1 = $state('');
	let city = $state('');
	let stateCode = $state('');
	let zipCode = $state('');
	let propertyType = $state('residential');
	let notes = $state('');
	let loading = $state(false);

	const referralOptions = [
		{ value: '', label: 'None' },
		{ value: 'google', label: 'Google Search' },
		{ value: 'referral', label: 'Customer Referral' },
		{ value: 'yelp', label: 'Yelp' },
		{ value: 'nextdoor', label: 'Nextdoor' },
		{ value: 'facebook', label: 'Facebook' },
		{ value: 'instagram', label: 'Instagram' },
		{ value: 'yard_sign', label: 'Yard Sign' },
		{ value: 'truck_wrap', label: 'Truck Wrap' },
		{ value: 'repeat', label: 'Repeat Customer' },
		{ value: 'other', label: 'Other' }
	];

	const propertyTypeOptions = [
		{ value: 'residential', label: 'Residential' },
		{ value: 'commercial', label: 'Commercial' },
		{ value: 'industrial', label: 'Industrial' },
		{ value: 'government', label: 'Government' },
		{ value: 'hoa', label: 'HOA' }
	];

	async function handleSubmit(e: SubmitEvent) {
		e.preventDefault();
		loading = true;

		const data = {
			first_name: firstName,
			last_name: lastName,
			email: email || null,
			phone: phone || null,
			company_name: companyName || null,
			referral_source: referralSource || null,
			property: addressLine1
				? {
						address_line1: addressLine1,
						city,
						state: stateCode,
						zip_code: zipCode,
						property_type: propertyType
					}
				: null,
			notes: notes || null
		};

		onsave?.(data);
		loading = false;
		resetForm();
		onclose();
	}

	function resetForm() {
		firstName = '';
		lastName = '';
		email = '';
		phone = '';
		companyName = '';
		referralSource = '';
		addressLine1 = '';
		city = '';
		stateCode = '';
		zipCode = '';
		propertyType = 'residential';
		notes = '';
	}
</script>

<Modal {open} title="Add New Customer" size="lg" {onclose}>
	<form onsubmit={handleSubmit} class="space-y-5">
		<!-- Name -->
		<div class="grid grid-cols-2 gap-4">
			<Input label="First Name" bind:value={firstName} required placeholder="John" />
			<Input label="Last Name" bind:value={lastName} required placeholder="Smith" />
		</div>

		<!-- Contact -->
		<div class="grid grid-cols-2 gap-4">
			<Input label="Phone" type="tel" bind:value={phone} placeholder="(512) 555-0100" />
			<Input label="Email" type="email" bind:value={email} placeholder="john@email.com" />
		</div>

		<!-- Company -->
		<Input label="Company Name" bind:value={companyName} placeholder="Optional — for commercial customers" />

		<!-- Referral -->
		<Select label="How did they find you?" options={referralOptions} bind:value={referralSource} placeholder="Select source..." />

		<!-- Primary Property -->
		<div>
			<p class="text-sm font-medium text-surface-700 mb-3">Primary Property</p>
			<div class="space-y-3 pl-4 border-l-2 border-surface-200">
				<Input label="Address" bind:value={addressLine1} placeholder="123 Main St" />
				<div class="grid grid-cols-3 gap-3">
					<Input label="City" bind:value={city} placeholder="Austin" />
					<Input label="State" bind:value={stateCode} placeholder="TX" />
					<Input label="ZIP" bind:value={zipCode} placeholder="78701" />
				</div>
				<Select label="Property Type" options={propertyTypeOptions} bind:value={propertyType} />
			</div>
		</div>

		<!-- Notes -->
		<Textarea label="Notes" bind:value={notes} placeholder="Gate codes, pet info, scheduling preferences..." rows={2} />
	</form>

	{#snippet footer()}
		<Button variant="ghost" onclick={onclose}>Cancel</Button>
		<Button variant="primary" {loading} onclick={() => { const form = document.querySelector('form'); form?.requestSubmit(); }}>
			Add Customer
		</Button>
	{/snippet}
</Modal>
