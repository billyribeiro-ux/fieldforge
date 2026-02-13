const API_BASE = import.meta.env.PUBLIC_API_URL ?? 'http://localhost:8080/api/v1';

interface ApiResponse<T> {
	data: T;
	meta: Record<string, unknown> | null;
	errors: Array<{ code: string; message: string }> | null;
}

class ApiClient {
	private baseUrl: string;
	private token: string | null = null;

	constructor(baseUrl: string) {
		this.baseUrl = baseUrl;
	}

	setToken(token: string | null) {
		this.token = token;
	}

	private async request<T>(
		method: string,
		path: string,
		body?: unknown,
		params?: Record<string, string>
	): Promise<ApiResponse<T>> {
		const url = new URL(`${this.baseUrl}${path}`);
		if (params) {
			Object.entries(params).forEach(([k, v]) => {
				if (v !== undefined && v !== null && v !== '') {
					url.searchParams.set(k, v);
				}
			});
		}

		const headers: Record<string, string> = {
			'Content-Type': 'application/json'
		};

		if (this.token) {
			headers['Authorization'] = `Bearer ${this.token}`;
		}

		const res = await fetch(url.toString(), {
			method,
			headers,
			body: body ? JSON.stringify(body) : undefined
		});

		const json = await res.json();

		if (!res.ok) {
			const errorMsg = json.errors?.[0]?.message ?? `HTTP ${res.status}`;
			throw new ApiError(res.status, json.errors?.[0]?.code ?? 'UNKNOWN', errorMsg);
		}

		return json as ApiResponse<T>;
	}

	get<T>(path: string, params?: Record<string, string>) {
		return this.request<T>('GET', path, undefined, params);
	}

	post<T>(path: string, body: unknown) {
		return this.request<T>('POST', path, body);
	}

	patch<T>(path: string, body: unknown) {
		return this.request<T>('PATCH', path, body);
	}

	put<T>(path: string, body: unknown) {
		return this.request<T>('PUT', path, body);
	}

	delete<T>(path: string) {
		return this.request<T>('DELETE', path);
	}
}

export class ApiError extends Error {
	status: number;
	code: string;

	constructor(status: number, code: string, message: string) {
		super(message);
		this.status = status;
		this.code = code;
	}
}

export const api = new ApiClient(API_BASE);
export type { ApiResponse };
