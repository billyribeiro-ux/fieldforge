import { api } from '$lib/api/client';

interface Notification {
	id: string;
	type: string;
	title: string;
	body: string;
	data: Record<string, unknown> | null;
	read_at: string | null;
	created_at: string;
}

class NotificationStore {
	notifications = $state<Notification[]>([]);
	unreadCount = $state(0);
	loading = $state(false);
	pollInterval: ReturnType<typeof setInterval> | null = null;

	async fetchNotifications() {
		this.loading = true;
		try {
			const res = await api.get<Notification[]>('/notifications');
			this.notifications = (res as unknown as Notification[]) ?? [];
			this.unreadCount = this.notifications.filter((n) => !n.read_at).length;
		} catch {
			// Silently fail — notifications are non-critical
		} finally {
			this.loading = false;
		}
	}

	async fetchUnreadCount() {
		try {
			const res = await api.get<{ count: number }>('/notifications/unread-count');
			this.unreadCount = (res as unknown as { count: number })?.count ?? 0;
		} catch {
			// Silently fail
		}
	}

	async markAsRead(id: string) {
		try {
			await api.post(`/notifications/${id}/read`, {});
			const notif = this.notifications.find((n) => n.id === id);
			if (notif) {
				notif.read_at = new Date().toISOString();
				this.unreadCount = Math.max(0, this.unreadCount - 1);
			}
		} catch {
			// Silently fail
		}
	}

	async markAllAsRead() {
		try {
			await api.post('/notifications/read-all', {});
			this.notifications.forEach((n) => {
				if (!n.read_at) n.read_at = new Date().toISOString();
			});
			this.unreadCount = 0;
		} catch {
			// Silently fail
		}
	}

	startPolling(intervalMs = 30000) {
		this.stopPolling();
		this.fetchUnreadCount();
		this.pollInterval = setInterval(() => this.fetchUnreadCount(), intervalMs);
	}

	stopPolling() {
		if (this.pollInterval) {
			clearInterval(this.pollInterval);
			this.pollInterval = null;
		}
	}
}

export const notificationStore = new NotificationStore();
