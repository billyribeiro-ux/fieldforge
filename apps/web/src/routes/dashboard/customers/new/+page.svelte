<script lang="ts">
	import { enhance } from '$app/forms';
	import TopBar from '$lib/components/layout/TopBar.svelte';
	import Card from '$lib/components/ui/Card.svelte';
	import Button from '$lib/components/ui/Button.svelte';
	import Input from '$lib/components/ui/Input.svelte';
	import Select from '$lib/components/ui/Select.svelte';
	import Textarea from '$lib/components/ui/Textarea.svelte';
	import { ArrowLeft, UserPlus, User, MapPin, Phone, Building2, Loader2 } from 'lucide-svelte';

	let { form } = $props();

	let submitting = $state(false);

	let firstName = $state('');
	let lastName = $state('');
	let email = $state('');
	let phone = $state('');
	let companyName = $state('');
	let customerType = $state('residential');
	let contactMethod = $state('any');
	let creditTerms = $state('cod');
	let notes = $state('');
	let source = $state('');

	let addressLine1 = $state('');
	let addressLine2 = $state('');
	let city = $state('');
	let addressState = $state('');
	let zip = $state('');

	const customerTypeOptions = [
		{ value: 'residential', label: 'Residential' },
		{ value: 'commercial', label: 'Commercial' },
		{ value: 'industrial', label: 'Industrial' },
		{ value: 'government', label: 'Government' },
		{ value: 'hoa', label: 'HOA' },
	];

	const contactMethodOptions = [
		{ value: 'any', label: 'Any' },
		{ value: 'email', label: 'Email' },
		{ value: 'phone', label: 'Phone' },
		{ value: 'sms', label: 'SMS' },
	];

	const creditTermsOptions = [
		{ value: 'cod', label: 'COD (Due on Completion)' },
		{ value: 'net_15', label: 'Net 15' },
		{ value: 'net_30', label: 'Net 30' },
		{ value: 'net_45', label: 'Net 45' },
		{ value: 'net_60', label: 'Net 60' },
	];

	const sourceOptions = [
		{ value: '', label: 'Select source...' },
		{ value: 'google', label: 'Google Search' },
		{ value: 'referral', label: 'Referral' },
		{ value: 'yelp', label: 'Yelp' },
		{ value: 'facebook', label: 'Facebook' },
		{ value: 'nextdoor', label: 'Nextdoor' },
		{ value: 'repeat', label: 'Repeat Customer' },
		{ value: 'other', label: 'Other' },
	];
</script>

<svelte:head>
	<title>New Customer — FieldForge</title>
</svelte:head>

<TopBar title="New Customer">
	{#snippet actions()}
		<Button variant="outline" size="sm" href="/dashboard/customers">
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

		<!-- Contact Information -->
		<Card>
			<h3 class="text-sm font-semibold text-surface-900 mb-4 flex items-center gap-2">
				<User class="w-4 h-4 text-surface-400" />
				Contact Information
			</h3>
			<div class="grid grid-cols-1 md:grid-cols-2 gap-4">
				<Input label="First Name" name="first_name" bind:value={firstName} placeholder="First name" required />
				<Input label="Last Name" name="last_name" bind:value={lastName} placeholder="Last name" required />
				<Input label="Email" name="email" type="email" bind:value={email} placeholder="email@example.com" />
				<Input label="Phone" name="phone" type="tel" bind:value={phone} placeholder="(555) 123-4567" />
			</div>
		</Card>

		<!-- Company -->
		<Card>
			<h3 class="text-sm font-semibold text-surface-900 mb-4 flex items-center gap-2">
				<Building2 class="w-4 h-4 text-surface-400" />
				Company & Type
			</h3>
			<div class="grid grid-cols-1 md:grid-cols-3 gap-4">
				<Input label="Company Name" name="company_name" bind:value={companyName} placeholder="Optional" />
				<Select
					label="Customer Type"
					name="customer_type"
					options={customerTypeOptions}
					bind:value={customerType}
				/>
				<Select
					label="Lead Source"
					name="source"
					options={sourceOptions}
					bind:value={source}
				/>
			</div>
		</Card>

		<!-- Primary Address -->
		<Card>
			<h3 class="text-sm font-semibold text-surface-900 mb-4 flex items-center gap-2">
				<MapPin class="w-4 h-4 text-surface-400" />
				Primary Address
			</h3>
			<div class="space-y-4">
				<Input label="Address Line 1" name="address_line1" bind:value={addressLine1} placeholder="123 Main St" />
				<Input label="Address Line 2" name="address_line2" bind:value={addressLine2} placeholder="Suite, Apt, Unit (optional)" />
				<div class="grid grid-cols-1 md:grid-cols-3 gap-4">
					<Input label="City" name="city" bind:value={city} placeholder="City" />
					<Input label="State" name="state" bind:value={addressState} placeholder="CA" />
					<Input label="ZIP Code" name="zip_code" bind:value={zip} placeholder="90210" />
				</div>
			</div>
		</Card>

		<!-- Preferences -->
		<Card>
			<h3 class="text-sm font-semibold text-surface-900 mb-4 flex items-center gap-2">
				<Phone class="w-4 h-4 text-surface-400" />
				Preferences
			</h3>
			<div class="grid grid-cols-1 md:grid-cols-2 gap-4">
				<Select
					label="Preferred Contact Method"
					name="contact_method"
					options={contactMethodOptions}
					bind:value={contactMethod}
				/>
				<Select
					label="Credit Terms"
					name="credit_terms"
					options={creditTermsOptions}
					bind:value={creditTerms}
				/>
			</div>
			<div class="mt-4">
				<Textarea
					label="Notes"
					name="notes"
					bind:value={notes}
					placeholder="Internal notes about this customer..."
					rows={3}
				/>
			</div>
		</Card>

		<!-- Actions -->
		<div class="flex justify-end gap-3 pb-8">
			<Button variant="outline" href="/dashboard/customers">Cancel</Button>
			<Button type="submit" disabled={submitting}>
				{#if submitting}
					<Loader2 class="w-4 h-4 animate-spin" />
					Creating...
				{:else}
					<UserPlus class="w-4 h-4" />
					Create Customer
				{/if}
			</Button>
		</div>
	</div>
</form>
