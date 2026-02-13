<script lang="ts">
	import { page } from '$app/state';
	import TopBar from '$lib/components/layout/TopBar.svelte';
	import Card from '$lib/components/ui/Card.svelte';
	import Button from '$lib/components/ui/Button.svelte';
	import Badge from '$lib/components/ui/Badge.svelte';
	import Input from '$lib/components/ui/Input.svelte';
	import Select from '$lib/components/ui/Select.svelte';
	import Modal from '$lib/components/ui/Modal.svelte';
	import {
		ArrowLeft,
		Send,
		DollarSign,
		Download,
		Copy,
		Printer,
		CreditCard,
		CheckCircle2,
		Clock,
		AlertTriangle,
		ExternalLink
	} from 'lucide-svelte';

	const invoice = {
		id: page.params.id,
		number: 'FF-001',
		status: 'sent',
		customer: { id: '1', name: 'Sarah Johnson', email: 'sarah@email.com', phone: '(512) 555-0101' },
		job: { id: '1', title: 'AC Unit Replacement' },
		property: '123 Oak St, Austin TX 78701',
		subtotal: 3850,
		discount_amount: 0,
		tax_rate: 8.25,
		tax_amount: 317.63,
		total: 4167.63,
		amount_paid: 0,
		amount_due: 4167.63,
		due_date: '2024-12-25',
		sent_at: '2024-12-15',
		payment_terms: 'Net 30',
		notes: 'Thank you for your business!',
		portal_url: 'https://pay.fieldforge.com/inv/abc123',
		line_items: [
			{ id: '1', description: 'Carrier 24ACC636 — 3 Ton AC Unit', category: 'materials', quantity: 1, unit: 'each', unit_price: 2400, total: 2400 },
			{ id: '2', description: 'Installation labor', category: 'labor', quantity: 6, unit: 'hours', unit_price: 125, total: 750 },
			{ id: '3', description: 'Refrigerant R-410A', category: 'materials', quantity: 1, unit: 'lot', unit_price: 200, total: 200 },
			{ id: '4', description: 'Electrical disconnect + whip', category: 'materials', quantity: 1, unit: 'each', unit_price: 150, total: 150 },
			{ id: '5', description: 'Concrete pad', category: 'materials', quantity: 1, unit: 'each', unit_price: 100, total: 100 },
			{ id: '6', description: 'Disposal of old unit', category: 'disposal', quantity: 1, unit: 'each', unit_price: 150, total: 150 }
		],
		payments: []
	};

	let showPaymentModal = $state(false);
	let paymentAmount = $state(invoice.amount_due.toString());
	let paymentMethod = $state('card');
	let tipAmount = $state('0');
	let checkNumber = $state('');
	let paymentNotes = $state('');
	let recordingPayment = $state(false);

	const paymentMethodOptions = [
		{ value: 'card', label: 'Credit/Debit Card' },
		{ value: 'ach', label: 'ACH / Bank Transfer' },
		{ value: 'cash', label: 'Cash' },
		{ value: 'check', label: 'Check' },
		{ value: 'venmo', label: 'Venmo' },
		{ value: 'zelle', label: 'Zelle' },
		{ value: 'apple_pay', label: 'Apple Pay' },
		{ value: 'other', label: 'Other' }
	];

	function formatCurrency(n: number): string {
		return new Intl.NumberFormat('en-US', { style: 'currency', currency: 'USD', minimumFractionDigits: 2 }).format(n);
	}

	function statusVariant(s: string): 'success' | 'warning' | 'danger' | 'info' | 'default' {
		switch (s) {
			case 'paid': return 'success';
			case 'sent': case 'viewed': return 'info';
			case 'partially_paid': return 'warning';
			case 'overdue': return 'danger';
			default: return 'default';
		}
	}

	function statusLabel(s: string): string {
		return s.replace(/_/g, ' ').replace(/\b\w/g, (c) => c.toUpperCase());
	}

	function categoryLabel(c: string): string {
		return c.charAt(0).toUpperCase() + c.slice(1);
	}

	async function recordPayment() {
		recordingPayment = true;
		// TODO: API call to record payment
		await new Promise((r) => setTimeout(r, 1000));
		recordingPayment = false;
		showPaymentModal = false;
	}
