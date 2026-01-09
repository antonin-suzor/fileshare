<script lang="ts">
    import { goto } from '$app/navigation';
    import { getCurrentUser, requireLoggedIn, clearToken } from '$lib/api/auth.svelte';
    import { axiosInstance } from '$lib/api/axios';
    import type { User } from '$lib/types';
    import { onMount } from 'svelte';

    let user = $state<User | null>(null);
    let currentPassword = $state('');
    let newPassword = $state('');
    let confirmPassword = $state('');
    let changePasswordMessage = $state('');
    let changePasswordError = $state('');
    let changePasswordLoading = $state(false);
    let deleteLoading = $state(false);
    let deleteDialog: HTMLDialogElement | undefined = $state();

    async function handleChangePassword() {
        changePasswordError = '';
        changePasswordMessage = '';

        if (!newPassword || !confirmPassword) {
            changePasswordError = 'Please fill in all password fields';
            return;
        }

        if (newPassword !== confirmPassword) {
            changePasswordError = 'Passwords do not match';
            return;
        }

        changePasswordLoading = true;
        try {
            await axiosInstance.patch(`/api/users/me/password`, {
                password: newPassword,
            });
            changePasswordMessage = 'Password changed successfully!';
            newPassword = '';
            confirmPassword = '';
            currentPassword = '';
        } catch (error: any) {
            changePasswordError = error.response?.data?.message ?? 'Failed to change password';
        } finally {
            changePasswordLoading = false;
        }
    }

    async function handleDeleteAccount() {
        deleteLoading = true;
        try {
            await axiosInstance.delete(`/api/users/me`);
            clearToken();
            await goto('/');
        } catch (error: any) {
            console.error('Failed to delete account:', error);
        } finally {
            deleteLoading = false;
            deleteDialog?.close();
        }
    }

    function openDeleteDialog() {
        deleteDialog?.showModal();
    }

    onMount(async () => {
        await requireLoggedIn();
        user = await getCurrentUser();
    });
</script>

<svelte:head>
    <title>Account Management | FileShare</title>
</svelte:head>

<div class="flex justify-center-safe">
    <div class="card w-full max-w-2xl bg-base-100 shadow-xl">
        <div class="card-body">
            <h2 class="card-title">Account Management</h2>

            {#if user}
                <!-- Account Details Section -->
                <div class="divider">Account Details</div>
                <div class="form-control w-full">
                    <label class="label" for="email">
                        <span class="label-text">Email</span>
                    </label>
                    <input id="email" type="email" value={user.email} disabled class="input-bordered input w-full" />
                </div>

                <div class="form-control w-full">
                    <label class="label" for="status">
                        <span class="label-text">Account Status</span>
                    </label>
                    <input
                        id="status"
                        type="text"
                        value={user.verified ? 'Verified' : 'Unverified'}
                        disabled
                        class="input-bordered input w-full"
                    />
                </div>

                <div class="form-control w-full">
                    <label class="label" for="created">
                        <span class="label-text">Account Created</span>
                    </label>
                    <input
                        id="created"
                        type="text"
                        value={new Date(user.created_at).toLocaleDateString()}
                        disabled
                        class="input-bordered input w-full"
                    />
                </div>

                <!-- Change Password Section -->
                <div class="divider">Change Password</div>

                {#if changePasswordError}
                    <div class="alert alert-error">
                        <span>{changePasswordError}</span>
                    </div>
                {/if}

                {#if changePasswordMessage}
                    <div class="alert alert-success">
                        <span>{changePasswordMessage}</span>
                    </div>
                {/if}

                <div class="form-control w-full">
                    <label class="label" for="current-password">
                        <span class="label-text">Current Password</span>
                    </label>
                    <input
                        id="current-password"
                        type="password"
                        placeholder="Enter current password"
                        class="input-bordered input w-full"
                        bind:value={currentPassword}
                        disabled={changePasswordLoading}
                    />
                </div>

                <div class="form-control w-full">
                    <label class="label" for="new-password">
                        <span class="label-text">New Password</span>
                    </label>
                    <input
                        id="new-password"
                        type="password"
                        placeholder="Enter new password"
                        class="input-bordered input w-full"
                        bind:value={newPassword}
                        disabled={changePasswordLoading}
                    />
                </div>

                <div class="form-control w-full">
                    <label class="label" for="confirm-password">
                        <span class="label-text">Confirm New Password</span>
                    </label>
                    <input
                        id="confirm-password"
                        type="password"
                        placeholder="Confirm new password"
                        class="input-bordered input w-full"
                        bind:value={confirmPassword}
                        disabled={changePasswordLoading}
                    />
                </div>

                <button class="btn w-full btn-primary" onclick={handleChangePassword} disabled={changePasswordLoading}>
                    {changePasswordLoading ? 'Changing Password...' : 'Change Password'}
                </button>

                <!-- Delete Account Section -->
                <div class="divider">Danger Zone</div>

                <button class="btn w-full btn-error" onclick={openDeleteDialog}> Delete Account </button>

                <!-- Delete Confirmation Modal -->
                <dialog bind:this={deleteDialog} class="modal">
                    <div class="modal-box">
                        <h3 class="text-lg font-bold text-error">Delete Account?</h3>
                        <p class="py-4">
                            This action cannot be undone. Your account will be permanently deleted. Are you absolutely
                            sure?
                        </p>
                        <div class="modal-action">
                            <form method="dialog">
                                <button class="btn" type="submit" disabled={deleteLoading}> Cancel </button>
                            </form>
                            <button class="btn btn-error" onclick={handleDeleteAccount} disabled={deleteLoading}>
                                {deleteLoading ? 'Deleting...' : 'Yes, Delete Forever'}
                            </button>
                        </div>
                    </div>
                    <form method="dialog" class="modal-backdrop">
                        <button type="submit" disabled={deleteLoading}>close</button>
                    </form>
                </dialog>
            {/if}
        </div>
    </div>
</div>
