<script lang="ts">
	import { page } from '$app/state';
	import Card from '$lib/components/ui/Card.svelte';
	import Button from '$lib/components/ui/Button.svelte';
	import Badge from '$lib/components/ui/Badge.svelte';
	import { Wrench, CheckCircle2, XCircle, Download, MessageSquare } from 'lucide-svelte';

	// Demo data — would come from public API using token
	const estimate = {
		number: 'EST-001',
		title: 'AC Unit Replacement — 3 Ton Carrier',
		company: { name: 'Smith HVAC & Plumbing', phone: '(512) 555-0100', email: 'office@smithhvac.com', address: '100 Main St, Suite 200, Austin TX 78701' },
		customer: 'Sarah Johnson',
		property: '123 Oak St, Austin TX 78701',
		scope: 'Remove existing 15-year-old Carrier AC unit and replace with new Carrier 24ACC636 3-ton unit. Includes all labor, materials, refrigerant, electrical connections, and disposal of old unit. System will be fully tested and operational upon completion.',
		valid_until: '2024-12-30',
		line_items: [
			{ description: 'Carrier 24ACC636 — 3 Ton AC Unit', category: 'Materials', qty: '1 each', total: 2400 },
			{ description: 'Installation labor', category: 'Labor', qty: '6 hours', total: 750 },
			{ description: 'Refrigerant R-410A', category: 'Materials', qty: '1 lot', total: 200 },
			{ description: 'Electrical disconnect + whip', category: 'Materials', qty: '1 each', total: 150 },
			{ description: 'Concrete pad', category: 'Materials', qty: '1 each', total: 100 },
			{ description: 'Disposal of old unit', category: 'Disposal', qty: '1 each', total: 150 }
		],
		subtotal: 3750,
		tax_amount: 317.63,
		total: 4067.63,
		payment_terms: '50% deposit required. Balance due on completion.',
		warranty_terms: '1-year parts and labor warranty. Manufacturer warranty: 10 years compressor, 5 years parts.',
		status: 'sent'
	};

	let approving = $state(false);
	let declining = $state(false);
	let showDeclineReason = $state(false);
	let declineReason = $state('');

	function formatCurrency(n: number): string {
		return new Intl.NumberFormat('en-US', { style: 'currency', currency: 'USD', minimumFractionDigits: 2 }).format(n);
	}

	async function approveEstimate() {
		approving = true;
		await new Promise((r) => setTimeout(r, 1500));
		approving = false;
	}

	async function declineEstimate() {
		declining = true;
		await new Promise((r) => setTimeout(r, 1000));
		declining = false;
		showDeclineReason = false;
	}
</script>

<svelte:head>
	<title>Estimate — FieldForge</title>
</svelte:head>

