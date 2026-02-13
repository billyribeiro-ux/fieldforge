<script lang="ts">
	import { page } from '$app/state';
	import Card from '$lib/components/ui/Card.svelte';
	import Button from '$lib/components/ui/Button.svelte';
	import Badge from '$lib/components/ui/Badge.svelte';
	import { Wrench, CreditCard, Download, MessageSquare, CheckCircle2, Clock } from 'lucide-svelte';

	// Demo data — would come from public API using token
	const invoice = {
		number: 'FF-001',
		status: 'sent',
		company: { name: 'Smith HVAC & Plumbing', phone: '(512) 555-0100', email: 'office@smithhvac.com', address: '100 Main St, Suite 200, Austin TX 78701' },
		customer: 'Sarah Johnson',
		property: '123 Oak St, Austin TX 78701',
		job: 'AC Unit Replacement',
		due_date: '2024-12-25',
		issued_date: '2024-12-15',
		payment_terms: 'Net 30',
		line_items: [
			{ description: 'Carrier 24ACC636 — 3 Ton AC Unit', category: 'Materials', qty: '1 each', unit_price: 2400, total: 2400 },
			{ description: 'Installation labor', category: 'Labor', qty: '6 hours', unit_price: 125, total: 750 },
			{ description: 'Refrigerant R-410A', category: 'Materials', qty: '1 lot', unit_price: 200, total: 200 },
			{ description: 'Electrical disconnect + whip', category: 'Materials', qty: '1 each', unit_price: 150, total: 150 },
			{ description: 'Concrete pad', category: 'Materials', qty: '1 each', unit_price: 100, total: 100 },
			{ description: 'Disposal of old unit', category: 'Disposal', qty: '1 each', unit_price: 150, total: 150 }
		],
		subtotal: 3750,
		discount_amount: 0,
		tax_rate: 8.47,
		tax_amount: 317.63,
		total: 4067.63,
		amount_paid: 0,
		amount_due: 4067.63,
		payments: [] as { amount: number; method: string; date: string }[],
		notes: 'Thank you for your business!'
	};

	let paying = $state(false);
	let paymentMethod = $state('card');
	let showPaymentForm = $state(false);

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

	async function handlePayment() {
		paying = true;
		await new Promise((r) => setTimeout(r, 2000));
		paying = false;
		showPaymentForm = false;
	}
</script>

