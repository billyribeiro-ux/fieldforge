<script lang="ts">
	import { cn } from '$lib/utils/cn';

	interface Props {
		class?: string;
		padding?: boolean;
		hover?: boolean;
		children?: import('svelte').Snippet;
	}

	let {
		class: className = '',
		padding = true,
		hover = false,
		children
	}: Props = $props();
</script>

<div
	class={cn(
		'bg-white rounded-[var(--radius-card)] shadow-[var(--shadow-card)] border border-surface-200',
		padding && 'p-6',
		hover && 'hover:shadow-[var(--shadow-card-hover)] transition-shadow duration-[var(--transition-normal)]',
		className
	)}
>
	{#if children}
		{@render children()}
	{/if}
</div>
