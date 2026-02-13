<script lang="ts">
	import { cn } from '$lib/utils/cn';
	import { Upload, X, FileIcon, Image } from 'lucide-svelte';
	import { formatFileSize } from '$lib/utils/format';

	interface Props {
		accept?: string;
		multiple?: boolean;
		maxSize?: number;
		onfiles?: (files: File[]) => void;
		class?: string;
	}

	let { accept = 'image/*', multiple = false, maxSize = 10 * 1024 * 1024, onfiles, class: className = '' }: Props = $props();

	let dragOver = $state(false);
	let files = $state<File[]>([]);
	let error = $state('');
	let inputEl: HTMLInputElement;

	function handleFiles(fileList: FileList | null) {
		if (!fileList) return;
		error = '';

		const newFiles: File[] = [];
		for (const file of Array.from(fileList)) {
			if (file.size > maxSize) {
				error = `${file.name} exceeds ${formatFileSize(maxSize)} limit`;
				continue;
			}
			newFiles.push(file);
		}

		if (multiple) {
			files = [...files, ...newFiles];
		} else {
			files = newFiles.slice(0, 1);
		}

		onfiles?.(files);
	}

	function removeFile(index: number) {
		files = files.filter((_, i) => i !== index);
		onfiles?.(files);
	}

	function handleDrop(e: DragEvent) {
		e.preventDefault();
		dragOver = false;
		handleFiles(e.dataTransfer?.files ?? null);
	}

	function handleDragOver(e: DragEvent) {
		e.preventDefault();
		dragOver = true;
	}

	function isImage(file: File): boolean {
		return file.type.startsWith('image/');
	}
</script>

<div class={cn('space-y-3', className)}>
	<!-- svelte-ignore a11y_click_events_have_key_events a11y_no_static_element_interactions -->
	<div
		class={cn(
			'border-2 border-dashed rounded-xl p-8 text-center transition-colors cursor-pointer',
			dragOver ? 'border-forge-400 bg-forge-50' : 'border-surface-300 hover:border-surface-400 bg-surface-50'
		)}
		ondrop={handleDrop}
		ondragover={handleDragOver}
		ondragleave={() => (dragOver = false)}
		onclick={() => inputEl?.click()}
	>
		<Upload class="w-8 h-8 text-surface-400 mx-auto mb-3" />
		<p class="text-sm font-medium text-surface-700">
			Drop files here or <span class="text-forge-600">browse</span>
		</p>
		<p class="text-xs text-surface-400 mt-1">
			Max {formatFileSize(maxSize)} per file
		</p>
	</div>

	<input
		bind:this={inputEl}
		type="file"
		{accept}
		{multiple}
		class="hidden"
		onchange={(e) => handleFiles((e.target as HTMLInputElement).files)}
	/>

	{#if error}
		<p class="text-xs text-danger-600">{error}</p>
	{/if}

	{#if files.length > 0}
		<div class="space-y-2">
			{#each files as file, i}
				<div class="flex items-center gap-3 p-3 bg-surface-50 rounded-lg border border-surface-200">
					{#if isImage(file)}
						<Image class="w-5 h-5 text-forge-500 flex-shrink-0" />
					{:else}
						<FileIcon class="w-5 h-5 text-surface-400 flex-shrink-0" />
					{/if}
					<div class="flex-1 min-w-0">
						<p class="text-sm font-medium text-surface-700 truncate">{file.name}</p>
						<p class="text-xs text-surface-400">{formatFileSize(file.size)}</p>
					</div>
					<button
						onclick={() => removeFile(i)}
						class="p-1 text-surface-400 hover:text-danger-500 transition-colors cursor-pointer"
					>
						<X class="w-4 h-4" />
					</button>
				</div>
			{/each}
		</div>
	{/if}
</div>
