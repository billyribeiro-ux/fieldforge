<script lang="ts">
	import TopBar from '$lib/components/layout/TopBar.svelte';
	import Card from '$lib/components/ui/Card.svelte';
	import Button from '$lib/components/ui/Button.svelte';
	import Input from '$lib/components/ui/Input.svelte';
	import {
		Building2,
		User,
		CreditCard,
		Bell,
		Shield,
		Palette,
		Users,
		Plug,
		FileText
	} from 'lucide-svelte';

	let activeTab = $state('company');

	const tabs = [
		{ id: 'company', label: 'Company', icon: Building2 },
		{ id: 'profile', label: 'Profile', icon: User },
		{ id: 'team', label: 'Team', icon: Users },
		{ id: 'billing', label: 'Billing', icon: CreditCard },
		{ id: 'notifications', label: 'Notifications', icon: Bell },
		{ id: 'integrations', label: 'Integrations', icon: Plug },
		{ id: 'templates', label: 'Templates', icon: FileText },
		{ id: 'appearance', label: 'Appearance', icon: Palette },
		{ id: 'security', label: 'Security', icon: Shield }
	];

	// Demo company data
	let companyName = $state('Smith HVAC & Plumbing');
	let companyPhone = $state('(512) 555-0100');
	let companyEmail = $state('office@smithhvac.com');
	let companyWebsite = $state('https://smithhvac.com');
	let companyAddress = $state('100 Main St, Suite 200');
	let companyCity = $state('Austin');
	let companyState = $state('TX');
	let companyZip = $state('78701');
	let defaultHourlyRate = $state('125.00');
	let taxRate = $state('8.25');
</script>

<TopBar title="Settings" />

<div class="p-6">
	<div class="flex gap-6">
		<!-- Settings Sidebar -->
		<nav class="w-56 flex-shrink-0">
			<div class="space-y-1">
				{#each tabs as tab}
					<button
						onclick={() => (activeTab = tab.id)}
						class="flex items-center gap-3 w-full px-3 py-2.5 rounded-lg text-sm font-medium transition-all cursor-pointer
							{activeTab === tab.id
							? 'bg-forge-50 text-forge-700'
							: 'text-surface-600 hover:bg-surface-100 hover:text-surface-900'}"
					>
						<tab.icon class="w-4.5 h-4.5" />
						<span>{tab.label}</span>
					</button>
				{/each}
			</div>
		</nav>

		<!-- Settings Content -->
		<div class="flex-1 max-w-2xl">
			{#if activeTab === 'company'}
				<div class="space-y-6">
					<div>
						<h2 class="text-lg font-semibold text-surface-900">Company Information</h2>
						<p class="text-sm text-surface-500 mt-1">Manage your company details and branding.</p>
					</div>

					<Card>
						<div class="space-y-4">
							<Input label="Company Name" bind:value={companyName} required />

							<div class="grid grid-cols-2 gap-4">
								<Input label="Phone" type="tel" bind:value={companyPhone} />
								<Input label="Email" type="email" bind:value={companyEmail} />
							</div>

							<Input label="Website" type="url" bind:value={companyWebsite} />

							<Input label="Address" bind:value={companyAddress} />

							<div class="grid grid-cols-3 gap-4">
								<Input label="City" bind:value={companyCity} />
								<Input label="State" bind:value={companyState} />
								<Input label="ZIP" bind:value={companyZip} />
							</div>
						</div>
					</Card>

					<Card>
						<h3 class="text-base font-semibold text-surface-900 mb-4">Defaults</h3>
						<div class="space-y-4">
							<div class="grid grid-cols-2 gap-4">
								<Input label="Default Hourly Rate ($)" type="number" bind:value={defaultHourlyRate} />
								<Input label="Tax Rate (%)" type="number" bind:value={taxRate} />
							</div>
						</div>
					</Card>

					<div class="flex justify-end">
						<Button variant="primary">Save Changes</Button>
					</div>
				</div>
			{:else if activeTab === 'billing'}
				<div class="space-y-6">
					<div>
						<h2 class="text-lg font-semibold text-surface-900">Billing & Subscription</h2>
						<p class="text-sm text-surface-500 mt-1">Manage your FieldForge subscription and payment methods.</p>
					</div>

					<Card>
						<div class="flex items-center justify-between">
							<div>
								<p class="text-sm font-medium text-surface-500">Current Plan</p>
								<p class="text-2xl font-bold text-surface-900 mt-1">Crew Plan</p>
								<p class="text-sm text-surface-500 mt-1">$79/month • Up to 10 team members</p>
							</div>
							<Button variant="outline">Upgrade Plan</Button>
						</div>
					</Card>

					<Card>
						<h3 class="text-base font-semibold text-surface-900 mb-4">Payment Method</h3>
						<div class="flex items-center gap-4 p-4 bg-surface-50 rounded-lg">
							<div class="w-12 h-8 bg-surface-200 rounded flex items-center justify-center text-xs font-bold text-surface-600">VISA</div>
							<div>
								<p class="text-sm font-medium text-surface-900">•••• •••• •••• 4242</p>
								<p class="text-xs text-surface-500">Expires 12/2026</p>
							</div>
							<Button variant="ghost" size="sm" class="ml-auto">Update</Button>
						</div>
					</Card>
				</div>
			{:else}
				<div class="space-y-6">
					<div>
						<h2 class="text-lg font-semibold text-surface-900">{tabs.find((t) => t.id === activeTab)?.label}</h2>
						<p class="text-sm text-surface-500 mt-1">This section is under development.</p>
					</div>
					<Card>
						<div class="py-12 text-center">
							<p class="text-sm text-surface-400">Settings for this section will be available soon.</p>
						</div>
					</Card>
				</div>
			{/if}
		</div>
	</div>
</div>
