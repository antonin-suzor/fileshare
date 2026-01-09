<script lang="ts">
    import { signup } from '$lib/api/auth.svelte';

    let email: string = $state('');
    let password: string = $state('');

    async function handleSignup() {
        if (await signup(email, password)) {
            alert('Sign up successful! Please check your email to verify your account.');
        }
    }
</script>

<svelte:head>
    <title>Sign Up | FileShare</title>
</svelte:head>

<div class="flex justify-center-safe">Glad to have you on board.</div>
<div class="flex justify-center-safe">
    <input type="text" placeholder="Email" class="input" bind:value={email} />
    <input type="text" placeholder="Password" class="input" bind:value={password} />
    <button class="btn" onclick={handleSignup}>Sign Up</button>
</div>
