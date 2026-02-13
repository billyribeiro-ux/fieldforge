<script lang="ts">
	import { page } from '$app/state';
	import Card from '$lib/components/ui/Card.svelte';
	import Button from '$lib/components/ui/Button.svelte';
	import Badge from '$lib/components/ui/Badge.svelte';
	import { Wrench, Calendar, FileText, Receipt, Clock, Star, MessageSquare, User, ChevronRight, CheckCircle2 } from 'lucide-svelte';

	// Demo data — would come from public API using token
	const company = {
		name: 'Smith HVAC & Plumbing',
		phone: '(512) 555-0100',
		email: 'office@smithhvac.com',
		logo: null
	};

	const customer = {
		name: 'Sarah Johnson',
		email: 'sarah@email.com',
		phone: '(512) 555-1234'
	};

	const upcomingAppointments = [
		{ id: '1', title: 'AC Unit Replacement', date: 'Dec 15, 2024', time: '9:00 AM - 1:00 PM', technician: 'Mike Thompson', status: 'scheduled' },
		{ id: '2', title: 'Annual HVAC Maintenance', date: 'Mar 15, 2025', time: '10:00 AM - 11:00 AM', technician: 'TBD', status: 'scheduled' }
	];

	const recentJobs = [
		{ id: '1', title: 'Furnace Inspection', date: 'Nov 20, 2024', status: 'completed', amount: 275, photos: 3 },
		{ id: '2', title: 'Thermostat Replacement', date: 'Sep 5, 2024', status: 'completed', amount: 450, photos: 2 },
		{ id: '3', title: 'Duct Cleaning', date: 'Jun 12, 2024', status: 'completed', amount: 350, photos: 5 }
	];

	const openInvoices = [
		{ id: '1', number: 'FF-001', title: 'AC Unit Replacement — Deposit', amount: 2100, status: 'sent', due_date: 'Dec 20, 2024' }
	];

	const openEstimates = [
		{ id: '1', number: 'EST-003', title: 'Bathroom Plumbing Upgrade', amount: 3200, status: 'sent', valid_until: 'Dec 30, 2024' }
	];

	function formatCurrency(n: number): string {
		return new Intl.NumberFormat('en-US', { style: 'currency', currency: 'USD', minimumFractionDigits: 0 }).format(n);
	}
</script>

