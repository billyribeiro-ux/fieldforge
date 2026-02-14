<script lang="ts">
	import { page } from '$app/state';
	import TopBar from '$lib/components/layout/TopBar.svelte';
	import Card from '$lib/components/ui/Card.svelte';
	import Button from '$lib/components/ui/Button.svelte';
	import Badge from '$lib/components/ui/Badge.svelte';
	import {
		ArrowLeft,
		Clock,
		MapPin,
		User,
		Phone,
		Mail,
		Camera,
		MessageSquare,
		FileText,
		CheckSquare,
		Play,
		Pause,
		Square,
		ChevronRight,
		DollarSign,
		Calendar,
		Wrench,
		AlertTriangle
	} from 'lucide-svelte';


	let { data } = $props();
	// Demo data — would come from API based on page.params.id
	const job = {
		id: page.params.id,
		title: 'AC Unit Replacement',
		description: 'Replace 3-ton Carrier AC unit. Customer reports intermittent cooling and high energy bills. Unit is 15 years old.',
		status: 'in_progress',
		priority: 'high',
		trade: 'HVAC',
		job_type: 'Replacement',
		source: 'Referral',
		customer: {
			id: '1',
			name: 'Sarah Johnson',
			phone: '(512) 555-0101',
			email: 'sarah@email.com'
		},
		property: {
			address: '123 Oak St, Austin TX 78701',
			type: 'Residential',
			sqft: 2400,
			year_built: 2005
		},
		assigned: { id: '1', name: 'Mike Thompson', avatar: null },
		scheduled_date: '2024-12-15',
		scheduled_time: '9:00 AM — 1:00 PM',
		estimated_duration: '4 hours',
		budget: 4200,
		total: 4200,
		tags: ['HVAC', 'Residential', 'Replacement'],
		po_number: 'PO-2024-0042',
		created_at: '2024-12-10',
		started_at: '2024-12-15 9:15 AM',
		notes: [
			{ id: '1', author: 'Mike T.', content: 'Arrived on site. Old unit is a Carrier 24ACC636. Confirmed 3-ton replacement needed.', time: '9:15 AM', type: 'text' },
			{ id: '2', author: 'System', content: 'Job status changed to In Progress', time: '9:14 AM', type: 'system' },
			{ id: '3', author: 'Office', content: 'Customer confirmed appointment via SMS', time: 'Yesterday', type: 'text' }
		],
		photos: [
			{ id: '1', url: '', caption: 'Old unit — front view', category: 'before' },
			{ id: '2', url: '', caption: 'Model/serial plate', category: 'before' },
			{ id: '3', url: '', caption: 'Electrical disconnect', category: 'during' }
		],
		time_entries: [
			{ id: '1', user: 'Mike T.', type: 'work', started: '9:15 AM', ended: null, duration: null }
		],
		checklist: [
			{ id: '1', text: 'Disconnect power', done: true },
			{ id: '2', text: 'Remove refrigerant', done: true },
			{ id: '3', text: 'Remove old unit', done: false },
			{ id: '4', text: 'Install new pad/base', done: false },
			{ id: '5', text: 'Set new unit', done: false },
			{ id: '6', text: 'Connect refrigerant lines', done: false },
			{ id: '7', text: 'Connect electrical', done: false },
			{ id: '8', text: 'Vacuum and charge system', done: false },
			{ id: '9', text: 'Test operation', done: false },
			{ id: '10', text: 'Clean up site', done: false }
		]
	};

	let timerRunning = $state(true);
	let timerElapsed = $state('1h 23m');

	const statusFlow = ['lead', 'estimated', 'approved', 'scheduled', 'en_route', 'in_progress', 'completed', 'invoiced', 'paid'];

	function statusVariant(status: string): 'success' | 'warning' | 'danger' | 'info' | 'default' {
		switch (status) {
			case 'completed': case 'paid': return 'success';
			case 'in_progress': case 'en_route': return 'info';
			case 'scheduled': case 'approved': return 'default';
			case 'lead': case 'estimated': return 'warning';
			case 'cancelled': return 'danger';
			default: return 'default';
		}
	}

	function statusLabel(s: string): string {
		return s.replace(/_/g, ' ').replace(/\b\w/g, (c) => c.toUpperCase());
	}

	function formatCurrency(n: number): string {
		return new Intl.NumberFormat('en-US', { style: 'currency', currency: 'USD', minimumFractionDigits: 0 }).format(n);
	}

	let checklistItems = $state(job.checklist.map(c => ({ ...c })));
	let completedCount = $derived(checklistItems.filter(c => c.done).length);
