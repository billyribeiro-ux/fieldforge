<script lang="ts">
	import { cn } from '$lib/utils/cn';

	interface Props {
		placeholder?: string;
		value?: string;
		name?: string;
		id?: string;
		label?: string;
		error?: string;
		required?: boolean;
		disabled?: boolean;
		rows?: number;
		class?: string;
	}

	let {
		placeholder = '',
		value = $bindable(''),
		name = '',
		id = '',
		label: labelText = '',
		error = '',
		required = false,
		disabled = false,
		rows = 3,
		class: className = ''
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

	<textarea
		{placeholder}
		{name}
		id={id || name}
		{required}
		{disabled}
		{rows}
		bind:value
		class={cn(
			'w-full rounded-[var(--radius-input)] border border-surface-300 bg-white px-3 py-2 text-sm text-surface-900 placeholder:text-surface-400 resize-y',
			'focus:outline-none focus:ring-2 focus:ring-forge-500 focus:border-forge-500',
			'disabled:bg-surface-50 disabled:text-surface-500 disabled:cursor-not-allowed',
			error && 'border-danger-500 focus:ring-danger-500 focus:border-danger-500',
			className
		)}
	></textarea>

	{#if error}
		<p class="text-xs text-danger-500">{error}</p>
	{/if}
</div>