<div class="min-h-screen bg-surface-50">
	<!-- Header -->
	<header class="bg-white border-b border-surface-200 sticky top-0 z-10">
		<div class="max-w-4xl mx-auto px-6 py-4 flex items-center justify-between">
			<div class="flex items-center gap-3">
				<div class="w-9 h-9 bg-forge-600 rounded-xl flex items-center justify-center">
					<Wrench class="w-5 h-5 text-white" />
				</div>
				<div>
					<p class="text-base font-bold text-surface-900">{company.name}</p>
					<p class="text-xs text-surface-400">{company.phone}</p>
				</div>
			</div>
			<div class="flex items-center gap-2">
				<div class="w-8 h-8 bg-forge-100 rounded-full flex items-center justify-center text-xs font-bold text-forge-700">
					{customer.name.split(' ').map((n) => n[0]).join('')}
				</div>
			</div>
		</div>
	</header>

	<main class="max-w-4xl mx-auto px-6 py-8 space-y-8">
		<!-- Welcome -->
		<div>
			<h1 class="text-2xl font-bold text-surface-900">Welcome, {customer.name.split(' ')[0]}!</h1>
			<p class="text-sm text-surface-500 mt-1">Manage your appointments, invoices, and service history.</p>
		</div>

		<!-- Quick Actions -->
		<div class="grid grid-cols-2 sm:grid-cols-4 gap-3">
			<button class="flex flex-col items-center gap-2 p-4 bg-white rounded-xl border border-surface-200 hover:border-forge-300 hover:shadow-sm transition-all cursor-pointer">
				<div class="w-10 h-10 bg-forge-50 rounded-lg flex items-center justify-center">
					<Calendar class="w-5 h-5 text-forge-600" />
				</div>
				<span class="text-xs font-medium text-surface-700">Request Service</span>
			</button>
			<button class="flex flex-col items-center gap-2 p-4 bg-white rounded-xl border border-surface-200 hover:border-forge-300 hover:shadow-sm transition-all cursor-pointer">
				<div class="w-10 h-10 bg-success-50 rounded-lg flex items-center justify-center">
					<Receipt class="w-5 h-5 text-success-600" />
				</div>
				<span class="text-xs font-medium text-surface-700">Pay Invoice</span>
			</button>
			<button class="flex flex-col items-center gap-2 p-4 bg-white rounded-xl border border-surface-200 hover:border-forge-300 hover:shadow-sm transition-all cursor-pointer">
				<div class="w-10 h-10 bg-accent-50 rounded-lg flex items-center justify-center">
					<MessageSquare class="w-5 h-5 text-accent-600" />
				</div>
				<span class="text-xs font-medium text-surface-700">Send Message</span>
			</button>
			<button class="flex flex-col items-center gap-2 p-4 bg-white rounded-xl border border-surface-200 hover:border-forge-300 hover:shadow-sm transition-all cursor-pointer">
				<div class="w-10 h-10 bg-warning-50 rounded-lg flex items-center justify-center">
					<Star class="w-5 h-5 text-warning-600" />
				</div>
				<span class="text-xs font-medium text-surface-700">Leave Review</span>
			</button>
		</div>

		<!-- Open Estimates -->
		{#if openEstimates.length > 0}
			<section>
				<h2 class="text-sm font-semibold text-surface-500 uppercase tracking-wider mb-3">Pending Estimates</h2>
				{#each openEstimates as est}
					<Card class="border-forge-200 bg-forge-50/30">
						<div class="flex items-center justify-between">
							<div>
								<p class="text-xs text-surface-400 font-mono">{est.number}</p>
								<p class="text-sm font-semibold text-surface-900">{est.title}</p>
								<p class="text-xs text-surface-500 mt-1">Valid until {est.valid_until}</p>
							</div>
							<div class="text-right">
								<p class="text-lg font-bold text-surface-900">{formatCurrency(est.amount)}</p>
								<a href="/portal/estimate/demo-token" class="inline-flex items-center gap-1 text-xs font-medium text-forge-600 hover:underline mt-1">
									View & Approve <ChevronRight class="w-3 h-3" />
								</a>
							</div>
						</div>
					</Card>
				{/each}
			</section>
		{/if}

		<!-- Open Invoices -->
		{#if openInvoices.length > 0}
			<section>
				<h2 class="text-sm font-semibold text-surface-500 uppercase tracking-wider mb-3">Outstanding Invoices</h2>
				{#each openInvoices as inv}
					<Card class="border-warning-200 bg-warning-50/30">
						<div class="flex items-center justify-between">
							<div>
								<p class="text-xs text-surface-400 font-mono">{inv.number}</p>
								<p class="text-sm font-semibold text-surface-900">{inv.title}</p>
								<p class="text-xs text-surface-500 mt-1">Due {inv.due_date}</p>
							</div>
							<div class="text-right">
								<p class="text-lg font-bold text-surface-900">{formatCurrency(inv.amount)}</p>
								<a href="/portal/invoice/demo-token" class="inline-flex items-center gap-1 text-xs font-medium text-forge-600 hover:underline mt-1">
									Pay Now <ChevronRight class="w-3 h-3" />
								</a>
							</div>
						</div>
					</Card>
				{/each}
			</section>
		{/if}

		<!-- Upcoming Appointments -->
		<section>
			<h2 class="text-sm font-semibold text-surface-500 uppercase tracking-wider mb-3">Upcoming Appointments</h2>
			<div class="space-y-3">
				{#each upcomingAppointments as appt}
					<Card>
						<div class="flex items-center justify-between">
							<div class="flex items-center gap-4">
								<div class="w-12 h-12 bg-forge-50 rounded-lg flex flex-col items-center justify-center">
									<Calendar class="w-5 h-5 text-forge-600" />
								</div>
								<div>
									<p class="text-sm font-semibold text-surface-900">{appt.title}</p>
									<p class="text-xs text-surface-500">{appt.date} • {appt.time}</p>
									<p class="text-xs text-surface-400 mt-0.5">Technician: {appt.technician}</p>
								</div>
							</div>
							<Badge variant="info" size="sm">Scheduled</Badge>
						</div>
					</Card>
				{/each}
			</div>
		</section>

		<!-- Job History -->
		<section>
			<h2 class="text-sm font-semibold text-surface-500 uppercase tracking-wider mb-3">Service History</h2>
			<Card padding={false}>
				<div class="divide-y divide-surface-100">
					{#each recentJobs as job}
						<div class="flex items-center justify-between px-5 py-4 hover:bg-surface-50 transition-colors">
							<div class="flex items-center gap-3">
								<div class="w-8 h-8 bg-success-50 rounded-full flex items-center justify-center">
									<CheckCircle2 class="w-4 h-4 text-success-500" />
								</div>
								<div>
									<p class="text-sm font-medium text-surface-900">{job.title}</p>
									<p class="text-xs text-surface-400">{job.date} • {job.photos} photos</p>
								</div>
							</div>
							<div class="text-right">
								<p class="text-sm font-semibold text-surface-900">{formatCurrency(job.amount)}</p>
								<button class="text-xs text-forge-600 hover:underline cursor-pointer">View Details</button>
							</div>
						</div>
					{/each}
				</div>
			</Card>
		</section>

		<!-- Contact -->
		<div class="text-center text-sm text-surface-400 py-6 border-t border-surface-200">
			<p>Need help? Contact us at <a href="tel:{company.phone}" class="text-forge-600 font-medium">{company.phone}</a> or <a href="mailto:{company.email}" class="text-forge-600 font-medium">{company.email}</a></p>
			<p class="mt-2 text-xs text-surface-300">Powered by <span class="font-semibold text-surface-400">FieldForge</span></p>
		</div>
	</main>
</div>
