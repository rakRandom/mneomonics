<script lang="ts">
    const promise = new Promise(resolve => {
        setTimeout(() => {
            resolve("foo");
        }, 300);
    });

    function exitPage() {
        window.location.href = "login";
    }
</script>

<div class="flex flex-col h-screen w-full px-16 pb-4">
    <button 
        type="button" 
        class="
            absolute top-4 left-6 
            py-1 w-[75px] 
            colored-button"
        on:click={exitPage}
        >
        Exit
    </button>

    <div class="font-semibold text-5xl text-center w-full my-8">
        Collections
    </div>
    <div class="flex h-full bg-[var(--color-1)] rounded-3xl regular-border shadow-inner overflow-hidden">
        <div class="grid max-[1280px]:grid-cols-4 grid-cols-6 p-8 gap-8 h-full w-full overflow-y-auto">
            {#await promise}
                {#each Array(8) as _}
                <div 
                    class="
                        flex-1 bg-[var(--color-2)] 
                        aspect-square rounded-2xl"
                    >
                </div>
                {/each}
            {:then _}
                {#each Array(16).keys() as id}
                <a 
                    class="
                        flex-1 p-4 
                        border-2 border-[var(--color-2)] 
                        aspect-square rounded-2xl shadow-lg
                        cursor-pointer 
                        transition-transform
                        hover:border-[var(--color-3)] hover:-translate-y-0.5"
                    href="main/collection/{id}"
                    >
                    <h2 class="font-semibold text-xl text-center">
                        Collection Name
                    </h2>

                    <hr class="regular-hr my-2">

                    <div role="table" class="flex w-full px-2">
                        <ol class="w-fit">
                            <li>Flash Cards:</li>
                            <li>Times seen:</li>
                            <li>Last seen (days):</li>
                        </ol>

                        <ol class="flex-1 text-right">
                            <li>4</li>
                            <li>0</li>
                            <li>7</li>
                        </ol>
                    </div>
                </a>
                {/each}
            {/await}
        </div>
    </div>
</div>

<style>
    
</style>
