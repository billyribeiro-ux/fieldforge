<script lang="ts">
	import TopBar from '$lib/components/layout/TopBar.svelte';
	import Card from '$lib/components/ui/Card.svelte';
	import Button from '$lib/components/ui/Button.svelte';
	import Badge from '$lib/components/ui/Badge.svelte';
	import Modal from '$lib/components/ui/Modal.svelte';
	import Input from '$lib/components/ui/Input.svelte';
	import Select from '$lib/components/ui/Select.svelte';
	import Avatar from '$lib/components/ui/Avatar.svelte';
	import { Plus, Search, Phone, Mail, MapPin, Clock, MoreHorizontal, Shield } from 'lucide-svelte';

	let searchQuery = $state('');
	let roleFilter = $state('all');
	let showInviteModal = $state(false);

	const members = [
		{ id: '1', first_name: 'Mike', last_name: 'Thompson', email: 'mike@smithhvac.com', phone: '(512) 555-0101', role: 'technician', status: 'on_job', avatar: null, hourly_rate: 45, jobs_today: 3, jobs_completed: 2, utilization: 87 },
		{ id: '2', first_name: 'Jake', last_name: 'Rodriguez', email: 'jake@smithhvac.com', phone: '(512) 555-0102', role: 'technician', status: 'available', avatar: null, hourly_rate: 42, jobs_today: 2, jobs_completed: 2, utilization: 78 },
		{ id: '3', first_name: 'Sarah', last_name: 'Lee', email: 'sarah@smithhvac.com', phone: '(512) 555-0103', role: 'technician', status: 'en_route', avatar: null, hourly_rate: 40, jobs_today: 4, jobs_completed: 1, utilization: 92 },
		{ id: '4', first_name: 'David', last_name: 'Smith', email: 'david@smithhvac.com', phone: '(512) 555-0100', role: 'owner', status: 'available', avatar: null, hourly_rate: 65, jobs_today: 1, jobs_completed: 0, utilization: 45 },
		{ id: '5', first_name: 'Emily', last_name: 'Chen', email: 'emily@smithhvac.com', phone: '(512) 555-0104', role: 'office_staff', status: 'available', avatar: null, hourly_rate: 25, jobs_today: 0, jobs_completed: 0, utilization: 0 },
		{ id: '6', first_name: 'Carlos', last_name: 'Martinez', email: 'carlos@smithhvac.com', phone: '(512) 555-0105', role: 'apprentice', status: 'off_duty', avatar: null, hourly_rate: 22, jobs_today: 0, jobs_completed: 0, utilization: 0 }
	];

	const roleOptions = [
		{ value: 'all', label: 'All Roles' },
		{ value: 'owner', label: 'Owner' },
		{ value: 'admin', label: 'Admin' },
		{ value: 'manager', label: 'Manager' },
		{ value: 'technician', label: 'Technician' },
		{ value: 'office_staff', label: 'Office Staff' },
		{ value: 'apprentice', label: 'Apprentice' }
	];

	function statusColor(status: string): 'success' | 'info' | 'warning' | 'default' {
		switch (status) {
			case 'available': return 'success';
			case 'on_job': case 'en_route': return 'info';
			case 'on_break': return 'warning';
			default: return 'default';
		}
	}

	function statusLabel(s: string): string {
		return s.replace(/_/g, ' ').replace(/\b\w/g, (c) => c.toUpperCase());
	}

	function roleLabel(r: string): string {
		return r.replace(/_/g, ' ').replace(/\b\w/g, (c) => c.toUpperCase());
	}

	function roleBadge(r: string): 'danger' | 'warning' | 'info' | 'default' {
		switch (r) {
			case 'owner': return 'danger';
			case 'admin': return 'warning';
			case 'manager': return 'info';
			default: return 'default';
		}
	}

	function initials(first: string, last: string): string {
		return `${first.charAt(0)}${last.charAt(0)}`.toUpperCase();
	}

	let filteredMembers = $derived(
		members.filter((m) => {
			if (roleFilter !== 'all' && m.role !== roleFilter) return false;
			if (searchQuery) {
				const q = searchQuery.toLowerCase();
				return `${m.first_name} ${m.last_name}`.toLowerCase().includes(q) || m.email.toLowerCase().includes(q);
			}
			return true;
		})
	);

	let activeCount = $derived(members.filter((m) => m.status !== 'off_duty').length);
	let avgUtilization = $derived(
		Math.round(members.filter((m) => m.role === 'technician').reduce((sum, m) => sum + m.utilization, 0) / members.filter((m) => m.role === 'technician').length)
	);

	// Invite form
	let inviteEmail = $state('');
	let inviteRole = $state('technician');
	let inviteName = $state('');
</script>

