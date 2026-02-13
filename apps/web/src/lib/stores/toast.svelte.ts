type ToastType = 'success' | 'error' | 'warning' | 'info';

interface Toast {
	id: string;
	type: ToastType;
	title: string;
	message?: string;
	duration: number;
}

class ToastStore {
	toasts = $state<Toast[]>([]);

	private add(type: ToastType, title: string, message?: string, duration = 4000) {
		const id = crypto.randomUUID();
		this.toasts = [...this.toasts, { id, type, title, message, duration }];

		setTimeout(() => {
			this.dismiss(id);
		}, duration);

		return id;
	}

	success(title: string, message?: string) {
		return this.add('success', title, message);
	}

	error(title: string, message?: string) {
		return this.add('error', title, message, 6000);
	}

	warning(title: string, message?: string) {
		return this.add('warning', title, message, 5000);
	}

	info(title: string, message?: string) {
		return this.add('info', title, message);
	}

	dismiss(id: string) {
		this.toasts = this.toasts.filter((t) => t.id !== id);
	}
}

export const toast = new ToastStore();
