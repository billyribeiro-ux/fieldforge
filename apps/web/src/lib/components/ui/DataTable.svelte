<script lang="ts">
	import { cn } from '$lib/utils/cn';
	import { ChevronUp, ChevronDown } from 'lucide-svelte';

	interface Column<T = Record<string, unknown>> {
		key: string;
		label: string;
		sortable?: boolean;
		width?: string;
		align?: 'left' | 'center' | 'right';
		render?: import('svelte').Snippet<[T]>;
	}

	interface Props {
		columns: Column[];
		rows: Record<string, unknown>[];
		sortKey?: string;
		sortDir?: 'asc' | 'desc';
		onsort?: (key: string, dir: 'asc' | 'desc') => void;
		onrowclick?: (row: Record<string, unknown>) => void;
		loading?: boolean;
		emptyMessage?: string;
		class?: string;
		striped?: boolean;
		hoverable?: boolean;
	}

	let {
		columns,
		rows,
		sortKey = '',
		sortDir = 'asc',
		onsort,
		onrowclick,
		loading = false,
		emptyMessage = 'No data found',
		class: className = '',
		striped = false,
		hoverable = true
	}: Props = $props();

	function handleSort(col: Column) {
		if (!col.sortable || !onsort) return;
		const newDir = sortKey === col.key && sortDir === 'asc' ? 'desc' : 'asc';
		onsort(col.key, newDir);
	}

	function getCellValue(row: Record<string, unknown>, key: string): unknown {
		return key.split('.').reduce((obj: unknown, k) => {
			if (obj && typeof obj === 'object') return (obj as Record<string, unknown>)[k];
			return undefined;
		}, row);
	}
</script>

<div class={cn('overflow-x-auto rounded-xl border border-surface-200 bg-white', className)}>
	<table class="w-full text-sm">
		<thead>
			<tr class="border-b border-surface-200 bg-surface-50">
				{#each columns as col}
					<th
						class={cn(
							'px-4 py-3 text-xs font-semibold text-surface-500 uppercase tracking-wider',
							col.align === 'center' && 'text-center',
							col.align === 'right' && 'text-right',
							col.sortable && 'cursor-pointer select-none hover:text-surface-700 transition-colors'
						)}
						style={col.width ? `width: ${col.width}` : ''}
						onclick={() => handleSort(col)}
					>
						<div class={cn('flex items-center gap-1', col.align === 'right' && 'justify-end', col.align === 'center' && 'justify-center')}>
							{col.label}
							{#if col.sortable && sortKey === col.key}
								{#if sortDir === 'asc'}
									<ChevronUp class="w-3.5 h-3.5" />
								{:else}
									<ChevronDown class="w-3.5 h-3.5" />
								{/if}
							{/if}
						</div>
					</th>
				{/each}
			</tr>
		</thead>
		<tbody>
			{#if loading}
				{#each Array(5) as _}
					<tr class="border-b border-surface-100">
						{#each columns as _col}
							<td class="px-4 py-3">
								<div class="h-4 bg-surface-100 rounded animate-pulse" style="width: {60 + Math.random() * 30}%"></div>
							</td>
						{/each}
					</tr>
				{/each}
			{:else if rows.length === 0}
				<tr>
					<td colspan={columns.length} class="px-4 py-12 text-center text-surface-400">
						{emptyMessage}
					</td>
				</tr>
			{:else}
				{#each rows as row, i}
					<tr
						class={cn(
							'border-b border-surface-100 transition-colors',
							striped && i % 2 === 1 && 'bg-surface-50/50',
							hoverable && 'hover:bg-surface-50',
							onrowclick && 'cursor-pointer'
						)}
						onclick={() => onrowclick?.(row)}
					>
						{#each columns as col}
							<td
								class={cn(
									'px-4 py-3 text-surface-700',
									col.align === 'center' && 'text-center',
									col.align === 'right' && 'text-right'
								)}
							>
								{#if col.render}
									{@render col.render(row)}
								{:else}
									{getCellValue(row, col.key) ?? '—'}
								{/if}
							</td>
						{/each}
					</tr>
				{/each}
			{/if}
		</tbody>
	</table>
</div>
