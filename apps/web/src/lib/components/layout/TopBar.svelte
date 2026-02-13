<script lang="ts">
	import { Search, Bell, Plus } from 'lucide-svelte';
	import Button from '$lib/components/ui/Button.svelte';

	interface Props {
		title?: string;
		children?: import('svelte').Snippet;
		actions?: import('svelte').Snippet;
	}

	let { title = '', children, actions }: Props = $props();

	let searchQuery = $state('');
</script>

<header class="sticky top-0 z-30 bg-white/80 backdrop-blur-md border-b border-surface-200">
	<div class="flex items-center justify-between h-16 px-6">
		<div class="flex items-center gap-4">
			{#if title}
				<h1 class="text-xl font-semibold text-surface-900">{title}</h1>
			{/if}
			{#if children}
				{@render children()}
			{/if}
		</div>

		<div class="flex items-center gap-3">
			<!-- Global search -->
			<div class="relative">
				<Search class="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-surface-400" />
				<input
					type="text"
					placeholder="Search jobs, customers..."
					bind:value={searchQuery}
					class="w-64 pl-9 pr-4 py-2 text-sm bg-surface-50 border border-surface-200 rounded-lg focus:outline-none focus:ring-2 focus:ring-forge-500 focus:border-forge-500 placeholder:text-surface-400"
				/>
				<kbd class="absolute right-3 top-1/2 -translate-y-1/2 text-xs text-surface-400 bg-surface-100 px-1.5 py-0.5 rounded border border-surface-200">⌘K</kbd>
			</div>

			<!-- Notifications -->
			<button class="relative p-2 text-surface-500 hover:text-surface-700 hover:bg-surface-100 rounded-lg transition-colors cursor-pointer">
				<Bell class="w-5 h-5" />
				<span class="absolute top-1.5 right-1.5 w-2 h-2 bg-danger-500 rounded-full"></span>
			</button>

			<!-- Quick actions -->
			{#if actions}
				{@render actions()}
			{/if}
		</div>
	</div>
</header>
