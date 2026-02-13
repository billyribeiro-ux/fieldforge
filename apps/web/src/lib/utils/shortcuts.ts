export interface Shortcut {
	key: string;
	ctrl?: boolean;
	meta?: boolean;
	shift?: boolean;
	alt?: boolean;
	action: () => void;
	description: string;
}

export function matchesShortcut(event: KeyboardEvent, shortcut: Shortcut): boolean {
	const ctrlOrMeta = shortcut.ctrl || shortcut.meta;
	if (ctrlOrMeta && !(event.ctrlKey || event.metaKey)) return false;
	if (shortcut.shift && !event.shiftKey) return false;
	if (shortcut.alt && !event.altKey) return false;
	return event.key.toLowerCase() === shortcut.key.toLowerCase();
}

export function handleShortcuts(event: KeyboardEvent, shortcuts: Shortcut[]): void {
	if (event.target instanceof HTMLInputElement || event.target instanceof HTMLTextAreaElement || event.target instanceof HTMLSelectElement) {
		return;
	}

	for (const shortcut of shortcuts) {
		if (matchesShortcut(event, shortcut)) {
			event.preventDefault();
			shortcut.action();
			return;
		}
	}
}
