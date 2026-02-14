import type { ServerLoad, Actions } from '@sveltejs/kit';
import { fail, redirect, isRedirect } from '@sveltejs/kit';
import { serverFetch, serverPost } from '$lib/api/server';

export const load: ServerLoad = async ({ locals, url }) => {
	const token = locals.token;
	const search = url.searchParams.get('q') ?? '';

	try {
		const { data, meta } = await serverFetch<unknown[]>('/customers', {
			token,
			params: { q: search }
		});

		return {
			customers: data ?? [],
			meta: meta ?? {},
			filters: { search }
		};
	} catch (e) {
		return {
			customers: [],
			meta: {},
			filters: { search },
			error: e instanceof Error ? e.message : 'Failed to load customers'
		};
	}
};

export const actions: Actions = {
	create: async ({ request, locals }) => {
		const token = locals.token;
		if (!token) return fail(401, { error: 'Not authenticated' });

		const formData = await request.formData();
		const first_name = formData.get('first_name') as string;
		const last_name = formData.get('last_name') as string;
		const email = formData.get('email') as string;
		const phone = formData.get('phone') as string;
		const company_name = formData.get('company_name') as string;
		const customer_type = formData.get('customer_type') as string;

		if (!first_name?.trim()) return fail(400, { error: 'First name is required' });
		if (!last_name?.trim()) return fail(400, { error: 'Last name is required' });

		try {
			const { data } = await serverPost<{ id: string }>('/customers', {
				first_name: first_name.trim(),
				last_name: last_name.trim(),
				email: email?.trim() || null,
				phone: phone?.trim() || null,
				company_name: company_name?.trim() || null,
				customer_type: customer_type || 'residential'
			}, { token });

			redirect(303, `/dashboard/customers/${(data as { id: string }).id}`);
		} catch (e) {
			if (isRedirect(e)) throw e;
			return fail(500, { error: e instanceof Error ? e.message : 'Failed to create customer' });
		}
	}
};
