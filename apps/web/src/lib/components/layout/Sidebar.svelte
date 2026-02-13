<script lang="ts">
	import { cn } from '$lib/utils/cn';
	import { page } from '$app/state';
	import {
		LayoutDashboard,
		Briefcase,
		Users,
		FileText,
		Receipt,
		Calendar,
		Package,
		Truck,
		BarChart3,
		Settings,
		ChevronLeft,
		ChevronRight,
		Wrench,
		LogOut,
		Menu,
		X
	} from 'lucide-svelte';
	import { auth } from '$lib/stores/auth.svelte';

	let collapsed = $state(false);
	let mobileOpen = $state(false);

	const navItems = [
		{ href: '/dashboard', label: 'Dashboard', icon: LayoutDashboard },
		{ href: '/dashboard/jobs', label: 'Jobs', icon: Briefcase },
		{ href: '/dashboard/customers', label: 'Customers', icon: Users },
		{ href: '/dashboard/schedule', label: 'Schedule', icon: Calendar },
		{ href: '/dashboard/estimates', label: 'Estimates', icon: FileText },
		{ href: '/dashboard/invoices', label: 'Invoices', icon: Receipt },
		{ href: '/dashboard/inventory', label: 'Inventory', icon: Package },
		{ href: '/dashboard/fleet', label: 'Fleet', icon: Truck },
		{ href: '/dashboard/reports', label: 'Reports', icon: BarChart3 },
		{ href: '/dashboard/settings', label: 'Settings', icon: Settings }
	];

	function isActive(href: string): boolean {
		if (href === '/dashboard') return page.url.pathname === '/dashboard';
		return page.url.pathname.startsWith(href);
	}

	function handleNavClick() {
		mobileOpen = false;
	}
</script>

<!-- Mobile hamburger button -->
<button
	onclick={() => (mobileOpen = !mobileOpen)}
	class="fixed top-4 left-4 z-50 lg:hidden p-2 bg-surface-900 text-white rounded-lg shadow-lg cursor-pointer"
>
	{#if mobileOpen}
		<X class="w-5 h-5" />
	{:else}
		<Menu class="w-5 h-5" />
	{/if}
</button>

<!-- Mobile backdrop -->
{#if mobileOpen}
	<!-- svelte-ignore a11y_click_events_have_key_events a11y_no_static_element_interactions -->
	<div class="fixed inset-0 bg-black/50 z-30 lg:hidden" onclick={() => (mobileOpen = false)}></div>
{/if}

<aside
	class={cn(
		'fixed left-0 top-0 h-screen bg-surface-900 text-white flex flex-col z-40 transition-all duration-[var(--transition-slow)]',
		// Desktop
		'hidden lg:flex',
		collapsed ? 'lg:w-[var(--spacing-sidebar-collapsed)]' : 'lg:w-[var(--spacing-sidebar)]',
		// Mobile override
		mobileOpen && '!flex w-[var(--spacing-sidebar)]'
	)}
>
	<!-- Logo -->
	<div class="flex items-center h-16 px-4 border-b border-surface-700/50">
		<div class="flex items-center gap-3">
			<div class="w-8 h-8 bg-forge-500 rounded-lg flex items-center justify-center flex-shrink-0">
				<Wrench class="w-5 h-5 text-white" />
			</div>
			{#if !collapsed || mobileOpen}
				<span class="text-lg font-bold tracking-tight">FieldForge</span>
			{/if}
		</div>
	</div>

	<!-- Navigation -->
	<nav class="flex-1 py-4 px-3 space-y-1 overflow-y-auto">
		{#each navItems as item}
			{@const active = isActive(item.href)}
			<a
				href={item.href}
				onclick={handleNavClick}
				class={cn(
					'flex items-center gap-3 px-3 py-2.5 rounded-lg text-sm font-medium transition-all duration-[var(--transition-fast)]',
					active
						? 'bg-forge-600 text-white shadow-sm'
						: 'text-surface-300 hover:bg-surface-800 hover:text-white'
				)}
				title={collapsed && !mobileOpen ? item.label : undefined}
			>
				<item.icon class="w-5 h-5 flex-shrink-0" />
				{#if !collapsed || mobileOpen}
					<span>{item.label}</span>
				{/if}
			</a>
		{/each}
	</nav>

	<!-- User section -->
	<div class="border-t border-surface-700/50 p-3">
		{#if !collapsed || mobileOpen}
			<div class="flex items-center gap-3 px-3 py-2">
				<div class="w-8 h-8 bg-forge-500 rounded-full flex items-center justify-center text-xs font-bold flex-shrink-0">
					{auth.initials || 'FF'}
				</div>
				<div class="flex-1 min-w-0">
					<p class="text-sm font-medium text-white truncate">{auth.fullName || 'FieldForge User'}</p>
					<p class="text-xs text-surface-400 truncate">{auth.user?.email || ''}</p>
				</div>
			</div>
		{/if}

		<button
			onclick={() => auth.logout()}
			class={cn(
				'flex items-center gap-3 w-full px-3 py-2 rounded-lg text-sm text-surface-400 hover:bg-surface-800 hover:text-white transition-all duration-[var(--transition-fast)] cursor-pointer',
				collapsed && !mobileOpen && 'justify-center'
			)}
		>
			<LogOut class="w-5 h-5 flex-shrink-0" />
			{#if !collapsed || mobileOpen}
				<span>Sign Out</span>
			{/if}
		</button>
	</div>

	<!-- Collapse toggle (desktop only) -->
	<button
		onclick={() => (collapsed = !collapsed)}
		class="absolute -right-3 top-20 w-6 h-6 bg-surface-700 border border-surface-600 rounded-full hidden lg:flex items-center justify-center text-surface-300 hover:text-white hover:bg-surface-600 transition-colors cursor-pointer z-50"
	>
		{#if collapsed}
			<ChevronRight class="w-3.5 h-3.5" />
		{:else}
			<ChevronLeft class="w-3.5 h-3.5" />
		{/if}
	</button>
</aside>
