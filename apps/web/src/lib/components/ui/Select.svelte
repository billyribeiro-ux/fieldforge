<script lang="ts">
	import { cn } from '$lib/utils/cn';

	interface Option {
		value: string;
		label: string;
	}

	interface Props {
		options: Option[];
		value?: string;
		name?: string;
		id?: string;
		label?: string;
		placeholder?: string;
		error?: string;
		required?: boolean;
		disabled?: boolean;
		class?: string;
		onchange?: (e: Event) => void;
	}

	let {
		options,
		value = $bindable(''),
		name = '',
		id = '',
		label: labelText = '',
		placeholder = 'Select...',
		error = '',
		required = false,
		disabled = false,
		class: className = '',
		onchange
	}: Props = $props();
</script>

<div class="flex flex-col gap-1.5">
	{#if labelText}
		<label for={id || name} class="text-sm font-medium text-surface-700">
			{labelText}
			{#if required}
				<span class="text-danger-500">*</span>
			{/if}
		</label>
	{/if}

	<select
		{name}
		id={id || name}
		{required}
		{disabled}
		bind:value
		{onchange}
		class={cn(
			'w-full rounded-[var(--radius-input)] border border-surface-300 bg-white px-3 py-2 text-sm text-surface-900',
			'focus:outline-none focus:ring-2 focus:ring-forge-500 focus:border-forge-500',
			'disabled:bg-surface-50 disabled:text-surface-500 disabled:cursor-not-allowed',
			error && 'border-danger-500 focus:ring-danger-500 focus:border-danger-500',
			className
		)}
	>
		{#if placeholder}
			<option value="" disabled>{placeholder}</option>
		{/if}
		{#each options as opt}
			<option value={opt.value}>{opt.label}</option>
		{/each}
	</select>

	{#if error}
		<p class="text-xs text-danger-500">{error}</p>
	{/if}
</div>