</script>

<svelte:head>
	<title>Job Detail — FieldForge</title>
</svelte:head>

<TopBar>
	{#snippet actions()}
		<div class="flex items-center gap-2">
			{#if job.status === 'in_progress'}
				<Button variant="primary" size="md">
					<Square class="w-4 h-4" />
					Complete Job
				</Button>
			{:else if job.status === 'completed'}
				<Button variant="primary" size="md">
					<FileText class="w-4 h-4" />
					Create Invoice
				</Button>
			{:else if job.status === 'scheduled'}
				<Button variant="primary" size="md">
					<Play class="w-4 h-4" />
					Start Job
				</Button>
			{/if}
		</div>
	{/snippet}
</TopBar>

<div class="p-6 space-y-6">
	<!-- Back + Title -->
	<div class="flex items-start gap-4">
		<a href="/dashboard/jobs" class="mt-1 p-1.5 hover:bg-surface-100 rounded-lg transition-colors">
			<ArrowLeft class="w-5 h-5 text-surface-500" />
		</a>
		<div class="flex-1">
			<div class="flex items-center gap-3 mb-1">
				<h1 class="text-2xl font-bold text-surface-900">{job.title}</h1>
				<Badge variant={statusVariant(job.status)} size="md">{statusLabel(job.status)}</Badge>
				{#if job.priority !== 'normal'}
					<Badge variant="warning" size="md">{job.priority}</Badge>
				{/if}
			</div>
			<div class="flex items-center gap-4 text-sm text-surface-500">
				<span>{job.trade} • {job.job_type}</span>
				<span>PO: {job.po_number}</span>
				<div class="flex items-center gap-1">
					{#each job.tags as tag}
						<Badge variant="outline" size="sm">{tag}</Badge>
					{/each}
				</div>
			</div>
		</div>
	</div>

	<!-- Status Pipeline -->
	<Card>
		<div class="flex items-center gap-1 overflow-x-auto pb-1">
			{#each statusFlow as step, i}
				{@const currentIdx = statusFlow.indexOf(job.status)}
				{@const isActive = step === job.status}
				{@const isPast = i < currentIdx}
				<div class="flex items-center gap-1 flex-shrink-0">
					<div class="flex items-center gap-2 px-3 py-1.5 rounded-full text-xs font-medium transition-colors
						{isActive ? 'bg-forge-100 text-forge-700 ring-2 ring-forge-500' : isPast ? 'bg-green-50 text-green-700' : 'bg-surface-100 text-surface-400'}">
						{#if isPast}
							<CheckSquare class="w-3.5 h-3.5" />
						{/if}
						{statusLabel(step)}
					</div>
					{#if i < statusFlow.length - 1}
						<ChevronRight class="w-4 h-4 text-surface-300 flex-shrink-0" />
					{/if}
				</div>
			{/each}
		</div>
	</Card>

	<div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
		<!-- Left Column (2/3) -->
		<div class="lg:col-span-2 space-y-6">
			<!-- Description -->
			<Card>
				<h3 class="text-sm font-semibold text-surface-900 mb-2">Description</h3>
				<p class="text-sm text-surface-600 leading-relaxed">{job.description}</p>
			</Card>

			<!-- Active Timer -->
			{#if timerRunning}
				<Card class="border-forge-200 bg-forge-50/50">
					<div class="flex items-center justify-between">
						<div class="flex items-center gap-3">
							<div class="w-10 h-10 bg-forge-100 rounded-full flex items-center justify-center">
								<Clock class="w-5 h-5 text-forge-600 animate-pulse" />
							</div>
							<div>
								<p class="text-sm font-medium text-surface-900">Timer Running — {job.time_entries[0].user}</p>
								<p class="text-2xl font-bold text-forge-700 font-mono">{timerElapsed}</p>
							</div>
						</div>
						<div class="flex items-center gap-2">
							<Button variant="outline" size="sm">
								<Pause class="w-4 h-4" />
								Pause
							</Button>
							<Button variant="danger" size="sm">
								<Square class="w-4 h-4" />
								Stop
							</Button>
						</div>
					</div>
				</Card>
			{/if}

			<!-- Checklist -->
			<Card padding={false}>
				<div class="px-6 py-4 border-b border-surface-200">
					<div class="flex items-center justify-between">
						<h3 class="text-base font-semibold text-surface-900">Checklist</h3>
						<span class="text-sm text-surface-500">{completedCount}/{checklistItems.length}</span>
					</div>
					<div class="mt-2 w-full bg-surface-200 rounded-full h-1.5">
						<div
							class="bg-forge-500 h-1.5 rounded-full transition-all duration-300"
							style="width: {(completedCount / checklistItems.length) * 100}%"
						></div>
					</div>
				</div>
				<div class="divide-y divide-surface-100">
					{#each checklistItems as item, i}
						<label class="flex items-center gap-3 px-6 py-3 hover:bg-surface-50 cursor-pointer transition-colors">
							<input
								type="checkbox"
								bind:checked={checklistItems[i].done}
								class="w-4 h-4 rounded border-surface-300 text-forge-600 focus:ring-forge-500"
							/>
							<span class="text-sm {item.done ? 'text-surface-400 line-through' : 'text-surface-700'}">{item.text}</span>
						</label>
					{/each}
				</div>
			</Card>

			<!-- Notes / Activity -->
			<Card padding={false}>
				<div class="px-6 py-4 border-b border-surface-200">
					<h3 class="text-base font-semibold text-surface-900">Activity & Notes</h3>
				</div>
				<div class="divide-y divide-surface-100">
					{#each job.notes as note}
						<div class="px-6 py-3">
							<div class="flex items-center gap-2 mb-1">
								<span class="text-sm font-medium text-surface-900">{note.author}</span>
								<span class="text-xs text-surface-400">{note.time}</span>
								{#if note.type === 'system'}
									<Badge variant="default" size="sm">System</Badge>
								{/if}
							</div>
							<p class="text-sm text-surface-600">{note.content}</p>
						</div>
					{/each}
				</div>
				<div class="px-6 py-4 border-t border-surface-200">
					<div class="flex items-center gap-2">
						<input
							type="text"
							placeholder="Add a note..."
							class="flex-1 px-3 py-2 text-sm bg-surface-50 border border-surface-200 rounded-lg focus:outline-none focus:ring-2 focus:ring-forge-500 placeholder:text-surface-400"
						/>
						<Button variant="primary" size="sm">Add Note</Button>
					</div>
				</div>
			</Card>

			<!-- Photos -->
			<Card padding={false}>
				<div class="px-6 py-4 border-b border-surface-200">
					<div class="flex items-center justify-between">
						<h3 class="text-base font-semibold text-surface-900">Photos ({job.photos.length})</h3>
						<Button variant="outline" size="sm">
							<Camera class="w-4 h-4" />
							Upload
						</Button>
					</div>
				</div>
				<div class="p-6">
					<div class="grid grid-cols-3 gap-3">
						{#each job.photos as photo}
							<div class="aspect-square bg-surface-100 rounded-lg flex flex-col items-center justify-center text-surface-400 border border-surface-200">
								<Camera class="w-8 h-8 mb-1" />
								<span class="text-xs">{photo.caption}</span>
								<Badge variant="outline" size="sm" class="mt-1">{photo.category}</Badge>
							</div>
						{/each}
						<button class="aspect-square bg-surface-50 rounded-lg flex flex-col items-center justify-center text-surface-400 border-2 border-dashed border-surface-300 hover:border-forge-400 hover:text-forge-500 transition-colors cursor-pointer">
							<Camera class="w-6 h-6 mb-1" />
							<span class="text-xs">Add Photo</span>
						</button>
					</div>
				</div>
			</Card>
		</div>

		<!-- Right Column (1/3) -->
		<div class="space-y-6">
			<!-- Customer Info -->
			<Card>
				<h3 class="text-sm font-semibold text-surface-500 uppercase tracking-wider mb-3">Customer</h3>
				<a href="/dashboard/customers/{job.customer.id}" class="block">
					<p class="text-base font-semibold text-surface-900 hover:text-forge-600">{job.customer.name}</p>
				</a>
				<div class="mt-3 space-y-2">
					<div class="flex items-center gap-2 text-sm text-surface-600">
						<Phone class="w-4 h-4 text-surface-400" />
						<a href="tel:{job.customer.phone}" class="hover:text-forge-600">{job.customer.phone}</a>
					</div>
					<div class="flex items-center gap-2 text-sm text-surface-600">
						<Mail class="w-4 h-4 text-surface-400" />
						<a href="mailto:{job.customer.email}" class="hover:text-forge-600">{job.customer.email}</a>
					</div>
				</div>
			</Card>

			<!-- Property -->
			<Card>
				<h3 class="text-sm font-semibold text-surface-500 uppercase tracking-wider mb-3">Property</h3>
				<div class="flex items-start gap-2">
					<MapPin class="w-4 h-4 text-surface-400 mt-0.5 flex-shrink-0" />
					<p class="text-sm text-surface-700">{job.property.address}</p>
				</div>
				<div class="mt-3 grid grid-cols-2 gap-2 text-sm">
					<div>
						<span class="text-surface-400">Type</span>
						<p class="font-medium text-surface-700">{job.property.type}</p>
					</div>
					<div>
						<span class="text-surface-400">Sq Ft</span>
						<p class="font-medium text-surface-700">{job.property.sqft.toLocaleString()}</p>
					</div>
					<div>
						<span class="text-surface-400">Year Built</span>
						<p class="font-medium text-surface-700">{job.property.year_built}</p>
					</div>
				</div>
			</Card>

			<!-- Schedule -->
			<Card>
				<h3 class="text-sm font-semibold text-surface-500 uppercase tracking-wider mb-3">Schedule</h3>
				<div class="space-y-2">
					<div class="flex items-center gap-2 text-sm">
						<Calendar class="w-4 h-4 text-surface-400" />
						<span class="text-surface-700">{job.scheduled_date}</span>
					</div>
					<div class="flex items-center gap-2 text-sm">
						<Clock class="w-4 h-4 text-surface-400" />
						<span class="text-surface-700">{job.scheduled_time}</span>
					</div>
					<div class="flex items-center gap-2 text-sm">
						<Wrench class="w-4 h-4 text-surface-400" />
						<span class="text-surface-700">Est. {job.estimated_duration}</span>
					</div>
				</div>
			</Card>

			<!-- Assigned -->
			<Card>
				<h3 class="text-sm font-semibold text-surface-500 uppercase tracking-wider mb-3">Assigned To</h3>
				<div class="flex items-center gap-3">
					<div class="w-10 h-10 bg-forge-100 text-forge-600 rounded-full flex items-center justify-center text-sm font-bold">
						MT
					</div>
					<div>
						<p class="text-sm font-medium text-surface-900">{job.assigned.name}</p>
						<p class="text-xs text-surface-400">HVAC Technician</p>
					</div>
				</div>
			</Card>

			<!-- Financials -->
			<Card>
				<h3 class="text-sm font-semibold text-surface-500 uppercase tracking-wider mb-3">Financials</h3>
				<div class="space-y-2">
					<div class="flex items-center justify-between text-sm">
						<span class="text-surface-500">Budget</span>
						<span class="font-medium text-surface-900">{formatCurrency(job.budget)}</span>
					</div>
					<div class="flex items-center justify-between text-sm">
						<span class="text-surface-500">Total</span>
						<span class="font-semibold text-surface-900">{formatCurrency(job.total)}</span>
					</div>
				</div>
				<div class="mt-4 pt-4 border-t border-surface-200">
					<Button variant="outline" size="sm" class="w-full">
						<FileText class="w-4 h-4" />
						View Estimate
					</Button>
				</div>
			</Card>
		</div>
	</div>
</div>
