<script lang="ts">
	import { cn } from '$lib/utils/cn';
	import { X } from 'lucide-svelte';

	interface Props {
		open: boolean;
		title?: string;
		size?: 'sm' | 'md' | 'lg' | 'xl';
		onclose: () => void;
		children?: import('svelte').Snippet;
		footer?: import('svelte').Snippet;
	}

	let {
		open = $bindable(false),
		title = '',
		size = 'md',
		onclose,
		children,
		footer
	}: Props = $props();

	const sizes = {
		sm: 'max-w-md',
		md: 'max-w-lg',
		lg: 'max-w-2xl',
		xl: 'max-w-4xl'
	};

	function handleKeydown(e: KeyboardEvent) {
		if (e.key === 'Escape') onclose();
	}

	function handleBackdrop(e: MouseEvent) {
		if (e.target === e.currentTarget) onclose();
	}
</script>

{#if open}
<svelte:window onkeydown={handleKeydown} />
	<!-- svelte-ignore a11y_click_events_have_key_events a11y_no_static_element_interactions -->
	<div
		class="fixed inset-0 z-50 flex items-center justify-center p-4"
		onclick={handleBackdrop}
	>
		<!-- Backdrop -->
		<div class="absolute inset-0 bg-black/50 backdrop-blur-sm"></div>

		<!-- Modal -->
		<div class={cn('relative bg-white rounded-xl shadow-2xl w-full', sizes[size])}>
			<!-- Header -->
			{#if title}
				<div class="flex items-center justify-between px-6 py-4 border-b border-surface-200">
					<h2 class="text-lg font-semibold text-surface-900">{title}</h2>
					<button
						onclick={onclose}
						class="p-1.5 text-surface-400 hover:text-surface-600 hover:bg-surface-100 rounded-lg transition-colors cursor-pointer"
					>
						<X class="w-5 h-5" />
					</button>
				</div>
			{/if}

			<!-- Body -->
			<div class="px-6 py-4 max-h-[70vh] overflow-y-auto">
				{#if children}
					{@render children()}
				{/if}
			</div>

			<!-- Footer -->
			{#if footer}
				<div class="px-6 py-4 border-t border-surface-200 flex items-center justify-end gap-3">
					{@render footer()}
				</div>
			{/if}
		</div>
	</div>
{/if}
