const API_BASE = process.env.PUBLIC_API_URL ?? 'http://localhost:8080/api/v1';

interface ServerFetchOptions {
	token?: string;
	params?: Record<string, string>;
}

export async function serverFetch<T>(
	path: string,
	options: ServerFetchOptions = {}
): Promise<{ data: T; meta: Record<string, unknown> | null }> {
	const url = new URL(`${API_BASE}${path}`);
	if (options.params) {
		Object.entries(options.params).forEach(([k, v]) => {
			if (v !== undefined && v !== null && v !== '') {
				url.searchParams.set(k, v);
			}
		});
	}

	const headers: Record<string, string> = {
		'Content-Type': 'application/json'
	};

	if (options.token) {
		headers['Authorization'] = `Bearer ${options.token}`;
	}

	const res = await fetch(url.toString(), { headers });

	if (!res.ok) {
		const text = await res.text().catch(() => '');
		throw new Error(`API ${res.status}: ${text}`);
	}

	return res.json();
}
