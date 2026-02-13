<script lang="ts">
	import { ChevronRight, Home } from 'lucide-svelte';

	interface Crumb {
		label: string;
		href?: string;
	}

	interface Props {
		items: Crumb[];
		showHome?: boolean;
	}

	let { items, showHome = true }: Props = $props();
</script>

<nav aria-label="Breadcrumb" class="flex items-center gap-1.5 text-sm">
	{#if showHome}
		<a href="/dashboard" class="text-surface-400 hover:text-surface-600 transition-colors">
			<Home class="w-4 h-4" />
		</a>
		{#if items.length > 0}
			<ChevronRight class="w-3.5 h-3.5 text-surface-300" />
		{/if}
	{/if}

	{#each items as crumb, i}
		{#if i > 0}
			<ChevronRight class="w-3.5 h-3.5 text-surface-300" />
		{/if}

		{#if crumb.href && i < items.length - 1}
			<a href={crumb.href} class="text-surface-500 hover:text-surface-700 transition-colors">
				{crumb.label}
			</a>
		{:else}
			<span class="text-surface-900 font-medium">{crumb.label}</span>
		{/if}
	{/each}
</nav>
