<script lang="ts">
	import { cn } from '$lib/utils/cn';

	interface Props {
		checked: boolean;
		onchange?: (checked: boolean) => void;
		disabled?: boolean;
		size?: 'sm' | 'md';
		label?: string;
		description?: string;
		class?: string;
	}

	let { checked = $bindable(false), onchange, disabled = false, size = 'md', label, description, class: className = '' }: Props = $props();

	function toggle() {
		if (disabled) return;
		checked = !checked;
		onchange?.(checked);
	}

	const trackSizes = { sm: 'w-8 h-[18px]', md: 'w-11 h-6' };
	const thumbSizes = { sm: 'w-3.5 h-3.5', md: 'w-5 h-5' };
	const thumbTranslate = { sm: 'translate-x-[16px]', md: 'translate-x-[22px]' };
</script>

<div class={cn('flex items-start gap-3', className)}>
	<button
		role="switch"
		aria-checked={checked}
		{disabled}
		onclick={toggle}
		class={cn(
			'relative inline-flex items-center rounded-full transition-colors duration-200 flex-shrink-0 cursor-pointer',
			trackSizes[size],
			checked ? 'bg-forge-600' : 'bg-surface-300',
			disabled && 'opacity-50 cursor-not-allowed'
		)}
	>
		<span
			class={cn(
				'inline-block rounded-full bg-white shadow-sm transform transition-transform duration-200',
				thumbSizes[size],
				checked ? thumbTranslate[size] : 'translate-x-0.5'
			)}
		></span>
	</button>

	{#if label || description}
		<!-- svelte-ignore a11y_click_events_have_key_events a11y_no_static_element_interactions -->
		<div class={disabled ? 'opacity-50' : 'cursor-pointer'} onclick={toggle}>
			{#if label}
				<p class="text-sm font-medium text-surface-900">{label}</p>
			{/if}
			{#if description}
				<p class="text-xs text-surface-500 mt-0.5">{description}</p>
			{/if}
		</div>
	{/if}
</div>
