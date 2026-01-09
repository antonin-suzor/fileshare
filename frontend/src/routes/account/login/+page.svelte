<script lang="ts">
    import { page } from '$app/state';
    import { login } from '$lib/api/auth.svelte';
    import { goto } from '$app/navigation';

    let email: string = $state('');
    let password: string = $state('');

    async function handleLogin() {
        if (await login(email, password)) {
            goto(page.url.searchParams.get('redirect') ?? '/uploads');
        }
    }
</script>

<svelte:head>
    <title>Log In | FileShare</title>
</svelte:head>

<div class="flex justify-center-safe">Welcome back.</div>
<div class="flex justify-center-safe">
    <input type="text" placeholder="Email" class="input" bind:value={email} />
    <input type="text" placeholder="Password" class="input" bind:value={password} />
    <button class="btn" onclick={handleLogin}>Log In</button>
</div>
