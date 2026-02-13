<script lang="ts">
	import { cn } from '$lib/utils/cn';

	interface DropdownItem {
		label: string;
		icon?: import('svelte').Component;
		onclick?: () => void;
		href?: string;
		variant?: 'default' | 'danger';
		divider?: boolean;
	}

	interface Props {
		items: DropdownItem[];
		trigger?: import('svelte').Snippet;
		align?: 'left' | 'right';
		class?: string;
	}

	let { items, trigger, align = 'right', class: className = '' }: Props = $props();

	let open = $state(false);

	function handleClickOutside(e: MouseEvent) {
		const target = e.target as HTMLElement;
		if (!target.closest('.dropdown-container')) {
			open = false;
		}
	}

	function handleSelect(item: DropdownItem) {
		open = false;
		item.onclick?.();
	}
</script>

<svelte:window onclick={open ? handleClickOutside : undefined} />

<div class={cn('relative inline-block dropdown-container', className)}>
	<!-- svelte-ignore a11y_click_events_have_key_events a11y_no_static_element_interactions -->
	<div onclick={() => (open = !open)} class="cursor-pointer">
		{#if trigger}
			{@render trigger()}
		{:else}
			<button class="p-1.5 text-surface-400 hover:text-surface-600 hover:bg-surface-100 rounded-lg transition-colors cursor-pointer">
				<svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
					<circle cx="12" cy="5" r="1" /><circle cx="12" cy="12" r="1" /><circle cx="12" cy="19" r="1" />
				</svg>
			</button>
		{/if}
	</div>

	{#if open}
		<div class={cn(
			'absolute top-full mt-1 z-50 min-w-[180px] bg-white rounded-xl shadow-xl border border-surface-200 py-1 overflow-hidden',
			align === 'right' ? 'right-0' : 'left-0'
		)}>
			{#each items as item}
				{#if item.divider}
					<div class="my-1 border-t border-surface-100"></div>
				{:else if item.href}
					<a
						href={item.href}
						class={cn(
							'flex items-center gap-2.5 px-3 py-2 text-sm transition-colors',
							item.variant === 'danger'
								? 'text-danger-600 hover:bg-danger-50'
								: 'text-surface-700 hover:bg-surface-50'
						)}
						onclick={() => (open = false)}
					>
						{#if item.icon}
							{@const Icon = item.icon}
							<Icon class="w-4 h-4" />
						{/if}
						{item.label}
					</a>
				{:else}
					<button
						class={cn(
							'flex items-center gap-2.5 w-full px-3 py-2 text-sm text-left transition-colors cursor-pointer',
							item.variant === 'danger'
								? 'text-danger-600 hover:bg-danger-50'
								: 'text-surface-700 hover:bg-surface-50'
						)}
						onclick={() => handleSelect(item)}
					>
						{#if item.icon}
							{@const Icon = item.icon}
							<Icon class="w-4 h-4" />
						{/if}
						{item.label}
					</button>
				{/if}
			{/each}
		</div>
	{/if}
</div>
