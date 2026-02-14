import type { ServerLoad, Actions } from '@sveltejs/kit';
import { fail, redirect, isRedirect } from '@sveltejs/kit';
import { serverFetch, serverPost } from '$lib/api/server';

export const load: ServerLoad = async ({ locals, url }) => {
	const token = locals.token;
	const status = url.searchParams.get('status') ?? '';
	const search = url.searchParams.get('q') ?? '';

	try {
		const { data, meta } = await serverFetch<unknown[]>('/jobs', {
			token,
			params: { status, q: search }
		});

		return {
			jobs: data ?? [],
			meta: meta ?? {},
			filters: { status, search }
		};
	} catch (e) {
		return {
			jobs: [],
			meta: {},
			filters: { status, search },
			error: e instanceof Error ? e.message : 'Failed to load jobs'
		};
	}
};

export const actions: Actions = {
	create: async ({ request, locals }) => {
		const token = locals.token;
		if (!token) return fail(401, { error: 'Not authenticated' });

		const formData = await request.formData();
		const title = formData.get('title') as string;
		const customer_id = formData.get('customer_id') as string;
		const description = formData.get('description') as string;
		const trade = formData.get('trade') as string;
		const priority = formData.get('priority') as string;

		if (!title?.trim()) return fail(400, { error: 'Title is required' });
		if (!customer_id) return fail(400, { error: 'Customer is required' });

		try {
			const { data } = await serverPost<{ id: string }>('/jobs', {
				title: title.trim(),
				customer_id,
				description: description?.trim() || null,
				trade: trade || 'general',
				priority: priority || 'medium'
			}, { token });

			redirect(303, `/dashboard/jobs/${(data as { id: string }).id}`);
		} catch (e) {
			if (isRedirect(e)) throw e;
			return fail(500, { error: e instanceof Error ? e.message : 'Failed to create job' });
		}
	}
};
