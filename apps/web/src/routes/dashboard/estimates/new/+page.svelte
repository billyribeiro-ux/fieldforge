<script lang="ts">
	import TopBar from '$lib/components/layout/TopBar.svelte';
	import Card from '$lib/components/ui/Card.svelte';
	import Button from '$lib/components/ui/Button.svelte';
	import Input from '$lib/components/ui/Input.svelte';
	import Select from '$lib/components/ui/Select.svelte';
	import Textarea from '$lib/components/ui/Textarea.svelte';
	import Badge from '$lib/components/ui/Badge.svelte';
	import {
		ArrowLeft,
		Plus,
		Trash2,
		GripVertical,
		Send,
		Save,
		Eye,
		Copy,
		DollarSign
	} from 'lucide-svelte';

	interface LineItem {
		id: string;
		description: string;
		category: string;
		quantity: number;
		unit: string;
		unitPrice: number;
		taxable: boolean;
	}

	let customerId = $state('');
	let title = $state('');
	let scopeOfWork = $state('');
	let validUntilDays = $state('15');
	let paymentTerms = $state('');
	let warrantyTerms = $state('');
	let discountType = $state<'percent' | 'fixed'>('percent');
	let discountValue = $state(0);
	let taxRate = $state(8.25);

	let lineItems = $state<LineItem[]>([
		{ id: crypto.randomUUID(), description: '', category: 'labor', quantity: 1, unit: 'hours', unitPrice: 0, taxable: true }
	]);

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

	const categoryOptions = [
		{ value: 'labor', label: 'Labor' },
		{ value: 'materials', label: 'Materials' },
		{ value: 'equipment', label: 'Equipment' },
		{ value: 'permits', label: 'Permits' },
		{ value: 'disposal', label: 'Disposal' },
		{ value: 'overhead', label: 'Overhead' },
		{ value: 'other', label: 'Other' }
	];

	const unitOptions = [
		{ value: 'hours', label: 'hours' },
		{ value: 'each', label: 'each' },
		{ value: 'sqft', label: 'sq ft' },
		{ value: 'lnft', label: 'ln ft' },
		{ value: 'lot', label: 'lot' },
		{ value: 'day', label: 'day' }
	];

	function addLineItem() {
		lineItems = [
			...lineItems,
			{ id: crypto.randomUUID(), description: '', category: 'materials', quantity: 1, unit: 'each', unitPrice: 0, taxable: true }
		];
	}

	function removeLineItem(id: string) {
		if (lineItems.length <= 1) return;
		lineItems = lineItems.filter((item) => item.id !== id);
	}

	function lineTotal(item: LineItem): number {
		return item.quantity * item.unitPrice;
	}

	let subtotal = $derived(lineItems.reduce((sum, item) => sum + lineTotal(item), 0));

	let discountAmount = $derived(
		discountType === 'percent' ? subtotal * (discountValue / 100) : discountValue
	);

	let taxableSubtotal = $derived(
		lineItems
			.filter((item) => item.taxable)
			.reduce((sum, item) => sum + lineTotal(item), 0) - discountAmount
	);

	let taxAmount = $derived(Math.max(0, taxableSubtotal) * (taxRate / 100));

	let total = $derived(subtotal - discountAmount + taxAmount);

	function formatCurrency(n: number): string {
		return new Intl.NumberFormat('en-US', { style: 'currency', currency: 'USD', minimumFractionDigits: 2 }).format(n);
	}

	let saving = $state(false);
	let sending = $state(false);
</script>