<div class="min-h-screen bg-surface-50">
	<!-- Header -->
	<header class="bg-white border-b border-surface-200">
		<div class="max-w-3xl mx-auto px-6 py-4 flex items-center justify-between">
			<div class="flex items-center gap-3">
				<div class="w-9 h-9 bg-forge-600 rounded-xl flex items-center justify-center">
					<Wrench class="w-5 h-5 text-white" />
				</div>
				<div>
					<p class="text-base font-bold text-surface-900">{invoice.company.name}</p>
					<p class="text-xs text-surface-400">{invoice.company.phone}</p>
				</div>
			</div>
			<Badge variant={statusVariant(invoice.status)} size="md">{statusLabel(invoice.status)}</Badge>
		</div>
	</header>

	<main class="max-w-3xl mx-auto px-6 py-8 space-y-6">
		<!-- Title + Amount Due -->
		<div class="flex items-start justify-between">
			<div>
				<p class="text-sm text-surface-400 font-mono">{invoice.number}</p>
				<h1 class="text-2xl font-bold text-surface-900 mt-1">Invoice</h1>
				<p class="text-sm text-surface-500 mt-2">For <strong>{invoice.customer}</strong> • {invoice.property}</p>
			</div>
			<div class="text-right">
				<p class="text-xs text-surface-400 uppercase tracking-wider">Amount Due</p>
				<p class="text-3xl font-bold text-surface-900 mt-1">{formatCurrency(invoice.amount_due)}</p>
				<p class="text-sm text-surface-500 mt-1">Due {invoice.due_date}</p>
			</div>
		</div>

		<!-- Pay Now CTA -->
		{#if invoice.amount_due > 0}
			<Card class="border-forge-200 bg-forge-50/30">
				{#if !showPaymentForm}
					<div class="text-center">
						<h3 class="text-lg font-semibold text-surface-900 mb-2">Pay this invoice online</h3>
						<p class="text-sm text-surface-500 mb-4">Secure payment powered by Stripe</p>
						<Button variant="primary" size="lg" onclick={() => (showPaymentForm = true)}>
							<CreditCard class="w-5 h-5" />
							Pay {formatCurrency(invoice.amount_due)}
						</Button>
					</div>
				{:else}
					<div class="max-w-md mx-auto space-y-4">
						<h3 class="text-lg font-semibold text-surface-900 text-center">Choose payment method</h3>

						<div class="grid grid-cols-2 gap-3">
							{#each [
								{ id: 'card', label: 'Credit / Debit Card', icon: '💳' },
								{ id: 'ach', label: 'Bank Transfer (ACH)', icon: '🏦' },
								{ id: 'apple_pay', label: 'Apple Pay', icon: '🍎' },
								{ id: 'google_pay', label: 'Google Pay', icon: '📱' }
							] as method}
								<button
									onclick={() => (paymentMethod = method.id)}
									class="p-3 rounded-lg border-2 text-left transition-all cursor-pointer
										{paymentMethod === method.id
										? 'border-forge-500 bg-forge-50'
										: 'border-surface-200 bg-white hover:border-surface-300'}"
								>
									<span class="text-lg">{method.icon}</span>
									<p class="text-sm font-medium text-surface-700 mt-1">{method.label}</p>
								</button>
							{/each}
						</div>

						{#if paymentMethod === 'card'}
							<div class="space-y-3">
								<div>
									<label class="text-sm font-medium text-surface-700 mb-1 block">Card Number</label>
									<input type="text" placeholder="4242 4242 4242 4242" class="w-full px-3 py-2 text-sm border border-surface-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-forge-500 placeholder:text-surface-400" />
								</div>
								<div class="grid grid-cols-2 gap-3">
									<div>
										<label class="text-sm font-medium text-surface-700 mb-1 block">Expiry</label>
										<input type="text" placeholder="MM / YY" class="w-full px-3 py-2 text-sm border border-surface-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-forge-500 placeholder:text-surface-400" />
									</div>
									<div>
										<label class="text-sm font-medium text-surface-700 mb-1 block">CVC</label>
										<input type="text" placeholder="123" class="w-full px-3 py-2 text-sm border border-surface-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-forge-500 placeholder:text-surface-400" />
									</div>
								</div>
							</div>
						{/if}

						<div class="flex items-center justify-center gap-3 pt-2">
							<Button variant="ghost" onclick={() => (showPaymentForm = false)}>Cancel</Button>
							<Button variant="primary" size="lg" loading={paying} onclick={handlePayment}>
								<CheckCircle2 class="w-5 h-5" />
								Pay {formatCurrency(invoice.amount_due)}
							</Button>
						</div>

						<p class="text-xs text-surface-400 text-center">
							🔒 Payments are processed securely. Your card details are never stored on our servers.
						</p>
					</div>
				{/if}
			</Card>
		{:else}
			<Card class="border-green-200 bg-green-50/50">
				<div class="flex items-center justify-center gap-3 py-4">
					<CheckCircle2 class="w-8 h-8 text-green-600" />
					<div>
						<p class="text-lg font-semibold text-green-800">Paid in Full</p>
						<p class="text-sm text-green-600">Thank you for your payment!</p>
					</div>
				</div>
			</Card>
		{/if}

		<!-- Invoice Details -->
		<Card>
			<div class="grid grid-cols-2 gap-6 text-sm">
				<div>
					<p class="text-xs font-medium text-surface-400 uppercase tracking-wider mb-2">Bill To</p>
					<p class="font-semibold text-surface-900">{invoice.customer}</p>
					<p class="text-surface-600">{invoice.property}</p>
				</div>
				<div class="text-right">
					<div class="space-y-1">
						<div class="flex justify-end gap-4">
							<span class="text-surface-400">Invoice Date</span>
							<span class="font-medium text-surface-700">{invoice.issued_date}</span>
						</div>
						<div class="flex justify-end gap-4">
							<span class="text-surface-400">Due Date</span>
							<span class="font-medium text-surface-700">{invoice.due_date}</span>
						</div>
						<div class="flex justify-end gap-4">
							<span class="text-surface-400">Terms</span>
							<span class="font-medium text-surface-700">{invoice.payment_terms}</span>
						</div>
					</div>
				</div>
			</div>
		</Card>

		<!-- Line Items -->
		<Card padding={false}>
			<table class="w-full">
				<thead>
					<tr class="border-b border-surface-200 bg-surface-50">
						<th class="text-left text-xs font-medium text-surface-500 uppercase tracking-wider px-6 py-3">Description</th>
						<th class="text-left text-xs font-medium text-surface-500 uppercase tracking-wider px-6 py-3">Category</th>
						<th class="text-right text-xs font-medium text-surface-500 uppercase tracking-wider px-6 py-3">Qty</th>
						<th class="text-right text-xs font-medium text-surface-500 uppercase tracking-wider px-6 py-3">Rate</th>
						<th class="text-right text-xs font-medium text-surface-500 uppercase tracking-wider px-6 py-3">Amount</th>
					</tr>
				</thead>
				<tbody class="divide-y divide-surface-100">
					{#each invoice.line_items as item}
						<tr>
							<td class="px-6 py-3 text-sm text-surface-900">{item.description}</td>
							<td class="px-6 py-3 text-sm text-surface-500">{item.category}</td>
							<td class="px-6 py-3 text-sm text-surface-700 text-right">{item.qty}</td>
							<td class="px-6 py-3 text-sm text-surface-700 text-right">{formatCurrency(item.unit_price)}</td>
							<td class="px-6 py-3 text-sm font-medium text-surface-900 text-right">{formatCurrency(item.total)}</td>
						</tr>
					{/each}
				</tbody>
			</table>

			<div class="px-6 py-4 border-t border-surface-200 bg-surface-50">
				<div class="flex justify-end">
					<div class="w-64 space-y-1.5">
						<div class="flex justify-between text-sm">
							<span class="text-surface-500">Subtotal</span>
							<span class="font-medium">{formatCurrency(invoice.subtotal)}</span>
						</div>
						{#if invoice.discount_amount > 0}
							<div class="flex justify-between text-sm">
								<span class="text-surface-500">Discount</span>
								<span class="text-danger-500">-{formatCurrency(invoice.discount_amount)}</span>
							</div>
						{/if}
						<div class="flex justify-between text-sm">
							<span class="text-surface-500">Tax ({invoice.tax_rate}%)</span>
							<span class="font-medium">{formatCurrency(invoice.tax_amount)}</span>
						</div>
						<div class="flex justify-between text-lg font-bold border-t border-surface-300 pt-2 mt-2">
							<span>Total</span>
							<span>{formatCurrency(invoice.total)}</span>
						</div>
						{#if invoice.amount_paid > 0}
							<div class="flex justify-between text-sm">
								<span class="text-surface-500">Paid</span>
								<span class="text-green-600 font-medium">-{formatCurrency(invoice.amount_paid)}</span>
							</div>
							<div class="flex justify-between text-lg font-bold border-t border-surface-300 pt-2">
								<span>Balance Due</span>
								<span class="text-forge-700">{formatCurrency(invoice.amount_due)}</span>
							</div>
						{/if}
					</div>
				</div>
			</div>
		</Card>

		<!-- Payment History -->
		{#if invoice.payments.length > 0}
			<Card padding={false}>
				<div class="px-6 py-4 border-b border-surface-200">
					<h3 class="text-sm font-semibold text-surface-500 uppercase tracking-wider">Payment History</h3>
				</div>
				<div class="divide-y divide-surface-100">
					{#each invoice.payments as payment}
						<div class="flex items-center justify-between px-6 py-3">
							<div>
								<p class="text-sm font-medium text-surface-900">{formatCurrency(payment.amount)}</p>
								<p class="text-xs text-surface-400">{payment.method} • {payment.date}</p>
							</div>
							<Badge variant="success" size="sm">Paid</Badge>
						</div>
					{/each}
				</div>
			</Card>
		{/if}

		<!-- Notes -->
		{#if invoice.notes}
			<Card>
				<h3 class="text-sm font-semibold text-surface-500 uppercase tracking-wider mb-2">Notes</h3>
				<p class="text-sm text-surface-700">{invoice.notes}</p>
			</Card>
		{/if}

		<!-- Contact -->
		<div class="text-center text-sm text-surface-400 py-4">
			<p>Questions? Contact us at <a href="tel:{invoice.company.phone}" class="text-forge-600 font-medium">{invoice.company.phone}</a> or <a href="mailto:{invoice.company.email}" class="text-forge-600 font-medium">{invoice.company.email}</a></p>
		</div>
	</main>
</div>
