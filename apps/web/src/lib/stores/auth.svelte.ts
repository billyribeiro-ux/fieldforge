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

	private setSession(token: string, refreshToken: string, user: User) {
		this.token = token;
		this.user = user;
		api.setToken(token);

		localStorage.setItem('ff_token', token);
		localStorage.setItem('ff_refresh_token', refreshToken);
		localStorage.setItem('ff_user', JSON.stringify(user));

		// Set cookie so SSR load functions can access the token
		document.cookie = `ff_token=${token}; path=/; max-age=${60 * 60 * 24 * 7}; SameSite=Lax`;
	}

	async login(email: string, password: string) {
		const res = await api.post<{
			token: string;
			refresh_token: string;
			user: User;
		}>('/auth/login', { email, password });

		this.setSession(res.data.token, res.data.refresh_token, res.data.user);
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

		this.setSession(res.data.token, res.data.refresh_token, res.data.user);
	}

	logout() {
		this.token = null;
		this.user = null;
		api.setToken(null);
		localStorage.removeItem('ff_token');
		localStorage.removeItem('ff_refresh_token');
		localStorage.removeItem('ff_user');
		// Clear the cookie
		document.cookie = 'ff_token=; path=/; max-age=0';
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
