<script lang="ts">
    import { page } from '$app/state';
    import { onMount } from 'svelte';

    import { isVerified, sendVerification, verify } from '$lib/api/auth.svelte';
    import { goto } from '$app/navigation';

    let id: string = $state('');

    async function handleNeedVerified() {
        if (await sendVerification()) {
            alert('Email sent! Please check your email to verify your account.');
        } else {
            alert('Hmm, something went wrong... Please try again later.');
        }
    }

    onMount(async () => {
        id = page.url.searchParams.get('id') ?? '';
        if (id !== '') {
            if (await verify(id)) {
                alert("Your account's email has been verified successfully!");
                goto(page.url.searchParams.get('redirect') ?? '/uploads');
            } else {
                alert(
                    'Hmm, something went wrong... Please try again later or verify that you copied the URL correctly.'
                );
            }
        }
    });
</script>

<svelte:head>
    <title>Email Verification | FileShare</title>
</svelte:head>

{#if isVerified()}
    <div class="flex justify-center-safe">All is well ! It looks like your account's email is already verified.</div>
{:else if id === ''}
    <div class="flex justify-center-safe">
        <button class="btn btn-wide" onclick={handleNeedVerified}>Receive verification email</button>
    </div>
    <div class="flex justify-center-safe">
        <button
            class="btn btn-wide"
            onclick={() =>
                page.url.searchParams.get('redirect') !== null
                    ? goto(page.url.searchParams.get('redirect') as string)
                    : history.back()}>Return where you left off</button
        >
    </div>
{/if}
