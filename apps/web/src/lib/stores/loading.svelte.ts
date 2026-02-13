class LoadingStore {
	active = $state(0);
	isLoading = $derived(this.active > 0);

	start() {
		this.active++;
	}

	stop() {
		this.active = Math.max(0, this.active - 1);
	}

	async wrap<T>(fn: () => Promise<T>): Promise<T> {
		this.start();
		try {
			return await fn();
		} finally {
			this.stop();
		}
	}
}

export const loadingStore = new LoadingStore();
