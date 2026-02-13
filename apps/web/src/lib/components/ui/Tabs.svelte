<script lang="ts">
	import { cn } from '$lib/utils/cn';

	interface Tab {
		id: string;
		label: string;
		icon?: import('svelte').Component;
		count?: number;
	}

	interface Props {
		tabs: Tab[];
		active: string;
		onchange: (id: string) => void;
		variant?: 'underline' | 'pills';
		size?: 'sm' | 'md';
		class?: string;
	}

	let { tabs, active, onchange, variant = 'underline', size = 'md', class: className = '' }: Props = $props();
</script>

<div class={cn(
	variant === 'underline' ? 'border-b border-surface-200' : '',
	className
)}>
	<nav class={cn('flex', variant === 'pills' ? 'gap-2' : 'gap-0')} role="tablist">
		{#each tabs as tab}
			<button
				role="tab"
				aria-selected={active === tab.id}
				class={cn(
					'flex items-center gap-2 font-medium transition-all cursor-pointer whitespace-nowrap',
					size === 'sm' ? 'text-xs px-3 py-2' : 'text-sm px-4 py-2.5',
					variant === 'underline'
						? active === tab.id
							? 'text-forge-600 border-b-2 border-forge-600 -mb-px'
							: 'text-surface-500 hover:text-surface-700 border-b-2 border-transparent -mb-px'
						: active === tab.id
							? 'bg-forge-100 text-forge-700 rounded-lg'
							: 'text-surface-500 hover:text-surface-700 hover:bg-surface-50 rounded-lg'
				)}
				onclick={() => onchange(tab.id)}
			>
				{#if tab.icon}
					{@const Icon = tab.icon}
					<Icon class={size === 'sm' ? 'w-3.5 h-3.5' : 'w-4 h-4'} />
				{/if}
				{tab.label}
				{#if tab.count !== undefined}
					<span class={cn(
						'rounded-full font-medium',
						size === 'sm' ? 'text-[10px] px-1.5 py-0.5' : 'text-xs px-2 py-0.5',
						active === tab.id
							? 'bg-forge-200 text-forge-700'
							: 'bg-surface-100 text-surface-500'
					)}>
						{tab.count}
					</span>
				{/if}
			</button>
		{/each}
	</nav>
</div>
