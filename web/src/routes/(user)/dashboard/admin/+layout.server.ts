import { redirect } from '@sveltejs/kit';
import type { LayoutServerLoad } from './$types';
import { UserRole } from '$lib/stores/user';

export const load: LayoutServerLoad = async ({ locals }) => {
	if (!locals.user) {
		throw redirect(307, '/login');
	}

	// Admin role required
	if (locals.user.role !== UserRole.ADMIN) {
		throw redirect(307, '/dashboard');
	}
};
