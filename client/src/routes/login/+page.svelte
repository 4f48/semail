<script lang="ts">
	import type { PageData } from './$types.js';
	import Login from '#/login.svelte';
	export let data: PageData;
</script>

<div class="flex h-[100%] w-[100%] flex-col items-center justify-center gap-10">
	<h1 class="text-3xl font-semibold">Welcome back</h1>
	<Login data={data.form} />
</div>
