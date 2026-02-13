<script lang="ts">
	import { cn } from '$lib/utils/cn';

	interface Props {
		text: string;
		position?: 'top' | 'bottom' | 'left' | 'right';
		children: import('svelte').Snippet;
		class?: string;
	}

	let { text, position = 'top', children, class: className = '' }: Props = $props();

	let visible = $state(false);

	const positions = {
		top: 'bottom-full left-1/2 -translate-x-1/2 mb-2',
		bottom: 'top-full left-1/2 -translate-x-1/2 mt-2',
		left: 'right-full top-1/2 -translate-y-1/2 mr-2',
		right: 'left-full top-1/2 -translate-y-1/2 ml-2'
	};
</script>

<div
	class={cn('relative inline-flex', className)}
	onmouseenter={() => (visible = true)}
	onmouseleave={() => (visible = false)}
	onfocusin={() => (visible = true)}
	onfocusout={() => (visible = false)}
	role="tooltip"
>
	{@render children()}

	{#if visible}
		<div class={cn(
			'absolute z-50 px-2.5 py-1.5 text-xs font-medium text-white bg-surface-800 rounded-lg shadow-lg whitespace-nowrap pointer-events-none',
			positions[position]
		)}>
			{text}
		</div>
	{/if}
</div>