</script>

<svelte:head>
	<title>Invoice Detail — FieldForge</title>
</svelte:head>

<TopBar>
	{#snippet actions()}
		<div class="flex items-center gap-2">
			<Button variant="ghost" size="md">
				<Printer class="w-4 h-4" />
				Print
			</Button>
			<Button variant="ghost" size="md">
				<Download class="w-4 h-4" />
				PDF
			</Button>
			{#if invoice.status !== 'paid' && invoice.status !== 'void'}
				<Button variant="primary" size="md" onclick={() => (showPaymentModal = true)}>
					<DollarSign class="w-4 h-4" />
					Record Payment
				</Button>
			{/if}
		</div>
	{/snippet}
</TopBar>

<div class="p-6 space-y-6">
	<!-- Back + Header -->
	<div class="flex items-start gap-4">
		<a href="/dashboard/invoices" class="mt-1 p-1.5 hover:bg-surface-100 rounded-lg transition-colors">
			<ArrowLeft class="w-5 h-5 text-surface-500" />
		</a>
		<div class="flex-1">
			<div class="flex items-center gap-3 mb-1">
				<h1 class="text-2xl font-bold text-surface-900">Invoice {invoice.number}</h1>
				<Badge variant={statusVariant(invoice.status)} size="md">{statusLabel(invoice.status)}</Badge>
			</div>
			<p class="text-sm text-surface-500">
				{invoice.job.title} • {invoice.customer.name}
			</p>
		</div>
	</div>

	<div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
		<!-- Invoice Document (2/3) -->
		<div class="lg:col-span-2">
			<Card padding={false}>
				<!-- Invoice Header -->
				<div class="px-8 py-6 border-b border-surface-200">
					<div class="flex items-start justify-between">
						<div>
							<h2 class="text-xl font-bold text-surface-900">INVOICE</h2>
							<p class="text-lg font-mono text-surface-600 mt-1">{invoice.number}</p>
						</div>
						<div class="text-right">
							<p class="text-lg font-bold text-surface-900">Smith HVAC & Plumbing</p>
							<p class="text-sm text-surface-500">100 Main St, Suite 200</p>
							<p class="text-sm text-surface-500">Austin, TX 78701</p>
							<p class="text-sm text-surface-500">(512) 555-0100</p>
						</div>
					</div>
				</div>

				<!-- Bill To + Details -->
				<div class="px-8 py-6 border-b border-surface-200">
					<div class="grid grid-cols-2 gap-8">
						<div>
							<p class="text-xs font-medium text-surface-400 uppercase tracking-wider mb-2">Bill To</p>
							<p class="text-sm font-semibold text-surface-900">{invoice.customer.name}</p>
							<p class="text-sm text-surface-600">{invoice.property}</p>
							<p class="text-sm text-surface-600">{invoice.customer.email}</p>
							<p class="text-sm text-surface-600">{invoice.customer.phone}</p>
						</div>
						<div class="text-right">
							<div class="space-y-1.5">
								<div class="flex justify-end gap-4 text-sm">
									<span class="text-surface-400">Invoice Date</span>
									<span class="font-medium text-surface-700">{invoice.sent_at}</span>
								</div>
								<div class="flex justify-end gap-4 text-sm">
									<span class="text-surface-400">Due Date</span>
									<span class="font-medium text-surface-700">{invoice.due_date}</span>
								</div>
								<div class="flex justify-end gap-4 text-sm">
									<span class="text-surface-400">Terms</span>
									<span class="font-medium text-surface-700">{invoice.payment_terms}</span>
								</div>
							</div>
						</div>
					</div>
				</div>

				<!-- Line Items Table -->
				<div class="px-8 py-4">
					<table class="w-full">
						<thead>
							<tr class="border-b border-surface-200">
								<th class="text-left text-xs font-medium text-surface-500 uppercase tracking-wider pb-3">Description</th>
								<th class="text-left text-xs font-medium text-surface-500 uppercase tracking-wider pb-3">Category</th>
								<th class="text-right text-xs font-medium text-surface-500 uppercase tracking-wider pb-3">Qty</th>
								<th class="text-right text-xs font-medium text-surface-500 uppercase tracking-wider pb-3">Rate</th>
								<th class="text-right text-xs font-medium text-surface-500 uppercase tracking-wider pb-3">Amount</th>
							</tr>
						</thead>
						<tbody class="divide-y divide-surface-100">
							{#each invoice.line_items as item}
								<tr>
									<td class="py-3 text-sm text-surface-900">{item.description}</td>
									<td class="py-3 text-sm text-surface-500">{categoryLabel(item.category)}</td>
									<td class="py-3 text-sm text-surface-700 text-right">{item.quantity} {item.unit}</td>
									<td class="py-3 text-sm text-surface-700 text-right">{formatCurrency(item.unit_price)}</td>
									<td class="py-3 text-sm font-medium text-surface-900 text-right">{formatCurrency(item.total)}</td>
								</tr>
							{/each}
						</tbody>
					</table>
				</div>

				<!-- Totals -->
				<div class="px-8 py-6 border-t border-surface-200 bg-surface-50">
					<div class="flex justify-end">
						<div class="w-64 space-y-2">
							<div class="flex justify-between text-sm">
								<span class="text-surface-500">Subtotal</span>
								<span class="font-medium text-surface-900">{formatCurrency(invoice.subtotal)}</span>
							</div>
							{#if invoice.discount_amount > 0}
								<div class="flex justify-between text-sm">
									<span class="text-surface-500">Discount</span>
									<span class="text-danger-500">-{formatCurrency(invoice.discount_amount)}</span>
								</div>
							{/if}
							<div class="flex justify-between text-sm">
								<span class="text-surface-500">Tax ({invoice.tax_rate}%)</span>
								<span class="font-medium text-surface-900">{formatCurrency(invoice.tax_amount)}</span>
							</div>
							<div class="flex justify-between text-base font-semibold border-t border-surface-300 pt-2">
								<span class="text-surface-900">Total</span>
								<span class="text-surface-900">{formatCurrency(invoice.total)}</span>
							</div>
							{#if invoice.amount_paid > 0}
								<div class="flex justify-between text-sm">
									<span class="text-surface-500">Paid</span>
									<span class="text-green-600 font-medium">-{formatCurrency(invoice.amount_paid)}</span>
								</div>
							{/if}
							<div class="flex justify-between text-lg font-bold border-t border-surface-300 pt-2">
								<span class="text-surface-900">Amount Due</span>
								<span class="text-forge-700">{formatCurrency(invoice.amount_due)}</span>
							</div>
						</div>
					</div>
				</div>

				<!-- Notes -->
				{#if invoice.notes}
					<div class="px-8 py-4 border-t border-surface-200">
						<p class="text-xs font-medium text-surface-400 uppercase tracking-wider mb-1">Notes</p>
						<p class="text-sm text-surface-600">{invoice.notes}</p>
					</div>
				{/if}
			</Card>
		</div>

		<!-- Right Column -->
		<div class="space-y-6">
			<!-- Payment Status -->
			<Card>
				<h3 class="text-sm font-semibold text-surface-500 uppercase tracking-wider mb-3">Payment Status</h3>
				<div class="space-y-3">
					<div class="flex items-center justify-between">
						<span class="text-sm text-surface-500">Total</span>
						<span class="text-sm font-medium text-surface-900">{formatCurrency(invoice.total)}</span>
					</div>
					<div class="flex items-center justify-between">
						<span class="text-sm text-surface-500">Paid</span>
						<span class="text-sm font-medium text-green-600">{formatCurrency(invoice.amount_paid)}</span>
					</div>
					<div class="flex items-center justify-between border-t border-surface-200 pt-2">
						<span class="text-sm font-semibold text-surface-900">Balance Due</span>
						<span class="text-lg font-bold text-forge-700">{formatCurrency(invoice.amount_due)}</span>
					</div>

					<!-- Progress bar -->
					<div class="w-full bg-surface-200 rounded-full h-2">
						<div
							class="bg-green-500 h-2 rounded-full transition-all duration-300"
							style="width: {(invoice.amount_paid / invoice.total) * 100}%"
						></div>
					</div>
				</div>

				{#if invoice.amount_due > 0}
					<Button variant="primary" size="md" class="w-full mt-4" onclick={() => (showPaymentModal = true)}>
						<DollarSign class="w-4 h-4" />
						Record Payment
					</Button>
				{/if}
			</Card>

			<!-- Customer Portal Link -->
			<Card>
				<h3 class="text-sm font-semibold text-surface-500 uppercase tracking-wider mb-3">Customer Portal</h3>
				<p class="text-xs text-surface-400 mb-3">Share this link with your customer to view and pay online.</p>
				<div class="flex items-center gap-2">
					<input
						type="text"
						value={invoice.portal_url}
						readonly
						class="flex-1 px-3 py-2 text-xs bg-surface-50 border border-surface-200 rounded-lg text-surface-600 font-mono"
					/>
					<Button variant="outline" size="sm">
						<Copy class="w-3.5 h-3.5" />
					</Button>
				</div>
			</Card>

			<!-- Actions -->
			<Card>
				<h3 class="text-sm font-semibold text-surface-500 uppercase tracking-wider mb-3">Actions</h3>
				<div class="space-y-2">
					{#if invoice.status === 'draft'}
						<Button variant="primary" size="sm" class="w-full">
							<Send class="w-4 h-4" />
							Send Invoice
						</Button>
					{/if}
					<Button variant="outline" size="sm" class="w-full">
						<Send class="w-4 h-4" />
						Resend
					</Button>
					<Button variant="outline" size="sm" class="w-full">
						<Download class="w-4 h-4" />
						Download PDF
					</Button>
					<Button variant="ghost" size="sm" class="w-full text-danger-500">
						Void Invoice
					</Button>
				</div>
			</Card>

			<!-- Payment History -->
			<Card padding={false}>
				<div class="px-6 py-4 border-b border-surface-200">
					<h3 class="text-sm font-semibold text-surface-500 uppercase tracking-wider">Payment History</h3>
				</div>
				{#if invoice.payments.length === 0}
					<div class="px-6 py-8 text-center">
						<Clock class="w-8 h-8 text-surface-300 mx-auto mb-2" />
						<p class="text-sm text-surface-400">No payments recorded</p>
					</div>
				{:else}
					<div class="divide-y divide-surface-100">
						{#each invoice.payments as payment}
							<div class="px-6 py-3">
								<div class="flex items-center justify-between">
									<div>
										<p class="text-sm font-medium text-surface-900">{formatCurrency(payment.amount)}</p>
										<p class="text-xs text-surface-400">{payment.method} • {payment.date}</p>
									</div>
									<Badge variant="success" size="sm">Paid</Badge>
								</div>
							</div>
						{/each}
					</div>
				{/if}
			</Card>
		</div>
	</div>
</div>

<!-- Record Payment Modal -->
<Modal open={showPaymentModal} title="Record Payment" size="md" onclose={() => (showPaymentModal = false)}>
	<div class="space-y-4">
		<div class="p-4 bg-forge-50 rounded-lg">
			<div class="flex items-center justify-between">
				<span class="text-sm text-surface-600">Amount Due</span>
				<span class="text-xl font-bold text-forge-700">{formatCurrency(invoice.amount_due)}</span>
			</div>
		</div>

		<Input label="Payment Amount" type="number" bind:value={paymentAmount} required />

		<Select label="Payment Method" options={paymentMethodOptions} bind:value={paymentMethod} required />

		{#if paymentMethod === 'check'}
			<Input label="Check Number" bind:value={checkNumber} placeholder="e.g. 1234" />
		{/if}

		<Input label="Tip Amount" type="number" bind:value={tipAmount} placeholder="0.00" />

		<Input label="Notes" bind:value={paymentNotes} placeholder="Optional payment notes..." />
	</div>

	{#snippet footer()}
		<Button variant="ghost" onclick={() => (showPaymentModal = false)}>Cancel</Button>
		<Button variant="primary" loading={recordingPayment} onclick={recordPayment}>
			<CheckCircle2 class="w-4 h-4" />
			Record Payment
		</Button>
	{/snippet}
</Modal>
