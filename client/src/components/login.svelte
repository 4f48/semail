<script lang="ts">
	import * as Form from '@/components/ui/form';
	import { Input } from '@/components/ui/input';
	import { Button } from '@/components/ui/button';
	import { login, type Login } from '@/forms';
	import { type SuperValidated, type Infer, superForm } from 'sveltekit-superforms';
	import { zodClient } from 'sveltekit-superforms/adapters';

	export let data: SuperValidated<Infer<Login>>;
	const form = superForm(data, {
		validators: zodClient(login)
	});

	const { form: formData, enhance } = form;
</script>

<form method="POST" use:enhance class="flex w-[24rem] flex-col gap-2">
	<Form.Field {form} name="username">
		<Form.Control let:attrs>
			<Form.Label>Username</Form.Label>
			<Input {...attrs} bind:value={$formData.username} type="text" />
		</Form.Control>
	</Form.Field>
	<Form.Field {form} name="password">
		<Form.Control let:attrs>
			<Form.Label>Password</Form.Label>
			<Input {...attrs} bind:value={$formData.password} type="password" />
		</Form.Control>
	</Form.Field>
	<Form.Button>Log in</Form.Button>
	<p class="text-sm text-muted-foreground flex items-center gap-1 self-center">No account? <a href="/register"><Button variant="link" class="m-0 p-0">Create one</Button></a></p>
</form>
