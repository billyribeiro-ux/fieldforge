<script lang="ts">
	import Button from './Button.svelte';
	import { AlertTriangle, Trash2, Info } from 'lucide-svelte';

	interface Props {
		open: boolean;
		title?: string;
		message?: string;
		confirmLabel?: string;
		cancelLabel?: string;
		variant?: 'danger' | 'warning' | 'info';
		onconfirm: () => void;
		oncancel: () => void;
	}

	let {
		open = $bindable(false),
		title = 'Are you sure?',
		message = 'This action cannot be undone.',
		confirmLabel = 'Confirm',
		cancelLabel = 'Cancel',
		variant = 'danger',
		onconfirm,
		oncancel
	}: Props = $props();

	const icons = { danger: Trash2, warning: AlertTriangle, info: Info };
	const iconColors = { danger: 'text-danger-500 bg-danger-50', warning: 'text-amber-500 bg-amber-50', info: 'text-forge-500 bg-forge-50' };
	const buttonVariants = { danger: 'danger' as const, warning: 'primary' as const, info: 'primary' as const };

	function handleConfirm() {
		open = false;
		onconfirm();
	}

	function handleCancel() {
		open = false;
		oncancel();
	}

	function handleKeydown(e: KeyboardEvent) {
		if (e.key === 'Escape') handleCancel();
	}
</script>

<svelte:window onkeydown={open ? handleKeydown : undefined} />

{#if open}
	<!-- svelte-ignore a11y_click_events_have_key_events a11y_no_static_element_interactions -->
	<div class="fixed inset-0 z-[70] flex items-center justify-center p-4" onclick={handleCancel}>
		<div class="absolute inset-0 bg-black/40 backdrop-blur-sm"></div>

		<!-- svelte-ignore a11y_click_events_have_key_events a11y_no_static_element_interactions -->
		<div class="relative bg-white rounded-xl shadow-2xl border border-surface-200 max-w-sm w-full p-6" onclick={(e) => e.stopPropagation()}>
			{@const Icon = icons[variant]}
			<div class="flex items-start gap-4">
				<div class="w-10 h-10 rounded-xl flex items-center justify-center flex-shrink-0 {iconColors[variant]}">
					<Icon class="w-5 h-5" />
				</div>
				<div class="flex-1">
					<h3 class="text-base font-semibold text-surface-900">{title}</h3>
					<p class="text-sm text-surface-500 mt-1">{message}</p>
				</div>
			</div>

			<div class="flex items-center justify-end gap-3 mt-6">
				<Button variant="ghost" size="sm" onclick={handleCancel}>{cancelLabel}</Button>
				<Button variant={buttonVariants[variant]} size="sm" onclick={handleConfirm}>{confirmLabel}</Button>
			</div>
		</div>
	</div>
{/if}
