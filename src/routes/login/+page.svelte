<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";

    async function login(email: string, password: string) {
        try {
            // const token: string = await invoke('login', { email, password });

            return true;
        } catch (error) { }
        return false;
    }

    async function form_login(e: Event) {
        e.preventDefault();

        //
        let email: string | null | undefined = 
            (document.getElementById("email") as HTMLInputElement).value;
        
        let password: string | null | undefined = 
            (document.getElementById("password") as HTMLInputElement).value;

        //
        if (email === undefined || password === undefined) {
            alert("Email or password undefined");
            return; 
        }
        if (email === null || password === null) {
            alert("Email or password null");
            return; 
        }

        //
        if (await login(email, password)) {
            window.location.href = "main";
        }
        else
            alert("Email or password incorrect");
    }
</script>

<div class="h-screen w-full">
    <p class="absolute bottom-4 left-4 select-none *:opacity-75">
        <span>
            Created by: 
        </span>
        <a href="https://github.com/rakRandom" target="_blank" class="hover:opacity-100">
            rakRandom
        </a>
    </p>

    <div class="w-[360px] mx-auto pt-20">
        <h1 class="font-semibold text-6xl text-center">
            mNeomonics
        </h1>

        <form class="flex flex-col gap-6 mt-20 p-8 regular-border">
            <div class="flex flex-col gap-1 indent-1">
                <label for="email">
                    Email
                </label>
                <input 
                    required
                    autocomplete="off"
                    type="email" 
                    id="email"
                    value="admin@admin.com"
                    class="regular-input px-2 py-1 focus:border-[var(--color-3)] mb-4">
                <label for="password">
                    Password
                </label>
                <input 
                    required
                    autocomplete="new-password"
                    type="password" 
                    id="password"
                    value="123456"
                    class="regular-input px-2 py-1 focus:border-[var(--color-3)]">
            </div>
            <hr class="regular-hr">
            <div class="flex items-center">
                <button 
                    type="submit" 
                    class="
                        block w-fit mx-auto px-8 py-1 colored-button" 
                    on:click={form_login}>
                    Login
                </button>
                <div class="relative">
                    <button type="reset" class="absolute right-0 hover:underline opacity-50">
                        reset
                    </button>
                </div>
            </div>
        </form>
    </div>
</div>

<style>
    label {
        font-size: 1.25rem;
    }

    ::-ms-reveal {
        filter: invert(100%);
    }
</style>
