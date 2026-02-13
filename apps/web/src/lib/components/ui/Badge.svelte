<script lang="ts">
	import { cn } from '$lib/utils/cn';

	interface Props {
		variant?: 'default' | 'success' | 'warning' | 'danger' | 'info' | 'outline';
		size?: 'sm' | 'md';
		class?: string;
		children?: import('svelte').Snippet;
	}

	let {
		variant = 'default',
		size = 'sm',
		class: className = '',
		children
	}: Props = $props();

	const variants = {
		default: 'bg-surface-100 text-surface-700',
		success: 'bg-green-50 text-green-700 ring-1 ring-green-600/20',
		warning: 'bg-yellow-50 text-yellow-700 ring-1 ring-yellow-600/20',
		danger: 'bg-red-50 text-red-700 ring-1 ring-red-600/20',
		info: 'bg-blue-50 text-blue-700 ring-1 ring-blue-600/20',
		outline: 'border border-surface-300 text-surface-600'
	};

	const sizes = {
		sm: 'px-2 py-0.5 text-xs',
		md: 'px-2.5 py-1 text-sm'
	};
</script>

<span
	class={cn(
		'inline-flex items-center font-medium rounded-[var(--radius-badge)]',
		variants[variant],
		sizes[size],
		className
	)}
>
	{#if children}
		{@render children()}
	{/if}
</span>
