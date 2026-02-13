<script lang="ts">
	import TopBar from '$lib/components/layout/TopBar.svelte';
	import Card from '$lib/components/ui/Card.svelte';
	import Button from '$lib/components/ui/Button.svelte';
	import Badge from '$lib/components/ui/Badge.svelte';
	import Tabs from '$lib/components/ui/Tabs.svelte';
	import { ShieldCheck, FileText, AlertTriangle, Plus, Calendar, Building2 } from 'lucide-svelte';

	let activeTab = $state('licenses');

	const licenses = [
		{ id: '1', license_type: 'HVAC Contractor', license_number: 'HV-2024-8834', issuing_state: 'California', expiry_date: '2025-06-15', status: 'active', user_name: 'Mike Johnson' },
		{ id: '2', license_type: 'EPA 608 Universal', license_number: 'EPA-U-44921', issuing_state: 'Federal', expiry_date: '2026-01-01', status: 'active', user_name: 'Mike Johnson' },
		{ id: '3', license_type: 'Journeyman Plumber', license_number: 'JP-2023-1192', issuing_state: 'California', expiry_date: '2025-01-30', status: 'expiring_soon', user_name: 'Jake Rodriguez' },
		{ id: '4', license_type: 'Electrical C-10', license_number: 'EC10-2022-5567', issuing_state: 'California', expiry_date: '2024-11-15', status: 'expired', user_name: 'Tom Williams' },
	];

	const policies = [
		{ id: '1', policy_type: 'General Liability', provider: 'State Farm', policy_number: 'GL-2024-001', coverage_amount: 2000000, expiry_date: '2025-03-01', auto_renew: true },
		{ id: '2', policy_type: 'Workers Compensation', provider: 'Hartford', policy_number: 'WC-2024-042', coverage_amount: 1000000, expiry_date: '2025-06-15', auto_renew: true },
		{ id: '3', policy_type: 'Commercial Auto', provider: 'Progressive', policy_number: 'CA-2024-118', coverage_amount: 500000, expiry_date: '2025-01-20', auto_renew: false },
		{ id: '4', policy_type: 'Professional Liability', provider: 'Nationwide', policy_number: 'PL-2024-077', coverage_amount: 1000000, expiry_date: '2025-09-30', auto_renew: true },
	];

	let activeLicenses = $derived(licenses.filter(l => l.status === 'active').length);
	let expiringLicenses = $derived(licenses.filter(l => l.status === 'expiring_soon').length);
	let expiredLicenses = $derived(licenses.filter(l => l.status === 'expired').length);

	function statusVariant(status: string): 'success' | 'warning' | 'danger' {
		if (status === 'active') return 'success';
		if (status === 'expiring_soon') return 'warning';
		return 'danger';
	}

	function formatCurrency(amount: number): string {
		return new Intl.NumberFormat('en-US', { style: 'currency', currency: 'USD', maximumFractionDigits: 0 }).format(amount);
	}

	function daysUntil(dateStr: string): number {
		const now = new Date();
		const date = new Date(dateStr);
		return Math.ceil((date.getTime() - now.getTime()) / (1000 * 60 * 60 * 24));
	}
</script>

<svelte:head>
	<title>Compliance — FieldForge</title>
</svelte:head>

