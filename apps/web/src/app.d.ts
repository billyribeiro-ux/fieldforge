// See https://svelte.dev/docs/kit/types#app.d.ts
// for information about these interfaces
declare global {
	namespace App {
		// interface Error {}
		interface Locals {
			token?: string;
			user?: {
				id: string;
				email: string | null;
				phone: string | null;
				first_name: string;
				last_name: string;
				role: string;
				team_id: string | null;
				avatar_url?: string | null;
			};
		}
		// interface PageData {}
		// interface PageState {}
		// interface Platform {}
	}
}

export {};
