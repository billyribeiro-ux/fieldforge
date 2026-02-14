<script lang="ts">
	import { page } from '$app/state';
	import TopBar from '$lib/components/layout/TopBar.svelte';
	import Card from '$lib/components/ui/Card.svelte';
	import Button from '$lib/components/ui/Button.svelte';
	import Badge from '$lib/components/ui/Badge.svelte';
	import {
		ArrowLeft,
		Phone,
		Mail,
		MapPin,
		Briefcase,
		FileText,
		Receipt,
		DollarSign,
		Clock,
		Star,
		Plus,
		MessageSquare,
		Home,
		Edit
	} from 'lucide-svelte';


	let { data } = $props();
	const customer = {
		id: page.params.id,
		first_name: 'Sarah',
		last_name: 'Johnson',
		email: 'sarah@email.com',
		phone: '(512) 555-0101',
		company: '',
		preferred_contact: 'SMS',
		referral_source: 'Google',
		tags: ['VIP', 'HVAC', 'Residential'],
		lifetime_value: 12450,
		outstanding_balance: 4200,
		total_jobs: 8,
		avg_rating: 4.9,
		last_service: '2024-12-10',
		created_at: '2023-06-15',
		notes_pinned: 'Prefers morning appointments. Has two dogs (friendly). Gate code: #1234.',
		properties: [
			{ id: '1', address: '123 Oak St, Austin TX 78701', type: 'Residential', sqft: 2400, is_primary: true },
			{ id: '2', address: '456 Business Park Dr, Austin TX 78702', type: 'Commercial', sqft: 5000, is_primary: false }
		],
		jobs: [
			{ id: '1', title: 'AC Unit Replacement', status: 'in_progress', date: '2024-12-15', amount: 4200 },
			{ id: '2', title: 'Annual HVAC Maintenance', status: 'completed', date: '2024-09-20', amount: 275 },
			{ id: '3', title: 'Thermostat Upgrade', status: 'paid', date: '2024-06-10', amount: 450 },
			{ id: '4', title: 'Duct Cleaning', status: 'paid', date: '2024-03-05', amount: 350 },
			{ id: '5', title: 'Emergency AC Repair', status: 'paid', date: '2023-08-12', amount: 890 }
		],
		invoices: [
			{ id: '1', number: 'FF-001', status: 'sent', total: 4200, due: '2024-12-25' },
			{ id: '2', number: 'FF-012', status: 'paid', total: 275, due: '2024-10-05' },
			{ id: '3', number: 'FF-008', status: 'paid', total: 450, due: '2024-07-01' }
		],
		communication: [
			{ type: 'sms', direction: 'outbound', content: 'Hi Sarah, your technician Mike is on the way. ETA 15 min.', time: 'Today, 9:00 AM' },
			{ type: 'sms', direction: 'inbound', content: 'Great, thanks!', time: 'Today, 9:02 AM' },
			{ type: 'email', direction: 'outbound', content: 'Estimate #EST-001 sent for AC Unit Replacement', time: 'Dec 10, 2024' }
		]
	};

	function formatCurrency(n: number): string {
		return new Intl.NumberFormat('en-US', { style: 'currency', currency: 'USD', minimumFractionDigits: 0 }).format(n);
	}

	function statusVariant(s: string): 'success' | 'warning' | 'danger' | 'info' | 'default' {
		switch (s) {
			case 'completed': case 'paid': return 'success';
			case 'in_progress': case 'en_route': return 'info';
			case 'sent': case 'viewed': return 'info';
			case 'overdue': return 'danger';
			default: return 'default';
		}
	}

	function statusLabel(s: string): string {
		return s.replace(/_/g, ' ').replace(/\b\w/g, (c) => c.toUpperCase());
	}

	let activeTab = $state<'overview' | 'jobs' | 'invoices' | 'communication' | 'properties'>('overview');
</script>

<svelte:head>
	<title>Customer Detail — FieldForge</title>
</svelte:head>