<TopBar>
	{#snippet actions()}
		<div class="flex items-center gap-2">
			<Button variant="ghost" size="md">
				<Eye class="w-4 h-4" />
				Preview
			</Button>
			<Button variant="outline" size="md" loading={saving}>
				<Save class="w-4 h-4" />
				Save Draft
			</Button>
			<Button variant="primary" size="md" loading={sending}>
				<Send class="w-4 h-4" />
				Send to Customer
			</Button>
		</div>
	{/snippet}
</TopBar>

<div class="p-6 space-y-6">
	<!-- Back + Title -->
	<div class="flex items-center gap-4">
		<a href="/dashboard/estimates" class="p-1.5 hover:bg-surface-100 rounded-lg transition-colors">
			<ArrowLeft class="w-5 h-5 text-surface-500" />
		</a>
		<div>
			<h1 class="text-2xl font-bold text-surface-900">New Estimate</h1>
			<p class="text-sm text-surface-500">Create a detailed estimate for your customer</p>
		</div>
	</div>

	<div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
		<!-- Main Content (2/3) -->
		<div class="lg:col-span-2 space-y-6">
			<!-- Customer & Details -->
			<Card>
				<div class="space-y-4">
					<Select label="Customer" options={customerOptions} bind:value={customerId} required placeholder="Select customer..." />
					<Input label="Estimate Title" bind:value={title} placeholder="e.g. AC Unit Replacement — 3 Ton Carrier" />
					<Textarea label="Scope of Work" bind:value={scopeOfWork} placeholder="Describe the work to be performed..." rows={3} />
				</div>
			</Card>

			<!-- Line Items -->
			<Card padding={false}>
				<div class="px-6 py-4 border-b border-surface-200">
					<div class="flex items-center justify-between">
						<h3 class="text-base font-semibold text-surface-900">Line Items</h3>
						<Button variant="outline" size="sm" onclick={addLineItem}>
							<Plus class="w-4 h-4" />
							Add Item
						</Button>
					</div>
				</div>

				<!-- Header -->
				<div class="grid grid-cols-[1fr_120px_80px_80px_100px_100px_40px] gap-2 px-6 py-2 bg-surface-50 border-b border-surface-200 text-xs font-medium text-surface-500 uppercase tracking-wider">
					<span>Description</span>
					<span>Category</span>
					<span>Qty</span>
					<span>Unit</span>
					<span>Unit Price</span>
					<span class="text-right">Total</span>
					<span></span>
				</div>

				<!-- Items -->
				<div class="divide-y divide-surface-100">
					{#each lineItems as item, i (item.id)}
						<div class="grid grid-cols-[1fr_120px_80px_80px_100px_100px_40px] gap-2 px-6 py-3 items-center">
							<input
								type="text"
								bind:value={lineItems[i].description}
								placeholder="Item description..."
								class="w-full px-2 py-1.5 text-sm border border-surface-200 rounded focus:outline-none focus:ring-1 focus:ring-forge-500"
							/>
							<select
								bind:value={lineItems[i].category}
								class="w-full px-2 py-1.5 text-sm border border-surface-200 rounded focus:outline-none focus:ring-1 focus:ring-forge-500"
							>
								{#each categoryOptions as opt}
									<option value={opt.value}>{opt.label}</option>
								{/each}
							</select>
							<input
								type="number"
								bind:value={lineItems[i].quantity}
								min="0"
								step="0.5"
								class="w-full px-2 py-1.5 text-sm border border-surface-200 rounded focus:outline-none focus:ring-1 focus:ring-forge-500 text-right"
							/>
							<select
								bind:value={lineItems[i].unit}
								class="w-full px-2 py-1.5 text-sm border border-surface-200 rounded focus:outline-none focus:ring-1 focus:ring-forge-500"
							>
								{#each unitOptions as opt}
									<option value={opt.value}>{opt.label}</option>
								{/each}
							</select>
							<div class="relative">
								<span class="absolute left-2 top-1/2 -translate-y-1/2 text-xs text-surface-400">$</span>
								<input
									type="number"
									bind:value={lineItems[i].unitPrice}
									min="0"
									step="0.01"
									class="w-full pl-5 pr-2 py-1.5 text-sm border border-surface-200 rounded focus:outline-none focus:ring-1 focus:ring-forge-500 text-right"
								/>
							</div>
							<span class="text-sm font-medium text-surface-900 text-right">
								{formatCurrency(lineTotal(item))}
							</span>
							<button
								onclick={() => removeLineItem(item.id)}
								class="p-1 text-surface-400 hover:text-danger-500 rounded transition-colors cursor-pointer"
								disabled={lineItems.length <= 1}
							>
								<Trash2 class="w-4 h-4" />
							</button>
						</div>
					{/each}
				</div>

				<!-- Add item row -->
				<div class="px-6 py-3 border-t border-surface-200">
					<button
						onclick={addLineItem}
						class="flex items-center gap-2 text-sm text-forge-600 hover:text-forge-700 font-medium cursor-pointer"
					>
						<Plus class="w-4 h-4" />
						Add line item
					</button>
				</div>
			</Card>

			<!-- Terms -->
			<Card>
				<div class="grid grid-cols-2 gap-4">
					<Textarea label="Payment Terms" bind:value={paymentTerms} placeholder="e.g. 50% deposit required, balance due on completion" rows={2} />
					<Textarea label="Warranty Terms" bind:value={warrantyTerms} placeholder="e.g. 1-year parts and labor warranty" rows={2} />
				</div>
			</Card>
		</div>

		<!-- Right Column — Totals (1/3) -->
		<div class="space-y-6">
			<!-- Totals Card -->
			<Card class="sticky top-20">
				<h3 class="text-base font-semibold text-surface-900 mb-4">Estimate Summary</h3>

				<div class="space-y-3">
					<div class="flex items-center justify-between text-sm">
						<span class="text-surface-500">Subtotal</span>
						<span class="font-medium text-surface-900">{formatCurrency(subtotal)}</span>
					</div>

					<!-- Discount -->
					<div class="flex items-center justify-between text-sm">
						<span class="text-surface-500">Discount</span>
						<div class="flex items-center gap-2">
							<div class="flex rounded border border-surface-200 overflow-hidden">
								<button
									onclick={() => (discountType = 'percent')}
									class="px-2 py-1 text-xs cursor-pointer {discountType === 'percent' ? 'bg-forge-100 text-forge-700' : 'bg-white text-surface-500'}"
								>%</button>
								<button
									onclick={() => (discountType = 'fixed')}
									class="px-2 py-1 text-xs cursor-pointer {discountType === 'fixed' ? 'bg-forge-100 text-forge-700' : 'bg-white text-surface-500'}"
								>$</button>
							</div>
							<input
								type="number"
								bind:value={discountValue}
								min="0"
								step="0.01"
								class="w-20 px-2 py-1 text-sm border border-surface-200 rounded text-right focus:outline-none focus:ring-1 focus:ring-forge-500"
							/>
						</div>
					</div>

					{#if discountAmount > 0}
						<div class="flex items-center justify-between text-sm">
							<span class="text-surface-400">Discount amount</span>
							<span class="text-danger-500 font-medium">-{formatCurrency(discountAmount)}</span>
						</div>
					{/if}

					<!-- Tax -->
					<div class="flex items-center justify-between text-sm">
						<span class="text-surface-500">Tax ({taxRate}%)</span>
						<span class="font-medium text-surface-900">{formatCurrency(taxAmount)}</span>
					</div>

					<div class="border-t border-surface-200 pt-3">
						<div class="flex items-center justify-between">
							<span class="text-base font-semibold text-surface-900">Total</span>
							<span class="text-2xl font-bold text-surface-900">{formatCurrency(total)}</span>
						</div>
					</div>
				</div>

				<!-- Margin indicator -->
				{#if subtotal > 0}
					<div class="mt-4 pt-4 border-t border-surface-200">
						<div class="flex items-center justify-between text-sm">
							<span class="text-surface-500">Labor</span>
							<span class="font-medium text-surface-700">
								{formatCurrency(lineItems.filter(i => i.category === 'labor').reduce((s, i) => s + lineTotal(i), 0))}
							</span>
						</div>
						<div class="flex items-center justify-between text-sm mt-1">
							<span class="text-surface-500">Materials</span>
							<span class="font-medium text-surface-700">
								{formatCurrency(lineItems.filter(i => i.category === 'materials').reduce((s, i) => s + lineTotal(i), 0))}
							</span>
						</div>
					</div>
				{/if}

				<!-- Valid Until -->
				<div class="mt-4 pt-4 border-t border-surface-200">
					<Select
						label="Valid For"
						options={[
							{ value: '7', label: '7 days' },
							{ value: '15', label: '15 days' },
							{ value: '30', label: '30 days' },
							{ value: '60', label: '60 days' },
							{ value: '90', label: '90 days' }
						]}
						bind:value={validUntilDays}
					/>
				</div>

				<!-- Actions -->
				<div class="mt-6 space-y-2">
					<Button variant="primary" size="lg" class="w-full" loading={sending}>
						<Send class="w-4 h-4" />
						Send to Customer
					</Button>
					<Button variant="outline" size="md" class="w-full" loading={saving}>
						<Save class="w-4 h-4" />
						Save as Draft
					</Button>
				</div>
			</Card>
		</div>
	</div>
</div>