<div class="min-h-screen bg-surface-50">
	<!-- Header -->
	<header class="bg-white border-b border-surface-200">
		<div class="max-w-3xl mx-auto px-6 py-4 flex items-center justify-between">
			<div class="flex items-center gap-3">
				<div class="w-9 h-9 bg-forge-600 rounded-xl flex items-center justify-center">
					<Wrench class="w-5 h-5 text-white" />
				</div>
				<div>
					<p class="text-base font-bold text-surface-900">{estimate.company.name}</p>
					<p class="text-xs text-surface-400">{estimate.company.phone}</p>
				</div>
			</div>
			<Badge variant="info" size="md">Estimate</Badge>
		</div>
	</header>

	<main class="max-w-3xl mx-auto px-6 py-8 space-y-6">
		<!-- Title -->
		<div>
			<p class="text-sm text-surface-400 font-mono">{estimate.number}</p>
			<h1 class="text-2xl font-bold text-surface-900 mt-1">{estimate.title}</h1>
			<p class="text-sm text-surface-500 mt-2">Prepared for <strong>{estimate.customer}</strong> • {estimate.property}</p>
			<p class="text-sm text-surface-400 mt-1">Valid until {estimate.valid_until}</p>
		</div>

		<!-- Scope -->
		<Card>
			<h3 class="text-sm font-semibold text-surface-500 uppercase tracking-wider mb-2">Scope of Work</h3>
			<p class="text-sm text-surface-700 leading-relaxed">{estimate.scope}</p>
		</Card>

		<!-- Line Items -->
		<Card padding={false}>
			<table class="w-full">
				<thead>
					<tr class="border-b border-surface-200 bg-surface-50">
						<th class="text-left text-xs font-medium text-surface-500 uppercase tracking-wider px-6 py-3">Description</th>
						<th class="text-left text-xs font-medium text-surface-500 uppercase tracking-wider px-6 py-3">Category</th>
						<th class="text-right text-xs font-medium text-surface-500 uppercase tracking-wider px-6 py-3">Qty</th>
						<th class="text-right text-xs font-medium text-surface-500 uppercase tracking-wider px-6 py-3">Amount</th>
					</tr>
				</thead>
				<tbody class="divide-y divide-surface-100">
					{#each estimate.line_items as item}
						<tr>
							<td class="px-6 py-3 text-sm text-surface-900">{item.description}</td>
							<td class="px-6 py-3 text-sm text-surface-500">{item.category}</td>
							<td class="px-6 py-3 text-sm text-surface-700 text-right">{item.qty}</td>
							<td class="px-6 py-3 text-sm font-medium text-surface-900 text-right">{formatCurrency(item.total)}</td>
						</tr>
					{/each}
				</tbody>
			</table>

			<div class="px-6 py-4 border-t border-surface-200 bg-surface-50">
				<div class="flex justify-end">
					<div class="w-56 space-y-1.5">
						<div class="flex justify-between text-sm">
							<span class="text-surface-500">Subtotal</span>
							<span class="font-medium">{formatCurrency(estimate.subtotal)}</span>
						</div>
						<div class="flex justify-between text-sm">
							<span class="text-surface-500">Tax</span>
							<span class="font-medium">{formatCurrency(estimate.tax_amount)}</span>
						</div>
						<div class="flex justify-between text-lg font-bold border-t border-surface-300 pt-2 mt-2">
							<span>Total</span>
							<span>{formatCurrency(estimate.total)}</span>
						</div>
					</div>
				</div>
			</div>
		</Card>

		<!-- Terms -->
		<div class="grid grid-cols-1 md:grid-cols-2 gap-4">
			<Card>
				<h3 class="text-sm font-semibold text-surface-500 uppercase tracking-wider mb-2">Payment Terms</h3>
				<p class="text-sm text-surface-700">{estimate.payment_terms}</p>
			</Card>
			<Card>
				<h3 class="text-sm font-semibold text-surface-500 uppercase tracking-wider mb-2">Warranty</h3>
				<p class="text-sm text-surface-700">{estimate.warranty_terms}</p>
			</Card>
		</div>

		<!-- Actions -->
		{#if estimate.status === 'sent' || estimate.status === 'viewed'}
			<Card class="border-forge-200 bg-forge-50/30">
				<div class="text-center">
					<h3 class="text-lg font-semibold text-surface-900 mb-2">Ready to proceed?</h3>
					<p class="text-sm text-surface-500 mb-6">Approve this estimate to schedule the work, or decline if you'd like changes.</p>

					{#if !showDeclineReason}
						<div class="flex items-center justify-center gap-3">
							<Button variant="primary" size="lg" loading={approving} onclick={approveEstimate}>
								<CheckCircle2 class="w-5 h-5" />
								Approve Estimate
							</Button>
							<Button variant="outline" size="lg" onclick={() => (showDeclineReason = true)}>
								Decline
							</Button>
						</div>
					{:else}
						<div class="max-w-md mx-auto space-y-3">
							<textarea
								bind:value={declineReason}
								placeholder="Please let us know why you're declining (optional)..."
								rows={3}
								class="w-full px-3 py-2 text-sm border border-surface-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-forge-500 placeholder:text-surface-400"
							></textarea>
							<div class="flex items-center justify-center gap-3">
								<Button variant="ghost" onclick={() => (showDeclineReason = false)}>Back</Button>
								<Button variant="danger" loading={declining} onclick={declineEstimate}>
									<XCircle class="w-4 h-4" />
									Confirm Decline
								</Button>
							</div>
						</div>
					{/if}
				</div>
			</Card>
		{/if}

		<!-- Contact -->
		<div class="text-center text-sm text-surface-400 py-4">
			<p>Questions? Contact us at <a href="tel:{estimate.company.phone}" class="text-forge-600 font-medium">{estimate.company.phone}</a> or <a href="mailto:{estimate.company.email}" class="text-forge-600 font-medium">{estimate.company.email}</a></p>
		</div>
	</main>
</div>
