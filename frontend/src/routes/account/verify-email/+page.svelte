<script lang="ts">
    import { page } from '$app/state';
    import { onMount } from 'svelte';

    import { login, sendVerification, signup, verify } from '$lib/api/auth.svelte';
    import { goto } from '$app/navigation';

    let email: string = $state('');
    let id: string = $state('');

    function redirect(ifNotSpecifiedThenGoTo: string = '') {
        goto(page.url.searchParams.get('redirect') ?? ifNotSpecifiedThenGoTo);
    }

    async function handleNeedVerified() {
        if (await sendVerification()) {
            alert('Email sent! Please check your email to verify your account.');
        }
    }

    onMount(async () => {
        id = page.url.searchParams.get('id') ?? '';
        if (id !== '') {
            if (await verify(id)) {
                alert('Your account has been verified successfully!');
                redirect('/uploads');
            } else {
                alert(
                    'Failed to verify your account. Please try again later or verify that you copied the URL correctly.'
                );
            }
        }
    });
</script>

<svelte:head>
    <title>Email Verification | FileShare</title>
</svelte:head>

{#if id === ''}
    <div class="flex justify-center-safe">Please enter your email to receive a new verification email.</div>
    <div class="flex justify-center-safe">
        <input type="text" placeholder="Email" class="input" bind:value={email} />
        <button class="btn" onclick={handleNeedVerified}>Receive verification email</button>
    </div>
    <div class="flex justify-center-safe">
        Once you have verified your account, to return to your content (if any), you can click here.
    </div>
    <div class="flex justify-center-safe">
        <button class="btn" onclick={() => redirect}>Return where you left off</button>
    </div>
{/if}
