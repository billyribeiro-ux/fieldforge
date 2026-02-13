<script lang="ts">
	import { cn } from '$lib/utils/cn';

	interface Props {
		value: number;
		max?: number;
		size?: 'sm' | 'md' | 'lg';
		variant?: 'default' | 'success' | 'warning' | 'danger';
		showLabel?: boolean;
		class?: string;
	}

	let { value, max = 100, size = 'md', variant = 'default', showLabel = false, class: className = '' }: Props = $props();

	let percent = $derived(Math.min(100, Math.max(0, (value / max) * 100)));

	const heights = { sm: 'h-1.5', md: 'h-2.5', lg: 'h-4' };

	const barColors = {
		default: 'bg-forge-500',
		success: 'bg-green-500',
		warning: 'bg-amber-500',
		danger: 'bg-red-500'
	};
</script>

<div class={cn('w-full', className)}>
	{#if showLabel}
		<div class="flex items-center justify-between mb-1">
			<span class="text-xs font-medium text-surface-600">{Math.round(percent)}%</span>
		</div>
	{/if}
	<div class={cn('w-full bg-surface-100 rounded-full overflow-hidden', heights[size])}>
		<div
			class={cn('h-full rounded-full transition-all duration-500 ease-out', barColors[variant])}
			style="width: {percent}%"
		></div>
	</div>
</div>
