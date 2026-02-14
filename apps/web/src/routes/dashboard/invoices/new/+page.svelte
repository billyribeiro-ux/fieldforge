<script lang="ts">
	import { enhance } from '$app/forms';
	import TopBar from '$lib/components/layout/TopBar.svelte';
	import Card from '$lib/components/ui/Card.svelte';
	import Button from '$lib/components/ui/Button.svelte';
	import Input from '$lib/components/ui/Input.svelte';
	import Select from '$lib/components/ui/Select.svelte';
	import Textarea from '$lib/components/ui/Textarea.svelte';
	import { ArrowLeft, Plus, Trash2, Send, Save, Loader2 } from 'lucide-svelte';

	let { form: formResult } = $props();

	let customerId = $state('');
	let dueDate = $state('');
	let paymentTerms = $state('Due on receipt');
	let notes = $state('');
	let taxRate = $state(8.25);

	interface LineItem {
		id: string;
		description: string;
		category: string;
		quantity: number;
		unit: string;
		unit_price: number;
	}

	let lineItems = $state<LineItem[]>([
		{ id: crypto.randomUUID(), description: '', category: 'labor', quantity: 1, unit: 'hours', unit_price: 0 }
	]);

	const customers = [
		{ value: '1', label: 'Sarah Johnson' },
		{ value: '2', label: 'Robert Chen' },
		{ value: '3', label: 'Mike Williams' },
		{ value: '4', label: 'Lisa Anderson' }
	];

	const categoryOptions = [
		{ value: 'labor', label: 'Labor' },
		{ value: 'materials', label: 'Materials' },
		{ value: 'equipment', label: 'Equipment' },
		{ value: 'permits', label: 'Permits' },
		{ value: 'disposal', label: 'Disposal' },
		{ value: 'subcontractor', label: 'Subcontractor' },
		{ value: 'other', label: 'Other' }
	];

	const paymentTermsOptions = [
		{ value: 'Due on receipt', label: 'Due on receipt' },
		{ value: 'Net 15', label: 'Net 15' },
		{ value: 'Net 30', label: 'Net 30' },
		{ value: 'Net 45', label: 'Net 45' },
		{ value: 'Net 60', label: 'Net 60' },
		{ value: '50% deposit, balance on completion', label: '50% deposit, balance on completion' }
	];

	function addLineItem() {
		lineItems.push({
			id: crypto.randomUUID(),
			description: '',
			category: 'labor',
			quantity: 1,
			unit: 'hours',
			unit_price: 0
		});
	}

	function removeLineItem(id: string) {
		lineItems = lineItems.filter((item) => item.id !== id);
	}

	let subtotal = $derived(lineItems.reduce((sum, item) => sum + item.quantity * item.unit_price, 0));
	let taxAmount = $derived(subtotal * (taxRate / 100));
	let total = $derived(subtotal + taxAmount);

	function formatCurrency(n: number): string {
		return new Intl.NumberFormat('en-US', { style: 'currency', currency: 'USD', minimumFractionDigits: 2 }).format(n);
	}

	let submitting = $state(false);

	let serializedLineItems = $derived(JSON.stringify(lineItems.map(item => ({
		description: item.description,
		category: item.category,
		quantity: item.quantity,
		unit: item.unit,
		unit_price: item.unit_price
	}))));
</script>

<svelte:head>
	<title>New Invoice — FieldForge</title>
</svelte:head>

