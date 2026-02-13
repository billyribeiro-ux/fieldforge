<script lang="ts">
	import { goto } from '$app/navigation';
	import {
		Search, Briefcase, Users, FileText, Receipt, Calendar, Package, Truck,
		BarChart3, Settings, Plus, ArrowRight
	} from 'lucide-svelte';

	let open = $state(false);
	let query = $state('');
	let selectedIndex = $state(0);
	let inputEl: HTMLInputElement;

	const commands = [
		{ id: 'nav-dashboard', label: 'Go to Dashboard', icon: BarChart3, section: 'Navigation', action: () => goto('/dashboard') },
		{ id: 'nav-jobs', label: 'Go to Jobs', icon: Briefcase, section: 'Navigation', action: () => goto('/dashboard/jobs') },
		{ id: 'nav-customers', label: 'Go to Customers', icon: Users, section: 'Navigation', action: () => goto('/dashboard/customers') },
		{ id: 'nav-estimates', label: 'Go to Estimates', icon: FileText, section: 'Navigation', action: () => goto('/dashboard/estimates') },
		{ id: 'nav-invoices', label: 'Go to Invoices', icon: Receipt, section: 'Navigation', action: () => goto('/dashboard/invoices') },
		{ id: 'nav-schedule', label: 'Go to Schedule', icon: Calendar, section: 'Navigation', action: () => goto('/dashboard/schedule') },
		{ id: 'nav-inventory', label: 'Go to Inventory', icon: Package, section: 'Navigation', action: () => goto('/dashboard/inventory') },
		{ id: 'nav-fleet', label: 'Go to Fleet', icon: Truck, section: 'Navigation', action: () => goto('/dashboard/fleet') },
		{ id: 'nav-reports', label: 'Go to Reports', icon: BarChart3, section: 'Navigation', action: () => goto('/dashboard/reports') },
		{ id: 'nav-settings', label: 'Go to Settings', icon: Settings, section: 'Navigation', action: () => goto('/dashboard/settings') },
		{ id: 'create-job', label: 'Create New Job', icon: Plus, section: 'Actions', action: () => goto('/dashboard/jobs?new=1') },
		{ id: 'create-customer', label: 'Create New Customer', icon: Plus, section: 'Actions', action: () => goto('/dashboard/customers?new=1') },
		{ id: 'create-estimate', label: 'Create New Estimate', icon: Plus, section: 'Actions', action: () => goto('/dashboard/estimates/new') },
	];

	let filtered = $derived(
		query.trim()
			? commands.filter((c) => c.label.toLowerCase().includes(query.toLowerCase()))
			: commands
	);

	let sections = $derived(() => {
		const map = new Map<string, typeof commands>();
		for (const cmd of filtered) {
			if (!map.has(cmd.section)) map.set(cmd.section, []);
			map.get(cmd.section)!.push(cmd);
		}
		return map;
	});

	function handleKeydown(e: KeyboardEvent) {
		if ((e.metaKey || e.ctrlKey) && e.key === 'k') {
			e.preventDefault();
			open = !open;
			if (open) {
				query = '';
				selectedIndex = 0;
				setTimeout(() => inputEl?.focus(), 50);
			}
		}

		if (!open) return;

		if (e.key === 'Escape') {
			open = false;
		} else if (e.key === 'ArrowDown') {
			e.preventDefault();
			selectedIndex = Math.min(selectedIndex + 1, filtered.length - 1);
		} else if (e.key === 'ArrowUp') {
			e.preventDefault();
			selectedIndex = Math.max(selectedIndex - 1, 0);
		} else if (e.key === 'Enter' && filtered[selectedIndex]) {
			e.preventDefault();
			execute(filtered[selectedIndex]);
		}
	}

	function execute(cmd: (typeof commands)[0]) {
		open = false;
		query = '';
		cmd.action();
	}

	function handleBackdrop(e: MouseEvent) {
		if (e.target === e.currentTarget) open = false;
	}
</script>

<svelte:window onkeydown={handleKeydown} />

{#if open}
	<!-- svelte-ignore a11y_click_events_have_key_events a11y_no_static_element_interactions -->
	<div class="fixed inset-0 z-[60] flex items-start justify-center pt-[20vh]" onclick={handleBackdrop}>
		<div class="absolute inset-0 bg-black/40 backdrop-blur-sm"></div>

		<div class="relative w-full max-w-lg bg-white rounded-xl shadow-2xl border border-surface-200 overflow-hidden">
			<!-- Search input -->
			<div class="flex items-center gap-3 px-4 border-b border-surface-200">
				<Search class="w-5 h-5 text-surface-400 flex-shrink-0" />
				<input
					bind:this={inputEl}
					bind:value={query}
					type="text"
					placeholder="Search commands, pages..."
					class="flex-1 py-4 text-sm bg-transparent outline-none placeholder:text-surface-400"
				/>
				<kbd class="text-xs text-surface-400 bg-surface-100 px-1.5 py-0.5 rounded border border-surface-200">ESC</kbd>
			</div>

			<!-- Results -->
			<div class="max-h-72 overflow-y-auto py-2">
				{#if filtered.length === 0}
					<div class="px-4 py-8 text-center">
						<p class="text-sm text-surface-400">No results for "{query}"</p>
					</div>
				{:else}
					{#each filtered as cmd, i}
						<!-- svelte-ignore a11y_click_events_have_key_events a11y_no_static_element_interactions -->
						<div
							class="flex items-center gap-3 px-4 py-2.5 mx-2 rounded-lg cursor-pointer transition-colors
								{i === selectedIndex ? 'bg-forge-50 text-forge-700' : 'text-surface-700 hover:bg-surface-50'}"
							onclick={() => execute(cmd)}
							onmouseenter={() => (selectedIndex = i)}
						>
							<cmd.icon class="w-4 h-4 flex-shrink-0 {i === selectedIndex ? 'text-forge-500' : 'text-surface-400'}" />
							<span class="text-sm font-medium flex-1">{cmd.label}</span>
							<span class="text-xs text-surface-400">{cmd.section}</span>
						</div>
					{/each}
				{/if}
			</div>

			<!-- Footer -->
			<div class="px-4 py-2.5 border-t border-surface-200 bg-surface-50 flex items-center gap-4 text-xs text-surface-400">
				<span class="flex items-center gap-1"><kbd class="px-1 py-0.5 bg-white border border-surface-200 rounded text-[10px]">↑↓</kbd> Navigate</span>
				<span class="flex items-center gap-1"><kbd class="px-1 py-0.5 bg-white border border-surface-200 rounded text-[10px]">↵</kbd> Select</span>
				<span class="flex items-center gap-1"><kbd class="px-1 py-0.5 bg-white border border-surface-200 rounded text-[10px]">ESC</kbd> Close</span>
			</div>
		</div>
	</div>
{/if}
