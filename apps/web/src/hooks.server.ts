import type { Handle } from '@sveltejs/kit';

const API_BASE = process.env.PUBLIC_API_URL ?? 'http://localhost:8080/api/v1';

export const handle: Handle = async ({ event, resolve }) => {
	// Extract auth token from cookies
	const token = event.cookies.get('ff_token');

	if (token) {
		event.locals.token = token;

		// Fetch user profile if not cached
		try {
			const res = await fetch(`${API_BASE}/auth/me`, {
				headers: { Authorization: `Bearer ${token}` }
			});

			if (res.ok) {
				const json = await res.json();
				event.locals.user = json.data;
			} else if (res.status === 401) {
				// Token expired — clear cookie
				event.cookies.delete('ff_token', { path: '/' });
				event.locals.token = undefined;
				event.locals.user = undefined;
			}
		} catch {
			// API unreachable — continue without user
		}
	}

	const response = await resolve(event);
	return response;
};
