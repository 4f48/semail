import type { PageServerLoad, Actions } from './$types.js';
import { fail } from '@sveltejs/kit';
import { superValidate } from 'sveltekit-superforms';
import { register } from '@/forms';
import { zod } from 'sveltekit-superforms/adapters';

export const load: PageServerLoad = async () => {
	return {
		form: await superValidate(zod(register))
	};
};

export const actions: Actions = {
	default: async (event) => {
		const form = await superValidate(event, zod(register));
		if (!form.valid) {
			return fail(400, {
				form
			});
		}

		// const response = await fetch('http://localhost:25052/auth/register', {
		// 	method: 'POST',
		// 	headers: {
		// 		'Content-Type': 'application/json'
		// 	},
		// 	body: JSON.stringify({
		// 		username: form.data.username,
		// 		password: form.data.password
		// 	})
		// });

		register_user(form.data.username, form.data.password);

		// console.debug(response);

		return {
			form
		};
	}
};

import { SRPClient } from '@windwalker-io/srp';

// experimental srp client implementation for registering user
async function register_user(identity: string, password: string) {
	const client = SRPClient.create();
	const { salt, verifier } = await client.register(identity, password);
	console.debug(salt);
	console.debug(verifier);

	// const response = await fetch('http://localhost:25052/auth/register', {
	// 	method: 'POST',
	// 	headers: {
	// 		'Content-Type': 'application/json'
	// 	},
	// 	body: JSON.stringify({
	//		identity: identity,
	// 		salt: salt.toString(16),
	// 		verifier: verifier.toString(16)
	// 	})
	// });
}
