<script lang="ts">
	import { cn } from '$lib/utils/cn';

	interface Props {
		type?: string;
		placeholder?: string;
		value?: string;
		name?: string;
		id?: string;
		label?: string;
		error?: string;
		required?: boolean;
		disabled?: boolean;
		class?: string;
		oninput?: (e: Event) => void;
		onchange?: (e: Event) => void;
	}

	let {
		type = 'text',
		placeholder = '',
		value = $bindable(''),
		name = '',
		id = '',
		label = '',
		error = '',
		required = false,
		disabled = false,
		class: className = '',
		oninput,
		onchange
	}: Props = $props();
</script>

<div class="flex flex-col gap-1.5">
	{#if label}
		<label for={id || name} class="text-sm font-medium text-surface-700">
			{label}
			{#if required}
				<span class="text-danger-500">*</span>
			{/if}
		</label>
	{/if}

	<input
		{type}
		{placeholder}
		{name}
		id={id || name}
		{required}
		{disabled}
		bind:value
		{oninput}
		{onchange}
		class={cn(
			'w-full rounded-[var(--radius-input)] border border-surface-300 bg-white px-3 py-2 text-sm text-surface-900 placeholder:text-surface-400',
			'focus:outline-none focus:ring-2 focus:ring-forge-500 focus:border-forge-500',
			'disabled:bg-surface-50 disabled:text-surface-500 disabled:cursor-not-allowed',
			error && 'border-danger-500 focus:ring-danger-500 focus:border-danger-500',
			className
		)}
	/>

	{#if error}
		<p class="text-xs text-danger-500">{error}</p>
	{/if}
</div>
