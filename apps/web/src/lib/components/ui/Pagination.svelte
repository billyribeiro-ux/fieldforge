<script lang="ts">
	import { cn } from '$lib/utils/cn';
	import { ChevronLeft, ChevronRight } from 'lucide-svelte';

	interface Props {
		currentPage: number;
		totalPages: number;
		onchange: (page: number) => void;
		class?: string;
	}

	let { currentPage, totalPages, onchange, class: className = '' }: Props = $props();

	let pages = $derived(() => {
		const result: (number | '...')[] = [];
		if (totalPages <= 7) {
			for (let i = 1; i <= totalPages; i++) result.push(i);
		} else {
			result.push(1);
			if (currentPage > 3) result.push('...');
			for (let i = Math.max(2, currentPage - 1); i <= Math.min(totalPages - 1, currentPage + 1); i++) {
				result.push(i);
			}
			if (currentPage < totalPages - 2) result.push('...');
			result.push(totalPages);
		}
		return result;
	});
</script>

{#if totalPages > 1}
	<nav class={cn('flex items-center gap-1', className)} aria-label="Pagination">
		<button
			disabled={currentPage <= 1}
			onclick={() => onchange(currentPage - 1)}
			class="p-2 rounded-lg text-surface-500 hover:bg-surface-100 disabled:opacity-40 disabled:cursor-not-allowed transition-colors cursor-pointer"
		>
			<ChevronLeft class="w-4 h-4" />
		</button>

		{#each pages() as p}
			{#if p === '...'}
				<span class="px-2 text-sm text-surface-400">…</span>
			{:else}
				<button
					class={cn(
						'w-9 h-9 rounded-lg text-sm font-medium transition-colors cursor-pointer',
						p === currentPage
							? 'bg-forge-600 text-white'
							: 'text-surface-600 hover:bg-surface-100'
					)}
					onclick={() => onchange(p)}
				>
					{p}
				</button>
			{/if}
		{/each}

		<button
			disabled={currentPage >= totalPages}
			onclick={() => onchange(currentPage + 1)}
			class="p-2 rounded-lg text-surface-500 hover:bg-surface-100 disabled:opacity-40 disabled:cursor-not-allowed transition-colors cursor-pointer"
		>
			<ChevronRight class="w-4 h-4" />
		</button>
	</nav>
{/if}
