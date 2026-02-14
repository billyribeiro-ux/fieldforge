<script lang="ts">
	import { page } from '$app/state';
	import { enhance } from '$app/forms';
	import TopBar from '$lib/components/layout/TopBar.svelte';
	import Card from '$lib/components/ui/Card.svelte';
	import Button from '$lib/components/ui/Button.svelte';
	import Badge from '$lib/components/ui/Badge.svelte';
	import {
		ArrowLeft,
		Send,
		Copy,
		Download,
		Printer,
		Edit,
		FileText,
		CheckCircle2,
		XCircle,
		ExternalLink,
		Clock
	} from 'lucide-svelte';


	let { data } = $props();

	// Demo fallback data
	const demoEstimate = {
		id: page.params.id,
		number: 'EST-001',
		title: 'AC Unit Replacement — 3 Ton Carrier',
		status: 'sent',
		customer: { id: '1', name: 'Sarah Johnson', email: 'sarah@email.com', phone: '(512) 555-0101' },
		job: { id: '1', title: 'AC Unit Replacement' },
		property: '123 Oak St, Austin TX 78701',
		scope_of_work: 'Remove existing 15-year-old Carrier AC unit and replace with new Carrier 24ACC636 3-ton unit. Includes all labor, materials, refrigerant, electrical connections, and disposal of old unit.',
		subtotal: 3750,
		discount_amount: 0,
		tax_rate: 8.25,
		tax_amount: 309.38,
		total: 4059.38,
		valid_until: '2024-12-30',
		sent_at: '2024-12-10',
		created_at: '2024-12-09',
		payment_terms: '50% deposit required. Balance due on completion.',
		warranty_terms: '1-year parts and labor warranty. Manufacturer warranty: 10 years compressor, 5 years parts.',
		portal_url: 'https://portal.fieldforge.com/est/abc123',
		line_items: [
			{ id: '1', description: 'Carrier 24ACC636 — 3 Ton AC Unit', category: 'materials', quantity: 1, unit: 'each', unit_price: 2400, total: 2400 },
			{ id: '2', description: 'Installation labor', category: 'labor', quantity: 6, unit: 'hours', unit_price: 125, total: 750 },
			{ id: '3', description: 'Refrigerant R-410A', category: 'materials', quantity: 1, unit: 'lot', unit_price: 200, total: 200 },
			{ id: '4', description: 'Electrical disconnect + whip', category: 'materials', quantity: 1, unit: 'each', unit_price: 150, total: 150 },
			{ id: '5', description: 'Concrete pad', category: 'materials', quantity: 1, unit: 'each', unit_price: 100, total: 100 },
			{ id: '6', description: 'Disposal of old unit', category: 'disposal', quantity: 1, unit: 'each', unit_price: 150, total: 150 }
		],
		activity: [
			{ action: 'Estimate created', user: 'Office', time: 'Dec 9, 2024 2:30 PM' },
			{ action: 'Estimate sent to customer', user: 'Office', time: 'Dec 10, 2024 9:00 AM' },
			{ action: 'Customer viewed estimate', user: 'System', time: 'Dec 10, 2024 9:15 AM' }
		]
	};

	// Use server data when available, fallback to demo
	const serverEstimate = data?.estimate as any;
	const estimate = serverEstimate ? {
		...demoEstimate,
		id: serverEstimate.id ?? demoEstimate.id,
		number: serverEstimate.estimate_number ?? demoEstimate.number,
		title: serverEstimate.title ?? demoEstimate.title,
		status: serverEstimate.status ?? demoEstimate.status,
		scope_of_work: serverEstimate.scope_of_work ?? demoEstimate.scope_of_work,
		subtotal: Number(serverEstimate.subtotal ?? demoEstimate.subtotal),
		discount_amount: Number(serverEstimate.discount_amount ?? demoEstimate.discount_amount),
		tax_amount: Number(serverEstimate.tax_amount ?? demoEstimate.tax_amount),
		total: Number(serverEstimate.total ?? demoEstimate.total),
		valid_until: serverEstimate.valid_until ?? demoEstimate.valid_until,
		sent_at: serverEstimate.sent_at?.split('T')[0] ?? demoEstimate.sent_at,
		created_at: serverEstimate.created_at?.split('T')[0] ?? demoEstimate.created_at,
		payment_terms: serverEstimate.payment_terms ?? demoEstimate.payment_terms,
		warranty_terms: serverEstimate.warranty_terms ?? demoEstimate.warranty_terms,
	} : demoEstimate;

	function formatCurrency(n: number): string {
		return new Intl.NumberFormat('en-US', { style: 'currency', currency: 'USD', minimumFractionDigits: 2 }).format(n);
	}

	function statusVariant(s: string): 'success' | 'warning' | 'danger' | 'info' | 'default' {
		switch (s) {
			case 'approved': case 'converted': return 'success';
			case 'sent': return 'info';
			case 'viewed': return 'warning';
			case 'declined': case 'expired': return 'danger';
			default: return 'default';
		}
	}

	function statusLabel(s: string): string {
		return s.charAt(0).toUpperCase() + s.slice(1);
	}

	function categoryLabel(c: string): string {
		return c.charAt(0).toUpperCase() + c.slice(1);
	}
