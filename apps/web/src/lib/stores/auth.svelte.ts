import { api } from '$lib/api/client';

interface User {
	id: string;
	email: string | null;
	phone: string | null;
	first_name: string;
	last_name: string;
	role: string;
	team_id: string | null;
	avatar_url?: string | null;
}

class AuthStore {
	user = $state<User | null>(null);
	token = $state<string | null>(null);
	isAuthenticated = $derived(this.user !== null && this.token !== null);
	isLoading = $state(true);

	constructor() {
		if (typeof window !== 'undefined') {
			const savedToken = localStorage.getItem('ff_token');
			const savedUser = localStorage.getItem('ff_user');
			if (savedToken && savedUser) {
				this.token = savedToken;
				this.user = JSON.parse(savedUser);
				api.setToken(savedToken);
			}
			this.isLoading = false;
		}
	}

	async login(email: string, password: string) {
		const res = await api.post<{
			token: string;
			refresh_token: string;
			user: User;
		}>('/auth/login', { email, password });

		this.token = res.data.token;
		this.user = res.data.user;
		api.setToken(res.data.token);

		localStorage.setItem('ff_token', res.data.token);
		localStorage.setItem('ff_refresh_token', res.data.refresh_token);
		localStorage.setItem('ff_user', JSON.stringify(res.data.user));
	}

	async register(data: {
		email?: string;
		phone?: string;
		password: string;
		first_name: string;
		last_name: string;
		trade?: string;
	}) {
		const res = await api.post<{
			token: string;
			refresh_token: string;
			user: User;
		}>('/auth/register', data);

		this.token = res.data.token;
		this.user = res.data.user;
		api.setToken(res.data.token);

		localStorage.setItem('ff_token', res.data.token);
		localStorage.setItem('ff_refresh_token', res.data.refresh_token);
		localStorage.setItem('ff_user', JSON.stringify(res.data.user));
	}

	logout() {
		this.token = null;
		this.user = null;
		api.setToken(null);
		localStorage.removeItem('ff_token');
		localStorage.removeItem('ff_refresh_token');
		localStorage.removeItem('ff_user');
	}

	get fullName() {
		if (!this.user) return '';
		return `${this.user.first_name} ${this.user.last_name}`;
	}

	get initials() {
		if (!this.user) return '';
		return `${this.user.first_name[0]}${this.user.last_name[0]}`.toUpperCase();
	}
}

export const auth = new AuthStore();
