import { z } from 'zod';

export const register = z.object({
	username: z.string().regex(new RegExp('^[a-zA-Z0-9._-]+@[a-zA-Z0-9.-]+.[a-zA-Z]{2,}$')).min(1),
	password: z.string().min(1).max(64)
});

export type Register = typeof register;