<TopBar>
	{#snippet actions()}
		<div class="flex items-center gap-2">
			<Button variant="outline" size="md">
				<Edit class="w-4 h-4" />
				Edit
			</Button>
			<Button variant="primary" size="md">
				<Plus class="w-4 h-4" />
				New Job
			</Button>
		</div>
	{/snippet}
</TopBar>

<div class="p-6 space-y-6">
	<!-- Header -->
	<div class="flex items-start gap-4">
		<a href="/dashboard/customers" class="mt-1 p-1.5 hover:bg-surface-100 rounded-lg transition-colors">
			<ArrowLeft class="w-5 h-5 text-surface-500" />
		</a>
		<div class="flex-1">
			<div class="flex items-center gap-4">
				<div class="w-14 h-14 bg-forge-100 text-forge-600 rounded-full flex items-center justify-center text-xl font-bold">
					{customer.first_name[0]}{customer.last_name[0]}
				</div>
				<div>
					<div class="flex items-center gap-3">
						<h1 class="text-2xl font-bold text-surface-900">{customer.first_name} {customer.last_name}</h1>
						{#each customer.tags as tag}
							<Badge variant="outline" size="sm">{tag}</Badge>
						{/each}
					</div>
					<div class="flex items-center gap-4 mt-1 text-sm text-surface-500">
						<div class="flex items-center gap-1">
							<Phone class="w-3.5 h-3.5" />
							<a href="tel:{customer.phone}" class="hover:text-forge-600">{customer.phone}</a>
						</div>
						<div class="flex items-center gap-1">
							<Mail class="w-3.5 h-3.5" />
							<a href="mailto:{customer.email}" class="hover:text-forge-600">{customer.email}</a>
						</div>
						<span>Customer since {customer.created_at}</span>
					</div>
				</div>
			</div>
		</div>
	</div>

	<!-- Stats Row -->
	<div class="grid grid-cols-2 md:grid-cols-4 gap-4">
		<Card>
			<div class="flex items-center gap-3">
				<div class="p-2 bg-green-50 rounded-lg"><DollarSign class="w-5 h-5 text-green-600" /></div>
				<div>
					<p class="text-xs text-surface-500">Lifetime Value</p>
					<p class="text-lg font-bold text-surface-900">{formatCurrency(customer.lifetime_value)}</p>
				</div>
			</div>
		</Card>
		<Card>
			<div class="flex items-center gap-3">
				<div class="p-2 bg-red-50 rounded-lg"><Receipt class="w-5 h-5 text-red-600" /></div>
				<div>
					<p class="text-xs text-surface-500">Outstanding</p>
					<p class="text-lg font-bold text-danger-500">{formatCurrency(customer.outstanding_balance)}</p>
				</div>
			</div>
		</Card>
		<Card>
			<div class="flex items-center gap-3">
				<div class="p-2 bg-forge-50 rounded-lg"><Briefcase class="w-5 h-5 text-forge-600" /></div>
				<div>
					<p class="text-xs text-surface-500">Total Jobs</p>
					<p class="text-lg font-bold text-surface-900">{customer.total_jobs}</p>
				</div>
			</div>
		</Card>
		<Card>
			<div class="flex items-center gap-3">
				<div class="p-2 bg-yellow-50 rounded-lg"><Star class="w-5 h-5 text-yellow-600" /></div>
				<div>
					<p class="text-xs text-surface-500">Avg Rating</p>
					<p class="text-lg font-bold text-surface-900">{customer.avg_rating} ★</p>
				</div>
			</div>
		</Card>
	</div>

	<!-- Tabs -->
	<div class="border-b border-surface-200">
		<div class="flex gap-6">
			{#each [
				{ id: 'overview', label: 'Overview' },
				{ id: 'jobs', label: 'Jobs' },
				{ id: 'invoices', label: 'Invoices' },
				{ id: 'communication', label: 'Communication' },
				{ id: 'properties', label: 'Properties' }
			] as tab}
				<button
					onclick={() => (activeTab = tab.id as typeof activeTab)}
					class="pb-3 text-sm font-medium border-b-2 transition-colors cursor-pointer
						{activeTab === tab.id
						? 'border-forge-600 text-forge-600'
						: 'border-transparent text-surface-500 hover:text-surface-700'}"
				>
					{tab.label}
				</button>
			{/each}
		</div>
	</div>

	<!-- Tab Content -->
	{#if activeTab === 'overview'}
		<div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
			<div class="lg:col-span-2 space-y-6">
				<!-- Pinned Notes -->
				{#if customer.notes_pinned}
					<Card class="border-yellow-200 bg-yellow-50/50">
						<div class="flex items-start gap-2">
							<MessageSquare class="w-4 h-4 text-yellow-600 mt-0.5 flex-shrink-0" />
							<div>
								<p class="text-sm font-medium text-yellow-800 mb-1">Pinned Note</p>
								<p class="text-sm text-yellow-700">{customer.notes_pinned}</p>
							</div>
						</div>
					</Card>
				{/if}

				<!-- Recent Jobs -->
				<Card padding={false}>
					<div class="px-6 py-4 border-b border-surface-200">
						<h3 class="text-base font-semibold text-surface-900">Recent Jobs</h3>
					</div>
					<div class="divide-y divide-surface-100">
						{#each customer.jobs.slice(0, 5) as job}
							<a href="/dashboard/jobs/{job.id}" class="flex items-center justify-between px-6 py-3 hover:bg-surface-50 transition-colors">
								<div>
									<p class="text-sm font-medium text-surface-900">{job.title}</p>
									<p class="text-xs text-surface-400">{job.date}</p>
								</div>
								<div class="flex items-center gap-3">
									<Badge variant={statusVariant(job.status)} size="sm">{statusLabel(job.status)}</Badge>
									<span class="text-sm font-medium text-surface-900">{formatCurrency(job.amount)}</span>
								</div>
							</a>
						{/each}
					</div>
				</Card>
			</div>

			<!-- Right Column -->
			<div class="space-y-6">
				<!-- Properties -->
				<Card padding={false}>
					<div class="px-6 py-4 border-b border-surface-200">
						<div class="flex items-center justify-between">
							<h3 class="text-base font-semibold text-surface-900">Properties</h3>
							<Button variant="ghost" size="sm"><Plus class="w-4 h-4" /></Button>
						</div>
					</div>
					{#each customer.properties as prop}
						<div class="px-6 py-3 border-b border-surface-100 last:border-0">
							<div class="flex items-start gap-2">
								<Home class="w-4 h-4 text-surface-400 mt-0.5 flex-shrink-0" />
								<div>
									<p class="text-sm text-surface-700">{prop.address}</p>
									<div class="flex items-center gap-2 mt-1">
										<Badge variant="outline" size="sm">{prop.type}</Badge>
										{#if prop.is_primary}
											<Badge variant="info" size="sm">Primary</Badge>
										{/if}
										<span class="text-xs text-surface-400">{prop.sqft.toLocaleString()} sqft</span>
									</div>
								</div>
							</div>
						</div>
					{/each}
				</Card>

				<!-- Contact Preferences -->
				<Card>
					<h3 class="text-sm font-semibold text-surface-500 uppercase tracking-wider mb-3">Contact Preferences</h3>
					<div class="space-y-2 text-sm">
						<div class="flex justify-between">
							<span class="text-surface-500">Preferred</span>
							<span class="font-medium text-surface-700">{customer.preferred_contact}</span>
						</div>
						<div class="flex justify-between">
							<span class="text-surface-500">Referral</span>
							<span class="font-medium text-surface-700">{customer.referral_source}</span>
						</div>
						<div class="flex justify-between">
							<span class="text-surface-500">Last Service</span>
							<span class="font-medium text-surface-700">{customer.last_service}</span>
						</div>
					</div>
				</Card>
			</div>
		</div>
	{:else if activeTab === 'jobs'}
		<Card padding={false}>
			<div class="divide-y divide-surface-100">
				{#each customer.jobs as job}
					<a href="/dashboard/jobs/{job.id}" class="flex items-center justify-between px-6 py-4 hover:bg-surface-50 transition-colors">
						<div>
							<p class="text-sm font-medium text-surface-900">{job.title}</p>
							<p class="text-xs text-surface-400">{job.date}</p>
						</div>
						<div class="flex items-center gap-3">
							<Badge variant={statusVariant(job.status)} size="sm">{statusLabel(job.status)}</Badge>
							<span class="text-sm font-medium text-surface-900">{formatCurrency(job.amount)}</span>
						</div>
					</a>
				{/each}
			</div>
		</Card>
	{:else if activeTab === 'invoices'}
		<Card padding={false}>
			<div class="divide-y divide-surface-100">
				{#each customer.invoices as inv}
					<a href="/dashboard/invoices/{inv.id}" class="flex items-center justify-between px-6 py-4 hover:bg-surface-50 transition-colors">
						<div>
							<p class="text-xs text-surface-400 font-mono">{inv.number}</p>
							<p class="text-sm font-medium text-surface-900">Due {inv.due}</p>
						</div>
						<div class="flex items-center gap-3">
							<Badge variant={statusVariant(inv.status)} size="sm">{statusLabel(inv.status)}</Badge>
							<span class="text-sm font-medium text-surface-900">{formatCurrency(inv.total)}</span>
						</div>
					</a>
				{/each}
			</div>
		</Card>
	{:else if activeTab === 'communication'}
		<Card padding={false}>
			<div class="divide-y divide-surface-100">
				{#each customer.communication as msg}
					<div class="px-6 py-3">
						<div class="flex items-center gap-2 mb-1">
							<Badge variant={msg.direction === 'inbound' ? 'info' : 'default'} size="sm">
								{msg.direction === 'inbound' ? '← In' : '→ Out'}
							</Badge>
							<Badge variant="outline" size="sm">{msg.type.toUpperCase()}</Badge>
							<span class="text-xs text-surface-400">{msg.time}</span>
						</div>
						<p class="text-sm text-surface-700">{msg.content}</p>
					</div>
				{/each}
			</div>
		</Card>
	{:else if activeTab === 'properties'}
		<div class="grid grid-cols-1 md:grid-cols-2 gap-4">
			{#each customer.properties as prop}
				<Card>
					<div class="flex items-start gap-3">
						<div class="p-2 bg-surface-100 rounded-lg">
							<Home class="w-5 h-5 text-surface-600" />
						</div>
						<div class="flex-1">
							<p class="text-sm font-medium text-surface-900">{prop.address}</p>
							<div class="flex items-center gap-2 mt-1">
								<Badge variant="outline" size="sm">{prop.type}</Badge>
								{#if prop.is_primary}
									<Badge variant="info" size="sm">Primary</Badge>
								{/if}
							</div>
							<p class="text-xs text-surface-400 mt-2">{prop.sqft.toLocaleString()} sqft</p>
						</div>
					</div>
				</Card>
			{/each}
			<button class="border-2 border-dashed border-surface-300 rounded-xl p-6 flex flex-col items-center justify-center text-surface-400 hover:border-forge-400 hover:text-forge-500 transition-colors cursor-pointer">
				<Plus class="w-6 h-6 mb-1" />
				<span class="text-sm">Add Property</span>
			</button>
		</div>
	{/if}
</div>