</script>

<svelte:head>
	<title>Estimate Detail — FieldForge</title>
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
			<Button variant="outline" size="md">
				<Edit class="w-4 h-4" />
				Edit
			</Button>
			{#if estimate.status === 'draft'}
				<form method="POST" action="?/send" use:enhance>
					<Button variant="primary" size="md" type="submit">
						<Send class="w-4 h-4" />
						Send
					</Button>
				</form>
			{:else if estimate.status === 'approved'}
				<form method="POST" action="?/convertToInvoice" use:enhance>
					<Button variant="primary" size="md" type="submit">
						<FileText class="w-4 h-4" />
						Convert to Invoice
					</Button>
				</form>
			{/if}
		</div>
	{/snippet}
</TopBar>

<div class="p-6 space-y-6">
	<!-- Header -->
	<div class="flex items-start gap-4">
		<a href="/dashboard/estimates" class="mt-1 p-1.5 hover:bg-surface-100 rounded-lg transition-colors">
			<ArrowLeft class="w-5 h-5 text-surface-500" />
		</a>
		<div class="flex-1">
			<div class="flex items-center gap-3 mb-1">
				<h1 class="text-2xl font-bold text-surface-900">Estimate {estimate.number}</h1>
				<Badge variant={statusVariant(estimate.status)} size="md">{statusLabel(estimate.status)}</Badge>
			</div>
			<p class="text-sm text-surface-500">{estimate.title} • {estimate.customer.name}</p>
		</div>
	</div>

	<div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
		<!-- Estimate Document (2/3) -->
		<div class="lg:col-span-2">
			<Card padding={false}>
				<!-- Header -->
				<div class="px-8 py-6 border-b border-surface-200">
					<div class="flex items-start justify-between">
						<div>
							<h2 class="text-xl font-bold text-surface-900">ESTIMATE</h2>
							<p class="text-lg font-mono text-surface-600 mt-1">{estimate.number}</p>
						</div>
						<div class="text-right">
							<p class="text-lg font-bold text-surface-900">Smith HVAC & Plumbing</p>
							<p class="text-sm text-surface-500">100 Main St, Suite 200</p>
							<p class="text-sm text-surface-500">Austin, TX 78701</p>
						</div>
					</div>
				</div>

				<!-- Customer + Details -->
				<div class="px-8 py-6 border-b border-surface-200">
					<div class="grid grid-cols-2 gap-8">
						<div>
							<p class="text-xs font-medium text-surface-400 uppercase tracking-wider mb-2">Prepared For</p>
							<p class="text-sm font-semibold text-surface-900">{estimate.customer.name}</p>
							<p class="text-sm text-surface-600">{estimate.property}</p>
							<p class="text-sm text-surface-600">{estimate.customer.email}</p>
						</div>
						<div class="text-right space-y-1.5">
							<div class="flex justify-end gap-4 text-sm">
								<span class="text-surface-400">Date</span>
								<span class="font-medium text-surface-700">{estimate.created_at}</span>
							</div>
							<div class="flex justify-end gap-4 text-sm">
								<span class="text-surface-400">Valid Until</span>
								<span class="font-medium text-surface-700">{estimate.valid_until}</span>
							</div>
						</div>
					</div>
				</div>

				<!-- Scope -->
				{#if estimate.scope_of_work}
					<div class="px-8 py-4 border-b border-surface-200">
						<p class="text-xs font-medium text-surface-400 uppercase tracking-wider mb-2">Scope of Work</p>
						<p class="text-sm text-surface-700 leading-relaxed">{estimate.scope_of_work}</p>
					</div>
				{/if}

				<!-- Line Items -->
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
							{#each estimate.line_items as item}
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
								<span class="font-medium text-surface-900">{formatCurrency(estimate.subtotal)}</span>
							</div>
							{#if estimate.discount_amount > 0}
								<div class="flex justify-between text-sm">
									<span class="text-surface-500">Discount</span>
									<span class="text-danger-500">-{formatCurrency(estimate.discount_amount)}</span>
								</div>
							{/if}
							<div class="flex justify-between text-sm">
								<span class="text-surface-500">Tax ({estimate.tax_rate}%)</span>
								<span class="font-medium text-surface-900">{formatCurrency(estimate.tax_amount)}</span>
							</div>
							<div class="flex justify-between text-lg font-bold border-t border-surface-300 pt-2">
								<span>Total</span>
								<span>{formatCurrency(estimate.total)}</span>
							</div>
						</div>
					</div>
				</div>

				<!-- Terms -->
				<div class="px-8 py-4 border-t border-surface-200 grid grid-cols-2 gap-6">
					{#if estimate.payment_terms}
						<div>
							<p class="text-xs font-medium text-surface-400 uppercase tracking-wider mb-1">Payment Terms</p>
							<p class="text-sm text-surface-600">{estimate.payment_terms}</p>
						</div>
					{/if}
					{#if estimate.warranty_terms}
						<div>
							<p class="text-xs font-medium text-surface-400 uppercase tracking-wider mb-1">Warranty</p>
							<p class="text-sm text-surface-600">{estimate.warranty_terms}</p>
						</div>
					{/if}
				</div>
			</Card>
		</div>

		<!-- Right Column -->
		<div class="space-y-6">
			<!-- Status -->
			<Card>
				<h3 class="text-sm font-semibold text-surface-500 uppercase tracking-wider mb-3">Status</h3>
				<div class="flex items-center gap-2 mb-4">
					<Badge variant={statusVariant(estimate.status)} size="md">{statusLabel(estimate.status)}</Badge>
					{#if estimate.sent_at}
						<span class="text-xs text-surface-400">Sent {estimate.sent_at}</span>
					{/if}
				</div>

				{#if estimate.status === 'sent' || estimate.status === 'viewed'}
					<div class="space-y-2">
						<Button variant="primary" size="sm" class="w-full">
							<Send class="w-4 h-4" />
							Resend to Customer
						</Button>
					</div>
				{:else if estimate.status === 'approved'}
					<Button variant="primary" size="sm" class="w-full">
						<FileText class="w-4 h-4" />
						Convert to Invoice
					</Button>
				{:else if estimate.status === 'draft'}
					<Button variant="primary" size="sm" class="w-full">
						<Send class="w-4 h-4" />
						Send to Customer
					</Button>
				{/if}
			</Card>

			<!-- Portal Link -->
			<Card>
				<h3 class="text-sm font-semibold text-surface-500 uppercase tracking-wider mb-3">Customer Portal</h3>
				<p class="text-xs text-surface-400 mb-3">Customer can view and approve/decline online.</p>
				<div class="flex items-center gap-2">
					<input
						type="text"
						value={estimate.portal_url}
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
					<Button variant="outline" size="sm" class="w-full">
						<Copy class="w-4 h-4" />
						Duplicate
					</Button>
					<Button variant="outline" size="sm" class="w-full">
						<Download class="w-4 h-4" />
						Download PDF
					</Button>
					<Button variant="outline" size="sm" class="w-full">
						<Edit class="w-4 h-4" />
						Edit Estimate
					</Button>
				</div>
			</Card>

			<!-- Activity -->
			<Card padding={false}>
				<div class="px-6 py-4 border-b border-surface-200">
					<h3 class="text-sm font-semibold text-surface-500 uppercase tracking-wider">Activity</h3>
				</div>
				<div class="divide-y divide-surface-100">
					{#each estimate.activity as event}
						<div class="px-6 py-3">
							<div class="flex items-center gap-2">
								<Clock class="w-3.5 h-3.5 text-surface-400" />
								<span class="text-xs text-surface-400">{event.time}</span>
							</div>
							<p class="text-sm text-surface-700 mt-0.5">{event.action}</p>
							<p class="text-xs text-surface-400">{event.user}</p>
						</div>
					{/each}
				</div>
			</Card>
		</div>
	</div>
</div>