<TopBar title="Team">
	{#snippet actions()}
		<Button variant="primary" size="md" onclick={() => (showInviteModal = true)}>
			<Plus class="w-4 h-4" />
			Invite Member
		</Button>
	{/snippet}
</TopBar>

<div class="p-6 space-y-6">
	<!-- Summary Cards -->
	<div class="grid grid-cols-1 sm:grid-cols-4 gap-4">
		<Card>
			<p class="text-xs text-surface-400 uppercase tracking-wider">Total Members</p>
			<p class="text-2xl font-bold text-surface-900 mt-1">{members.length}</p>
		</Card>
		<Card>
			<p class="text-xs text-surface-400 uppercase tracking-wider">Active Now</p>
			<p class="text-2xl font-bold text-success-600 mt-1">{activeCount}</p>
		</Card>
		<Card>
			<p class="text-xs text-surface-400 uppercase tracking-wider">Avg Utilization</p>
			<p class="text-2xl font-bold text-forge-600 mt-1">{avgUtilization}%</p>
		</Card>
		<Card>
			<p class="text-xs text-surface-400 uppercase tracking-wider">Jobs Today</p>
			<p class="text-2xl font-bold text-surface-900 mt-1">{members.reduce((s, m) => s + m.jobs_today, 0)}</p>
		</Card>
	</div>

	<!-- Filters -->
	<div class="flex items-center gap-3">
		<div class="relative flex-1 max-w-sm">
			<Search class="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-surface-400" />
			<input
				type="text"
				placeholder="Search team members..."
				bind:value={searchQuery}
				class="w-full pl-9 pr-4 py-2 text-sm bg-white border border-surface-200 rounded-lg focus:outline-none focus:ring-2 focus:ring-forge-500 placeholder:text-surface-400"
			/>
		</div>
		<select
			bind:value={roleFilter}
			class="px-3 py-2 text-sm bg-white border border-surface-200 rounded-lg focus:outline-none focus:ring-2 focus:ring-forge-500 text-surface-700 cursor-pointer"
		>
			{#each roleOptions as opt}
				<option value={opt.value}>{opt.label}</option>
			{/each}
		</select>
	</div>

	<!-- Team Grid -->
	<div class="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-4">
		{#each filteredMembers as member (member.id)}
			<Card>
				<div class="flex items-start justify-between mb-4">
					<div class="flex items-center gap-3">
						<div class="relative">
							<div class="w-12 h-12 bg-forge-100 text-forge-700 rounded-full flex items-center justify-center text-sm font-bold">
								{initials(member.first_name, member.last_name)}
							</div>
							<div class="absolute -bottom-0.5 -right-0.5 w-3.5 h-3.5 rounded-full border-2 border-white {member.status === 'available' ? 'bg-success-500' : member.status === 'off_duty' ? 'bg-surface-300' : 'bg-forge-500'}"></div>
						</div>
						<div>
							<p class="text-sm font-semibold text-surface-900">{member.first_name} {member.last_name}</p>
							<Badge variant={roleBadge(member.role)} size="sm">{roleLabel(member.role)}</Badge>
						</div>
					</div>
					<Badge variant={statusColor(member.status)} size="sm">{statusLabel(member.status)}</Badge>
				</div>

				<div class="space-y-2 text-sm">
					<div class="flex items-center gap-2 text-surface-500">
						<Mail class="w-3.5 h-3.5" />
						<span class="truncate">{member.email}</span>
					</div>
					<div class="flex items-center gap-2 text-surface-500">
						<Phone class="w-3.5 h-3.5" />
						<span>{member.phone}</span>
					</div>
				</div>

				{#if member.role === 'technician'}
					<div class="mt-4 pt-4 border-t border-surface-100">
						<div class="grid grid-cols-3 gap-2 text-center">
							<div>
								<p class="text-xs text-surface-400">Jobs Today</p>
								<p class="text-sm font-bold text-surface-900">{member.jobs_today}</p>
							</div>
							<div>
								<p class="text-xs text-surface-400">Completed</p>
								<p class="text-sm font-bold text-success-600">{member.jobs_completed}</p>
							</div>
							<div>
								<p class="text-xs text-surface-400">Utilization</p>
								<p class="text-sm font-bold text-forge-600">{member.utilization}%</p>
							</div>
						</div>
						<div class="mt-2 h-1.5 bg-surface-100 rounded-full overflow-hidden">
							<div class="h-full bg-forge-500 rounded-full transition-all" style="width: {member.utilization}%"></div>
						</div>
					</div>
				{/if}

				<div class="mt-4 flex gap-2">
					<Button variant="outline" size="sm" class="flex-1">View Profile</Button>
					<button class="p-1.5 text-surface-400 hover:text-surface-600 rounded cursor-pointer">
						<MoreHorizontal class="w-4 h-4" />
					</button>
				</div>
			</Card>
		{/each}
	</div>
</div>

<!-- Invite Modal -->
<Modal bind:open={showInviteModal} title="Invite Team Member" size="md">
	<div class="space-y-4">
		<Input label="Full Name" bind:value={inviteName} required placeholder="John Smith" />
		<Input label="Email Address" type="email" bind:value={inviteEmail} required placeholder="john@company.com" />
		<Select label="Role" options={roleOptions.filter((r) => r.value !== 'all')} bind:value={inviteRole} />
	</div>
	{#snippet footer()}
		<div class="flex justify-end gap-3">
			<Button variant="outline" size="md" onclick={() => (showInviteModal = false)}>Cancel</Button>
			<Button variant="primary" size="md" onclick={() => (showInviteModal = false)}>Send Invite</Button>
		</div>
	{/snippet}
</Modal>
