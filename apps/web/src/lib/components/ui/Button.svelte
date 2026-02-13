<script lang="ts">
	import { cn } from '$lib/utils/cn';

	interface Props {
		variant?: 'primary' | 'secondary' | 'outline' | 'ghost' | 'danger';
		size?: 'sm' | 'md' | 'lg';
		disabled?: boolean;
		loading?: boolean;
		class?: string;
		type?: 'button' | 'submit' | 'reset';
		onclick?: (e: MouseEvent) => void;
		children?: import('svelte').Snippet;
	}

	let {
		variant = 'primary',
		size = 'md',
		disabled = false,
		loading = false,
		class: className = '',
		type = 'button',
		onclick,
		children
	}: Props = $props();

	const variants = {
		primary: 'bg-forge-600 text-white hover:bg-forge-700 active:bg-forge-800 shadow-sm',
		secondary: 'bg-surface-100 text-surface-700 hover:bg-surface-200 active:bg-surface-300',
		outline: 'border border-surface-300 text-surface-700 hover:bg-surface-50 active:bg-surface-100',
		ghost: 'text-surface-600 hover:bg-surface-100 active:bg-surface-200',
		danger: 'bg-danger-500 text-white hover:bg-danger-600 active:bg-red-700 shadow-sm'
	};

	const sizes = {
		sm: 'px-3 py-1.5 text-sm gap-1.5',
		md: 'px-4 py-2 text-sm gap-2',
		lg: 'px-6 py-3 text-base gap-2.5'
	};
</script>

<button
	{type}
	{onclick}
	disabled={disabled || loading}
	class={cn(
		'inline-flex items-center justify-center font-medium rounded-[var(--radius-button)] transition-all duration-[var(--transition-fast)] focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-forge-500 disabled:opacity-50 disabled:cursor-not-allowed cursor-pointer',
		variants[variant],
		sizes[size],
		className
	)}
>
	{#if loading}
		<svg class="animate-spin -ml-1 h-4 w-4" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
			<circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
			<path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
		</svg>
	{/if}
	{#if children}
		{@render children()}
	{/if}
</button>