<TopBar>
	{#snippet actions()}
		<Button variant="outline" size="sm" href="/dashboard/invoices">
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
	<input type="hidden" name="customer_id" value={customerId} />
	<input type="hidden" name="due_date" value={dueDate} />
	<input type="hidden" name="line_items" value={serializedLineItems} />

<div class="p-6 space-y-6">
	{#if formResult?.error}
		<div class="bg-danger-50 border border-danger-200 text-danger-700 px-4 py-3 rounded-xl text-sm">
			{formResult.error}
		</div>
	{/if}

	<!-- Header -->
	<div class="flex items-center gap-4">
		<a href="/dashboard/invoices" class="p-1.5 hover:bg-surface-100 rounded-lg transition-colors">
			<ArrowLeft class="w-5 h-5 text-surface-500" />
		</a>
		<h1 class="text-2xl font-bold text-surface-900">New Invoice</h1>
	</div>

	<div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
		<!-- Main Content -->
		<div class="lg:col-span-2 space-y-6">
			<!-- Customer & Details -->
			<Card>
				<h2 class="text-base font-semibold text-surface-900 mb-4">Invoice Details</h2>
				<div class="space-y-4">
					<Select label="Customer" options={customers} bind:value={customerId} required />
					<div class="grid grid-cols-2 gap-4">
						<Input label="Due Date" type="date" bind:value={dueDate} />
						<Select label="Payment Terms" options={paymentTermsOptions} bind:value={paymentTerms} />
					</div>
				</div>
			</Card>

			<!-- Line Items -->
			<Card>
				<div class="flex items-center justify-between mb-4">
					<h2 class="text-base font-semibold text-surface-900">Line Items</h2>
					<Button variant="outline" size="sm" onclick={addLineItem}>
						<Plus class="w-4 h-4" />
						Add Item
					</Button>
				</div>

				<div class="space-y-3">
					{#each lineItems as item, i (item.id)}
						<div class="flex items-start gap-3 p-4 bg-surface-50 rounded-lg">
							<div class="flex-1 space-y-3">
								<div class="grid grid-cols-[1fr_auto] gap-3">
									<input
										type="text"
										placeholder="Description"
										bind:value={item.description}
										class="w-full px-3 py-2 text-sm bg-white border border-surface-200 rounded-lg focus:outline-none focus:ring-2 focus:ring-forge-500 placeholder:text-surface-400"
									/>
									<select
										bind:value={item.category}
										class="px-3 py-2 text-sm bg-white border border-surface-200 rounded-lg focus:outline-none focus:ring-2 focus:ring-forge-500 text-surface-700 cursor-pointer"
									>
										{#each categoryOptions as cat}
											<option value={cat.value}>{cat.label}</option>
										{/each}
									</select>
								</div>
								<div class="grid grid-cols-4 gap-3">
									<div>
										<label class="text-xs text-surface-500 mb-1 block">Qty</label>
										<input
											type="number"
											bind:value={item.quantity}
											min="0"
											step="0.5"
											class="w-full px-3 py-2 text-sm bg-white border border-surface-200 rounded-lg focus:outline-none focus:ring-2 focus:ring-forge-500"
										/>
									</div>
									<div>
										<label class="text-xs text-surface-500 mb-1 block">Unit</label>
										<input
											type="text"
											bind:value={item.unit}
											class="w-full px-3 py-2 text-sm bg-white border border-surface-200 rounded-lg focus:outline-none focus:ring-2 focus:ring-forge-500"
										/>
									</div>
									<div>
										<label class="text-xs text-surface-500 mb-1 block">Rate</label>
										<input
											type="number"
											bind:value={item.unit_price}
											min="0"
											step="0.01"
											class="w-full px-3 py-2 text-sm bg-white border border-surface-200 rounded-lg focus:outline-none focus:ring-2 focus:ring-forge-500"
										/>
									</div>
									<div>
										<label class="text-xs text-surface-500 mb-1 block">Amount</label>
										<p class="px-3 py-2 text-sm font-medium text-surface-900">{formatCurrency(item.quantity * item.unit_price)}</p>
									</div>
								</div>
							</div>
							{#if lineItems.length > 1}
								<button onclick={() => removeLineItem(item.id)} class="mt-2 p-1.5 text-surface-400 hover:text-danger-500 rounded cursor-pointer">
									<Trash2 class="w-4 h-4" />
								</button>
							{/if}
						</div>
					{/each}
				</div>
			</Card>

			<!-- Notes -->
			<Card>
				<h2 class="text-base font-semibold text-surface-900 mb-4">Notes</h2>
				<Textarea label="Notes (visible to customer)" bind:value={notes} rows={3} placeholder="Thank you for your business!" />
			</Card>
		</div>

		<!-- Sidebar -->
		<div class="space-y-6">
			<!-- Totals -->
			<Card>
				<h3 class="text-sm font-semibold text-surface-500 uppercase tracking-wider mb-4">Summary</h3>
				<div class="space-y-3">
					<div class="flex justify-between text-sm">
						<span class="text-surface-500">Subtotal</span>
						<span class="font-medium text-surface-900">{formatCurrency(subtotal)}</span>
					</div>
					<div class="flex justify-between text-sm items-center">
						<div class="flex items-center gap-2">
							<span class="text-surface-500">Tax</span>
							<input
								type="number"
								bind:value={taxRate}
								min="0"
								max="100"
								step="0.01"
								class="w-16 px-2 py-1 text-xs bg-white border border-surface-200 rounded focus:outline-none focus:ring-2 focus:ring-forge-500 text-right"
							/>
							<span class="text-xs text-surface-400">%</span>
						</div>
						<span class="font-medium text-surface-900">{formatCurrency(taxAmount)}</span>
					</div>
					<div class="flex justify-between text-lg font-bold border-t border-surface-200 pt-3">
						<span>Total</span>
						<span class="text-forge-600">{formatCurrency(total)}</span>
					</div>
				</div>
			</Card>

			<!-- Quick Info -->
			<Card>
				<h3 class="text-sm font-semibold text-surface-500 uppercase tracking-wider mb-3">Info</h3>
				<div class="space-y-2 text-sm">
					<div class="flex justify-between">
						<span class="text-surface-400">Items</span>
						<span class="text-surface-700">{lineItems.length}</span>
					</div>
					<div class="flex justify-between">
						<span class="text-surface-400">Payment Terms</span>
						<span class="text-surface-700">{paymentTerms}</span>
					</div>
				</div>
			</Card>
			<!-- Submit -->
			<Card>
				<Button type="submit" variant="primary" size="lg" class="w-full" disabled={submitting}>
					{#if submitting}
						<Loader2 class="w-4 h-4 animate-spin" />
						Creating...
					{:else}
						<Send class="w-4 h-4" />
						Send Invoice
					{/if}
				</Button>
			</Card>
		</div>
	</div>
</div>
</form>
