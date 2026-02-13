<script lang="ts">
	import { cn } from '$lib/utils/cn';

	interface Props {
		class?: string;
		variant?: 'text' | 'circle' | 'rect' | 'card';
		width?: string;
		height?: string;
		lines?: number;
	}

	let { class: className = '', variant = 'text', width, height, lines = 1 }: Props = $props();
</script>

{#if variant === 'card'}
	<div class={cn('bg-white rounded-xl border border-surface-200 p-6 space-y-4', className)}>
		<div class="flex items-center gap-3">
			<div class="w-10 h-10 bg-surface-200 rounded-full animate-pulse"></div>
			<div class="space-y-2 flex-1">
				<div class="h-4 bg-surface-200 rounded w-1/3 animate-pulse"></div>
				<div class="h-3 bg-surface-100 rounded w-1/2 animate-pulse"></div>
			</div>
		</div>
		<div class="space-y-2">
			<div class="h-3 bg-surface-100 rounded w-full animate-pulse"></div>
			<div class="h-3 bg-surface-100 rounded w-4/5 animate-pulse"></div>
		</div>
	</div>
{:else if variant === 'circle'}
	<div
		class={cn('bg-surface-200 rounded-full animate-pulse', className)}
		style="width: {width ?? '2.5rem'}; height: {height ?? '2.5rem'}"
	></div>
{:else if variant === 'rect'}
	<div
		class={cn('bg-surface-200 rounded-lg animate-pulse', className)}
		style="width: {width ?? '100%'}; height: {height ?? '8rem'}"
	></div>
{:else}
	<div class={cn('space-y-2', className)}>
		{#each Array(lines) as _, i}
			<div
				class="h-4 bg-surface-200 rounded animate-pulse"
				style="width: {i === lines - 1 && lines > 1 ? '60%' : width ?? '100%'}"
			></div>
		{/each}
	</div>
{/if}