<TopBar title="Compliance">
	{#snippet actions()}
		<Button size="sm">
			<Plus class="w-4 h-4" />
			Add License
		</Button>
	{/snippet}
</TopBar>

<div class="p-6 space-y-6">
	<!-- Summary Cards -->
	<div class="grid grid-cols-1 md:grid-cols-4 gap-4">
		<Card>
			<div class="flex items-center gap-3">
				<div class="w-10 h-10 bg-success-50 text-success-600 rounded-lg flex items-center justify-center">
					<ShieldCheck class="w-5 h-5" />
				</div>
				<div>
					<p class="text-2xl font-bold text-surface-900">{activeLicenses}</p>
					<p class="text-xs text-surface-500">Active Licenses</p>
				</div>
			</div>
		</Card>
		<Card>
			<div class="flex items-center gap-3">
				<div class="w-10 h-10 bg-warning-50 text-warning-600 rounded-lg flex items-center justify-center">
					<AlertTriangle class="w-5 h-5" />
				</div>
				<div>
					<p class="text-2xl font-bold text-surface-900">{expiringLicenses}</p>
					<p class="text-xs text-surface-500">Expiring Soon</p>
				</div>
			</div>
		</Card>
		<Card>
			<div class="flex items-center gap-3">
				<div class="w-10 h-10 bg-danger-50 text-danger-600 rounded-lg flex items-center justify-center">
					<FileText class="w-5 h-5" />
				</div>
				<div>
					<p class="text-2xl font-bold text-surface-900">{expiredLicenses}</p>
					<p class="text-xs text-surface-500">Expired</p>
				</div>
			</div>
		</Card>
		<Card>
			<div class="flex items-center gap-3">
				<div class="w-10 h-10 bg-forge-50 text-forge-600 rounded-lg flex items-center justify-center">
					<Building2 class="w-5 h-5" />
				</div>
				<div>
					<p class="text-2xl font-bold text-surface-900">{policies.length}</p>
					<p class="text-xs text-surface-500">Insurance Policies</p>
				</div>
			</div>
		</Card>
	</div>

	<!-- Tabs -->
	<Tabs
		tabs={[
			{ id: 'licenses', label: 'Licenses & Certifications' },
			{ id: 'insurance', label: 'Insurance Policies' },
		]}
		active={activeTab}
		onchange={(id) => (activeTab = id)}
	/>

	{#if activeTab === 'licenses'}
		<div class="space-y-3">
			{#each licenses as license (license.id)}
				<Card>
					<div class="flex items-center justify-between">
						<div class="flex items-center gap-4">
							<div class="w-10 h-10 bg-surface-100 rounded-lg flex items-center justify-center">
								<FileText class="w-5 h-5 text-surface-500" />
							</div>
							<div>
								<p class="text-sm font-semibold text-surface-900">{license.license_type}</p>
								<p class="text-xs text-surface-500">{license.license_number} · {license.issuing_state}</p>
								<p class="text-xs text-surface-400 mt-0.5">Assigned to {license.user_name}</p>
							</div>
						</div>
						<div class="flex items-center gap-3">
							<div class="text-right">
								<div class="flex items-center gap-1.5 text-xs text-surface-500">
									<Calendar class="w-3.5 h-3.5" />
									Expires {license.expiry_date}
								</div>
								{#if license.status === 'expiring_soon'}
									<p class="text-xs text-warning-600 font-medium mt-0.5">{daysUntil(license.expiry_date)} days left</p>
								{/if}
							</div>
							<Badge variant={statusVariant(license.status)} size="sm">
								{license.status === 'expiring_soon' ? 'Expiring' : license.status === 'active' ? 'Active' : 'Expired'}
							</Badge>
						</div>
					</div>
				</Card>
			{/each}
		</div>
	{:else}
		<div class="space-y-3">
			{#each policies as policy (policy.id)}
				<Card>
					<div class="flex items-center justify-between">
						<div class="flex items-center gap-4">
							<div class="w-10 h-10 bg-forge-50 rounded-lg flex items-center justify-center">
								<ShieldCheck class="w-5 h-5 text-forge-600" />
							</div>
							<div>
								<p class="text-sm font-semibold text-surface-900">{policy.policy_type}</p>
								<p class="text-xs text-surface-500">{policy.provider} · {policy.policy_number}</p>
								<p class="text-xs text-surface-400 mt-0.5">Coverage: {formatCurrency(policy.coverage_amount)}</p>
							</div>
						</div>
						<div class="flex items-center gap-3">
							<div class="text-right">
								<div class="flex items-center gap-1.5 text-xs text-surface-500">
									<Calendar class="w-3.5 h-3.5" />
									Expires {policy.expiry_date}
								</div>
								{#if policy.auto_renew}
									<p class="text-xs text-success-600 mt-0.5">Auto-renew</p>
								{/if}
							</div>
							<Badge variant={daysUntil(policy.expiry_date) < 30 ? 'warning' : 'success'} size="sm">
								{daysUntil(policy.expiry_date) < 30 ? 'Expiring' : 'Active'}
							</Badge>
						</div>
					</div>
				</Card>
			{/each}
		</div>
	{/if}
</div>
