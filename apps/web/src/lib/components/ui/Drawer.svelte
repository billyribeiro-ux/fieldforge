<script lang="ts">
	import { cn } from '$lib/utils/cn';
	import { X } from 'lucide-svelte';

	interface Props {
		open: boolean;
		title?: string;
		side?: 'left' | 'right';
		size?: 'sm' | 'md' | 'lg';
		onclose: () => void;
		children?: import('svelte').Snippet;
		footer?: import('svelte').Snippet;
	}

	let {
		open = $bindable(false),
		title = '',
		side = 'right',
		size = 'md',
		onclose,
		children,
		footer
	}: Props = $props();

	const sizes = {
		sm: 'max-w-sm',
		md: 'max-w-md',
		lg: 'max-w-xl'
	};

	function handleKeydown(e: KeyboardEvent) {
		if (e.key === 'Escape') onclose();
	}

	function handleBackdrop(e: MouseEvent) {
		if (e.target === e.currentTarget) onclose();
	}
</script>

<svelte:window onkeydown={open ? handleKeydown : undefined} />

{#if open}
	<!-- svelte-ignore a11y_click_events_have_key_events a11y_no_static_element_interactions -->
	<div class="fixed inset-0 z-50 flex" onclick={handleBackdrop}>
		<div class="absolute inset-0 bg-black/50 backdrop-blur-sm"></div>

		<div
			class={cn(
				'relative bg-white shadow-2xl w-full h-full flex flex-col',
				sizes[size],
				side === 'right' ? 'ml-auto' : 'mr-auto'
			)}
		>
			{#if title}
				<div class="flex items-center justify-between px-6 py-4 border-b border-surface-200 flex-shrink-0">
					<h2 class="text-lg font-semibold text-surface-900">{title}</h2>
					<button
						onclick={onclose}
						class="p-1.5 text-surface-400 hover:text-surface-600 hover:bg-surface-100 rounded-lg transition-colors cursor-pointer"
					>
						<X class="w-5 h-5" />
					</button>
				</div>
			{/if}

			<div class="flex-1 overflow-y-auto px-6 py-4">
				{#if children}
					{@render children()}
				{/if}
			</div>

			{#if footer}
				<div class="px-6 py-4 border-t border-surface-200 flex items-center justify-end gap-3 flex-shrink-0">
					{@render footer()}
				</div>
			{/if}
		</div>
	</div>
{/if}
